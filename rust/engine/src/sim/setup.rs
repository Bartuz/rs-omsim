//! Glyph footprints, wheels, cabinets, initial_setup and destroy.
use super::*;

pub(crate) static mut c2rust_lvalue: [vector; 1] = [vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_0: [vector; 4] = [
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_1: [vector; 2] =
    [vector { u: 1_i32, v: 0_i32 }, vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_2: [vector; 5] = [
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_3: [vector; 3] = [
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_4: [vector; 2] =
    [vector { u: 1_i32, v: 0_i32 }, vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_5: [vector; 5] = [
    vector { u: 0_i32, v: 1_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_6: [vector; 2] =
    [vector { u: 1_i32, v: 0_i32 }, vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_7: [vector; 2] =
    [vector { u: 1_i32, v: 0_i32 }, vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_8: [vector; 3] = [
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_9: [vector; 4] = [
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_10: [vector; 7] = [
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 0_i32, v: 1_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_11: [vector; 1] = [vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_12: [vector; 2] =
    [vector { u: 1_i32, v: 0_i32 }, vector { u: 0_i32, v: 0_i32 }];
pub(crate) static mut c2rust_lvalue_13: [vector; 3] = [
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut c2rust_lvalue_14: [vector; 6] = [
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 0_i32, v: 0_i32 },
];
pub(crate) static mut rs_footprints: [footprint; 16] = [
    footprint {
        r#type: (CALCIFICATION) as u32,
        hexes: &raw const c2rust_lvalue as *const vector,
    },
    footprint {
        r#type: (ANIMISMUS) as u32,
        hexes: &raw const c2rust_lvalue_0 as *const vector,
    },
    footprint {
        r#type: (PROJECTION) as u32,
        hexes: &raw const c2rust_lvalue_1 as *const vector,
    },
    footprint {
        r#type: (DISPERSION) as u32,
        hexes: &raw const c2rust_lvalue_2 as *const vector,
    },
    footprint {
        r#type: (PURIFICATION) as u32,
        hexes: &raw const c2rust_lvalue_3 as *const vector,
    },
    footprint {
        r#type: (DUPLICATION) as u32,
        hexes: &raw const c2rust_lvalue_4 as *const vector,
    },
    footprint {
        r#type: (UNIFICATION) as u32,
        hexes: &raw const c2rust_lvalue_5 as *const vector,
    },
    footprint {
        r#type: (BONDING) as u32,
        hexes: &raw const c2rust_lvalue_6 as *const vector,
    },
    footprint {
        r#type: (UNBONDING) as u32,
        hexes: &raw const c2rust_lvalue_7 as *const vector,
    },
    footprint {
        r#type: (TRIPLEX_BONDING) as u32,
        hexes: &raw const c2rust_lvalue_8 as *const vector,
    },
    footprint {
        r#type: (MULTI_BONDING) as u32,
        hexes: &raw const c2rust_lvalue_9 as *const vector,
    },
    footprint {
        r#type: (DISPOSAL) as u32,
        hexes: &raw const c2rust_lvalue_10 as *const vector,
    },
    footprint {
        r#type: (EQUILIBRIUM) as u32,
        hexes: &raw const c2rust_lvalue_11 as *const vector,
    },
    footprint {
        r#type: (REJECTION) as u32,
        hexes: &raw const c2rust_lvalue_12 as *const vector,
    },
    footprint {
        r#type: (DIVISION) as u32,
        hexes: &raw const c2rust_lvalue_13 as *const vector,
    },
    footprint {
        r#type: (PROLIFERATION) as u32,
        hexes: &raw const c2rust_lvalue_14 as *const vector,
    },
];
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_glyph_footprint(mechanism_type: u32) -> *const vector {
    let mut i: i32 = 0_i32;
    while (i as usize)
        < ::core::mem::size_of::<[footprint; 16]>()
            .wrapping_div(::core::mem::size_of::<footprint>())
    {
        if mechanism_type & rs_footprints[i as usize].r#type != 0 {
            return rs_footprints[i as usize].hexes;
        }
        i += 1;
    }
    static mut rs_trivial_footprint: [vector; 1] = [vector { u: 0_i32, v: 0_i32 }];
    &raw const rs_trivial_footprint as *const vector
}
pub(crate) unsafe extern "C" fn rs_create_wheel_atom(
    board: *mut board,
    m: mechanism,
    du: i32,
    dv: i32,
    element: atom,
) {
    let p: vector = rs_mechanism_relative_position(m, du, dv, 1_i32);
    rs_insert_atom(
        board,
        p,
        (VALID | GRABBED_ONCE | WHEEL_ATOM | element) as atom,
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_cabinet_for_position(solution: *mut solution, position: vector) -> u8 {
    let u: i32 = position.u + (32_i32 >> 1_i32);
    let v: i32 = position.v + (32_i32 >> 1_i32);
    if u >= 0_i32 && u < 32_i32 && v >= 0_i32 && v < 32_i32 {
        return (*solution).cabinet_map[u as usize][v as usize];
    }
    0_u8
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_initial_setup(
    solution: *mut solution,
    board: *mut board,
    initial_board_size: u32,
) {
    (*board).overlap = (*solution).track_overlap;
    (*board).half_cycle = 1_i32;
    (*board).wrong_output_index = 18446744073709551615 as c_ulong as usize;
    let mut initial_hashtable_size: u32 = initial_board_size.wrapping_div(8_u32);
    if initial_hashtable_size < 1_u32 {
        initial_hashtable_size = 1_u32;
    }
    if initial_hashtable_size > 99999_u32 {
        initial_hashtable_size = 99999_u32;
    }
    (*board).grid.board = board as *mut board;
    rs_rehash(&raw mut (*board).grid, initial_hashtable_size);
    let mut i: u32 = 0_u32;
    while (i as usize) < (*solution).number_of_cabinet_walls {
        rs_mark_used_area(board, *(*solution).cabinet_walls.offset(i as isize));
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0_u32;
    while (i_0 as usize) < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.offset(i_0 as isize);
        let mut j: u32 = 0_u32;
        while j < (*io).number_of_atoms {
            rs_mark_used_area_with_overlap(
                board,
                (*(*io).atoms.offset(j as isize)).position,
                &raw mut (*board).overlap,
            );
            j = j.wrapping_add(1);
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: u32 = 0_u32;
    while (i_1 as usize) < (*solution).number_of_glyphs {
        let m: mechanism = *(*solution).glyphs.offset(i_1 as isize);
        if m.r#type as c_ulong & CONDUIT == 0 {
            let footprint: *const vector = rs_glyph_footprint(m.r#type);
            let mut j_0: i32 = 0_i32;
            loop {
                let p: vector = *footprint.offset(j_0 as isize);
                rs_mark_used_area_with_overlap(
                    board,
                    rs_mechanism_relative_position(m, p.u, p.v, 1_i32),
                    &raw mut (*board).overlap,
                );
                if rs_vectors_equal(p, rs_zero_vector) {
                    break;
                }
                j_0 += 1;
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: u32 = 0_u32;
    while (i_2 as usize) < (*solution).number_of_conduits {
        let conduit: *mut conduit = (*solution).conduits.offset(i_2 as isize);
        let m_0: mechanism = *(*solution).glyphs.offset((*conduit).glyph_index as isize);
        let mut j_1: u32 = 0_u32;
        while j_1 < (*conduit).number_of_positions {
            let p_0: vector = *(*conduit).positions.offset(j_1 as isize);
            rs_mark_used_area_with_overlap(
                board,
                rs_mechanism_relative_position(m_0, p_0.u, p_0.v, 1_i32),
                &raw mut (*board).overlap,
            );
            j_1 = j_1.wrapping_add(1);
        }
        i_2 = i_2.wrapping_add(1);
    }
    let mut i_3: usize = 0_usize;
    while i_3 < (*solution).number_of_arms {
        rs_mark_used_area_with_overlap(
            board,
            (*(*solution).arms.offset(i_3 as isize)).position,
            &raw mut (*board).overlap,
        );
        i_3 = i_3.wrapping_add(1);
    }
    let mut i_4: usize = 0_usize;
    while i_4 < (*solution).number_of_arms {
        let c2rust_lvalue_ptr =
            &raw mut *rs_lookup_atom(board, (*(*solution).arms.offset(i_4 as isize)).position);
        *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr | VISITED) as atom;
        i_4 = i_4.wrapping_add(1);
    }
    let mut i_5: u32 = 0_u32;
    while i_5 < (*solution).track_table_size {
        let p_1: vector = *(*solution).track_positions.offset(i_5 as isize);
        if !(p_1.u == -2147483647_i32 - 1_i32 && p_1.v == -2147483647_i32 - 1_i32) {
            rs_mark_used_area_with_overlap(board, p_1, &raw mut (*board).overlap);
        }
        i_5 = i_5.wrapping_add(1);
    }
    let mut i_6: usize = 0_usize;
    while i_6 < (*solution).number_of_arms {
        let c2rust_lvalue_ptr_0 =
            &raw mut *rs_lookup_atom(board, (*(*solution).arms.offset(i_6 as isize)).position);
        *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0 & !(VISITED)) as atom;
        i_6 = i_6.wrapping_add(1);
    }
    let mut i_7: usize = 0_usize;
    while i_7 < (*solution).number_of_arms {
        if (*(*solution).arms.offset(i_7 as isize)).r#type & rs_ANY_WHEEL != 0 {
            (*(*solution).arms.offset(i_7 as isize)).r#type =
                ((*(*solution).arms.offset(i_7 as isize)).r#type as c_ulong | GRABBING) as u32;
            (*(*solution).arms.offset(i_7 as isize)).r#type |= rs_GRABBING_EVERYTHING;
            if (*(*solution).arms.offset(i_7 as isize)).r#type as c_ulong & VAN_BERLO != 0 {
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    1_i32,
                    0_i32,
                    (SALT) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    0_i32,
                    1_i32,
                    (WATER) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    -1_i32,
                    1_i32,
                    (AIR) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    -1_i32,
                    0_i32,
                    (SALT) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    0_i32,
                    -1_i32,
                    (FIRE) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    1_i32,
                    -1_i32,
                    (EARTH) as atom,
                );
            } else if (*(*solution).arms.offset(i_7 as isize)).r#type as c_ulong & RAVARI != 0 {
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    1_i32,
                    0_i32,
                    (TIN) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    0_i32,
                    1_i32,
                    (LEAD) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    -1_i32,
                    1_i32,
                    (GOLD) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    -1_i32,
                    0_i32,
                    (SILVER) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    0_i32,
                    -1_i32,
                    (COPPER) as atom,
                );
                rs_create_wheel_atom(
                    board,
                    *(*solution).arms.offset(i_7 as isize),
                    1_i32,
                    -1_i32,
                    (IRON) as atom,
                );
            }
        }
        i_7 = i_7.wrapping_add(1);
    }
    rs_mark_arm_area(solution, board);
    rs_flag_blocked_inputs(solution, board);
    rs_spawn_inputs(solution, board);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_destroy(solution: *mut solution, board: *mut board) {
    if !solution.is_null() {
        free((*solution).glyphs as *mut c_void);
        free((*solution).arms as *mut c_void);
        let mut i: u32 = 0_u32;
        while (i as usize) < (*solution).number_of_arms {
            free(*(*solution).arm_tape.offset(i as isize) as *mut c_void);
            i = i.wrapping_add(1);
        }
        free((*solution).arm_tape as *mut c_void);
        free((*solution).arm_tape_length as *mut c_void);
        free((*solution).arm_tape_start_cycle as *mut c_void);
        free((*solution).arm_tape_halt_index as *mut c_void);
        free((*solution).track_positions as *mut c_void);
        free((*solution).track_plus_motions as *mut c_void);
        free((*solution).track_minus_motions as *mut c_void);
        let mut i_0: usize = 0_usize;
        while i_0 < (*solution).number_of_conduits {
            free((*(*solution).conduits.offset(i_0 as isize)).positions as *mut c_void);
            free((*(*solution).conduits.offset(i_0 as isize)).atoms as *mut c_void);
            free((*(*solution).conduits.offset(i_0 as isize)).molecule_lengths as *mut c_void);
            free(
                (*(*solution).conduits.offset(i_0 as isize))
                    .disjoint_bond_table
                    .bonds as *mut c_void,
            );
            i_0 = i_0.wrapping_add(1);
        }
        free((*solution).conduits as *mut c_void);
        free((*solution).cabinet_walls as *mut c_void);
        let mut i_1: usize = 0_usize;
        while i_1 < (*solution).number_of_inputs_and_outputs {
            if (*(*solution).inputs_and_outputs.offset(i_1 as isize)).original_atoms
                != (*(*solution).inputs_and_outputs.offset(i_1 as isize)).atoms
            {
                free(
                    (*(*solution).inputs_and_outputs.offset(i_1 as isize)).original_atoms
                        as *mut c_void,
                );
                free(
                    (*(*solution).inputs_and_outputs.offset(i_1 as isize)).disjoint_bonds
                        as *mut c_void,
                );
            }
            free((*(*solution).inputs_and_outputs.offset(i_1 as isize)).atoms as *mut c_void);
            free((*(*solution).inputs_and_outputs.offset(i_1 as isize)).row_min_u as *mut c_void);
            free((*(*solution).inputs_and_outputs.offset(i_1 as isize)).row_max_u as *mut c_void);
            free(
                (*(*solution).inputs_and_outputs.offset(i_1 as isize)).output_cycles as *mut c_void,
            );
            i_1 = i_1.wrapping_add(1);
        }
        free((*solution).inputs_and_outputs as *mut c_void);
        memset(
            solution as *mut c_void,
            0_i32,
            ::core::mem::size_of::<solution>(),
        );
    }
    if !board.is_null() {
        crate::board_index::destroy(board);
        crate::glyph_plan::destroy(board);
        free((*board).grid.atoms_at_positions as *mut c_void);
        free((*board).flag_reset as *mut c_void);
        free((*board).movements.movements as *mut c_void);
        free((*board).moving_atoms.atoms_at_positions as *mut c_void);
        free((*board).overlapped_atoms as *mut c_void);
        free((*board).chain_atoms as *mut c_void);
        free((*board).chain_atom_table as *mut c_void);
        let mut i_2: u32 = 0_u32;
        while i_2 < (*board).number_of_area_directions {
            free(
                (*(*board).area_directions.offset(i_2 as isize))
                    .footprint_at_infinity
                    .atoms_at_positions as *mut c_void,
            );
            i_2 = i_2.wrapping_add(1);
        }
        free((*board).disjoint_bond_table.bonds as *mut c_void);
        free((*board).area_directions as *mut c_void);
        free((*board).molecule.atoms as *mut c_void);
        memset(board as *mut c_void, 0_i32, ::core::mem::size_of::<board>());
    }
}
