// Generated baseline plus scripts/engine-optimizations.patch; see reports/OPTIMIZATION.md.
use crate::collision_math::{
    contact_distance_cutoff, external_collider_indices, round_hex_component,
    squared_distance_is_less_than,
};
#[allow(unused_imports)]
use crate::flags::*;
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
unsafe extern "C" {
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
    unsafe fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
    unsafe fn labs(__x: c_long) -> c_long;
    unsafe fn rs_lookup_atom_at_position(
        grid: *mut atom_grid,
        query: vector,
    ) -> *mut atom_at_position;
    unsafe fn rs_mark_used_area(board: *mut board, point: vector) -> atom;
    unsafe fn rs_lookup_chain_atom(board: *mut board, query: vector) -> u32;
    unsafe fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: u32,
        __function: *const c_char,
    ) -> !;
    unsafe fn cos(__x: f64) -> f64;
    unsafe fn sin(__x: f64) -> f64;
    unsafe fn sqrt(__x: f64) -> f64;
    unsafe fn fabs(__x: f64) -> f64;
    unsafe fn round(__x: f64) -> f64;
    unsafe fn lrint(__x: f64) -> c_long;
    unsafe fn fabsf(__x: f32) -> f32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collider_list {
    pub colliders: *mut collider,
    pub cursor: usize,
    pub length: usize,
    pub chain_atom_colliders: *mut chain_atom_collider,
    pub number_of_chain_atom_colliders: usize,
    pub bounding_box: xy_rect,
    pub bounding_box_up_to_cursor: xy_rect,
    pub rs_collision: bool,
    pub collision_location: *mut vector,
    pub ignore_board: bool,
    pub bounding_box_unused: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy_rect {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chain_atom_collider {
    pub v: xy_vector,
    pub r: xy_vector,
    pub offset_from_grab: vector,
    pub per_period_motion: vector,
    pub movement_index: usize,
    pub movement_collider_start: usize,
    pub movement_collider_end: usize,
    pub extra_periods: i32,
    pub area_direction: u32,
    pub in_repeating_segment: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy_vector {
    pub x: f32,
    pub y: f32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct collider {
    pub center: xy_vector,
    pub radius: f32,
}

static mut rs_zero_vector: vector = vector { u: 0, v: 0 };
#[inline]
unsafe extern "C" fn rs_vectors_equal(a: vector, b: vector) -> bool {
    a.u == b.u && a.v == b.v
}
static mut rs_empty_rect: xy_rect = xy_rect {
    min_x: ::core::f32::INFINITY,
    max_x: -::core::f32::INFINITY,
    min_y: ::core::f32::INFINITY,
    max_y: -::core::f32::INFINITY,
};
static mut rs_hexSizeX: f32 = 82_f32;
static mut rs_hexSizeY: f32 = 71_f32;
static mut rs_atomRadius: f32 = 29_f32;
static mut rs_producedAtomRadius: f32 = 15_f32;
static mut rs_armBaseRadius: f32 = 20_f32;
unsafe extern "C" fn rs_to_xy(p: vector) -> xy_vector {
    xy_vector {
        x: rs_hexSizeX * (p.u as f32 + 0.5f32 * p.v as f32),
        y: rs_hexSizeY * p.v as f32,
    }
}
unsafe extern "C" fn rs_from_xy(xy: xy_vector) -> vector {
    let u: f32 = xy.x / rs_hexSizeX - 0.5f32 * xy.y / rs_hexSizeY;
    let v: f32 = xy.y / rs_hexSizeY;
    let w: f32 = 0.0f32 - u - v;
    let mut ui: i32 = round_hex_component(u);
    let mut vi: i32 = round_hex_component(v);
    let wi: i32 = round_hex_component(w);
    let uf: f32 = fabsf(u - ui as f32);
    let vf: f32 = fabsf(v - vi as f32);
    let wf: f32 = fabsf(w - wi as f32);
    if uf > vf && uf > wf {
        ui = -vi - wi;
    } else if vf > wf {
        vi = -ui - wi;
    }
    vector { u: ui, v: vi }
}
#[cfg(test)]
pub(crate) fn from_xy_for_tests(x: f32, y: f32) -> (i32, i32) {
    let p = unsafe { rs_from_xy(xy_vector { x, y }) };
    (p.u, p.v)
}
#[cfg(test)]
pub(crate) fn to_xy_for_tests(u: i32, v: i32) -> (f32, f32) {
    let p = unsafe { rs_to_xy(vector { u, v }) };
    (p.x, p.y)
}
unsafe extern "C" fn rs_xy_len2(xy: xy_vector) -> f32 {
    xy.x * xy.x + xy.y * xy.y
}
unsafe extern "C" fn rs_xy_sub(a: xy_vector, b: xy_vector) -> xy_vector {
    xy_vector {
        x: a.x - b.x,
        y: a.y - b.y,
    }
}
unsafe extern "C" fn rs_xy_len(xy: xy_vector) -> f32 {
    sqrt(rs_xy_len2(xy) as f64) as f32
}
unsafe extern "C" fn rs_xy_dist(a: xy_vector, b: xy_vector) -> f32 {
    rs_xy_len(rs_xy_sub(a, b))
}

#[inline]
unsafe fn xy_distance_is_less_than(a: xy_vector, b: xy_vector, radius: f32) -> bool {
    squared_distance_is_less_than(rs_xy_len2(rs_xy_sub(a, b)), radius)
}
unsafe extern "C" fn rs_to_radians(r: i32) -> f32 {
    (r * 60_i32) as f32 * (3.14159265358979323846f64 as f32 / 180.0f32)
}
unsafe extern "C" fn rs_xy_rect_add_collider(mut rect: xy_rect, collider: collider) -> xy_rect {
    if collider.center.x - collider.radius < rect.min_x {
        rect.min_x = collider.center.x - collider.radius;
    }
    if collider.center.x + collider.radius > rect.max_x {
        rect.max_x = collider.center.x + collider.radius;
    }
    if collider.center.y - collider.radius < rect.min_y {
        rect.min_y = collider.center.y - collider.radius;
    }
    if collider.center.y + collider.radius > rect.max_y {
        rect.max_y = collider.center.y + collider.radius;
    }
    rect
}
unsafe extern "C" fn rs_xy_rect_contains_collider(rect: xy_rect, collider: collider) -> bool {
    if collider.center.x + collider.radius < rect.min_x {
        return false;
    }
    if collider.center.x - collider.radius > rect.max_x {
        return false;
    }
    if collider.center.y + collider.radius < rect.min_y {
        return false;
    }
    if collider.center.y - collider.radius > rect.max_y {
        return false;
    }
    true
}
unsafe extern "C" fn rs_xy_rect_intersects_ray(
    mut rect: xy_rect,
    origin: xy_vector,
    mut motion_delta_x: f64,
    mut motion_delta_y: f64,
    radius: f32,
) -> bool {
    if motion_delta_x < 0 as f64 {
        let tmp: f32 = rect.min_x;
        rect.min_x = -rect.max_x;
        rect.max_x = -tmp;
        motion_delta_x = -motion_delta_x;
    }
    if motion_delta_y < 0 as f64 {
        let tmp_0: f32 = rect.min_y;
        rect.min_y = -rect.max_y;
        rect.max_y = -tmp_0;
        motion_delta_y = -motion_delta_y;
    }
    let max_x: f64 = (rect.max_x + radius - origin.x) as f64;
    let max_y: f64 = (rect.max_y + radius - origin.y) as f64;
    if max_x < 0 as f64 || max_y < 0 as f64 {
        return false;
    }
    let min_x: f64 = (rect.min_x - radius - origin.x) as f64;
    let min_y: f64 = (rect.min_y - radius - origin.y) as f64;
    if min_x * motion_delta_y > max_y * motion_delta_x
        || min_y * motion_delta_x > max_x * motion_delta_y
    {
        return false;
    }
    true
}
unsafe extern "C" fn rs_mark_area_and_check_board(
    list: *mut collider_list,
    board: *mut board,
    collider: collider,
    contact_cutoff: f32,
    u: i32,
    v: i32,
) {
    let p: vector = vector { u, v };
    let center: xy_vector = rs_to_xy(p);
    if !(rs_xy_len2(rs_xy_sub(collider.center, center)) < contact_cutoff) {
        return;
    }
    let a: atom = rs_mark_used_area(board, p);
    if a as u64 & REMOVED != 0 {
        return;
    }
    (*list).rs_collision = true;
    if !(*list).collision_location.is_null() {
        *(*list).collision_location = p;
    }
}
#[inline(always)]
unsafe extern "C" fn rs_add_collider(
    list: *mut collider_list,
    board: *mut board,
    collider: collider,
) {
    if !(*list).ignore_board {
        (*board).collision_checks = (*board)
            .collision_checks
            .wrapping_add((7_i32 * 10_i32) as u64);
        let p: vector = rs_from_xy(collider.center);
        // This radius is shared by all seven neighboring board cells.
        let cutoff = contact_distance_cutoff(collider.radius + rs_atomRadius);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u, p.v);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u + 1_i32, p.v);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u, p.v + 1_i32);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u - 1_i32, p.v + 1_i32);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u - 1_i32, p.v);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u, p.v - 1_i32);
        rs_mark_area_and_check_board(list, board, collider, cutoff, p.u + 1_i32, p.v - 1_i32);
    }
    let index = (*list).length;
    (*list).length = (*list).length.wrapping_add(1);
    *(*list).colliders.add(index) = collider;
    if !(*list).bounding_box_unused {
        (*list).bounding_box = rs_xy_rect_add_collider((*list).bounding_box, collider);
    }
    if !rs_xy_rect_contains_collider((*list).bounding_box_up_to_cursor, collider) {
        return;
    }
    (*board).collision_checks =
        ((*board).collision_checks as c_ulong).wrapping_add((*list).cursor as c_ulong);
    let mut rs_collision: bool = false;
    let mut i: usize = 0_usize;
    while i < (*list).cursor {
        let other: collider = *(*list).colliders.add(i);
        rs_collision = rs_collision as i32
            | (rs_xy_dist(other.center, collider.center) < other.radius + collider.radius) as i32
            != 0;
        i = i.wrapping_add(1);
    }
    if rs_collision {
        (*list).rs_collision = true;
        if !(*list).collision_location.is_null() {
            *(*list).collision_location = rs_from_xy(collider.center);
        }
    }
}
/// rs_add_collider for a moving atom, with the board part done by `sweep`: the same
/// swept cells, each marked/checked once per atom instead of once per sample.
#[inline(always)]
unsafe fn add_atom_collider_swept(
    list: *mut collider_list,
    board: *mut board,
    collider: collider,
    sweep: &mut crate::sweep::Sweep,
    atom: usize,
    movement_index: usize,
) {
    (*board).collision_checks = (*board).collision_checks.wrapping_add(70);
    let cutoff = contact_distance_cutoff(collider.radius + rs_atomRadius);
    sweep.sample(
        atom,
        collider.center.x,
        collider.center.y,
        cutoff,
        &mut |u, v| {
            let a: atom = rs_mark_used_area(board, vector { u, v });
            a as u64 & (1u64 << 27) == 0
        },
        &mut |u, v| {
            (*list).rs_collision = true;
            if !(*list).collision_location.is_null() {
                *(*list).collision_location = vector { u, v };
            }
        },
        &|u, v| {
            let c = rs_to_xy(vector { u, v });
            (c.x, c.y)
        },
        &|x, y| {
            let p = rs_from_xy(xy_vector { x, y });
            (p.u, p.v)
        },
    );
    let index = (*list).length;
    (*list).length = (*list).length.wrapping_add(1);
    *(*list).colliders.add(index) = collider;
    if !(*list).bounding_box_unused {
        (*list).bounding_box = rs_xy_rect_add_collider((*list).bounding_box, collider);
    }
    if !rs_xy_rect_contains_collider((*list).bounding_box_up_to_cursor, collider) {
        return;
    }
    (*board).collision_checks =
        ((*board).collision_checks as c_ulong).wrapping_add((*list).cursor as c_ulong);
    // Only colliders that can be within reach of this atom's whole motion are tested;
    // the rest provably fail the distance test (see Sweep::prepare).
    let mut rs_collision: bool = false;
    let test = |i: usize| {
        let other: collider = *(*list).colliders.add(i);
        rs_xy_dist(other.center, collider.center) < other.radius + collider.radius
    };
    for &i in sweep.fixed_candidates(atom) {
        rs_collision |= test(i as usize);
    }
    for i in sweep.fixed_end()..sweep.movement_start(0).min((*list).cursor) {
        rs_collision |= test(i);
    }
    for j in 0..movement_index {
        if sweep.movements_may_touch(j, movement_index) {
            for i in sweep.movement_range(j) {
                rs_collision |= test(i);
            }
        }
    }
    if rs_collision {
        (*list).rs_collision = true;
        if !(*list).collision_location.is_null() {
            *(*list).collision_location = rs_from_xy(collider.center);
        }
    }
}
unsafe extern "C" fn rs_add_chain_atom_collider(
    list: *mut collider_list,
    board: *mut board,
    movement_index: usize,
    chain_atom: u32,
) {
    let ca: chain_atom = *(*board).chain_atoms.add(chain_atom as usize);
    let mut absolute_grab_position: vector = rs_zero_vector;
    if movement_index < (*board).movements.length {
        absolute_grab_position =
            (*(*board).movements.movements.add(movement_index)).absolute_grab_position;
    }
    let index = (*list).number_of_chain_atom_colliders;
    (*list).number_of_chain_atom_colliders = (*list).number_of_chain_atom_colliders.wrapping_add(1);
    *(*list).chain_atom_colliders.add(index) = chain_atom_collider {
        v: xy_vector { x: 0., y: 0. },
        r: xy_vector {
            x: 1_f32,
            y: 0 as f32,
        },
        offset_from_grab: vector {
            u: ca.current_position.u - absolute_grab_position.u,
            v: ca.current_position.v - absolute_grab_position.v,
        },
        per_period_motion: vector {
            u: ca.current_position.u - ca.original_position.u,
            v: ca.current_position.v - ca.original_position.v,
        },
        movement_index,
        movement_collider_start: 0,
        movement_collider_end: 0,
        extra_periods: 0,
        area_direction: ca.area_direction,
        in_repeating_segment: ca.flags & 1_u32 << 3_i32 != 0,
    };
}
unsafe extern "C" fn rs_chain_atom_center_for_period(
    collider: chain_atom_collider,
    period: i32,
) -> xy_vector {
    let xy: xy_vector = rs_to_xy(vector {
        u: collider.offset_from_grab.u + period * collider.per_period_motion.u,
        v: collider.offset_from_grab.v + period * collider.per_period_motion.v,
    });
    xy_vector {
        x: collider.v.x + (xy.x * collider.r.x - xy.y * collider.r.y),
        y: collider.v.y + (xy.x * collider.r.y + xy.y * collider.r.x),
    }
}
unsafe extern "C" fn rs_compare_vectors(a: vector, b: vector) -> i32 {
    if a.u < b.u {
        return -1_i32;
    }
    if a.u > b.u {
        return 1_i32;
    }
    if a.v < b.v {
        return -1_i32;
    }
    if a.v > b.v {
        return 1_i32;
    }
    0_i32
}
unsafe extern "C" fn rs_get_period_and_equivalence_class(
    collider: chain_atom_collider,
    period: *mut i32,
    equivalence_class: *mut vector,
) {
    if collider.per_period_motion.u != 0_i32 {
        *period = collider.offset_from_grab.u / collider.per_period_motion.u;
    } else if collider.per_period_motion.v != 0_i32 {
        *period = collider.offset_from_grab.v / collider.per_period_motion.v;
    } else {
        *period = 0_i32;
    }
    (*equivalence_class).u = collider.offset_from_grab.u - *period * collider.per_period_motion.u;
    (*equivalence_class).v = collider.offset_from_grab.v - *period * collider.per_period_motion.v;
}
unsafe extern "C" fn rs_compare_chain_atom_colliders_modulo_period(
    a: chain_atom_collider,
    b: chain_atom_collider,
    a_period: *mut i32,
    b_period: *mut i32,
) -> i32 {
    let cmp: i32 = rs_compare_vectors(a.per_period_motion, b.per_period_motion);
    if cmp != 0_i32 {
        return cmp;
    }
    let mut a_equivalence_class: vector = vector { u: 0, v: 0 };
    let mut b_equivalence_class: vector = vector { u: 0, v: 0 };
    rs_get_period_and_equivalence_class(a, a_period, &raw mut a_equivalence_class);
    rs_get_period_and_equivalence_class(b, b_period, &raw mut b_equivalence_class);
    rs_compare_vectors(a_equivalence_class, b_equivalence_class)
}
unsafe extern "C" fn rs_compare_chain_atom_colliders_within_movement(
    a: *const c_void,
    b: *const c_void,
) -> i32 {
    let mut a_period: i32 = 0;
    let mut b_period: i32 = 0;
    let cmp: i32 = rs_compare_chain_atom_colliders_modulo_period(
        *(a as *mut chain_atom_collider),
        *(b as *mut chain_atom_collider),
        &raw mut a_period,
        &raw mut b_period,
    );
    if cmp != 0_i32 {
        return cmp;
    }
    a_period - b_period
}
unsafe extern "C" fn rs_combine_chain_atoms(list: *mut collider_list, start_index: usize) {
    let count = (*list).number_of_chain_atom_colliders - start_index;
    if count > 1 {
        // The allocation belongs exclusively to this local collision list.
        // Compute the division/remainder once per element, not per comparison.
        // Stable sorting preserves equal-key order and collision diagnostics.
        let colliders =
            core::slice::from_raw_parts_mut((*list).chain_atom_colliders.add(start_index), count);
        colliders.sort_by_cached_key(|c| {
            let mut period = 0;
            let mut class = vector { u: 0, v: 0 };
            rs_get_period_and_equivalence_class(*c, &mut period, &mut class);
            (
                c.per_period_motion.u,
                c.per_period_motion.v,
                class.u,
                class.v,
                period,
            )
        });
    }
    let mut removed: usize = 0_usize;
    let mut combine_into: usize = start_index;
    let mut i: usize = start_index.wrapping_add(1_usize);
    while i < (*list).number_of_chain_atom_colliders {
        let extra_periods: i32 =
            (*(*list).chain_atom_colliders.add(combine_into)).extra_periods;
        let mut prev_period: i32 = 0;
        let mut period: i32 = 0;
        let cmp: i32 = rs_compare_chain_atom_colliders_modulo_period(
            *(*list).chain_atom_colliders.add(combine_into),
            *(*list).chain_atom_colliders.add(i),
            &raw mut prev_period,
            &raw mut period,
        );
        if cmp == 0_i32
            && period - prev_period == extra_periods + 1_i32
            && (*(*list).chain_atom_colliders.add(combine_into)).in_repeating_segment
                as i32
                != 0
        {
            (*(*list).chain_atom_colliders.add(combine_into)).extra_periods += 1;
            removed = removed.wrapping_add(1);
        } else {
            *(*list)
                .chain_atom_colliders
                .offset(i.wrapping_sub(removed) as isize) =
                *(*list).chain_atom_colliders.add(i);
            combine_into = i.wrapping_sub(removed);
        }
        i = i.wrapping_add(1);
    }
    (*list).number_of_chain_atom_colliders =
        (*list).number_of_chain_atom_colliders.wrapping_sub(removed);
}
unsafe extern "C" fn rs_get_per_period_motion(
    collider: chain_atom_collider,
    per_period_motion_x: *mut f64,
    per_period_motion_y: *mut f64,
) {
    let x: f64 = rs_hexSizeX as f64
        * (collider.per_period_motion.u as f64 + 0.5f64 * collider.per_period_motion.v as f64);
    let y: f64 = rs_hexSizeY as f64 * collider.per_period_motion.v as f64;
    *per_period_motion_x = x * collider.r.x as f64 - y * collider.r.y as f64;
    *per_period_motion_y = x * collider.r.y as f64 + y * collider.r.x as f64;
}
unsafe extern "C" fn rs_minimum_approach_period(
    motion_delta_x: f64,
    motion_delta_y: f64,
    origin_a: xy_vector,
    origin_b: xy_vector,
) -> i32 {
    let mut period: f64 = motion_delta_x * (origin_b.x - origin_a.x) as f64
        + motion_delta_y * (origin_b.y - origin_a.y) as f64;
    period /= motion_delta_x * motion_delta_x + motion_delta_y * motion_delta_y;
    if !(period >= 0 as f64) {
        period = 0 as f64;
    }
    if period > 500_f64 {
        period = 500_f64;
    }
    lrint(period) as i32
}

