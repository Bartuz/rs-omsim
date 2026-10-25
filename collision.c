#include "collision.h"

#include <assert.h>
#include <math.h>
#include <stdlib.h>

// the swept cell test uses SSE2 where the compiler can't fuse a multiply and an add
// (that would make it round differently from the plain C test it replaces).
#if defined(__SSE2__) && !defined(__FMA__) && !defined(OMSIM_NO_SSE)
#define SWEPT_CELLS_SSE2 1
#include <emmintrin.h>
#else
#define SWEPT_CELLS_SSE2 0
#endif

// M_PI isn't standard, so we define it ourselves here.
#ifdef M_PI
#undef M_PI
#endif
#define M_PI 3.14159265358979323846

// only try to find collisions up to this number of periods.
#define CHAIN_ATOM_PERIOD_LIMIT 500

struct xy_vector {
    float x;
    float y;
};

struct xy_rect {
    float min_x;
    float max_x;
    float min_y;
    float max_y;
};
static const struct xy_rect empty_rect = { INFINITY, -INFINITY, INFINITY, -INFINITY };

struct collider {
    struct xy_vector center;
    float radius;
};
struct chain_atom_collider {
    struct xy_vector v;
    struct xy_vector r;
    struct vector offset_from_grab;
    struct vector per_period_motion;
    size_t movement_index;
    size_t movement_collider_start;
    size_t movement_collider_end;
    int32_t extra_periods;
    uint32_t area_direction;
    bool in_repeating_segment;
};

struct collider_list {
    struct collider *colliders;
    size_t cursor;
    size_t length;

    struct chain_atom_collider *chain_atom_colliders;
    size_t number_of_chain_atom_colliders;

    struct xy_rect bounding_box;
    struct xy_rect bounding_box_up_to_cursor;

    bool collision;
    struct vector *collision_location;

    bool ignore_board;
    bool bounding_box_unused;
};

// based on image dimensions in textures/board/hex_tile.lighting/
static const float hexSizeX = 82;
static const float hexSizeY = 71;

static const float atomRadius = 29;
static const float producedAtomRadius = 15;
static const float armBaseRadius = 20;

static struct xy_vector to_xy(struct vector p)
{
    return (struct xy_vector){
        hexSizeX * (p.u + 0.5f * p.v),
        hexSizeY * p.v,
    };
}
static struct vector from_xy(struct xy_vector xy)
{
    float u = xy.x / hexSizeX - 0.5f * xy.y / hexSizeY;
    float v = xy.y / hexSizeY;
    float w = 0.f - u - v;
    int ui = (int)round(u);
    int vi = (int)round(v);
    int wi = (int)round(w);
    // the original code uses double-precision fabs, but since the argument is
    // single-precision, fabsf should return the same result.  it's faster not
    // to have to convert to double and back.
    float uf = fabsf(u - (float)ui);
    float vf = fabsf(v - (float)vi);
    float wf = fabsf(w - (float)wi);
    if (uf > vf && uf > wf)
        ui = -vi - wi;
    else if (vf > wf)
        vi = -ui - wi;
    return (struct vector){ ui, vi };
}
static float xy_len2(struct xy_vector xy)
{
    return xy.x * xy.x + xy.y * xy.y;
}
static struct xy_vector xy_sub(struct xy_vector a, struct xy_vector b)
{
    return (struct xy_vector){ a.x - b.x, a.y - b.y };
}
static float xy_len(struct xy_vector xy)
{
    return (float)sqrt(xy_len2(xy));
}
static float xy_dist(struct xy_vector a, struct xy_vector b)
{
    return xy_len(xy_sub(a, b));
}
static float to_radians(int32_t r)
{
    return (float)(r * 60) * ((float)M_PI / 180.f);
}

static struct xy_rect xy_rect_add_collider(struct xy_rect rect, struct collider collider)
{
    if (collider.center.x - collider.radius < rect.min_x)
        rect.min_x = collider.center.x - collider.radius;
    if (collider.center.x + collider.radius > rect.max_x)
        rect.max_x = collider.center.x + collider.radius;
    if (collider.center.y - collider.radius < rect.min_y)
        rect.min_y = collider.center.y - collider.radius;
    if (collider.center.y + collider.radius > rect.max_y)
        rect.max_y = collider.center.y + collider.radius;
    return rect;
}
static bool xy_rect_contains_collider(struct xy_rect rect, struct collider collider)
{
    if (collider.center.x + collider.radius < rect.min_x)
        return false;
    if (collider.center.x - collider.radius > rect.max_x)
        return false;
    if (collider.center.y + collider.radius < rect.min_y)
        return false;
    if (collider.center.y - collider.radius > rect.max_y)
        return false;
    return true;
}

static bool xy_rect_intersects_ray(struct xy_rect rect, struct xy_vector origin, double motion_delta_x, double motion_delta_y, float radius)
{
    if (motion_delta_x < 0) {
        float tmp = rect.min_x;
        rect.min_x = -rect.max_x;
        rect.max_x = -tmp;
        motion_delta_x = -motion_delta_x;
    }
    if (motion_delta_y < 0) {
        float tmp = rect.min_y;
        rect.min_y = -rect.max_y;
        rect.max_y = -tmp;
        motion_delta_y = -motion_delta_y;
    }
    double max_x = rect.max_x + radius - origin.x;
    double max_y = rect.max_y + radius - origin.y;
    if (max_x < 0 || max_y < 0)
        return false;
    double min_x = rect.min_x - radius - origin.x;
    double min_y = rect.min_y - radius - origin.y;
    if (min_x * motion_delta_y > max_y * motion_delta_x || min_y * motion_delta_x > max_x * motion_delta_y)
        return false;
    return true;
}

static void mark_area_and_check_board(struct collider_list *list, struct board *board, struct collider collider, int32_t u, int32_t v)
{
    struct vector p = { u, v };
    struct xy_vector center = to_xy(p);
    float dist = xy_dist(collider.center, center);
    if (!(dist < collider.radius + atomRadius))
        return;
    // mark area.  also, this *could* be a collision.
    atom a = mark_used_area(board, p);
    if (a & REMOVED)
        return;
    list->collision = true;
    if (list->collision_location)
        *list->collision_location = p;
}

