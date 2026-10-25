#include "fastforward.h"

#include <stdlib.h>
#include <string.h>

// the simulation is deterministic, so if the state of the board at the start of a tape
// period equals its state at the start of an earlier period, the solution repeats with
// that period until something stops it.  instead of simulating k more periods,
// run_until_complete advances the cycle and every counter that grows with time (the
// output counts, the log of output cycles, atom grabs and collision checks) by k
// periods, and then simulates the rest normally.  k stops short of completion, the
// cycle limit and the collision check limit, so every measurement is the one the plain
// loop would give.
//
// the "state" is everything a later cycle can read: atoms with all of their flags
// (including the REMOVED | MOVED markers left behind by motion, which get_atom can
// still see), overlapped atoms, disjoint bonds, pending flag resets, and the state of
// arms, glyphs, conduits and inputs/outputs.  cells that are only marked as used area
// don't need to be compared: once a period repeats, it can't use any new area.
// pivot_parity is left out because nothing reads it.

#ifdef OMSIM_NO_FAST_FORWARD
#define USE_FAST_FORWARD 0
#else
#define USE_FAST_FORWARD 1
#endif

// the most recent states are kept in full and older ones only as hashes.  a hash
// match against an older state is confirmed one period later, by checking that the
// state then equals the current one.
#define RECENT_STATES 64
// solutions that finish this early don't benefit, so don't spend time on them.
#define FIRST_CAPTURE_CYCLE 16
#define MAXIMUM_CAPTURE_STRIDE 4096

static const atom transient_flags = CONDUIT_SHAPE | VISITED;

struct words {
    uint64_t *data;
    size_t length;
    size_t capacity;
};

struct counters {
    uint64_t cycle;
    uint64_t *outputs;
    uint64_t *output_cycles;
    uint32_t atom_grabs[NUMBER_OF_ATOM_TYPES];
    uint64_t collision_checks;
};

struct snapshot {
    struct words state;
    struct counters counters;
};

// maps state hashes (which are already well mixed) to the cycle they were seen on.
struct hash_map {
    uint64_t *keys;
    uint64_t *values;
    bool *used;
    size_t capacity;
    size_t count;
};

static void push(struct words *w, uint64_t x)
{
    if (w->length == w->capacity) {
        w->capacity = w->capacity ? w->capacity * 2 : 256;
        w->data = realloc(w->data, w->capacity * sizeof(uint64_t));
    }
    w->data[w->length++] = x;
}

static uint64_t position_key(struct vector p)
{
    return ((uint64_t)(uint32_t)p.u << 32) | (uint32_t)p.v;
}

static uint64_t mix(uint64_t x)
{
    x ^= x >> 33;
    x *= 0xff51afd7ed558ccdULL;
    x ^= x >> 33;
    x *= 0xc4ceb9fe1a85ec53ULL;
    return x ^ (x >> 33);
}

static uint64_t hash_words(const struct words *w)
{
    uint64_t h = 0x9e3779b97f4a7c15ULL;
    for (size_t i = 0; i < w->length; ++i)
        h = mix(h ^ w->data[i]) + 0x9e3779b97f4a7c15ULL;
    return h;
}

static bool words_equal(const struct words *a, const struct words *b)
{
    return a->length == b->length && !memcmp(a->data, b->data, a->length * sizeof(uint64_t));
}

static int compare_pairs(const void *aa, const void *bb)
{
    const uint64_t *a = aa;
    const uint64_t *b = bb;
    if (a[0] != b[0])
        return a[0] < b[0] ? -1 : 1;
    if (a[1] != b[1])
        return a[1] < b[1] ? -1 : 1;
    return 0;
}

static int compare_words(const void *aa, const void *bb)
{
    uint64_t a = *(const uint64_t *)aa;
    uint64_t b = *(const uint64_t *)bb;
    return a < b ? -1 : a > b;
}

static bool hash_map_get(struct hash_map *map, uint64_t key, uint64_t *value)
{
    if (!map->capacity)
        return false;
    for (size_t i = key & (map->capacity - 1); map->used[i]; i = (i + 1) & (map->capacity - 1)) {
        if (map->keys[i] == key) {
            *value = map->values[i];
            return true;
        }
    }
    return false;
}