#[inline(never)]
unsafe extern "C" fn rs_resolve_chain_atom_collisions(list: *mut collider_list) {
    let mut i: usize = 0_usize;
    while i < (*list).number_of_chain_atom_colliders {
        let a: chain_atom_collider = *(*list).chain_atom_colliders.add(i);
        let origin_a: xy_vector = rs_chain_atom_center_for_period(a, 0_i32);
        let mut motion_ax: f64 = 0 as f64;
        let mut motion_ay: f64 = 0 as f64;
        rs_get_per_period_motion(a, &raw mut motion_ax, &raw mut motion_ay);
        let motion_length_a: f64 = sqrt(motion_ax * motion_ax + motion_ay * motion_ay);
        let perp_ax: f64 = -motion_ay / motion_length_a * rs_atomRadius as f64 * 2_f64;
        let perp_ay: f64 = motion_ax / motion_length_a * rs_atomRadius as f64 * 2_f64;
        if rs_xy_rect_intersects_ray(
            (*list).bounding_box,
            origin_a,
            motion_ax,
            motion_ay,
            rs_atomRadius,
        ) {
            // Same test order as the C loop, but never visit the collider's own
            // movement. For a long polymer that can skip thousands of atoms
            // per chain collider and collision substep.
            for j in external_collider_indices(
                (*list).length,
                a.movement_collider_start,
                a.movement_collider_end,
            ) {
                let b: collider = *(*list).colliders.add(j);
                let period: i32 =
                    rs_minimum_approach_period(motion_ax, motion_ay, origin_a, b.center);
                if xy_distance_is_less_than(
                    b.center,
                    rs_chain_atom_center_for_period(a, period),
                    b.radius + rs_atomRadius,
                ) {
                    (*list).rs_collision = true;
                    if !(*list).collision_location.is_null() {
                        *(*list).collision_location = rs_from_xy(b.center);
                    }
                }
            }
        }
        let mut j_0: usize = 0_usize;
        while j_0 < i {
            let b_0: chain_atom_collider = *(*list).chain_atom_colliders.add(j_0);
            if b_0.movement_index == a.movement_index {
                break;
            }
            let origin_b: xy_vector = rs_chain_atom_center_for_period(b_0, 0_i32);
            let mut motion_bx: f64 = 0 as f64;
            let mut motion_by: f64 = 0 as f64;
            rs_get_per_period_motion(b_0, &raw mut motion_bx, &raw mut motion_by);
            's_80: {
                if !a.in_repeating_segment && !b_0.in_repeating_segment {
                    let period_0: i32 = rs_minimum_approach_period(
                        motion_ax - motion_bx,
                        motion_ay - motion_by,
                        origin_a,
                        origin_b,
                    );
                    if xy_distance_is_less_than(
                        rs_chain_atom_center_for_period(a, period_0),
                        rs_chain_atom_center_for_period(b_0, period_0),
                        rs_atomRadius + rs_atomRadius,
                    ) {
                        (*list).rs_collision = true;
                        if !(*list).collision_location.is_null() {
                            *(*list).collision_location =
                                rs_from_xy(rs_chain_atom_center_for_period(a, period_0));
                        }
                    }
                } else {
                    let mut start: i32 = 0_i32;
                    let mut end: i32 = 500_i32;
                    let d: f64 = motion_bx * motion_ay - motion_ax * motion_by;
                    if fabs(d) < 1e-6f64 {
                        let mut closest_period: f64 = motion_ax * (origin_b.x - origin_a.x) as f64
                            + motion_ay * (origin_b.y - origin_a.y) as f64;
                        closest_period /= motion_ax * motion_ax + motion_ay * motion_ay;
                        let closest_x: f64 =
                            (origin_b.x - origin_a.x) as f64 - closest_period * motion_ax;
                        let closest_y: f64 =
                            (origin_b.y - origin_a.y) as f64 - closest_period * motion_ay;
                        if closest_x * closest_x + closest_y * closest_y
                            > (4_f32 * rs_atomRadius * rs_atomRadius) as f64
                        {
                            break 's_80;
                        }
                    } else {
                        let mut t1: f64 = (((origin_b.y - origin_a.y) as f64 - perp_ay)
                            * motion_ax
                            - ((origin_b.x - origin_a.x) as f64 - perp_ax) * motion_ay)
                            / d;
                        let mut t2: f64 = (((origin_b.y - origin_a.y) as f64 + perp_ay)
                            * motion_ax
                            - ((origin_b.x - origin_a.x) as f64 + perp_ax) * motion_ay)
                            / d;
                        if t2 < t1 {
                            let tmp: f64 = t1;
                            t1 = t2;
                            t2 = tmp;
                        }
                        if t1 > 500_f64 || t2 < 0 as f64 {
                            break 's_80;
                        } else {
                            if !(t1 >= 0 as f64) {
                                t1 = 0 as f64;
                            }
                            if !(t2 <= 500_f64) {
                                t2 = 500_f64;
                            }
                            start = t1 as i32;
                            end = t2 as i32;
                        }
                    }
                    let mut k: i32 = start;
                    while k <= end {
                        let origin_b_at_period: xy_vector = rs_chain_atom_center_for_period(b_0, k);
                        let mut period_1: i32 = rs_minimum_approach_period(
                            motion_ax,
                            motion_ay,
                            origin_a,
                            origin_b_at_period,
                        );
                        if a.in_repeating_segment as i32 != 0
                            && !b_0.in_repeating_segment
                            && period_1 > k + a.extra_periods
                        {
                            period_1 = k + a.extra_periods;
                        }
                        if !a.in_repeating_segment
                            && b_0.in_repeating_segment as i32 != 0
                            && period_1 < k - b_0.extra_periods
                        {
                            period_1 = k - b_0.extra_periods;
                        }
                        if xy_distance_is_less_than(
                            origin_b_at_period,
                            rs_chain_atom_center_for_period(a, period_1),
                            rs_atomRadius + rs_atomRadius,
                        ) {
                            (*list).rs_collision = true;
                            if !(*list).collision_location.is_null() {
                                *(*list).collision_location = rs_from_xy(origin_b_at_period);
                            }
                        }
                        k += 1;
                    }
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn rs_floor_div(a: i32, b: i32) -> i32 {
    if a == 0_i32 {
        0_i32
    } else if (a < 0_i32) as i32 == (b < 0_i32) as i32 {
        (labs(a as c_long) / labs(b as c_long)) as i32
    } else {
        (-1 as c_long - (labs(a as c_long) - 1 as c_long) / labs(b as c_long)) as i32
    }
}
unsafe extern "C" fn rs_mark_area_at_infinity_in_direction(
    d: *mut linear_area_direction,
    center: xy_vector,
    divisions: i32,
    u: i32,
    v: i32,
) {
    let mut p: vector = vector { u, v };
    let grid_center: xy_vector = rs_to_xy(p);
    if !xy_distance_is_less_than(center, grid_center, rs_atomRadius + rs_atomRadius) {
        return;
    }
    let mut period: i32 = 0_i32;
    if (*d).direction.u != 0_i32 {
        period = rs_floor_div(p.u, (*d).direction.u);
    } else {
        period = rs_floor_div(p.v, (*d).direction.v);
    }
    p.u -= period * (*d).direction.u;
    p.v -= period * (*d).direction.v;
    let ap: *mut atom_at_position =
        rs_lookup_atom_at_position(&raw mut (*d).footprint_at_infinity, p);
    if (*ap).atom & VALID == 0 || (*ap).atom >> 1_i32 > divisions as atom {
        (*ap).position = p;
        (*ap).atom = (VALID | ((divisions as atom) << 1_i32)) as atom;
    }
}
unsafe extern "C" fn rs_mark_area_at_infinity(board: *mut board, collider: chain_atom_collider) {
    if std::env::var_os("OMSIM_TRACE_INF").is_some() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            eprintln!(
                "mark_area_at_infinity called; chain_mode {}",
                (*board).chain_mode.0
            )
        });
    }
    let d: *mut linear_area_direction = (*board)
        .area_directions
        .add(collider.area_direction as usize);
    let mut divisions: i32 = 1_i32;
    if collider.per_period_motion.u != 0_i32 {
        divisions = (*d).direction.u / collider.per_period_motion.u;
    } else {
        divisions = (*d).direction.v / collider.per_period_motion.v;
    }
    let mut i: i32 = 0_i32;
    while i < divisions {
        let center: xy_vector = rs_chain_atom_center_for_period(collider, i);
        let p: vector = rs_from_xy(center);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u + 1_i32, p.v);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v + 1_i32);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u - 1_i32, p.v + 1_i32);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u - 1_i32, p.v);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u, p.v - 1_i32);
        rs_mark_area_at_infinity_in_direction(d, center, divisions, p.u + 1_i32, p.v - 1_i32);
        i += 1;
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_collision(
    solution: *mut solution,
    board: *mut board,
    increment: f32,
    collision_location: *mut vector,
) -> bool {
    if (*board).chain_mode.0 != chain_mode::DISCOVER_CHAIN.0 {
        return collision_impl(solution, board, increment, collision_location, None);
    }
    // Fast board sweep: same swept cells, same collision events and location.
    crate::sweep::SCRATCH.with(|scratch| {
        let mut sweep = scratch.borrow_mut();
        sweep.reset((*board).moving_atoms.length);
        collision_impl(
            solution,
            board,
            increment,
            collision_location,
            Some(&mut sweep),
        )
    })
}