__attribute__((always_inline))
static inline void add_collider(struct collider_list *list, struct board *board, struct collider collider)
{
    if (!list->ignore_board) {
        // multiply by 10 to capture the difference between atom/board and atom/atom collision checks.
        board->collision_checks += 7 * 10;
        struct vector p = from_xy(collider.center);
        mark_area_and_check_board(list, board, collider, p.u, p.v);
        mark_area_and_check_board(list, board, collider, p.u + 1, p.v);
        mark_area_and_check_board(list, board, collider, p.u, p.v + 1);
        mark_area_and_check_board(list, board, collider, p.u - 1, p.v + 1);
        mark_area_and_check_board(list, board, collider, p.u - 1, p.v);
        mark_area_and_check_board(list, board, collider, p.u, p.v - 1);
        mark_area_and_check_board(list, board, collider, p.u + 1, p.v - 1);
    }
    list->colliders[list->length++] = collider;
    if (!list->bounding_box_unused)
        list->bounding_box = xy_rect_add_collider(list->bounding_box, collider);
    if (!xy_rect_contains_collider(list->bounding_box_up_to_cursor, collider))
        return;
    board->collision_checks += list->cursor;
    bool collision = false;
    for (size_t i = 0; i < list->cursor; ++i) {
        struct collider other = list->colliders[i];
        // using `|` here allows the loop to be auto-vectorized.
        collision = collision | (xy_dist(other.center, collider.center) < other.radius + collider.radius);
    }
    if (collision) {
        list->collision = true;
        if (list->collision_location)
            *list->collision_location = from_xy(collider.center);
    }
}

// a faster way to do what add_collider does for moving atoms, used outside of
// EXTEND_CHAIN mode.  compile with -DOMSIM_NO_SWEPT_CELLS to turn it off.
//
// add_collider rounds every sample of a moving atom to a hex and tests the seven cells
// around it.  samples are dense (the increment shrinks as the rotation radius grows), so
// consecutive samples of one atom keep landing in the same hex and re-testing the same
// cells.  a `swept_atom` remembers the atom's current hex, the centers of the seven
// cells around it, and which of those cells have already been marked.  the result is
// the same as calling add_collider for every sample:
// - a sample closer than 40.4 to a cell center rounds to that cell (the region that
//   from_xy rounds to a cell is a hexagon with inradius 1/hypot(1/82, 1.5/71) = 40.996),
//   so from_xy is only needed near the corners of a hex;
// - every distance test is the original test, written as a squared distance (see
//   `contact_cutoff`);
// - each cell is marked once per atom.  marking is idempotent, the board can't change
//   during a motion check, and the first mark of every cell happens at the same point
//   as before, so the hash table sees the same insertions in the same order;
// - cells that hold a non-removed atom stay "hot" and report a collision on every
//   sample that reaches them, so the collision flag and the last reported collision
//   location are unchanged.
// the pairwise tests against other colliders are unchanged too, except that colliders
// which can't come within reach of the atom at any sample are skipped (see
// prepare_sweep).  both arguments need float error far below the margins used here, so
// the fast path is only used while every coordinate is smaller than 2^18.
#ifdef OMSIM_NO_SWEPT_CELLS
#define USE_SWEPT_CELLS 0
#else
#define USE_SWEPT_CELLS 1
#endif

static const int32_t neighbor_u[7] = { 0, 1, 0, -1, -1, 0, 1 };
static const int32_t neighbor_v[7] = { 0, 0, 1, 1, 0, -1, -1 };

// the original test is `(float)sqrt(d2) < 58` with `d2` the squared distance as a
// float.  if d2 >= 58*58, the square root is at least 58.  if d2 < 58*58, then d2 is at
// most 58*58 - 2^-12, its square root is at most 58 - 2.1e-6, and that rounds down to
// the float below 58 (58 - 3.8e-6).  so the two tests agree for every d2.
static const float contact_cutoff = (29.f + 29.f) * (29.f + 29.f);
static const float same_hex_radius2 = 40.4f * 40.4f;
static const float sweep_coordinate_limit = 262144.f;
// with fewer substeps per half-cycle, samples rarely re-test a cell and the setup
// isn't worth it.
#ifndef SWEPT_CELLS_MINIMUM_SUBSTEPS
#define SWEPT_CELLS_MINIMUM_SUBSTEPS 7
#endif

struct swept_atom {
    int32_t u;
    int32_t v;
    // lane 7 is padding that never passes a test.
    float cx[8];
    float cy[8];
    // cells that passed the distance test and were marked.
    uint8_t marked;
    // marked cells that hold a non-removed atom.
    uint8_t hot;
    bool valid;
};

struct movement_transform {
    struct xy_vector v;
    float rx;
    float ry;
};

// the center of an atom at offset xy from the grab position of a movement.
__attribute__((always_inline))
static inline struct xy_vector moving_atom_center(struct movement_transform t, struct xy_vector xy)
{
    return (struct xy_vector){
        t.v.x + (xy.x * t.rx - xy.y * t.ry),
        t.v.y + (xy.x * t.ry + xy.y * t.rx),
    };
}

struct sweep {
    struct swept_atom *atoms;
    // transforms[step * number_of_movements + i] is movement i at substep `step`.
    struct movement_transform *transforms;
    size_t number_of_movements;
    size_t fixed_colliders;
    // the fixed colliders that can reach atom i are
    // candidates[candidate_start[i]] .. candidates[candidate_start[i + 1] - 1].
    uint32_t *candidates;
    size_t *candidate_start;
    // may_touch[j * number_of_movements + i]: movements j < i can come within reach.
    bool *may_touch;
    // the colliders of movement i in the current substep start at movement_start[i].
    size_t *movement_start;
};