static void hash_map_set(struct hash_map *map, uint64_t key, uint64_t value)
{
    if (2 * (map->count + 1) > map->capacity) {
        struct hash_map old = *map;
        map->capacity = old.capacity ? old.capacity * 2 : 64;
        map->keys = calloc(map->capacity, sizeof(uint64_t));
        map->values = calloc(map->capacity, sizeof(uint64_t));
        map->used = calloc(map->capacity, sizeof(bool));
        map->count = 0;
        for (size_t i = 0; i < old.capacity; ++i) {
            if (old.used[i])
                hash_map_set(map, old.keys[i], old.values[i]);
        }
        free(old.keys);
        free(old.values);
        free(old.used);
    }
    size_t i = key & (map->capacity - 1);
    while (map->used[i] && map->keys[i] != key)
        i = (i + 1) & (map->capacity - 1);
    if (!map->used[i])
        map->count++;
    map->used[i] = true;
    map->keys[i] = key;
    map->values[i] = value;
}

static bool is_output(struct input_output *io)
{
    return io->type & OUTPUT;
}

static void get_counters(struct solution *solution, struct board *board, struct counters *c)
{
    size_t n = solution->number_of_inputs_and_outputs;
    *c = (struct counters){
        .cycle = board->cycle,
        .outputs = calloc(n, sizeof(uint64_t)),
        .output_cycles = calloc(n, sizeof(uint64_t)),
        .collision_checks = board->collision_checks,
    };
    memcpy(c->atom_grabs, board->atom_grabs, sizeof(c->atom_grabs));
    for (size_t i = 0; i < n; ++i) {
        c->outputs[i] = solution->inputs_and_outputs[i].number_of_outputs;
        c->output_cycles[i] = solution->inputs_and_outputs[i].number_of_output_cycles;
    }
}

static void free_snapshot(struct snapshot *s)
{
    free(s->state.data);
    free(s->counters.outputs);
    free(s->counters.output_cycles);
    *s = (struct snapshot){ 0 };
}

static void copy_words(struct words *dst, const struct words *src)
{
    dst->length = 0;
    for (size_t i = 0; i < src->length; ++i)
        push(dst, src->data[i]);
}

// pushes (position, atom) for a cell that holds an atom or a MOVED marker.
static void capture_cell(struct words *w, struct vector p, atom a)
{
    if (!(a & VALID) || ((a & REMOVED) && !(a & MOVED)))
        return;
    push(w, position_key(p));
    push(w, a & ~transient_flags);
}

static void sort_pairs_from(struct words *w, size_t count_index)
{
    size_t n = (w->length - count_index - 1) / 2;
    qsort(w->data + count_index + 1, n, 2 * sizeof(uint64_t), compare_pairs);
    w->data[count_index] = n;
}

