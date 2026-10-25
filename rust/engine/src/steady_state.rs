#[allow(unused_imports)]
use crate::flags::*;
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_ulonglong, c_void};
unsafe extern "C" {
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
    unsafe fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
    unsafe fn labs(__x: c_long) -> c_long;
    unsafe fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
    unsafe fn memset(__s: *mut c_void, __c: i32, __n: usize) -> *mut c_void;
    unsafe fn rs_run(solution: *mut solution, board: *mut board);
    unsafe fn rs_lookup_atom_in_grid(grid: *mut atom_grid, query: vector) -> *mut atom;
    unsafe fn rs_add_chain_atom_to_table(board: *mut board, chain_atom_index: u32);
    unsafe fn rs_move_chain_atom_to_list(board: *mut board, chain_atom_index: u32, list: *mut u32);
    unsafe fn rs_polymer_feed_rate_divisor(io: *mut input_output) -> i32;
    unsafe fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: u32,
        __function: *const c_char,
    ) -> !;
    unsafe fn atan2(__y: f64, __x: f64) -> f64;
    unsafe fn sqrt(__x: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct snapshot {
    pub arms: *mut mechanism,
    pub grid: atom_grid,
    pub number_of_atoms: u64,
    pub cycle: u64,
    pub output_count: *mut u64,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chain_swing {
    pub direction: vector,
    pub length_squared: i32,
    pub sextant: i32,
}
#[inline]
unsafe extern "C" fn rs_vectors_equal(a: vector, b: vector) -> bool {
    a.u == b.u && a.v == b.v
}
#[inline]
unsafe extern "C" fn rs_cycle(solution: *mut solution, board: *mut board) {
    rs_run(solution, board);
}
#[inline]
unsafe extern "C" fn rs_position_may_be_visible_to_solution(
    solution: *mut solution,
    p: vector,
) -> bool {
    p.u >= (*solution).min_visible_u
        && p.u <= (*solution).max_visible_u
        && p.v >= (*solution).min_visible_v
        && p.v <= (*solution).max_visible_v
}
static mut rs_growth_directions: [vector; 12] = [
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 0_i32, v: 1_i32 },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: -1_i32,
    },
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
];
unsafe extern "C" fn rs_take_snapshot(
    solution: *mut solution,
    board: *mut board,
    snapshot: *mut snapshot,
) {
    (*snapshot).arms = realloc(
        (*snapshot).arms as *mut c_void,
        ::core::mem::size_of::<mechanism>().wrapping_mul((*solution).number_of_arms),
    ) as *mut mechanism;
    memcpy(
        (*snapshot).arms as *mut c_void,
        (*solution).arms as *const c_void,
        ::core::mem::size_of::<mechanism>().wrapping_mul((*solution).number_of_arms),
    );
    (*snapshot).grid.atoms_at_positions = realloc(
        (*snapshot).grid.atoms_at_positions as *mut c_void,
        ::core::mem::size_of::<atom_at_position>().wrapping_mul(
            (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
                .wrapping_add((*board).grid.hash_capacity) as usize,
        ),
    ) as *mut atom_at_position;
    memcpy(
        (*snapshot).grid.atoms_at_positions as *mut c_void,
        (*board).grid.atoms_at_positions as *const c_void,
        ::core::mem::size_of::<atom_at_position>().wrapping_mul(
            (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
                .wrapping_add((*board).grid.hash_capacity) as usize,
        ),
    );
    (*snapshot).grid.hash_capacity = (*board).grid.hash_capacity;
    (*snapshot).cycle = (*board).cycle;
    (*snapshot).number_of_atoms = 0_u64;
    (*board).number_of_chain_atoms = 0_u32;
    let mut i: u32 = 0_u32;
    while i
        < (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
            .wrapping_add((*board).grid.hash_capacity)
    {
        let a: *mut atom_at_position = (*board).grid.atoms_at_positions.offset(i as isize);
        if !((*a).atom & VALID == 0 || (*a).atom & REMOVED != 0) {
            if !rs_position_may_be_visible_to_solution(solution, (*a).position) {
                (*a).atom = ((*a).atom | IS_CHAIN_ATOM) as atom;
                (*board).number_of_chain_atoms = (*board).number_of_chain_atoms.wrapping_add(1);
            }
            (*snapshot).number_of_atoms = (*snapshot).number_of_atoms.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    (*snapshot).output_count = realloc(
        (*snapshot).output_count as *mut c_void,
        ::core::mem::size_of::<u64>().wrapping_mul((*solution).number_of_inputs_and_outputs),
    ) as *mut u64;
    let mut i_0: usize = 0_usize;
    while i_0 < (*solution).number_of_inputs_and_outputs {
        *(*snapshot).output_count.offset(i_0 as isize) =
            (*(*solution).inputs_and_outputs.offset(i_0 as isize)).number_of_outputs;
        i_0 = i_0.wrapping_add(1);
    }
    (*board).chain_atoms = realloc(
        (*board).chain_atoms as *mut c_void,
        ::core::mem::size_of::<chain_atom>().wrapping_mul((*board).number_of_chain_atoms as usize),
    ) as *mut chain_atom;
    (*board).chain_atom_table_size = 1_u32;
    while (*board).chain_atom_table_size < (*board).number_of_chain_atoms {
        (*board).chain_atom_table_size = (*board).chain_atom_table_size.wrapping_mul(2_u32);
    }
    (*board).chain_atom_table = realloc(
        (*board).chain_atom_table as *mut c_void,
        ::core::mem::size_of::<u32>().wrapping_mul((*board).chain_atom_table_size as usize),
    ) as *mut u32;
    memset(
        (*board).chain_atom_table as *mut c_void,
        0xff_i32,
        ::core::mem::size_of::<u32>().wrapping_mul((*board).chain_atom_table_size as usize),
    );
    let mut chain_atom_index: u32 = 0_u32;
    let mut i_1: u32 = 0_u32;
    while i_1
        < (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
            .wrapping_add((*board).grid.hash_capacity)
    {
        let a_0: *mut atom_at_position = (*board).grid.atoms_at_positions.offset(i_1 as isize);
        if !((*a_0).atom & VALID == 0
            || (*a_0).atom & REMOVED != 0
            || (*a_0).atom & IS_CHAIN_ATOM == 0)
        {
            *(*board).chain_atoms.offset(chain_atom_index as isize) = chain_atom {
                prev_in_list: ::core::ptr::null_mut::<u32>(),
                next_in_list: 4294967295_u32,
                flags: 0,
                current_position: (*a_0).position,
                original_position: (*a_0).position,
                area_direction: 0,
            };
            rs_add_chain_atom_to_table(board, chain_atom_index);
            chain_atom_index = chain_atom_index.wrapping_add(1);
        }
        i_1 = i_1.wrapping_add(1);
    }
}
unsafe extern "C" fn rs_arm_directions_equivalent(a: *mut mechanism, b: *mut mechanism) -> bool {
    if (*a).direction_u.u == (*b).direction_u.u && (*a).direction_u.v == (*b).direction_u.v {
        true
    } else if (*a).r#type as c_ulong & SIX_ARM != 0 {
        true
    } else if (*a).r#type as c_ulong & TWO_ARM != 0 {
        (*a).direction_u.u == -(*b).direction_u.u && (*a).direction_u.v == -(*b).direction_u.v
    } else if (*a).r#type as c_ulong & THREE_ARM != 0 {
        -(*a).direction_u.u + (*a).direction_v.u == (*b).direction_u.u
            && -(*a).direction_u.v + (*a).direction_v.v == (*b).direction_u.v
            || -(*a).direction_v.u == (*b).direction_u.u
                && -(*a).direction_v.v == (*b).direction_u.v
    } else {
        false
    }
}
unsafe extern "C" fn rs_check_snapshot(
    solution: *mut solution,
    board: *mut board,
    snapshot: *mut snapshot,
) -> bool {
    if (*board).rs_collision {
        return false;
    }
    let mut i: u32 = 0_u32;
    while (i as usize) < (*solution).number_of_arms {
        let mut a: mechanism = *(*snapshot).arms.offset(i as isize);
        let mut b: mechanism = *(*solution).arms.offset(i as isize);
        if a.position.u != b.position.u
            || a.position.v != b.position.v
            || !rs_arm_directions_equivalent(&raw mut a, &raw mut b)
        {
            return false;
        }
        i = i.wrapping_add(1);
    }
    let mut number_of_atoms: u64 = 0_u64;
    let mut i_0: u32 = 0_u32;
    while i_0
        < (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
            .wrapping_add((*board).grid.hash_capacity)
    {
        let a_0: atom = (*(*board).grid.atoms_at_positions.offset(i_0 as isize)).atom;
        if !(a_0 as u64 & VALID == 0 || a_0 as u64 & REMOVED != 0) {
            if a_0 as u64 & IS_CHAIN_ATOM == 0 {
                let b_0: atom = *rs_lookup_atom_in_grid(
                    &raw mut (*snapshot).grid,
                    (*(*board).grid.atoms_at_positions.offset(i_0 as isize)).position,
                );
                if b_0 as u64 & VALID == 0 || b_0 as u64 & REMOVED != 0 {
                    return false;
                }
                if a_0 as u64
                    & (0x3f_u64 << 35_i32
                        | (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32)
                        | (SALT
                            | (WATER | FIRE | EARTH | AIR)
                            | QUICKSILVER
                            | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                            | VITAE
                            | MORS
                            | QUINTESSENCE)
                        | 0x1f_u64.wrapping_mul(GRABBED_ONCE))
                    != b_0 as u64
                        & (0x3f_u64 << 35_i32
                            | (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32)
                            | (SALT
                                | (WATER | FIRE | EARTH | AIR)
                                | QUICKSILVER
                                | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                                | VITAE
                                | MORS
                                | QUINTESSENCE)
                            | 0x1f_u64.wrapping_mul(GRABBED_ONCE))
                {
                    return false;
                }
                number_of_atoms = number_of_atoms.wrapping_add(1);
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: u32 = 0_u32;
    while i_1 < (*board).number_of_chain_atoms {
        let ca: *mut chain_atom = (*board).chain_atoms.offset(i_1 as isize);
        if !(*ca).prev_in_list.is_null() {
            if (*ca).flags & 7_u32 != 0_u32 {
                return false;
            }
            let original: atom =
                *rs_lookup_atom_in_grid(&raw mut (*board).grid, (*ca).original_position);
            if original as u64 & VALID != 0
                && original as u64 & REMOVED == 0
                && original as u64 & IS_CHAIN_ATOM == 0
            {
                (*ca).flags = (*ca).flags | 1_u32 << 3_i32;
            } else {
                (*ca).flags = (*ca).flags & !(1_u32 << 3_i32);
                number_of_atoms = number_of_atoms.wrapping_add(1);
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    if number_of_atoms != (*snapshot).number_of_atoms {
        return false;
    }
    true
}
unsafe extern "C" fn rs_destroy_snapshot(snapshot: *mut snapshot) {
    free((*snapshot).arms as *mut c_void);
    free((*snapshot).grid.atoms_at_positions as *mut c_void);
    free((*snapshot).output_count as *mut c_void);
}
unsafe extern "C" fn rs_max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
unsafe extern "C" fn rs_dot(p: vector, q: vector) -> f64 {
    p.u as f64 * (q.u as f64 + 0.5f64 * q.v as f64)
        + p.v as f64 * (q.v as f64 + 0.5f64 * q.u as f64)
}
unsafe extern "C" fn rs_length_squared(p: vector) -> i32 {
    p.u * p.u + p.v * p.v + p.v * p.u
}
unsafe extern "C" fn rs_gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0_u64 {
        let t: u64 = b;
        b = a.wrapping_rem(b);
        a = t;
    }
    a
}
unsafe extern "C" fn rs_chain_atom_motion(ca: *mut chain_atom) -> vector {
    vector {
        u: (*ca).current_position.u - (*ca).original_position.u,
        v: (*ca).current_position.v - (*ca).original_position.v,
    }
}
unsafe extern "C" fn rs_chain_atom_direction(ca: *mut chain_atom) -> vector {
    let motion: vector = rs_chain_atom_motion(ca);
    let divisor: i32 = rs_gcd(
        labs(motion.u as c_long) as u64,
        labs(motion.v as c_long) as u64,
    ) as i32;
    vector {
        u: motion.u / divisor,
        v: motion.v / divisor,
    }
}
unsafe extern "C" fn rs_compare_chain_atoms_by_direction(
    a: *const c_void,
    b: *const c_void,
) -> i32 {
    let a_dir: vector = rs_chain_atom_direction(*(a as *const *mut chain_atom));
    let b_dir: vector = rs_chain_atom_direction(*(b as *const *mut chain_atom));
    if a_dir.u - b_dir.u != 0 {
        return a_dir.u - b_dir.u;
    }
    a_dir.v - b_dir.v
}
unsafe extern "C" fn rs_compare_chain_swings_by_direction(a: chain_swing, b: chain_swing) -> i32 {
    if a.sextant != b.sextant {
        return a.sextant - b.sextant;
    }
    a.direction.v * b.direction.u - b.direction.v * a.direction.u
}
unsafe extern "C" fn rs_compare_chain_swings(aa: *const c_void, bb: *const c_void) -> i32 {
    let a: chain_swing = *(aa as *const chain_swing);
    let b: chain_swing = *(bb as *const chain_swing);
    let order: i32 = rs_compare_chain_swings_by_direction(a, b);
    if order != 0_i32 {
        return order;
    }
    b.length_squared - a.length_squared
}
unsafe extern "C" fn rs_end_of_swing(mut a: chain_swing) -> chain_swing {
    a.sextant += 1;
    a
}
unsafe extern "C" fn rs_measure_quadratic_swing_area(
    starting_point: *mut chain_swing,
    length_squared: i32,
    start: chain_swing,
    end: chain_swing,
) -> f64 {
    if end.sextant == 0_i32 {
        return 0 as f64;
    }
    if (*starting_point).sextant == 0_i32 {
        *starting_point = end;
        (*starting_point).sextant += 6_i32;
        return 0 as f64;
    }
    let mut delta: chain_swing = chain_swing {
        direction: vector {
            u: start.direction.u * end.direction.u
                + start.direction.v * end.direction.u
                + start.direction.v * end.direction.v,
            v: start.direction.u * end.direction.v - start.direction.v * end.direction.u,
        },
        length_squared,
        sextant: end.sextant - start.sextant,
    };
    if delta.direction.u <= 0_i32 || delta.direction.v < 0_i32 {
        delta.sextant -= 1;
        let tmp: i32 = delta.direction.u;
        delta.direction.u = -delta.direction.v;
        delta.direction.v += tmp;
    }
    if delta.sextant == 1_i32 && delta.direction.v == 0_i32 {
        return length_squared as f64;
    }
    if delta.sextant == 0_i32 {
    } else {
        __assert_fail(
            b"delta.sextant == 0\0".as_ptr() as *const c_char,
            b"steady-state.c\0"
                .as_ptr() as *const c_char,
            250_u32,
            b"double measure_quadratic_swing_area(struct chain_swing *, int32_t, struct chain_swing, struct chain_swing)\0"
                .as_ptr() as *const c_char,
        );
    };
    atan2(
        0.8660254037844386f64 * delta.direction.v as f64,
        delta.direction.u as f64 + 0.5f64 * delta.direction.v as f64,
    ) / 3.14159265358979323846f64
        * length_squared as f64
        * 3_f64
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_run_until_steady_state(
    solution: *mut solution,
    board: *mut board,
    cycle_limit: u64,
) -> steady_state {
    let mut snapshot: snapshot = snapshot {
        arms: ::core::ptr::null_mut::<mechanism>(),
        grid: atom_grid {
            atoms_at_positions: ::core::ptr::null_mut::<atom_at_position>(),
            hash_capacity: 0,
            board: ::core::ptr::null_mut::<board>(),
        },
        number_of_atoms: 0,
        cycle: 0,
        output_count: ::core::ptr::null_mut::<u64>(),
    };
    let mut check_period: u64 = (*solution).tape_period;
    if check_period == 0_u64 {
        check_period = 1_u64;
    }
    let mut next_snapshot_cycle: u64 = check_period.wrapping_mul(
        1_u64.wrapping_add(
            (*board)
                .cycle
                .wrapping_add(check_period)
                .wrapping_sub(1_u64)
                .wrapping_div(check_period),
        ),
    );
    let mut snapshot_period: u64 = next_snapshot_cycle;
    let mut i: u32 = 0_u32;
    while (i as usize) < (*solution).number_of_arms {
        let mut start_cycle: u64 = *(*solution).arm_tape_start_cycle.offset(i as isize) as u64;
        if *(*solution).arm_tape_halt_index.offset(i as isize) != 18446744073709551615_usize {
            start_cycle = (*(*solution).arm_tape_start_cycle.offset(i as isize) as usize)
                .wrapping_add(*(*solution).arm_tape_halt_index.offset(i as isize))
                as u64;
        }
        let period_aligned_start_cycle: u64 = check_period.wrapping_mul(
            start_cycle
                .wrapping_add(check_period)
                .wrapping_sub(1_u64)
                .wrapping_div(check_period),
        );
        if period_aligned_start_cycle > next_snapshot_cycle {
            next_snapshot_cycle = period_aligned_start_cycle;
        }
        i = i.wrapping_add(1);
    }
    let mut disable_check_until_next_snapshot: bool = true;
    while (*board).cycle < cycle_limit && !(*board).rs_collision {
        if !disable_check_until_next_snapshot
            && (*board).cycle.wrapping_rem(check_period) == 0_u64
            && rs_check_snapshot(solution, board, &raw mut snapshot) as i32 != 0
        {
            let repetition_period_length: u64 = (*board).cycle.wrapping_sub(snapshot.cycle);
            let mut result: steady_state = steady_state {
                eventual_behavior: eventual_behavior::EVENTUALLY_ENTERS_STEADY_STATE,
                number_of_cycles: (*board).cycle.wrapping_sub(snapshot.cycle),
                number_of_outputs: if (*solution).number_of_inputs_and_outputs != 0 {
                    18446744073709551615_u64
                } else {
                    0_u64
                },
                outputs_repeat_after_cycle: (*board).cycle,
                area_growth_order: growth_order::GROWTH_NONE,
                linear_area_growth: 0,
                quadratic_area_growth: 0.,
                directional_growth: [0.; 12],
                pivot_parity: false,
            };
            let mut i_0: u32 = 0_u32;
            while (i_0 as usize) < (*solution).number_of_arms {
                let a: mechanism = *snapshot.arms.offset(i_0 as isize);
                let b: mechanism = *(*solution).arms.offset(i_0 as isize);
                result.pivot_parity = a.pivot_parity != b.pivot_parity;
                if result.pivot_parity {
                    break;
                }
                i_0 = i_0.wrapping_add(1);
            }
            let mut i_1: usize = 0_usize;
            while i_1 < (*solution).number_of_inputs_and_outputs {
                if (*(*solution).inputs_and_outputs.offset(i_1 as isize))
                    .r#type
                    .0
                    & input_output_type::SINGLE_OUTPUT.0
                    != 0
                {
                    let outputs: u64 = (*(*solution).inputs_and_outputs.offset(i_1 as isize))
                        .number_of_outputs
                        .wrapping_sub(*snapshot.output_count.offset(i_1 as isize));
                    if outputs < result.number_of_outputs {
                        result.number_of_outputs = outputs;
                    }
                }
                i_1 = i_1.wrapping_add(1);
            }
            let mut number_of_active_chain_atoms: u32 = 0_u32;
            let mut number_of_swinging_chain_atoms: u32 = 0_u32;
            let mut i_2: u32 = 0_u32;
            while i_2 < (*board).number_of_chain_atoms {
                let ca: chain_atom = *(*board).chain_atoms.offset(i_2 as isize);
                if !ca.prev_in_list.is_null() {
                    if ca.current_position.u == ca.original_position.u
                        && ca.current_position.v == ca.original_position.v
                    {
                        let a_0: *mut atom =
                            rs_lookup_atom_in_grid(&raw mut (*board).grid, ca.current_position);
                        *a_0 = (*a_0 & !(IS_CHAIN_ATOM)) as atom;
                        rs_move_chain_atom_to_list(board, i_2, ::core::ptr::null_mut::<u32>());
                    } else {
                        number_of_active_chain_atoms = number_of_active_chain_atoms.wrapping_add(1);
                        if ca.flags & 63_u32 << 4_i32 != 0 {
                            number_of_swinging_chain_atoms =
                                number_of_swinging_chain_atoms.wrapping_add(1);
                        }
                    }
                }
                i_2 = i_2.wrapping_add(1);
            }
            if number_of_active_chain_atoms == 0_u32 {
                (*board).area_growth_order = growth_order::GROWTH_NONE;
            } else if number_of_swinging_chain_atoms == 0_u32 {
                (*board).area_growth_order = growth_order::GROWTH_LINEAR;
                let mut i_3: u32 = 0_u32;
                while i_3 < (*board).number_of_area_directions {
                    free(
                        (*(*board).area_directions.offset(i_3 as isize))
                            .footprint_at_infinity
                            .atoms_at_positions as *mut c_void,
                    );
                    i_3 = i_3.wrapping_add(1);
                }
                free((*board).area_directions as *mut c_void);
                let chain_atoms_by_direction: *mut *mut chain_atom = calloc(
                    number_of_active_chain_atoms as usize,
                    ::core::mem::size_of::<*mut chain_atom>(),
                )
                    as *mut *mut chain_atom;
                number_of_active_chain_atoms = 0_u32;
                let mut i_4: u32 = 0_u32;
                while i_4 < (*board).number_of_chain_atoms {
                    let ca_0: *mut chain_atom = (*board).chain_atoms.offset(i_4 as isize);
                    if !(*ca_0).prev_in_list.is_null() {
                        let c2rust_fresh0 = number_of_active_chain_atoms;
                        number_of_active_chain_atoms = number_of_active_chain_atoms.wrapping_add(1);
                        *chain_atoms_by_direction.offset(c2rust_fresh0 as isize) = ca_0;
                    }
                    i_4 = i_4.wrapping_add(1);
                }
                qsort(
                    chain_atoms_by_direction as *mut c_void,
                    number_of_active_chain_atoms as usize,
                    ::core::mem::size_of::<*mut chain_atom>(),
                    Some(
                        rs_compare_chain_atoms_by_direction
                            as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
                    ),
                );
                (*board).number_of_area_directions = 0_u32;
                let mut previous_direction: vector = vector { u: 0, v: 0 };
                let mut i_5: u32 = 0_u32;
                while i_5 < number_of_active_chain_atoms {
                    let direction: vector =
                        rs_chain_atom_direction(*chain_atoms_by_direction.offset(i_5 as isize));
                    if i_5 == 0_u32 || !rs_vectors_equal(previous_direction, direction) {
                        (*board).number_of_area_directions =
                            (*board).number_of_area_directions.wrapping_add(1);
                        previous_direction = direction;
                    }
                    i_5 = i_5.wrapping_add(1);
                }
                (*board).area_directions = calloc(
                    (*board).number_of_area_directions as usize,
                    ::core::mem::size_of::<linear_area_direction>(),
                ) as *mut linear_area_direction;
                (*board).number_of_area_directions = 0_u32;
                let mut i_6: u32 = 0_u32;
                while i_6 < number_of_active_chain_atoms {
                    let direction_0: vector =
                        rs_chain_atom_direction(*chain_atoms_by_direction.offset(i_6 as isize));
                    if i_6 == 0_u32 || !rs_vectors_equal(previous_direction, direction_0) {
                        let c2rust_fresh1 = (*board).number_of_area_directions;
                        (*board).number_of_area_directions =
                            (*board).number_of_area_directions.wrapping_add(1);
                        (*(*board).area_directions.offset(c2rust_fresh1 as isize)).direction =
                            direction_0;
                        previous_direction = direction_0;
                    }
                    let area_direction: u32 =
                        (*board).number_of_area_directions.wrapping_sub(1_u32);
                    (**chain_atoms_by_direction.offset(i_6 as isize)).area_direction =
                        area_direction;
                    let mut multiplier: i32 = 1_i32;
                    if direction_0.u != 0_i32 {
                        multiplier = labs(
                            ((**chain_atoms_by_direction.offset(i_6 as isize))
                                .current_position
                                .u
                                - (**chain_atoms_by_direction.offset(i_6 as isize))
                                    .original_position
                                    .u) as c_long,
                        ) as i32;
                        multiplier /= rs_gcd(
                            multiplier as u64,
                            labs(
                                (*(*board).area_directions.offset(area_direction as isize))
                                    .direction
                                    .u as c_long,
                            ) as u64,
                        ) as i32;
                    } else {
                        multiplier = labs(
                            ((**chain_atoms_by_direction.offset(i_6 as isize))
                                .current_position
                                .v
                                - (**chain_atoms_by_direction.offset(i_6 as isize))
                                    .original_position
                                    .v) as c_long,
                        ) as i32;
                        multiplier /= rs_gcd(
                            multiplier as u64,
                            labs(
                                (*(*board).area_directions.offset(area_direction as isize))
                                    .direction
                                    .v as c_long,
                            ) as u64,
                        ) as i32;
                    }
                    (*(*board).area_directions.offset(area_direction as isize))
                        .direction
                        .u *= multiplier;
                    (*(*board).area_directions.offset(area_direction as isize))
                        .direction
                        .v *= multiplier;
                    i_6 = i_6.wrapping_add(1);
                }
                free(chain_atoms_by_direction as *mut c_void);
            } else {
                (*board).area_growth_order = growth_order::GROWTH_QUADRATIC;
            }
            result.area_growth_order = (*board).area_growth_order;
            (*board).chain_mode = chain_mode::EXTEND_CHAIN;
            (*board).chain_will_become_visible = false;
            let mut i_7: usize = 0_usize;
            while i_7 < (*solution).number_of_inputs_and_outputs {
                (*(*solution).inputs_and_outputs.offset(i_7 as isize)).maximum_feed_rate = 0_i32;
                i_7 = i_7.wrapping_add(1);
            }
            let mut i_8: u64 = 0_u64;
            while i_8 < result.number_of_cycles && !(*board).rs_collision {
                rs_cycle(solution, board);
                i_8 = i_8.wrapping_add(1);
            }
            (*board).chain_mode = chain_mode::DISCOVER_CHAIN;
            if (*board).rs_collision {
                break;
            }
            if (*board).chain_will_become_visible {
                disable_check_until_next_snapshot = true;
            } else {
                if (*board).area_growth_order.0 != growth_order::GROWTH_NONE.0 {
                    let mut i_9: u32 = 0_u32;
                    while i_9 < (*board).number_of_chain_atoms {
                        let ca_1: *mut chain_atom = (*board).chain_atoms.offset(i_9 as isize);
                        if !(*ca_1).prev_in_list.is_null() {
                            let mut p: vector = rs_chain_atom_motion(ca_1);
                            let mut j: i32 = 0_i32;
                            while j < 12_i32 {
                                result.directional_growth[j as usize] = rs_max(
                                    result.directional_growth[j as usize],
                                    rs_dot(rs_growth_directions[j as usize], p),
                                );
                                j += 1;
                            }
                            if (*ca_1).flags & 63_u32 << 4_i32 != 0 {
                                let mut sextant: u32 = 0_u32;
                                while sextant < 6_u32 {
                                    let q: vector = vector {
                                        u: -p.v,
                                        v: p.u + p.v,
                                    };
                                    if (*ca_1).flags & 1_u32 << 4_u32.wrapping_add(sextant) != 0 {
                                        let mut j_0: i32 = 0_i32;
                                        while j_0 < 12_i32 {
                                            let growth_direction: vector =
                                                rs_growth_directions[j_0 as usize];
                                            result.directional_growth[j_0 as usize] = rs_max(
                                                result.directional_growth[j_0 as usize],
                                                rs_max(
                                                    rs_dot(growth_direction, p),
                                                    rs_dot(growth_direction, q),
                                                ),
                                            );
                                            if growth_direction.v * p.u - growth_direction.u * p.v
                                                > 0_i32
                                                && growth_direction.v * q.u
                                                    - growth_direction.u * q.v
                                                    < 0_i32
                                            {
                                                let length_in_direction: f64 = sqrt(
                                                    rs_length_squared(p) as f64
                                                        * rs_length_squared(growth_direction)
                                                            as f64,
                                                );
                                                result.directional_growth[j_0 as usize] = rs_max(
                                                    result.directional_growth[j_0 as usize],
                                                    length_in_direction,
                                                );
                                            }
                                            j_0 += 1;
                                        }
                                    }
                                    p = q;
                                    sextant = sextant.wrapping_add(1);
                                }
                            }
                        }
                        i_9 = i_9.wrapping_add(1);
                    }
                }
                let mut repeating_outputs: u64 = 1_u64;
                let mut repeating_periods: u64 = 0_u64;
                let mut periods_to_run: u64 = 1_u64;
                let mut i_10: usize = 0_usize;
                while i_10 < (*solution).number_of_inputs_and_outputs {
                    let io: *mut input_output =
                        (*solution).inputs_and_outputs.offset(i_10 as isize);
                    if (*io).r#type.0 & input_output_type::REPEATING_OUTPUT.0 != 0 {
                        let divisor: i32 = rs_polymer_feed_rate_divisor(io);
                        if divisor <= 0_i32 {
                            repeating_outputs = 0_u64;
                            repeating_periods = 1_u64;
                            break;
                        } else {
                            if ((*io).outputs_per_repetition as u64)
                                .wrapping_mul((*io).maximum_feed_rate as u64)
                                .wrapping_mul(repeating_periods)
                                < repeating_outputs.wrapping_mul(divisor as u64)
                            {
                                repeating_outputs = ((*io).outputs_per_repetition as u64)
                                    .wrapping_mul((*io).maximum_feed_rate as u64);
                                repeating_periods = divisor as u64;
                            }
                            if (*io).maximum_feed_rate > 0_i32 {
                                let output_interval_periods: u64 = (divisor as u64).wrapping_div(
                                    rs_gcd((*io).maximum_feed_rate as u64, divisor as u64),
                                );
                                (*io).output_cycles_end =
                                    result.outputs_repeat_after_cycle.wrapping_add(
                                        result
                                            .number_of_cycles
                                            .wrapping_mul(output_interval_periods),
                                    );
                                if output_interval_periods > periods_to_run {
                                    periods_to_run = output_interval_periods;
                                }
                            }
                        }
                    }
                    i_10 = i_10.wrapping_add(1);
                }
                let additional_cycles_to_run: u64 = result
                    .number_of_cycles
                    .wrapping_mul(periods_to_run.wrapping_sub(1_u64));
                if repeating_outputs < result.number_of_outputs.wrapping_mul(repeating_periods) {
                    let d: u64 = rs_gcd(repeating_outputs, repeating_periods);
                    repeating_outputs = repeating_outputs.wrapping_div(d);
                    repeating_periods = repeating_periods.wrapping_div(d);
                    result.number_of_outputs = repeating_outputs;
                    result.number_of_cycles =
                        result.number_of_cycles.wrapping_mul(repeating_periods);
                    let mut i_11: i32 = 0_i32;
                    while i_11 < 12_i32 {
                        result.directional_growth[i_11 as usize] *= repeating_periods as f64;
                        i_11 += 1;
                    }
                    if repeating_periods.wrapping_rem(2_u64) == 0_u64 {
                        result.pivot_parity = false;
                    }
                }
                if (*board).area_growth_order.0 == growth_order::GROWTH_LINEAR.0 {
                    let mut linear_area_growth: u64 = 0_u64;
                    let mut linear_area_growth_periods: u64 = 1_u64;
                    let mut i_12: u32 = 0_u32;
                    while i_12 < (*board).number_of_area_directions {
                        if !(*(*board).area_directions.offset(i_12 as isize))
                            .footprint_at_infinity
                            .atoms_at_positions
                            .is_null()
                        {
                            let mut j_1: u32 = 0_u32;
                            while j_1
                                < (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32).wrapping_add(
                                    (*(*board).area_directions.offset(i_12 as isize))
                                        .footprint_at_infinity
                                        .hash_capacity,
                                )
                            {
                                let ap: atom_at_position =
                                    *(*(*board).area_directions.offset(i_12 as isize))
                                        .footprint_at_infinity
                                        .atoms_at_positions
                                        .offset(j_1 as isize);
                                if ap.atom & VALID != 0 {
                                    let divisions: u64 = ap.atom >> 1_i32;
                                    let d_0: u64 = rs_gcd(divisions, linear_area_growth_periods);
                                    linear_area_growth = linear_area_growth
                                        .wrapping_mul(divisions.wrapping_div(d_0));
                                    linear_area_growth = linear_area_growth
                                        .wrapping_add(linear_area_growth_periods.wrapping_div(d_0));
                                    linear_area_growth_periods = linear_area_growth_periods
                                        .wrapping_mul(divisions.wrapping_div(d_0));
                                }
                                j_1 = j_1.wrapping_add(1);
                            }
                        }
                        i_12 = i_12.wrapping_add(1);
                    }
                    linear_area_growth = linear_area_growth.wrapping_mul(
                        result
                            .number_of_cycles
                            .wrapping_div(repetition_period_length),
                    );
                    let d_1: u64 = rs_gcd(linear_area_growth, linear_area_growth_periods);
                    linear_area_growth = linear_area_growth.wrapping_div(d_1);
                    linear_area_growth_periods = linear_area_growth_periods.wrapping_div(d_1);
                    result.number_of_outputs = result
                        .number_of_outputs
                        .wrapping_mul(linear_area_growth_periods);
                    result.number_of_cycles = result
                        .number_of_cycles
                        .wrapping_mul(linear_area_growth_periods);
                    let mut i_13: i32 = 0_i32;
                    while i_13 < 12_i32 {
                        result.directional_growth[i_13 as usize] *=
                            linear_area_growth_periods as f64;
                        i_13 += 1;
                    }
                    result.linear_area_growth = linear_area_growth;
                } else if (*board).area_growth_order.0 == growth_order::GROWTH_QUADRATIC.0 {
                    let chain_swings: *mut chain_swing = calloc(
                        number_of_swinging_chain_atoms
                            .wrapping_mul(6_u32)
                            .wrapping_mul(2_u32) as usize,
                        ::core::mem::size_of::<chain_swing>(),
                    ) as *mut chain_swing;
                    let mut number_of_chain_swings: u32 = 0_u32;
                    let mut i_14: u32 = 0_u32;
                    while i_14 < (*board).number_of_chain_atoms {
                        let ca_2: *mut chain_atom = (*board).chain_atoms.offset(i_14 as isize);
                        if !(*ca_2).prev_in_list.is_null() {
                            let mut swing: chain_swing = chain_swing {
                                direction: rs_chain_atom_direction(ca_2),
                                length_squared: rs_length_squared(rs_chain_atom_motion(ca_2)),
                                sextant: 0,
                            };
                            while swing.direction.u <= 0_i32 || swing.direction.v < 0_i32 {
                                swing.sextant += 1;
                                let tmp: i32 = swing.direction.u;
                                swing.direction.u += swing.direction.v;
                                swing.direction.v = -tmp;
                            }
                            let mut j_2: i32 = 0_i32;
                            while j_2 < 6_i32 {
                                if (*ca_2).flags & 1_u32 << 4_i32 + j_2 != 0 {
                                    let c2rust_fresh2 = number_of_chain_swings;
                                    number_of_chain_swings = number_of_chain_swings.wrapping_add(1);
                                    *chain_swings.offset(c2rust_fresh2 as isize) = swing;
                                    if swing.sextant == 0_i32 {
                                        let mut duplicate: chain_swing = swing;
                                        duplicate.sextant += 6_i32;
                                        let c2rust_fresh3 = number_of_chain_swings;
                                        number_of_chain_swings =
                                            number_of_chain_swings.wrapping_add(1);
                                        *chain_swings.offset(c2rust_fresh3 as isize) = duplicate;
                                    }
                                }
                                swing.sextant = (swing.sextant + 1_i32) % 6_i32;
                                j_2 += 1;
                            }
                        }
                        i_14 = i_14.wrapping_add(1);
                    }
                    qsort(
                        chain_swings as *mut c_void,
                        number_of_chain_swings as usize,
                        ::core::mem::size_of::<chain_swing>(),
                        Some(
                            rs_compare_chain_swings
                                as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
                        ),
                    );
                    let mut deleted: u32 = 0_u32;
                    let mut first_active_index: u32 = 0_u32;
                    let mut longest_active_swing: chain_swing = chain_swing {
                        direction: vector { u: 0_i32, v: 0 },
                        length_squared: 0,
                        sextant: 0,
                    };
                    let mut longest_active_swing_start: chain_swing = chain_swing {
                        direction: vector { u: 0_i32, v: 0 },
                        length_squared: 0,
                        sextant: 0,
                    };
                    let mut starting_point: chain_swing = chain_swing {
                        direction: vector { u: 0_i32, v: 0 },
                        length_squared: 0,
                        sextant: 0,
                    };
                    let mut i_15: u32 = 0_u32;
                    while i_15 < number_of_chain_swings.wrapping_add(1_u32) {
                        let current_swing: chain_swing = if i_15 < number_of_chain_swings {
                            *chain_swings.offset(i_15 as isize)
                        } else {
                            starting_point
                        };
                        's_859: {
                            if i_15 < number_of_chain_swings {
                                if i_15 != 0_u32
                                    && rs_compare_chain_swings_by_direction(
                                        *chain_swings.offset(i_15.wrapping_sub(1_u32) as isize),
                                        current_swing,
                                    ) == 0_i32
                                {
                                    deleted = deleted.wrapping_add(1);
                                    break 's_859;
                                } else {
                                    *chain_swings.offset(i_15.wrapping_sub(deleted) as isize) =
                                        current_swing;
                                }
                            }
                            while rs_compare_chain_swings_by_direction(
                                current_swing,
                                rs_end_of_swing(longest_active_swing),
                            ) >= 0_i32
                            {
                                result.quadratic_area_growth += rs_measure_quadratic_swing_area(
                                    &raw mut starting_point,
                                    longest_active_swing.length_squared,
                                    longest_active_swing_start,
                                    rs_end_of_swing(longest_active_swing),
                                );
                                let last_end: chain_swing = rs_end_of_swing(longest_active_swing);
                                while first_active_index < i_15.wrapping_sub(deleted)
                                    && rs_compare_chain_swings_by_direction(
                                        *chain_swings.offset(first_active_index as isize),
                                        longest_active_swing,
                                    ) <= 0_i32
                                {
                                    first_active_index = first_active_index.wrapping_add(1);
                                }
                                longest_active_swing = current_swing;
                                longest_active_swing_start = current_swing;
                                let mut j_3: u32 = first_active_index;
                                while j_3 < i_15.wrapping_sub(deleted) {
                                    if rs_compare_chain_swings_by_direction(
                                        last_end,
                                        longest_active_swing_start,
                                    ) < 0_i32
                                        || (*chain_swings.offset(j_3 as isize)).length_squared
                                            > longest_active_swing.length_squared
                                    {
                                        longest_active_swing = *chain_swings.offset(j_3 as isize);
                                        longest_active_swing_start = last_end;
                                    }
                                    j_3 = j_3.wrapping_add(1);
                                }
                            }
                            if current_swing.length_squared > longest_active_swing.length_squared {
                                result.quadratic_area_growth += rs_measure_quadratic_swing_area(
                                    &raw mut starting_point,
                                    longest_active_swing.length_squared,
                                    longest_active_swing_start,
                                    current_swing,
                                );
                                longest_active_swing = current_swing;
                                longest_active_swing_start = current_swing;
                            }
                        }
                        i_15 = i_15.wrapping_add(1);
                    }
                    free(chain_swings as *mut c_void);
                    let cycles_scale_factor: f64 = result
                        .number_of_cycles
                        .wrapping_div(repetition_period_length)
                        as f64;
                    result.quadratic_area_growth *= cycles_scale_factor * cycles_scale_factor;
                }
                let mut i_16: u64 = 0_u64;
                while i_16 < additional_cycles_to_run && !(*board).rs_collision {
                    rs_cycle(solution, board);
                    i_16 = i_16.wrapping_add(1);
                }
                rs_destroy_snapshot(&raw mut snapshot);
                return result;
            }
        } else {
            while (*board).cycle > next_snapshot_cycle {
                next_snapshot_cycle = next_snapshot_cycle.wrapping_add(snapshot_period);
            }
            if (*board).cycle == next_snapshot_cycle {
                rs_take_snapshot(solution, board, &raw mut snapshot);
                disable_check_until_next_snapshot = false;
                next_snapshot_cycle = next_snapshot_cycle.wrapping_add(snapshot_period);
                snapshot_period = snapshot_period.wrapping_mul(2_u64);
            }
            rs_cycle(solution, board);
        }
    }
    rs_destroy_snapshot(&raw mut snapshot);
    steady_state {
        eventual_behavior: eventual_behavior(
            (if (*board).rs_collision as i32 != 0 {
                eventual_behavior::EVENTUALLY_STOPS_RUNNING.0 as i32
            } else {
                eventual_behavior::EVENTUALLY_REACHES_CYCLE_LIMIT.0 as i32
            }) as u32,
        ),
        number_of_cycles: 0,
        number_of_outputs: 0,
        outputs_repeat_after_cycle: 0,
        area_growth_order: growth_order::GROWTH_NONE,
        linear_area_growth: 0,
        quadratic_area_growth: 0.,
        directional_growth: [0.; 12],
        pivot_parity: false,
    }
}