static void swept_atom_set_hex(struct swept_atom *a, int32_t u, int32_t v)
{
    a->u = u;
    a->v = v;
    for (int i = 0; i < 7; ++i) {
        struct xy_vector c = to_xy((struct vector){ u + neighbor_u[i], v + neighbor_v[i] });
        a->cx[i] = c.x;
        a->cy[i] = c.y;
    }
    a->cx[7] = INFINITY;
    a->cy[7] = INFINITY;
    a->marked = 0;
    a->hot = 0;
    a->valid = true;
}

// bit i of *pass: cell i is within contact distance.  bit i of *inside: closer than
// the radius that guarantees rounding to cell i.
static void swept_atom_test(const struct swept_atom *a, struct xy_vector p, uint32_t *pass, uint32_t *inside)
{
#if SWEPT_CELLS_SSE2
    // xy_len2(xy_sub(p, center)) for four cells at a time: the same operations.
    __m128 x = _mm_set1_ps(p.x);
    __m128 y = _mm_set1_ps(p.y);
    __m128 dx0 = _mm_sub_ps(x, _mm_loadu_ps(a->cx));
    __m128 dy0 = _mm_sub_ps(y, _mm_loadu_ps(a->cy));
    __m128 dx1 = _mm_sub_ps(x, _mm_loadu_ps(a->cx + 4));
    __m128 dy1 = _mm_sub_ps(y, _mm_loadu_ps(a->cy + 4));
    __m128 d0 = _mm_add_ps(_mm_mul_ps(dx0, dx0), _mm_mul_ps(dy0, dy0));
    __m128 d1 = _mm_add_ps(_mm_mul_ps(dx1, dx1), _mm_mul_ps(dy1, dy1));
    __m128 cutoff = _mm_set1_ps(contact_cutoff);
    __m128 same_hex = _mm_set1_ps(same_hex_radius2);
    *pass = (uint32_t)(_mm_movemask_ps(_mm_cmplt_ps(d0, cutoff)) | _mm_movemask_ps(_mm_cmplt_ps(d1, cutoff)) << 4);
    *inside = (uint32_t)(_mm_movemask_ps(_mm_cmplt_ps(d0, same_hex)) | _mm_movemask_ps(_mm_cmplt_ps(d1, same_hex)) << 4);
#else
    uint32_t pass_bits = 0;
    uint32_t inside_bits = 0;
    for (int i = 0; i < 8; ++i) {
        float d2 = xy_len2(xy_sub(p, (struct xy_vector){ a->cx[i], a->cy[i] }));
        pass_bits |= (uint32_t)(d2 < contact_cutoff) << i;
        inside_bits |= (uint32_t)(d2 < same_hex_radius2) << i;
    }
    *pass = pass_bits;
    *inside = inside_bits;
#endif
}

static void mark_swept_cells(struct collider_list *list, struct board *board, struct swept_atom *a, struct xy_vector p)
{
    if (!a->valid) {
        struct vector h = from_xy(p);
        swept_atom_set_hex(a, h.u, h.v);
    }
    uint32_t pass, inside;
    swept_atom_test(a, p, &pass, &inside);
    if (!(inside & 1)) {
        // the sample left the disk around the current hex.  if it's in a neighbor's
        // disk, that's the hex from_xy would return; otherwise ask from_xy.
        struct vector h;
        uint32_t neighbors = inside & 0x7e;
        if (neighbors) {
            int i = 1;
            while (!(neighbors & (1u << i)))
                i++;
            h = (struct vector){ a->u + neighbor_u[i], a->v + neighbor_v[i] };
        } else
            h = from_xy(p);
        if (h.u != a->u || h.v != a->v) {
            swept_atom_set_hex(a, h.u, h.v);
            swept_atom_test(a, p, &pass, &inside);
        }
    }
    pass &= 0x7f;
    uint32_t new_cells = pass & ~(uint32_t)a->marked;
    if (!new_cells && !(pass & a->hot))
        return;
    a->marked |= new_cells;
    for (int i = 0; i < 7; ++i) {
        if (!(new_cells & (1u << i)))
            continue;
        atom found = mark_used_area(board, (struct vector){ a->u + neighbor_u[i], a->v + neighbor_v[i] });
        if (!(found & REMOVED))
            a->hot |= 1u << i;
    }
    // report collisions in the original order, so the last location is the same.
    uint32_t events = pass & a->hot;
    for (int i = 0; i < 7; ++i) {
        if (!(events & (1u << i)))
            continue;
        list->collision = true;
        if (list->collision_location)
            *list->collision_location = (struct vector){ a->u + neighbor_u[i], a->v + neighbor_v[i] };
    }
}

// add_collider for a moving atom, with the board check done by mark_swept_cells and
// the pairwise checks limited to colliders that can be in reach.
static void add_swept_atom_collider(struct collider_list *list, struct board *board, struct collider collider, struct sweep *sweep, size_t atom_index, size_t movement_index)
{
    board->collision_checks += 7 * 10;
    mark_swept_cells(list, board, &sweep->atoms[atom_index], collider.center);
    list->colliders[list->length++] = collider;
    if (!list->bounding_box_unused)
        list->bounding_box = xy_rect_add_collider(list->bounding_box, collider);
    if (!xy_rect_contains_collider(list->bounding_box_up_to_cursor, collider))
        return;
    board->collision_checks += list->cursor;
    bool collision = false;
    for (size_t k = sweep->candidate_start[atom_index]; k < sweep->candidate_start[atom_index + 1]; ++k) {
        struct collider other = list->colliders[sweep->candidates[k]];
        collision = collision | (xy_dist(other.center, collider.center) < other.radius + collider.radius);
    }
    // the bases of arms that move on tracks are added after the fixed colliders.
    size_t moving_bases_end = sweep->movement_start[0] < list->cursor ? sweep->movement_start[0] : list->cursor;
    for (size_t k = sweep->fixed_colliders; k < moving_bases_end; ++k) {
        struct collider other = list->colliders[k];
        collision = collision | (xy_dist(other.center, collider.center) < other.radius + collider.radius);
    }
    for (size_t j = 0; j < movement_index; ++j) {
        if (!sweep->may_touch[j * sweep->number_of_movements + movement_index])
            continue;
        for (size_t k = sweep->movement_start[j]; k < sweep->movement_start[j + 1]; ++k) {
            struct collider other = list->colliders[k];
            collision = collision | (xy_dist(other.center, collider.center) < other.radius + collider.radius);
        }
    }
    if (collision) {
        list->collision = true;
        if (list->collision_location)
            *list->collision_location = from_xy(collider.center);
    }
}