/// Position and rotation of a movement's grabbed atoms at `progress` (verbatim from the
/// substep loop, so the cached and recomputed values are identical).
#[inline(always)]
unsafe fn movement_transform(m: &movement, progress: f32) -> (xy_vector, f32, f32) {
    let m_0 = *m;
    let mut v: xy_vector = rs_to_xy(m_0.base);
    let rotation: f32 = rs_to_radians(m_0.rotation);
    let mut armRotation: f32 = 0 as f32;
    let r: f32 = (0.0f32 - rotation) * (1.0f32 - progress);
    match m_0.r#type & 3_u32 {
        0 => {
            armRotation = rotation;
        }
        3 => {
            let tr_0: xy_vector = rs_to_xy(m_0.translation);
            v.x -= tr_0.x * (1.0f32 - progress);
            v.y -= tr_0.y * (1.0f32 - progress);
        }
        _ => {}
    }
    let baseRotation: f32 = rs_to_radians(m_0.base_rotation);
    let mut g: xy_vector = rs_to_xy(m_0.grabber_offset);
    if m_0.r#type & movement_type::IS_PISTON.0 as i32 as u32 != 0 {
        let len: f32 = rs_xy_len(g);
        let newlen: f32 =
            rs_xy_len(g) - m_0.piston_extension as f32 * rs_hexSizeX * (1.0f32 - progress);
        g.x = newlen / len * g.x;
        g.y = newlen / len * g.y;
    }
    let grabberRotation: f32 = baseRotation - armRotation * (1.0f32 - progress);
    let (grx, gry) = crate::trig_memo::cos_sin(grabberRotation);
    v.x += g.x * grx - g.y * gry;
    v.y += g.x * gry + g.y * grx;
    let (rx, ry) = crate::trig_memo::cos_sin(r);
    (v, rx, ry)
}
/// Fills the sweep's pairwise prefilter: per-substep movement transforms (the values the
/// substep loop then uses), each atom's sample bounding box, the fixed colliders that can
/// reach it, and which movement pairs can come within contact range.
/// Returns false if a sample is outside the range where the sweep's exactness argument
/// holds (|coordinate| < 2^18: float error stays far below its 0.6 px and 1 px margins);
/// the caller then uses the original path.
unsafe fn sweep_prepare(
    sw: &mut crate::sweep::Sweep,
    board: *mut board,
    increment: f32,
    offsets: *const xy_vector,
    colliders: *const collider,
    fixed: usize,
) -> bool {
    let n = (*board).movements.length;
    let natoms = (*board).moving_atoms.length;
    let pairs = &mut sw.pairs;
    pairs.movements = n;
    pairs.fixed_end = fixed;
    pairs.starts.clear();
    pairs.starts.resize(n + 1, 0);
    pairs.transforms.clear();
    const EMPTY: [f32; 4] = [
        f32::INFINITY,
        f32::NEG_INFINITY,
        f32::INFINITY,
        f32::NEG_INFINITY,
    ];
    let mut bb = std::mem::take(&mut pairs.atom_boxes);
    bb.clear();
    bb.resize(natoms, EMPTY);
    let mut progress: f32 = increment;
    while progress < 1.0f32 {
        let mut atom = 0usize;
        for i in 0..n {
            let m = &*(*board).movements.movements.add(i);
            let (v, rx, ry) = movement_transform(m, progress);
            pairs.transforms.push((v.x, v.y, rx, ry));
            for _ in 0..m.number_of_atoms {
                let xy = *offsets.add(atom);
                let cx = v.x + (xy.x * rx - xy.y * ry);
                let cy = v.y + (xy.x * ry + xy.y * rx);
                let b = &mut bb[atom];
                b[0] = b[0].min(cx);
                b[1] = b[1].max(cx);
                b[2] = b[2].min(cy);
                b[3] = b[3].max(cy);
                atom += 1;
            }
        }
        progress += increment;
    }
    const LIMIT: f32 = 262144.0;
    if !bb.iter().all(|b| b.iter().all(|c| c.abs() < LIMIT)) {
        pairs.atom_boxes = bb;
        return false;
    }
    let dist2 = |b: &[f32; 4], x: f32, y: f32| {
        let dx = (b[0] as f64 - x as f64)
            .max(x as f64 - b[1] as f64)
            .max(0.0);
        let dy = (b[2] as f64 - y as f64)
            .max(y as f64 - b[3] as f64)
            .max(0.0);
        dx * dx + dy * dy
    };
    pairs.candidates.clear();
    pairs.candidate_list.clear();
    let mut mov_bb = std::mem::take(&mut pairs.movement_boxes);
    mov_bb.clear();
    mov_bb.resize(n, EMPTY);
    let mut atom = 0usize;
    for i in 0..n {
        let m = &*(*board).movements.movements.add(i);
        for _ in 0..m.number_of_atoms {
            let b = &bb[atom];
            let start = pairs.candidate_list.len() as u32;
            for f in 0..fixed {
                let c = *colliders.add(f);
                let reach = (c.radius + rs_atomRadius) as f64 + 1.0;
                if dist2(b, c.center.x, c.center.y) < reach * reach {
                    pairs.candidate_list.push(f as u32);
                }
            }
            pairs
                .candidates
                .push((start, pairs.candidate_list.len() as u32));
            let mb = &mut mov_bb[i];
            mb[0] = mb[0].min(b[0]);
            mb[1] = mb[1].max(b[1]);
            mb[2] = mb[2].min(b[2]);
            mb[3] = mb[3].max(b[3]);
            atom += 1;
        }
    }
    let reach = (2.0 * rs_atomRadius) as f64 + 1.0;
    pairs.may_touch.clear();
    pairs.may_touch.resize(n * n, false);
    for j in 0..n {
        for i in j + 1..n {
            let (a, b) = (&mov_bb[j], &mov_bb[i]);
            let gx = (a[0] as f64 - b[1] as f64)
                .max(b[0] as f64 - a[1] as f64)
                .max(0.0);
            let gy = (a[2] as f64 - b[3] as f64)
                .max(b[2] as f64 - a[3] as f64)
                .max(0.0);
            pairs.may_touch[j * n + i] = gx * gx + gy * gy < reach * reach;
        }
    }
    pairs.atom_boxes = bb;
    pairs.movement_boxes = mov_bb;
    true
}
unsafe fn collision_impl(
    solution: *mut solution,
    board: *mut board,
    increment: f32,
    collision_location: *mut vector,
    mut sweep: Option<&mut crate::sweep::Sweep>,
) -> bool {
    let chain_mode_0: chain_mode = (*board).chain_mode;
    let occupied_slots = if chain_mode_0 == chain_mode::EXTEND_CHAIN {
        crate::board_index::occupied_slots(board.cast())
    } else {
        Vec::new()
    };
    let mut number_of_colliders: usize = (*solution)
        .number_of_arms
        .wrapping_add((*solution).number_of_cabinet_walls)
        .wrapping_add((*board).number_of_atoms_being_produced as usize)
        .wrapping_add((*board).moving_atoms.length);
    let mut number_of_chain_atom_colliders: usize = 0_usize;
    if chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0 {
        for i in occupied_slots.iter().copied() {
            let ap: atom_at_position = *(*board).grid.atoms_at_positions.add(i);
            if !(ap.atom & VALID == 0 || ap.atom & REMOVED != 0 || ap.atom & IS_CHAIN_ATOM != 0) {
                number_of_colliders = number_of_colliders.wrapping_add(1);
            }
        }
        let mut i_0: u32 = 0_u32;
        while i_0 < (*board).number_of_chain_atoms {
            let ca: *mut chain_atom = (*board).chain_atoms.add(i_0 as usize);
            if !(*ca).prev_in_list.is_null() {
                number_of_chain_atom_colliders = number_of_chain_atom_colliders.wrapping_add(1);
            }
            i_0 = i_0.wrapping_add(1);
        }
    }
    let mut list: collider_list = collider_list {
        colliders: calloc(number_of_colliders, ::core::mem::size_of::<collider>()) as *mut collider,
        cursor: 0,
        length: 0,
        chain_atom_colliders: calloc(
            number_of_chain_atom_colliders,
            ::core::mem::size_of::<chain_atom_collider>(),
        ) as *mut chain_atom_collider,
        number_of_chain_atom_colliders: 0,
        bounding_box: rs_empty_rect,
        bounding_box_up_to_cursor: rs_empty_rect,
        rs_collision: false,
        collision_location,
        ignore_board: false,
        bounding_box_unused: false,
    };
    if chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0 {
        list.ignore_board = true;
        for i_1 in occupied_slots.iter().copied() {
            let ap_0: atom_at_position = *(*board).grid.atoms_at_positions.add(i_1);
            if !(ap_0.atom & VALID == 0 || ap_0.atom & REMOVED != 0) {
                if ap_0.atom & IS_CHAIN_ATOM != 0 {
                    rs_add_chain_atom_collider(
                        &raw mut list,
                        board,
                        18446744073709551615_usize,
                        rs_lookup_chain_atom(board, ap_0.position),
                    );
                } else {
                    rs_add_collider(
                        &raw mut list,
                        board,
                        collider {
                            center: rs_to_xy(ap_0.position),
                            radius: rs_atomRadius,
                        },
                    );
                }
            }
        }
        list.ignore_board = (*board).area_growth_order.0 >= growth_order::GROWTH_LINEAR.0;
        rs_combine_chain_atoms(&raw mut list, 0_usize);
        list.cursor = list.length;
        list.bounding_box_up_to_cursor = list.bounding_box;
    }
    let fixed_chain_atom_colliders: usize = list.number_of_chain_atom_colliders;
    let mut atom_index: usize = 0_usize;
    let moving_atom_offsets: *mut xy_vector = calloc(
        (*board).moving_atoms.length,
        ::core::mem::size_of::<xy_vector>(),
    ) as *mut xy_vector;
    let mut i_2: usize = 0_usize;
    while i_2 < (*board).movements.length {
        let m: movement = *(*board).movements.movements.add(i_2);
        let mut j: usize = 0_usize;
        while j < m.number_of_atoms {
            if !(chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0
                && (*(*board)
                    .moving_atoms
                    .atoms_at_positions
                    .add(atom_index))
                .atom
                    & IS_CHAIN_ATOM
                    != 0)
            {
                let mut p: vector = (*(*board)
                    .moving_atoms
                    .atoms_at_positions
                    .add(atom_index))
                .position;
                p.u -= m.absolute_grab_position.u;
                p.v -= m.absolute_grab_position.v;
                let index = atom_index;
                atom_index = atom_index.wrapping_add(1);
                *moving_atom_offsets.add(index) = rs_to_xy(p);
            }
            j = j.wrapping_add(1);
        }
        if chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0 {
            let first_collider_index: usize = list.number_of_chain_atom_colliders;
            let mut chain: u32 = m.first_chain_atom;
            while chain != 4294967295_u32 {
                rs_add_chain_atom_collider(&raw mut list, board, i_2, chain);
                chain = (*(*board).chain_atoms.add(chain as usize)).next_in_list;
            }
            rs_combine_chain_atoms(&raw mut list, first_collider_index);
        }
        i_2 = i_2.wrapping_add(1);
    }
    let mut i_3: usize = 0_usize;
    while i_3 < (*solution).number_of_arms {
        if rs_vectors_equal(
            (*(*solution).arms.add(i_3)).movement,
            rs_zero_vector,
        ) {
            rs_add_collider(
                &raw mut list,
                board,
                collider {
                    center: rs_to_xy((*(*solution).arms.add(i_3)).position),
                    radius: rs_armBaseRadius,
                },
            );
            list.cursor = list.length;
            list.bounding_box_up_to_cursor = list.bounding_box;
        }
        i_3 = i_3.wrapping_add(1);
    }
    let mut i_4: usize = 0_usize;
    while i_4 < (*solution).number_of_cabinet_walls {
        rs_add_collider(
            &raw mut list,
            board,
            collider {
                center: rs_to_xy(*(*solution).cabinet_walls.add(i_4)),
                radius: rs_armBaseRadius,
            },
        );
        i_4 = i_4.wrapping_add(1);
    }
    let mut i_5: usize = 0_usize;
    while i_5 < (*board).number_of_atoms_being_produced as usize {
        rs_add_collider(
            &raw mut list,
            board,
            collider {
                center: rs_to_xy(*(*board).atoms_being_produced.add(i_5)),
                radius: rs_producedAtomRadius,
            },
        );
        i_5 = i_5.wrapping_add(1);
    }
    let fixed_colliders: usize = list.length;
    if chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0 {
        let mut i_6: usize = 0_usize;
        while i_6 < list.number_of_chain_atom_colliders {
            (*list.chain_atom_colliders.add(i_6)).movement_collider_start = 0_usize;
            (*list.chain_atom_colliders.add(i_6)).movement_collider_end =
                fixed_colliders;
            i_6 = i_6.wrapping_add(1);
        }
    }
    let fixed_bounding_box: xy_rect = list.bounding_box;
    if let Some(sw) = sweep.as_deref_mut()
        && !sweep_prepare(sw, board, increment, moving_atom_offsets, list.colliders, fixed_colliders)
    {
        sweep = None;
    }
    let mut k_step = 0usize;
    let mut progress: f32 = increment;
    while progress < 1.0f32 {
        list.bounding_box_unused = false;
        list.cursor = fixed_colliders;
        list.length = fixed_colliders;
        list.bounding_box = fixed_bounding_box;
        list.bounding_box_up_to_cursor = fixed_bounding_box;
        let mut i_7: usize = 0_usize;
        while i_7 < (*solution).number_of_arms {
            if !rs_vectors_equal(
                (*(*solution).arms.add(i_7)).movement,
                rs_zero_vector,
            ) {
                let mut xy: xy_vector = rs_to_xy((*(*solution).arms.add(i_7)).position);
                let tr: xy_vector = rs_to_xy((*(*solution).arms.add(i_7)).movement);
                xy.x -= tr.x * (1.0f32 - progress);
                xy.y -= tr.y * (1.0f32 - progress);
                rs_add_collider(
                    &raw mut list,
                    board,
                    collider {
                        center: xy,
                        radius: rs_armBaseRadius,
                    },
                );
                list.cursor = list.length;
                list.bounding_box_up_to_cursor = list.bounding_box;
            }
            i_7 = i_7.wrapping_add(1);
        }
        let mut atom_index_0: usize = 0_usize;
        let mut chain_atom_cursor: usize = fixed_chain_atom_colliders;
        let mut i_8: usize = 0_usize;
        while i_8 < (*board).movements.length {
            list.bounding_box_unused = i_8.wrapping_add(1_usize) == (*board).movements.length;
            let m_0: movement = *(*board).movements.movements.add(i_8);
            let (v, rx, ry) = match sweep.as_deref() {
                Some(sw) => {
                    let (x, y, rx, ry) = sw.transform(k_step, i_8);
                    (xy_vector { x, y }, rx, ry)
                }
                None => movement_transform(&m_0, progress),
            };
            if let Some(sw) = sweep.as_deref_mut() {
                sw.begin_movement(i_8, list.length);
            }
            let mut j_0: usize = 0_usize;
            while j_0 < m_0.number_of_atoms {
                if !(chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0
                    && (*(*board)
                        .moving_atoms
                        .atoms_at_positions
                        .add(atom_index_0))
                    .atom
                        & IS_CHAIN_ATOM
                        != 0)
                {
                    let c2rust_fresh1 = atom_index_0;
                    atom_index_0 = atom_index_0.wrapping_add(1);
                    let xy_0: xy_vector = *moving_atom_offsets.add(c2rust_fresh1);
                    let c = collider {
                        center: xy_vector {
                            x: v.x + (xy_0.x * rx - xy_0.y * ry),
                            y: v.y + (xy_0.x * ry + xy_0.y * rx),
                        },
                        radius: rs_atomRadius,
                    };
                    match sweep.as_deref_mut() {
                        Some(sw) => {
                            add_atom_collider_swept(&raw mut list, board, c, sw, c2rust_fresh1, i_8)
                        }
                        None => rs_add_collider(&raw mut list, board, c),
                    }
                }
                j_0 = j_0.wrapping_add(1);
            }
            while chain_atom_cursor < list.number_of_chain_atom_colliders
                && (*list.chain_atom_colliders.add(chain_atom_cursor)).movement_index
                    == i_8
            {
                (*list.chain_atom_colliders.add(chain_atom_cursor)).v = v;
                (*list.chain_atom_colliders.add(chain_atom_cursor)).r =
                    xy_vector { x: rx, y: ry };
                (*list.chain_atom_colliders.add(chain_atom_cursor))
                    .movement_collider_start = list.cursor;
                (*list.chain_atom_colliders.add(chain_atom_cursor))
                    .movement_collider_end = list.length;
                if (*board).area_growth_order.0 == growth_order::GROWTH_LINEAR.0 {
                    rs_mark_area_at_infinity(
                        board,
                        *list.chain_atom_colliders.add(chain_atom_cursor),
                    );
                }
                chain_atom_cursor = chain_atom_cursor.wrapping_add(1);
            }
            list.cursor = list.length;
            list.bounding_box_up_to_cursor = list.bounding_box;
            i_8 = i_8.wrapping_add(1);
        }
        if chain_atom_cursor == list.number_of_chain_atom_colliders {
        } else {
            __assert_fail(
                b"chain_atom_cursor == list.number_of_chain_atom_colliders\0".as_ptr()
                    as *const c_char,
                b"collision.c\0".as_ptr()
                    as *const c_char,
                669_u32,
                b"_Bool collision(struct solution *, struct board *, float, struct vector *)\0"
                    .as_ptr() as *const c_char,
            );
        };
        if chain_mode_0.0 == chain_mode::EXTEND_CHAIN.0 {
            rs_resolve_chain_atom_collisions(&raw mut list);
        }
        progress += increment;
        k_step += 1;
    }
    free(moving_atom_offsets as *mut c_void);
    free(list.colliders as *mut c_void);
    free(list.chain_atom_colliders as *mut c_void);
    list.rs_collision
}
