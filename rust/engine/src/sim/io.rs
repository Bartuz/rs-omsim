//! Inputs (blocking, spawning) and outputs (single molecules, polymers, chain matching).
use super::*;

pub(crate) unsafe extern "C" fn rs_flag_blocked_inputs(solution: *mut solution, board: *mut board) {
    let mut i: usize = 0_usize;
    while i < (*solution).number_of_inputs_and_outputs {
        (*(*solution).inputs_and_outputs.offset(i as isize)).r#type = input_output_type(
            (*(*solution).inputs_and_outputs.offset(i as isize))
                .r#type
                .0
                & !(input_output_type::BLOCKED.0 as i32) as u32,
        );
        i = i.wrapping_add(1);
    }
    let mut i_0: usize = 0_usize;
    while i_0 < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.offset(i_0 as isize);
        if (*io).r#type.0 & input_output_type::INPUT.0 != 0 {
            let mut j: u32 = 0_u32;
            while j < (*io).number_of_atoms {
                let a: atom = *rs_lookup_atom(board, (*(*io).atoms.offset(j as isize)).position);
                if a as u64 & VALID != 0 && a as u64 & REMOVED == 0 {
                    (*io).r#type = input_output_type((*io).r#type.0 | input_output_type::BLOCKED.0);
                    break;
                } else {
                    j = j.wrapping_add(1);
                }
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
}
pub(crate) unsafe extern "C" fn rs_spawn_inputs(solution: *mut solution, board: *mut board) {
    let mut i: usize = 0_usize;
    while i < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.offset(i as isize);
        if !((*io).r#type.0 & input_output_type::INPUT.0 == 0
            || (*io).r#type.0 & input_output_type::BLOCKED.0 != 0)
        {
            let mut j: u32 = 0_u32;
            while j < (*io).number_of_atoms {
                let input: atom = (*(*io).atoms.offset(j as isize)).atom;
                let p: vector = (*(*io).atoms.offset(j as isize)).position;
                rs_insert_atom(board, p, (VALID | input as u64) as atom);
                j = j.wrapping_add(1);
            }
            let mut j_0: u32 = 0_u32;
            while j_0 < (*io).number_of_disjoint_bonds {
                rs_insert_disjoint_bond(
                    &raw mut (*board).disjoint_bond_table,
                    *(*io).disjoint_bonds.offset(j_0 as isize),
                );
                j_0 = j_0.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_polymer_feed_rate_divisor(io: *mut input_output) -> i32 {
    let placeholder: atom_at_position = *(*io)
        .original_atoms
        .offset((*io).number_of_original_atoms.wrapping_sub(1_u32) as isize);
    rs_vector_hexicab_length(vector {
        u: placeholder.position.u - (*io).repetition_origin.u,
        v: placeholder.position.v - (*io).repetition_origin.v,
    }) as i32
}
pub(crate) unsafe extern "C" fn rs_match_repeating_output_with_chain_atoms(
    board: *mut board,
    io: *mut input_output,
) {
    let placeholder: atom_at_position = *(*io)
        .original_atoms
        .offset((*io).number_of_original_atoms.wrapping_sub(1_u32) as isize);
    let offset: vector = vector {
        u: placeholder.position.u - (*io).repetition_origin.u,
        v: placeholder.position.v - (*io).repetition_origin.v,
    };
    if offset.u == 0_i32 && offset.v == 0_i32 {
        return;
    }
    let mut maximum_feed_rate: i32 = 0_i32;
    let mut i: u32 = 0_u32;
    while i < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
        let output: atom_at_position = *(*io).atoms.offset(
            i.wrapping_add((*io).number_of_original_atoms)
                .wrapping_sub(1_u32) as isize,
        );
        let mut repeats_after: u32 = 4294967295_u32;
        let mut j: u32 = 0_u32;
        while j < repeats_after {
            let mut p: vector = output.position;
            p.u = (p.u as u32).wrapping_add(j.wrapping_mul(offset.u as u32)) as i32;
            p.v = (p.v as u32).wrapping_add(j.wrapping_mul(offset.v as u32)) as i32;
            let mut a: atom = *rs_lookup_atom(board, p);
            if a as u64 & VALID == 0 || a as u64 & REMOVED != 0 || a as u64 & IS_CHAIN_ATOM != 0 {
                let mut found_chain_atom: bool = false;
                let mut k: u32 = 0_u32;
                while k < (*board).number_of_chain_atoms {
                    let ca: chain_atom = *(*board).chain_atoms.offset(k as isize);
                    if !(ca.prev_in_list.is_null() || ca.flags & 1_u32 << 3_i32 == 0) {
                        let chain_offset: vector = vector {
                            u: ca.current_position.u - ca.original_position.u,
                            v: ca.current_position.v - ca.original_position.v,
                        };
                        let mut repetition: i32 = 0_i32;
                        if chain_offset.u != 0_i32 {
                            repetition = (p.u - ca.current_position.u) / chain_offset.u;
                        } else {
                            repetition = (p.v - ca.current_position.v) / chain_offset.v;
                        }
                        if repetition >= 0_i32 {
                            if ca.current_position.u + repetition * chain_offset.u == p.u {
                                if ca.current_position.v + repetition * chain_offset.v == p.v {
                                    if repeats_after == 4294967295_u32 {
                                        if chain_offset.u != 0_i32 {
                                            repeats_after = j.wrapping_add(
                                                (chain_offset.u / rs_gcd(offset.u, chain_offset.u))
                                                    as u32,
                                            );
                                        } else {
                                            repeats_after = j.wrapping_add(
                                                (chain_offset.v / rs_gcd(offset.v, chain_offset.v))
                                                    as u32,
                                            );
                                        }
                                    }
                                    let feed_rate: i32 =
                                        rs_vector_hexicab_length(chain_offset) as i32;
                                    if feed_rate > maximum_feed_rate {
                                        maximum_feed_rate = feed_rate;
                                    }
                                    found_chain_atom = true;
                                    a = *rs_lookup_atom(board, ca.current_position);
                                    break;
                                }
                            }
                        }
                    }
                    k = k.wrapping_add(1);
                }
                if !found_chain_atom {
                    return;
                }
            }
            if a as u64
                & (SALT
                    | (WATER | FIRE | EARTH | AIR)
                    | QUICKSILVER
                    | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                    | VITAE
                    | MORS
                    | QUINTESSENCE
                    | 0x3f_u64.wrapping_mul(
                        RECENT_BOND0
                            | NORMAL_BOND0
                            | TRIPLEX_R_BOND0
                            | TRIPLEX_Y_BOND0
                            | TRIPLEX_K_BOND0,
                    ) & !(0x3f_u64 << 29_i32))
                != output.atom
            {
                return;
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    if maximum_feed_rate > (*io).maximum_feed_rate {
        (*io).maximum_feed_rate = maximum_feed_rate;
    }
}
pub(crate) unsafe extern "C" fn rs_is_ignored_output_position(
    io: *mut input_output,
    mut pos: vector,
    monomer_count: i32,
) -> bool {
    pos = rs_polymer_position_from_global_position(io, pos);
    if pos.v < (*io).min_v || pos.v > (*io).max_v {
        return false;
    }
    let row: usize = (pos.v - (*io).min_v) as usize;
    pos.u < *(*io).row_min_u.offset(row as isize)
        || *(*io).row_max_u.offset(row as isize) != 2147483647_i32
            && pos.u
                > *(*io).row_max_u.offset(row as isize)
                    + (monomer_count - 1_i32) * (*io).monomer_width
}
pub(crate) unsafe extern "C" fn rs_consume_single_output(
    board: *mut board,
    io: *mut input_output,
    output_index: i32,
    molecule: *mut molecule,
) {
    let fail_on_wrong_output: bool =
        (*board).fails_on_wrong_output_mask & 1_u64 << (*io).puzzle_index != 0;
    let fail_on_wrong_bonds: bool =
        (*board).fails_on_wrong_output_bonds_mask & 1_u64 << (*io).puzzle_index != 0;
    let mut wrong_output: bool = false;
    let mut wrong_bonds: bool = false;
    if (*molecule).size != (*io).number_of_atoms as usize {
        return;
    }
    let mut i: u32 = 0_u32;
    while (i as usize) < (*molecule).size {
        let a: atom_ref_at_position = *(*molecule).atoms.offset(i as isize) as atom_ref_at_position;
        if *a.atom & 0x1f_u64.wrapping_mul(GRABBED_ONCE) != 0 {
            return;
        }
        let mut target: atom = 0 as atom;
        let mut j: u32 = 0_u32;
        while j < (*io).number_of_atoms {
            if rs_vectors_equal(a.position, (*(*io).atoms.offset(j as isize)).position) {
                target = (*(*io).atoms.offset(j as isize)).atom;
                break;
            } else {
                j = j.wrapping_add(1);
            }
        }
        if target == 0 {
            return;
        }
        let differences: atom = *a.atom ^ target;
        wrong_output = wrong_output as i32 != 0
            || differences as u64
                & (SALT
                    | (WATER | FIRE | EARTH | AIR)
                    | QUICKSILVER
                    | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                    | VITAE
                    | MORS
                    | QUINTESSENCE)
                != 0
                && target as u64 & VARIABLE_OUTPUT == 0;
        wrong_bonds = wrong_bonds as i32 != 0
            || differences as u64
                & (0x3f_u64.wrapping_mul(
                    RECENT_BOND0
                        | NORMAL_BOND0
                        | TRIPLEX_R_BOND0
                        | TRIPLEX_Y_BOND0
                        | TRIPLEX_K_BOND0,
                ) & !(0x3f_u64 << 29_i32))
                != 0;
        if !(fail_on_wrong_output as i32 != 0 || fail_on_wrong_bonds as i32 != 0)
            && (wrong_output as i32 != 0 || wrong_bonds as i32 != 0)
        {
            return;
        }
        i = i.wrapping_add(1);
    }
    if fail_on_wrong_output as i32 != 0 && wrong_output as i32 != 0 {
        rs_report_collision(
            board,
            (*(*io).atoms.offset(0isize)).position,
            b"output didn't match\0".as_ptr() as *const c_char,
        );
        (*board).wrong_output_index = output_index as usize;
    }
    if fail_on_wrong_bonds as i32 != 0 && wrong_bonds as i32 != 0 {
        rs_report_collision(
            board,
            (*(*io).atoms.offset(0isize)).position,
            b"output bonds didn't match\0".as_ptr() as *const c_char,
        );
        (*board).wrong_output_index = output_index as usize;
    }
    if wrong_output as i32 != 0 || wrong_bonds as i32 != 0 {
        return;
    }
    let mut i_0: u32 = 0_u32;
    while (i_0 as usize) < (*molecule).size {
        let a_0: atom_ref_at_position =
            *(*molecule).atoms.offset(i_0 as isize) as atom_ref_at_position;
        if *a_0.atom & HAS_DISJOINT_BOND != 0 {
            rs_remove_disjoint_bond(&raw mut (*board).disjoint_bond_table, a_0.position);
        }
        i_0 = i_0.wrapping_add(1);
    }
    rs_remove_molecule(board, molecule);
    (*io).number_of_outputs = (*io).number_of_outputs.wrapping_add(1);
}
pub(crate) unsafe extern "C" fn rs_check_repeating_output_at_monomer_count(
    board: *mut board,
    io: *mut input_output,
    molecule: *mut molecule,
    monomer_count: u32,
) -> bool {
    let mut ignorable_atoms: i32 = (*molecule).size.wrapping_sub(
        monomer_count.wrapping_mul((*io).number_of_original_atoms.wrapping_sub(1_u32)) as usize,
    ) as i32;
    if ignorable_atoms < 0_i32 {
        return false;
    }
    let mut i: u32 = 0_u32;
    while (i as usize) < (*molecule).size {
        let a: atom_ref_at_position = *(*molecule).atoms.offset(i as isize) as atom_ref_at_position;
        if rs_is_ignored_output_position(io, a.position, monomer_count as i32) {
            ignorable_atoms -= 1;
            if ignorable_atoms < 0_i32 {
                return false;
            }
        } else {
            let mut target: atom = 0 as atom;
            let mut j: u32 = 0_u32;
            while j < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
                let monomer_atom: atom_at_position = *(*io).atoms.offset(
                    j.wrapping_add((*io).number_of_original_atoms.wrapping_sub(1_u32)) as isize,
                );
                let atom_position_polymer_space: vector =
                    rs_polymer_position_from_global_position(io, a.position);
                let monomer_atom_position_polymer_space: vector =
                    rs_polymer_position_from_global_position(io, monomer_atom.position);
                if (atom_position_polymer_space.u - monomer_atom_position_polymer_space.u)
                    % (*io).monomer_width
                    == 0_i32
                    && atom_position_polymer_space.v == monomer_atom_position_polymer_space.v
                {
                    let monomer_index: i32 = 1_i32
                        + (atom_position_polymer_space.u - monomer_atom_position_polymer_space.u)
                            / (*io).monomer_width;
                    if monomer_index >= 0_i32 && (monomer_index as u32) < monomer_count {
                        target = monomer_atom.atom;
                        break;
                    }
                }
                j = j.wrapping_add(1);
            }
            if target == 0 {
                return false;
            }
            let mut differences: atom = *a.atom ^ target;
            if differences as u64
                & (0x3f_u64.wrapping_mul(
                    RECENT_BOND0
                        | NORMAL_BOND0
                        | TRIPLEX_R_BOND0
                        | TRIPLEX_Y_BOND0
                        | TRIPLEX_K_BOND0,
                ) & !(0x3f_u64 << 29_i32))
                != 0
            {
                let mut dir: i32 = 0_i32;
                while dir < 6_i32 {
                    let d: vector = rs_u_offset_for_direction(dir);
                    let neighbor: vector = vector {
                        u: a.position.u + d.u,
                        v: a.position.v + d.v,
                    };
                    if rs_is_ignored_output_position(io, neighbor, monomer_count as i32) {
                        differences = (differences as u64
                            & !((RECENT_BOND0
                                | NORMAL_BOND0
                                | TRIPLEX_R_BOND0
                                | TRIPLEX_Y_BOND0
                                | TRIPLEX_K_BOND0)
                                << dir)) as atom;
                    }
                    dir += 1;
                }
            }
            if differences as u64
                & (SALT
                    | (WATER | FIRE | EARTH | AIR)
                    | QUICKSILVER
                    | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                    | VITAE
                    | MORS
                    | QUINTESSENCE)
                != 0
                && target as u64 & VARIABLE_OUTPUT == 0
            {
                return false;
            }
            if differences as u64
                & (0x3f_u64.wrapping_mul(
                    RECENT_BOND0
                        | NORMAL_BOND0
                        | TRIPLEX_R_BOND0
                        | TRIPLEX_Y_BOND0
                        | TRIPLEX_K_BOND0,
                ) & !(0x3f_u64 << 29_i32))
                != 0
            {
                return false;
            }
        }
        i = i.wrapping_add(1);
    }
    true
}
pub(crate) unsafe extern "C" fn rs_check_repeating_output(
    board: *mut board,
    io: *mut input_output,
    molecule: *mut molecule,
) {
    let mut monomer_count: u32 = 1_u64.wrapping_add(
        (*io)
            .number_of_outputs
            .wrapping_div((*io).outputs_per_repetition as u64),
    ) as u32;
    if (*board).chain_mode.0 == chain_mode::EXTEND_CHAIN.0 {
        rs_match_repeating_output_with_chain_atoms(board, io);
    }
    while rs_check_repeating_output_at_monomer_count(board, io, molecule, monomer_count) {
        (*io).number_of_outputs = monomer_count.wrapping_mul((*io).outputs_per_repetition) as u64;
        monomer_count = monomer_count.wrapping_add(1);
    }
}
pub(crate) unsafe extern "C" fn rs_consume_output(
    board: *mut board,
    io: *mut input_output,
    output_index: i32,
) {
    let m: mechanism = mechanism {
        r#type: 0_u32,
        position: (*(*io).atoms.offset((*io).center_atom_index as isize)).position,
        direction_u: vector { u: 0, v: 0 },
        direction_v: vector { u: 0, v: 0 },
        conversion: 0,
        conduit_index: 0,
        movement: vector { u: 0, v: 0 },
        arm_rotation: 0,
        pivot_parity: 0,
    };
    let molecule: *mut molecule = rs_get_molecule(board, rs_get_atom(board, m, 0_i32, 0_i32));
    if (*io).r#type.0 & input_output_type::REPEATING_OUTPUT.0 != 0 {
        rs_check_repeating_output(board, io, molecule);
    } else {
        rs_consume_single_output(board, io, output_index, molecule);
    };
}
pub(crate) unsafe extern "C" fn rs_consume_outputs(solution: *mut solution, board: *mut board) {
    let mut i: usize = 0_usize;
    while i < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.offset(i as isize);
        if (*io).r#type.0 & input_output_type::OUTPUT.0 != 0 {
            rs_consume_output(board, io, i as i32);
        }
        i = i.wrapping_add(1);
    }
}