static void add_chain_atom_collider(struct collider_list *list, struct board *board, size_t movement_index, uint32_t chain_atom)
{
    struct chain_atom ca = board->chain_atoms[chain_atom];
    struct vector absolute_grab_position = zero_vector;
    if (movement_index < board->movements.length)
        absolute_grab_position = board->movements.movements[movement_index].absolute_grab_position;
    list->chain_atom_colliders[list->number_of_chain_atom_colliders++] = (struct chain_atom_collider){
        .r = { 1, 0 },
        .offset_from_grab = { ca.current_position.u - absolute_grab_position.u, ca.current_position.v - absolute_grab_position.v },
        .per_period_motion = { ca.current_position.u - ca.original_position.u, ca.current_position.v - ca.original_position.v },
        .movement_index = movement_index,
        .area_direction = ca.area_direction,
        .in_repeating_segment = ca.flags & CHAIN_ATOM_IN_REPEATING_SEGMENT,
    };
}

static struct xy_vector chain_atom_center_for_period(struct chain_atom_collider collider, int32_t period)
{
    struct xy_vector xy = to_xy((struct vector){
        collider.offset_from_grab.u + period * collider.per_period_motion.u,
        collider.offset_from_grab.v + period * collider.per_period_motion.v,
    });
    return (struct xy_vector){
        collider.v.x + (xy.x * collider.r.x - xy.y * collider.r.y),
        collider.v.y + (xy.x * collider.r.y + xy.y * collider.r.x),
    };
}

static int compare_vectors(struct vector a, struct vector b)
{
    if (a.u < b.u)
        return -1;
    if (a.u > b.u)
        return 1;
    if (a.v < b.v)
        return -1;
    if (a.v > b.v)
        return 1;
    return 0;
}

static void get_period_and_equivalence_class(struct chain_atom_collider collider, int32_t *period, struct vector *equivalence_class)
{
    if (collider.per_period_motion.u != 0)
        *period = collider.offset_from_grab.u / collider.per_period_motion.u;
    else if (collider.per_period_motion.v != 0)
        *period = collider.offset_from_grab.v / collider.per_period_motion.v;
    else
        *period = 0;
    equivalence_class->u = collider.offset_from_grab.u - *period * collider.per_period_motion.u;
    equivalence_class->v = collider.offset_from_grab.v - *period * collider.per_period_motion.v;
}

static int compare_chain_atom_colliders_modulo_period(struct chain_atom_collider a, struct chain_atom_collider b, int32_t *a_period, int32_t *b_period)
{
    int cmp = compare_vectors(a.per_period_motion, b.per_period_motion);
    if (cmp != 0)
        return cmp;
    struct vector a_equivalence_class, b_equivalence_class;
    get_period_and_equivalence_class(a, a_period, &a_equivalence_class);
    get_period_and_equivalence_class(b, b_period, &b_equivalence_class);
    return compare_vectors(a_equivalence_class, b_equivalence_class);
}

static int compare_chain_atom_colliders_within_movement(const void *a, const void *b)
{
    int32_t a_period, b_period;
    int cmp = compare_chain_atom_colliders_modulo_period(*(struct chain_atom_collider *)a, *(struct chain_atom_collider *)b, &a_period, &b_period);
    if (cmp != 0)
        return cmp;
    return a_period - b_period;
}

static void combine_chain_atoms(struct collider_list *list, size_t start_index)
{
    qsort(list->chain_atom_colliders + start_index, list->number_of_chain_atom_colliders - start_index, sizeof(struct chain_atom_collider), compare_chain_atom_colliders_within_movement);
    size_t removed = 0;
    size_t combine_into = start_index;
    for (size_t i = start_index + 1; i < list->number_of_chain_atom_colliders; ++i) {
        int32_t extra_periods = list->chain_atom_colliders[combine_into].extra_periods;
        int32_t prev_period, period;
        int cmp = compare_chain_atom_colliders_modulo_period(list->chain_atom_colliders[combine_into], list->chain_atom_colliders[i], &prev_period, &period);
        if (cmp == 0 && period - prev_period == extra_periods + 1 && list->chain_atom_colliders[combine_into].in_repeating_segment) {
            list->chain_atom_colliders[combine_into].extra_periods++;
            removed++;
        } else {
            list->chain_atom_colliders[i - removed] = list->chain_atom_colliders[i];
            combine_into = i - removed;
        }
    }
    list->number_of_chain_atom_colliders -= removed;
}

static void get_per_period_motion(struct chain_atom_collider collider, double *per_period_motion_x, double *per_period_motion_y)
{
    double x = (double)hexSizeX * (collider.per_period_motion.u + 0.5 * collider.per_period_motion.v);
    double y = (double)hexSizeY * collider.per_period_motion.v;
    *per_period_motion_x = x * collider.r.x - y * collider.r.y;
    *per_period_motion_y = x * collider.r.y + y * collider.r.x;
}

static int32_t minimum_approach_period(double motion_delta_x, double motion_delta_y, struct xy_vector origin_a, struct xy_vector origin_b)
{
    double period = motion_delta_x * (origin_b.x - origin_a.x) + motion_delta_y * (origin_b.y - origin_a.y);
    period /= motion_delta_x * motion_delta_x + motion_delta_y * motion_delta_y;
    if (!(period >= 0))
        period = 0;
    if (period > CHAIN_ATOM_PERIOD_LIMIT)
        period = CHAIN_ATOM_PERIOD_LIMIT;
    return lrint(period);
}