// writes the state of the board as a list of words, with every unordered part sorted.
// returns the number of grid slots it looked at.
static size_t capture(struct solution *solution, struct board *board, struct words *w)
{
    w->length = 0;
    size_t scanned = 0;

    // atoms.  the array part of the grid only holds atoms inside the written box.
    size_t count_index = w->length;
    push(w, 0);
    int32_t min_u = board->written_min_u < GRID_ARRAY_MIN ? GRID_ARRAY_MIN : board->written_min_u;
    int32_t max_u = board->written_max_u >= GRID_ARRAY_MAX ? GRID_ARRAY_MAX - 1 : board->written_max_u;
    int32_t min_v = board->written_min_v < GRID_ARRAY_MIN ? GRID_ARRAY_MIN : board->written_min_v;
    int32_t max_v = board->written_max_v >= GRID_ARRAY_MAX ? GRID_ARRAY_MAX - 1 : board->written_max_v;
    for (int32_t u = min_u; u <= max_u; ++u) {
        for (int32_t v = min_v; v <= max_v; ++v) {
            size_t i = (u - GRID_ARRAY_MIN) * (GRID_ARRAY_MAX - GRID_ARRAY_MIN) + v - GRID_ARRAY_MIN;
            capture_cell(w, (struct vector){ u, v }, board->grid.atoms_at_positions[i].atom);
            scanned++;
        }
    }
    for (uint32_t i = 0; i < board->grid.hash_capacity; ++i) {
        struct atom_at_position ap = board->grid.atoms_at_positions[GRID_ARRAY_PREFIX + i];
        capture_cell(w, ap.position, ap.atom);
    }
    scanned += board->grid.hash_capacity;
    sort_pairs_from(w, count_index);

    // overlapped atoms are looked up in order, so their order matters.
    push(w, board->number_of_overlapped_atoms);
    for (uint32_t i = 0; i < board->number_of_overlapped_atoms; ++i) {
        push(w, position_key(board->overlapped_atoms[i].position));
        push(w, board->overlapped_atoms[i].atom & ~transient_flags);
    }

    // disjoint bonds behave as a set; where they are in the table doesn't matter.
    count_index = w->length;
    push(w, 0);
    for (uint32_t i = 0; i < board->disjoint_bond_table.size; ++i) {
        struct disjoint_bond b = board->disjoint_bond_table.bonds[i];
        uint64_t from = position_key(b.from_position);
        uint64_t to = position_key(b.to_position);
        bool uninitialized = from == 0 && to == 0;
        bool removed = from == position_key((struct vector){ 0, 1 }) && to == from;
        if (uninitialized || removed)
            continue;
        push(w, from);
        push(w, to);
    }
    sort_pairs_from(w, count_index);

    // pending flag resets, as a set of grid slot offsets.
    count_index = w->length;
    push(w, 0);
    for (uint32_t i = 0; i < board->flag_reset_length; ++i)
        push(w, (uint64_t)((uintptr_t)board->flag_reset[i] - (uintptr_t)board->grid.atoms_at_positions));
    size_t n = w->length - count_index - 1;
    qsort(w->data + count_index + 1, n, sizeof(uint64_t), compare_words);
    size_t unique = 0;
    for (size_t i = 0; i < n; ++i) {
        uint64_t x = w->data[count_index + 1 + i];
        if (!unique || w->data[count_index + unique] != x)
            w->data[count_index + 1 + unique++] = x;
    }
    w->length = count_index + 1 + unique;
    w->data[count_index] = unique;

    for (size_t i = 0; i < solution->number_of_arms; ++i) {
        struct mechanism m = solution->arms[i];
        push(w, m.type);
        push(w, position_key(m.position));
        push(w, position_key(m.direction_u));
        push(w, position_key(m.direction_v));
        // the raw rotation: -1 and 5 point the same way but give different trig.
        push(w, (uint32_t)m.arm_rotation);
        push(w, position_key(m.movement));
        push(w, m.conversion);
    }
    for (size_t i = 0; i < solution->number_of_glyphs; ++i)
        push(w, solution->glyphs[i].conversion);
    for (size_t i = 0; i < solution->number_of_conduits; ++i) {
        struct conduit *c = &solution->conduits[i];
        push(w, c->number_of_molecules);
        size_t total = 0;
        for (uint32_t j = 0; j < c->number_of_molecules; ++j) {
            push(w, c->molecule_lengths[j]);
            total += c->molecule_lengths[j];
        }
        for (size_t j = 0; j < total; ++j) {
            push(w, position_key(c->atoms[j].position));
            push(w, c->atoms[j].atom);
        }
        push(w, c->disjoint_bond_table.count);
    }
    for (size_t i = 0; i < solution->number_of_inputs_and_outputs; ++i)
        push(w, solution->inputs_and_outputs[i].type);
    return scanned;
}

static uint64_t saturating_add(uint64_t a, uint64_t b)
{
    return a + b < a ? UINT64_MAX : a + b;
}

static bool should_continue(struct board *board, uint64_t cycle_limit)
{
    return board->cycle < cycle_limit && !board->complete && !board->collision;
}