// static void print_collider(struct chain_atom_collider a)
// {
//     printf("v=(%f %f) r=(%f %f) ofg=(%d %d) ppm=(%d %d) ep=%d irs=%d\n", a.v.x, a.v.y, a.r.x, a.r.y,
//         a.offset_from_grab.u, a.offset_from_grab.v,
//         a.per_period_motion.u, a.per_period_motion.v,
//         a.extra_periods, a.in_repeating_segment);
// }

__attribute__((noinline))
static void resolve_chain_atom_collisions(struct collider_list *list)
{
    for (size_t i = 0; i < list->number_of_chain_atom_colliders; ++i) {
        struct chain_atom_collider a = list->chain_atom_colliders[i];
        struct xy_vector origin_a = chain_atom_center_for_period(a, 0);
        double motion_ax = 0;
        double motion_ay = 0;
        get_per_period_motion(a, &motion_ax, &motion_ay);
        double motion_length_a = sqrt(motion_ax * motion_ax + motion_ay * motion_ay);
        double perp_ax = -motion_ay / motion_length_a * atomRadius * 2;
        double perp_ay = motion_ax / motion_length_a * atomRadius * 2;
        // first, check for collisions between the chain atom and normal colliders.
        if (xy_rect_intersects_ray(list->bounding_box, origin_a, motion_ax, motion_ay, atomRadius)) {
            for (size_t j = 0; j < list->length; ++j) {
                if (j >= a.movement_collider_start && j < a.movement_collider_end)
                    continue;
                struct collider b = list->colliders[j];
                int32_t period = minimum_approach_period(motion_ax, motion_ay, origin_a, b.center);
                if (xy_dist(b.center, chain_atom_center_for_period(a, period)) < b.radius + atomRadius) {
                    list->collision = true;
                    if (list->collision_location)
                        *list->collision_location = from_xy(b.center);
                }
            }
        }
        // then, check for collisions between the chain atom and other chain atoms.
        for (size_t j = 0; j < i; ++j) {
            struct chain_atom_collider b = list->chain_atom_colliders[j];
            if (b.movement_index == a.movement_index)
                break;
            struct xy_vector origin_b = chain_atom_center_for_period(b, 0);
            double motion_bx = 0;
            double motion_by = 0;
            get_per_period_motion(b, &motion_bx, &motion_by);
            if (!a.in_repeating_segment && !b.in_repeating_segment) {
                // if neither atom is in a repeating segment, subtracting their motions reduces this to the stationary case.
                int32_t period = minimum_approach_period(motion_ax - motion_bx, motion_ay - motion_by, origin_a, origin_b);
                if (xy_dist(chain_atom_center_for_period(a, period), chain_atom_center_for_period(b, period)) < atomRadius + atomRadius) {
                    list->collision = true;
                    if (list->collision_location)
                        *list->collision_location = from_xy(chain_atom_center_for_period(a, period));
                }
            } else {
                // one of the two atoms is in a repeating segment.  find where the two lines of motion get close enough to interact.
                int32_t start = 0;
                int32_t end = CHAIN_ATOM_PERIOD_LIMIT;
                double d = motion_bx * motion_ay - motion_ax * motion_by;
                if (fabs(d) < 1e-6) {
                    // the lines of motion are approximately parallel.
                    // skip checking if they're further away than 2 atom radiuses.
                    double closest_period = motion_ax * (origin_b.x - origin_a.x) + motion_ay * (origin_b.y - origin_a.y);
                    closest_period /= motion_ax * motion_ax + motion_ay * motion_ay;
                    double closest_x = origin_b.x - origin_a.x - closest_period * motion_ax;
                    double closest_y = origin_b.y - origin_a.y - closest_period * motion_ay;
                    if (closest_x * closest_x + closest_y * closest_y > 4 * atomRadius * atomRadius)
                        continue;
                } else {
                    // determine the range of periods where intersections can occur.
                    double t1 = ((origin_b.y - origin_a.y - perp_ay) * motion_ax - (origin_b.x - origin_a.x - perp_ax) * motion_ay) / d;
                    double t2 = ((origin_b.y - origin_a.y + perp_ay) * motion_ax - (origin_b.x - origin_a.x + perp_ax) * motion_ay) / d;
                    if (t2 < t1) {
                        double tmp = t1;
                        t1 = t2;
                        t2 = tmp;
                    }
                    if (t1 > CHAIN_ATOM_PERIOD_LIMIT || t2 < 0)
                        continue;
                    if (!(t1 >= 0))
                        t1 = 0;
                    if (!(t2 <= CHAIN_ATOM_PERIOD_LIMIT))
                        t2 = CHAIN_ATOM_PERIOD_LIMIT;
                    start = t1;
                    end = t2;
                }
                // search the range of possible intersecting periods for a collision.
                for (int32_t k = start; k <= end; ++k) {
                    struct xy_vector origin_b_at_period = chain_atom_center_for_period(b, k);
                    int32_t period = minimum_approach_period(motion_ax, motion_ay, origin_a, origin_b_at_period);
                    if (a.in_repeating_segment && !b.in_repeating_segment && period > k + a.extra_periods)
                        period = k + a.extra_periods;
                    if (!a.in_repeating_segment && b.in_repeating_segment && period < k - b.extra_periods)
                        period = k - b.extra_periods;
                    if (xy_dist(origin_b_at_period, chain_atom_center_for_period(a, period)) < atomRadius + atomRadius) {
                        list->collision = true;
                        if (list->collision_location)
                            *list->collision_location = from_xy(origin_b_at_period);
                    }
                }
            }
        }
    }
}

// stolen from python
static int32_t floor_div(int32_t a, int32_t b)
{
    if (a == 0)
        return 0;
    else if ((a < 0) == (b < 0))
        return labs(a) / labs(b);
    else
        return -1 - (labs(a) - 1) / labs(b);
}