// the largest k such that the run is still going after k more periods, or 0 if it
// shouldn't skip at all.
static uint64_t periods_to_skip(struct solution *solution, struct board *board, uint64_t cycle_limit, const struct counters *then, const struct counters *now)
{
    uint64_t t = now->cycle;
    uint64_t period = t - then->cycle;
    uint64_t k = UINT64_MAX;
    uint64_t target = solution->target_number_of_outputs;
    if (target != UINT64_MAX) {
        // the run completes once every output reaches the target.
        uint64_t k_complete = 0;
        bool incomplete = false;
        for (size_t i = 0; i < solution->number_of_inputs_and_outputs; ++i) {
            if (!is_output(&solution->inputs_and_outputs[i]))
                continue;
            uint64_t n = now->outputs[i];
            if (n >= target)
                continue;
            incomplete = true;
            uint64_t d = n - then->outputs[i];
            uint64_t kk = d == 0 ? UINT64_MAX : (target - n - 1) / d;
            if (kk > k_complete)
                k_complete = kk;
        }
        k = incomplete ? k_complete : 0;
    }
    if (cycle_limit != UINT64_MAX) {
        if ((cycle_limit - t) / period < k)
            k = (cycle_limit - t) / period;
    } else if (k == UINT64_MAX) {
        // the plain loop would never stop; don't change that.
        return 0;
    }
    uint64_t dcc = now->collision_checks - then->collision_checks;
    uint64_t cc_limit = board->collision_check_limit;
    if (cc_limit > 0 && dcc > 0) {
        uint64_t left = cc_limit > now->collision_checks ? cc_limit - now->collision_checks : 0;
        if (left / dcc < k)
            k = left / dcc;
    }
    // stay within the range of the integers that measurements are reported in.
    uint64_t room = t < INT32_MAX ? INT32_MAX - t : 0;
    if (room / period < k)
        k = room / period;
    return k;
}

static void skip_periods(struct solution *solution, struct board *board, const struct counters *then, const struct counters *now, uint64_t k)
{
    uint64_t period = now->cycle - then->cycle;
    board->cycle = now->cycle + k * period;
    board->collision_checks = now->collision_checks + k * (now->collision_checks - then->collision_checks);
    for (int i = 0; i < NUMBER_OF_ATOM_TYPES; ++i)
        board->atom_grabs[i] = now->atom_grabs[i] + (uint32_t)k * (now->atom_grabs[i] - then->atom_grabs[i]);
    for (size_t i = 0; i < solution->number_of_inputs_and_outputs; ++i) {
        struct input_output *io = &solution->inputs_and_outputs[i];
        if (!is_output(io))
            continue;
        io->number_of_outputs = now->outputs[i] + k * (now->outputs[i] - then->outputs[i]);
        // repeat the log entries of one period k times, shifted by the period.
        uint64_t first = then->output_cycles[i];
        uint64_t end = now->output_cycles[i];
        uint64_t needed = end + k * (end - first);
        if (needed > io->output_cycles_capacity) {
            io->output_cycles_capacity = needed < 6 ? 6 : needed;
            io->output_cycles = realloc(io->output_cycles, io->output_cycles_capacity * sizeof(uint64_t));
        }
        uint64_t w = end;
        for (uint64_t m = 1; m <= k; ++m) {
            for (uint64_t e = first; e < end; ++e)
                io->output_cycles[w++] = io->output_cycles[e] + m * period;
        }
        io->number_of_output_cycles = w;
    }
}

void run_until_complete(struct solution *solution, struct board *board, uint64_t cycle_limit)
{
    // states are compared at the start of a tape period, once every arm has started
    // its tape (and, for tapes that halt or don't repeat, finished it).
    uint64_t unit = solution->tape_period ? solution->tape_period : 1;
    uint64_t first_cycle = 0;
    for (size_t i = 0; i < solution->number_of_arms; ++i) {
        uint64_t start = solution->arm_tape_start_cycle[i] > 0 ? (uint64_t)solution->arm_tape_start_cycle[i] : 0;
        uint64_t ready = start;
        if (solution->arm_tape_halt_index[i] != SIZE_MAX)
            ready = saturating_add(start, solution->arm_tape_halt_index[i]);
        else if (solution->tape_period == 0)
            ready = saturating_add(start, solution->arm_tape_length[i]);
        if (ready > first_cycle)
            first_cycle = ready;
    }
    if (first_cycle < FIRST_CAPTURE_CYCLE)
        first_cycle += (FIRST_CAPTURE_CYCLE - first_cycle + unit - 1) / unit * unit;
    // skipping can't extend the log of output cycles if it stops being recorded.
    bool active = USE_FAST_FORWARD;
    for (size_t i = 0; i < solution->number_of_inputs_and_outputs; ++i) {
        if (is_output(&solution->inputs_and_outputs[i]) && solution->inputs_and_outputs[i].output_cycles_end != UINT64_MAX)
            active = false;
    }

    struct hash_map seen = { 0 };
    struct snapshot recent[RECENT_STATES] = { 0 };
    size_t number_of_recent = 0;
    size_t next_recent = 0;
    // a state to compare against on cycle `anchor_cycle`.
    struct snapshot anchor = { 0 };
    uint64_t anchor_cycle = 0;
    bool has_anchor = false;
    struct words state = { 0 };
    // capture every `stride`-th period start.  the stride doubles while a capture
    // costs more than 1/16 of the simulation since the previous one (estimated from
    // the grid slots scanned, the words captured and the collision checks done).
    uint64_t stride = 1;
    uint64_t last_capture_cycle = board->cycle;
    uint64_t last_capture_checks = board->collision_checks;
    uint64_t cost_per_cycle = 100 + 20 * (solution->number_of_arms + solution->number_of_glyphs);

    while (should_continue(board, cycle_limit)) {
        cycle(solution, board);
        uint64_t t = board->cycle;
        if (!active || t < first_cycle || (t - first_cycle) % unit != 0 || !should_continue(board, cycle_limit))
            continue;
        bool anchored = has_anchor && anchor_cycle == t;
        if (((t - first_cycle) / unit) % stride != 0 && !anchored)
            continue;
        size_t scanned = capture(solution, board, &state);
        uint64_t h = hash_words(&state);
        const struct counters *then = 0;
        if (anchored) {
            has_anchor = false;
            if (words_equal(&anchor.state, &state))
                then = &anchor.counters;
        }
        uint64_t seen_cycle;
        if (!then && hash_map_get(&seen, h, &seen_cycle)) {
            bool kept = false;
            for (size_t i = 0; i < number_of_recent; ++i) {
                if (recent[i].counters.cycle != seen_cycle)
                    continue;
                kept = true;
                if (words_equal(&recent[i].state, &state))
                    then = &recent[i].counters;
            }
            if (!kept && !has_anchor) {
                // too old to compare directly: check the state one period from now.
                free_snapshot(&anchor);
                copy_words(&anchor.state, &state);
                get_counters(solution, board, &anchor.counters);
                anchor_cycle = t + (t - seen_cycle);
                has_anchor = true;
            }
        }
        if (!then) {
            hash_map_set(&seen, h, t);
            struct snapshot *s = &recent[next_recent];
            free_snapshot(s);
            copy_words(&s->state, &state);
            get_counters(solution, board, &s->counters);
            next_recent = (next_recent + 1) % RECENT_STATES;
            if (number_of_recent < RECENT_STATES)
                number_of_recent++;
            uint64_t capture_cost = scanned + 4 * state.length;
            uint64_t simulation_cost = (board->collision_checks - last_capture_checks) / 4 + (t - last_capture_cycle) * cost_per_cycle;
            if (capture_cost * 16 > simulation_cost && stride < MAXIMUM_CAPTURE_STRIDE)
                stride *= 2;
            last_capture_cycle = t;
            last_capture_checks = board->collision_checks;
            continue;
        }
        // the state repeats: skip as many periods as the limits allow, once.
        active = false;
        struct counters now;
        get_counters(solution, board, &now);
        uint64_t k = periods_to_skip(solution, board, cycle_limit, then, &now);
        if (k > 0)
            skip_periods(solution, board, then, &now, k);
        free(now.outputs);
        free(now.output_cycles);
    }

    for (size_t i = 0; i < RECENT_STATES; ++i)
        free_snapshot(&recent[i]);
    free_snapshot(&anchor);
    free(state.data);
    free(seen.keys);
    free(seen.values);
    free(seen.used);
}