static void mark_area_at_infinity_in_direction(struct linear_area_direction *d, struct xy_vector center, int32_t divisions, int32_t u, int32_t v)
{
    struct vector p = { u, v };
    struct xy_vector grid_center = to_xy(p);
    float dist = xy_dist(center, grid_center);
    if (!(dist < atomRadius + atomRadius))
        return;
    int32_t period = 0;
    if (d->direction.u != 0)
        period = floor_div(p.u, d->direction.u);
    else
        period = floor_div(p.v, d->direction.v);
    p.u -= period * d->direction.u;
    p.v -= period * d->direction.v;
    struct atom_at_position *ap = lookup_atom_at_position(&d->footprint_at_infinity, p);
    if (!(ap->atom & VALID) || ((ap->atom >> 1) > divisions)) {
        ap->position = p;
        ap->atom = VALID | ((atom)divisions << 1);
    }
}

static void mark_area_at_infinity(struct board *board, struct chain_atom_collider collider)
{
    struct linear_area_direction *d = &board->area_directions[collider.area_direction];
    int32_t divisions = 1;
    if (collider.per_period_motion.u != 0)
        divisions = d->direction.u / collider.per_period_motion.u;
    else
        divisions = d->direction.v / collider.per_period_motion.v;
    for (int32_t i = 0; i < divisions; ++i) {
        struct xy_vector center = chain_atom_center_for_period(collider, i);
        struct vector p = from_xy(center);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u + 1, p.v);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v + 1);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u - 1, p.v + 1);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u - 1, p.v);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v - 1);
        mark_area_at_infinity_in_direction(d, center, divisions, p.u + 1, p.v - 1);
    }
}

// where the atoms of a movement are at `progress`: an atom at offset xy from the grab
// position is at v + xy rotated by (rx, ry).
__attribute__((always_inline))
static inline struct movement_transform movement_transform(const struct movement *m, float progress)
{
    struct xy_vector v = to_xy(m->base);
    float rotation = to_radians(m->rotation);
    float armRotation = 0;
    float r = (0.f - rotation) * (1.f - progress);
    switch (m->type & 3) {
    case SWING_MOVEMENT:
        armRotation = rotation;
        break;
    case TRACK_MOVEMENT: {
        struct xy_vector tr = to_xy(m->translation);
        v.x -= tr.x * (1.f - progress);
        v.y -= tr.y * (1.f - progress);
        break;
    }
    default:
        break;
    }
    float baseRotation = to_radians(m->base_rotation);
    struct xy_vector g = to_xy(m->grabber_offset);
    if (m->type & IS_PISTON) {
        float len = xy_len(g);
        float newlen = xy_len(g) - ((float)m->piston_extension) * hexSizeX * (1.f - progress);
        g.x = newlen / len * g.x;
        g.y = newlen / len * g.y;
    }
    float grabberRotation = baseRotation - armRotation * (1.f - progress);
    float grx = (float)cos(grabberRotation);
    float gry = (float)sin(grabberRotation);
    v.x += g.x * grx - g.y * gry;
    v.y += g.x * gry + g.y * grx;
    float rx = (float)cos(r);
    float ry = (float)sin(r);
    return (struct movement_transform){ v, rx, ry };
}

static double rect_distance2(struct xy_rect rect, struct xy_vector p)
{
    double dx = 0;
    if ((double)rect.min_x - p.x > dx)
        dx = (double)rect.min_x - p.x;
    if ((double)p.x - rect.max_x > dx)
        dx = (double)p.x - rect.max_x;
    double dy = 0;
    if ((double)rect.min_y - p.y > dy)
        dy = (double)rect.min_y - p.y;
    if ((double)p.y - rect.max_y > dy)
        dy = (double)p.y - rect.max_y;
    return dx * dx + dy * dy;
}

static double rect_gap2(struct xy_rect a, struct xy_rect b)
{
    double gx = 0;
    if ((double)a.min_x - b.max_x > gx)
        gx = (double)a.min_x - b.max_x;
    if ((double)b.min_x - a.max_x > gx)
        gx = (double)b.min_x - a.max_x;
    double gy = 0;
    if ((double)a.min_y - b.max_y > gy)
        gy = (double)a.min_y - b.max_y;
    if ((double)b.min_y - a.max_y > gy)
        gy = (double)b.min_y - a.max_y;
    return gx * gx + gy * gy;
}

static void free_sweep(struct sweep *sweep)
{
    free(sweep->atoms);
    free(sweep->transforms);
    free(sweep->candidates);
    free(sweep->candidate_start);
    free(sweep->may_touch);
    free(sweep->movement_start);
}

// computes every movement transform up front (the substep loop then reuses them), the
// box each moving atom's samples lie in, the fixed colliders that can reach that box,
// and the pairs of movements whose boxes can come within reach.  a collider that can't
// reach the box (radius sum plus a margin of 1) fails every distance test against the
// atom, so skipping it changes nothing.  returns false, with nothing allocated, if the
// sweep isn't worth it or a sample is outside the range where it is exact.
static bool prepare_sweep(struct sweep *sweep, struct board *board, float increment, const struct xy_vector *offsets, const struct collider *colliders, size_t fixed_colliders)
{
    size_t n = board->movements.length;
    size_t number_of_atoms = board->moving_atoms.length;
    size_t number_of_steps = 0;
    for (float progress = increment; progress < 1.f; progress += increment)
        number_of_steps++;
    if (number_of_steps < SWEPT_CELLS_MINIMUM_SUBSTEPS || number_of_atoms == 0)
        return false;
    *sweep = (struct sweep){
        .atoms = calloc(number_of_atoms, sizeof(struct swept_atom)),
        .transforms = calloc(number_of_steps * n, sizeof(struct movement_transform)),
        .number_of_movements = n,
        .fixed_colliders = fixed_colliders,
        .candidate_start = calloc(number_of_atoms + 1, sizeof(size_t)),
        .may_touch = calloc(n * n, sizeof(bool)),
        .movement_start = calloc(n + 1, sizeof(size_t)),
    };
    struct xy_rect *atom_boxes = calloc(number_of_atoms, sizeof(struct xy_rect));
    for (size_t i = 0; i < number_of_atoms; ++i)
        atom_boxes[i] = empty_rect;
    size_t step = 0;
    for (float progress = increment; progress < 1.f; progress += increment, ++step) {
        size_t atom_index = 0;
        for (size_t i = 0; i < n; ++i) {
            const struct movement *m = &board->movements.movements[i];
            struct movement_transform t = movement_transform(m, progress);
            sweep->transforms[step * n + i] = t;
            for (size_t j = 0; j < m->number_of_atoms; ++j) {
                struct xy_vector center = moving_atom_center(t, offsets[atom_index]);
                struct xy_rect *b = &atom_boxes[atom_index++];
                if (center.x < b->min_x)
                    b->min_x = center.x;
                if (center.x > b->max_x)
                    b->max_x = center.x;
                if (center.y < b->min_y)
                    b->min_y = center.y;
                if (center.y > b->max_y)
                    b->max_y = center.y;
            }
        }
    }
    for (size_t i = 0; i < number_of_atoms; ++i) {
        struct xy_rect b = atom_boxes[i];
        if (!(fabsf(b.min_x) < sweep_coordinate_limit && fabsf(b.max_x) < sweep_coordinate_limit
                && fabsf(b.min_y) < sweep_coordinate_limit && fabsf(b.max_y) < sweep_coordinate_limit)) {
            free(atom_boxes);
            free_sweep(sweep);
            return false;
        }
    }
    struct xy_rect *movement_boxes = calloc(n, sizeof(struct xy_rect));
    size_t candidates_capacity = 64;
    size_t number_of_candidates = 0;
    sweep->candidates = calloc(candidates_capacity, sizeof(uint32_t));
    size_t atom_index = 0;
    for (size_t i = 0; i < n; ++i) {
        movement_boxes[i] = empty_rect;
        for (size_t j = 0; j < board->movements.movements[i].number_of_atoms; ++j) {
            struct xy_rect b = atom_boxes[atom_index];
            sweep->candidate_start[atom_index] = number_of_candidates;
            for (size_t k = 0; k < fixed_colliders; ++k) {
                double reach = (double)(colliders[k].radius + atomRadius) + 1;
                if (!(rect_distance2(b, colliders[k].center) < reach * reach))
                    continue;
                if (number_of_candidates == candidates_capacity) {
                    candidates_capacity *= 2;
                    sweep->candidates = realloc(sweep->candidates, candidates_capacity * sizeof(uint32_t));
                }
                sweep->candidates[number_of_candidates++] = (uint32_t)k;
            }
            if (b.min_x < movement_boxes[i].min_x)
                movement_boxes[i].min_x = b.min_x;
            if (b.max_x > movement_boxes[i].max_x)
                movement_boxes[i].max_x = b.max_x;
            if (b.min_y < movement_boxes[i].min_y)
                movement_boxes[i].min_y = b.min_y;
            if (b.max_y > movement_boxes[i].max_y)
                movement_boxes[i].max_y = b.max_y;
            atom_index++;
        }
    }
    sweep->candidate_start[number_of_atoms] = number_of_candidates;
    double reach = (double)(atomRadius + atomRadius) + 1;
    for (size_t j = 0; j < n; ++j) {
        for (size_t i = j + 1; i < n; ++i)
            sweep->may_touch[j * n + i] = rect_gap2(movement_boxes[j], movement_boxes[i]) < reach * reach;
    }
    free(atom_boxes);
    free(movement_boxes);
    return true;
}

bool collision(struct solution *solution, struct board *board, float increment, struct vector *collision_location)
{
    enum chain_mode chain_mode = board->chain_mode;
    size_t number_of_colliders = solution->number_of_arms + solution->number_of_cabinet_walls
        + board->number_of_atoms_being_produced + board->moving_atoms.length;
    size_t number_of_chain_atom_colliders = 0;
    if (chain_mode == EXTEND_CHAIN) {
        for (size_t i = 0; i < BOARD_CAPACITY(board); ++i) {
            struct atom_at_position ap = board->grid.atoms_at_positions[i];
            if (!(ap.atom & VALID) || (ap.atom & REMOVED) || (ap.atom & IS_CHAIN_ATOM))
                continue;
            number_of_colliders++;
        }
        for (uint32_t i = 0; i < board->number_of_chain_atoms; ++i) {
            struct chain_atom *ca = &board->chain_atoms[i];
            if (ca->prev_in_list)
                number_of_chain_atom_colliders++;
        }
    }
    struct collider_list list = {
        .colliders = calloc(number_of_colliders, sizeof(struct collider)),
        .chain_atom_colliders = calloc(number_of_chain_atom_colliders, sizeof(struct chain_atom_collider)),
        .bounding_box = empty_rect,
        .bounding_box_up_to_cursor = empty_rect,
        .collision_location = collision_location,
    };
    if (chain_mode == EXTEND_CHAIN) {
        list.ignore_board = true;
        for (size_t i = 0; i < BOARD_CAPACITY(board); ++i) {
            struct atom_at_position ap = board->grid.atoms_at_positions[i];
            if (!(ap.atom & VALID) || (ap.atom & REMOVED))
                continue;
            if (ap.atom & IS_CHAIN_ATOM)
                add_chain_atom_collider(&list, board, SIZE_MAX, lookup_chain_atom(board, ap.position));
            else {
                add_collider(&list, board, (struct collider){
                    .center = to_xy(ap.position),
                    .radius = atomRadius,
                });
            }
        }
        list.ignore_board = board->area_growth_order >= GROWTH_LINEAR;
        combine_chain_atoms(&list, 0);
        list.cursor = list.length;
        list.bounding_box_up_to_cursor = list.bounding_box;
    }
    size_t fixed_chain_atom_colliders = list.number_of_chain_atom_colliders;
    size_t atom_index = 0;
    struct xy_vector *moving_atom_offsets = calloc(board->moving_atoms.length, sizeof(struct xy_vector));
    for (size_t i = 0; i < board->movements.length; ++i) {
        struct movement m = board->movements.movements[i];
        for (size_t j = 0; j < m.number_of_atoms; ++j) {
            if (chain_mode == EXTEND_CHAIN && board->moving_atoms.atoms_at_positions[atom_index].atom & IS_CHAIN_ATOM)
                continue;
            struct vector p = board->moving_atoms.atoms_at_positions[atom_index].position;
            p.u -= m.absolute_grab_position.u;
            p.v -= m.absolute_grab_position.v;
            moving_atom_offsets[atom_index++] = to_xy(p);
        }
        if (chain_mode == EXTEND_CHAIN) {
            size_t first_collider_index = list.number_of_chain_atom_colliders;
            uint32_t chain = m.first_chain_atom;
            while (chain != UINT32_MAX) {
                add_chain_atom_collider(&list, board, i, chain);
                chain = board->chain_atoms[chain].next_in_list;
            }
            combine_chain_atoms(&list, first_collider_index);
        }
    }
    for (size_t i = 0; i < solution->number_of_arms; ++i) {
        if (!vectors_equal(solution->arms[i].movement, zero_vector))
            continue;
        add_collider(&list, board, (struct collider){
            .center = to_xy(solution->arms[i].position),
            .radius = armBaseRadius,
        });
        list.cursor = list.length;
        list.bounding_box_up_to_cursor = list.bounding_box;
    }
    for (size_t i = 0; i < solution->number_of_cabinet_walls; ++i) {
        add_collider(&list, board, (struct collider){
            .center = to_xy(solution->cabinet_walls[i]),
            .radius = armBaseRadius,
        });
    }
    for (size_t i = 0; i < board->number_of_atoms_being_produced; ++i) {
        add_collider(&list, board, (struct collider){
            .center = to_xy(board->atoms_being_produced[i]),
            .radius = producedAtomRadius,
        });
    }
    size_t fixed_colliders = list.length;
    if (chain_mode == EXTEND_CHAIN) {
        for (size_t i = 0; i < list.number_of_chain_atom_colliders; ++i) {
            list.chain_atom_colliders[i].movement_collider_start = 0;
            list.chain_atom_colliders[i].movement_collider_end = fixed_colliders;
        }
    }
    struct xy_rect fixed_bounding_box = list.bounding_box;
    struct sweep sweep = { 0 };
    bool swept = USE_SWEPT_CELLS && chain_mode != EXTEND_CHAIN
        && prepare_sweep(&sweep, board, increment, moving_atom_offsets, list.colliders, fixed_colliders);
    size_t step = 0;
    for (float progress = increment; progress < 1.f; progress += increment, ++step) {
        list.bounding_box_unused = false;
        list.cursor = fixed_colliders;
        list.length = fixed_colliders;
        list.bounding_box = fixed_bounding_box;
        list.bounding_box_up_to_cursor = fixed_bounding_box;
        for (size_t i = 0; i < solution->number_of_arms; ++i) {
            if (vectors_equal(solution->arms[i].movement, zero_vector))
                continue;
            struct xy_vector xy = to_xy(solution->arms[i].position);
            struct xy_vector tr = to_xy(solution->arms[i].movement);
            xy.x -= tr.x * (1.f - progress);
            xy.y -= tr.y * (1.f - progress);
            add_collider(&list, board, (struct collider){
                .center = xy,
                .radius = armBaseRadius,
            });
            list.cursor = list.length;
            list.bounding_box_up_to_cursor = list.bounding_box;
        }
        size_t atom_index = 0;
        size_t chain_atom_cursor = fixed_chain_atom_colliders;
        for (size_t i = 0; i < board->movements.length; ++i) {
            // skip updating the bounding box for the last movement.
            list.bounding_box_unused = i + 1 == board->movements.length;
            struct movement m = board->movements.movements[i];
            struct movement_transform t;
            if (swept) {
                t = sweep.transforms[step * sweep.number_of_movements + i];
                sweep.movement_start[i] = list.length;
            } else
                t = movement_transform(&m, progress);
            struct xy_vector v = t.v;
            float rx = t.rx;
            float ry = t.ry;
            for (size_t j = 0; j < m.number_of_atoms; ++j) {
                // xx does this have a perf impact?
                if (chain_mode == EXTEND_CHAIN && board->moving_atoms.atoms_at_positions[atom_index].atom & IS_CHAIN_ATOM)
                    continue;
                struct collider collider = {
                    .center = moving_atom_center(t, moving_atom_offsets[atom_index++]),
                    .radius = atomRadius,
                };
                if (swept)
                    add_swept_atom_collider(&list, board, collider, &sweep, atom_index - 1, i);
                else
                    add_collider(&list, board, collider);
            }
            while (chain_atom_cursor < list.number_of_chain_atom_colliders && list.chain_atom_colliders[chain_atom_cursor].movement_index == i) {
                list.chain_atom_colliders[chain_atom_cursor].v = v;
                list.chain_atom_colliders[chain_atom_cursor].r = (struct xy_vector){ rx, ry };
                list.chain_atom_colliders[chain_atom_cursor].movement_collider_start = list.cursor;
                list.chain_atom_colliders[chain_atom_cursor].movement_collider_end = list.length;
                if (board->area_growth_order == GROWTH_LINEAR)
                    mark_area_at_infinity(board, list.chain_atom_colliders[chain_atom_cursor]);
                chain_atom_cursor++;
            }
            list.cursor = list.length;
            list.bounding_box_up_to_cursor = list.bounding_box;
        }
        assert(chain_atom_cursor == list.number_of_chain_atom_colliders);
        if (chain_mode == EXTEND_CHAIN)
            resolve_chain_atom_collisions(&list);

        // printf("[");
        // for (size_t i = 0; i < list.length; ++i) {
        //     if (i > 0)
        //         printf(",");
        //     printf("[%f,%f,%f]", list.colliders[i].radius, list.colliders[i].center.x, list.colliders[i].center.y);
        // }
        // printf("],");
    }
    if (swept)
        free_sweep(&sweep);
    free(moving_atom_offsets);
    free(list.colliders);
    free(list.chain_atom_colliders);
    return list.collision;
}
