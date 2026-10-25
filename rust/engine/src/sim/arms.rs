//! Arm instructions: grab/drop, deferred molecule movements, the motion check call and
//! reinsertion; swing and arm area.
use super::*;

pub(crate) unsafe extern "C" fn rs_record_swing_area(
    solution: *mut solution,
    board: *mut board,
    base: vector,
    position: vector,
    rotation: i32,
) {
    let mut offset: vector = vector {
        u: position.u - base.u,
        v: position.v - base.v,
    };
    if rotation == -1_i32 {
        offset = vector {
            u: offset.u + offset.v,
            v: -offset.u,
        };
    }
    let arm_length: i32 = abs(offset.u) | abs(offset.v);
    offset = rs_normalize_axis(offset);
    if (*solution).production {
        let original_cabinet: u8 = rs_cabinet_for_position(
            solution,
            vector {
                u: base.u + arm_length * offset.u,
                v: base.v + arm_length * offset.v,
            },
        );
        if rs_cabinet_for_position(
            solution,
            vector {
                u: base.u - arm_length * offset.v,
                v: base.v + arm_length * (offset.u + offset.v),
            },
        ) as i32
            != original_cabinet as i32
        {
            rs_report_collision(
                board,
                vector {
                    u: base.u - arm_length * offset.v,
                    v: base.v + arm_length * (offset.u + offset.v),
                },
                b"grabber went outside cabinet wall\0".as_ptr() as *const c_char,
            );
            return;
        }
        if arm_length == 3_i32
            && rs_cabinet_for_position(
                solution,
                vector {
                    u: base.u + 2_i32 * offset.u - 2_i32 * offset.v,
                    v: base.v + 2_i32 * offset.u + 4_i32 * offset.v,
                },
            ) as i32
                != original_cabinet as i32
        {
            rs_report_collision(
                board,
                vector {
                    u: base.u + 2_i32 * offset.u - 2_i32 * offset.v,
                    v: base.v + 2_i32 * offset.u + 4_i32 * offset.v,
                },
                b"grabber went outside cabinet wall\0".as_ptr() as *const c_char,
            );
            return;
        }
    }
    'c_19449: {
        match arm_length {
            3 => {
                rs_mark_used_area(
                    board,
                    vector {
                        u: base.u + 3_i32 * offset.u - 1_i32 * offset.v,
                        v: base.v + 1_i32 * offset.u + 4_i32 * offset.v,
                    },
                );
                rs_mark_used_area(
                    board,
                    vector {
                        u: base.u + 2_i32 * offset.u - 2_i32 * offset.v,
                        v: base.v + 2_i32 * offset.u + 4_i32 * offset.v,
                    },
                );
                rs_mark_used_area(
                    board,
                    vector {
                        u: base.u + 1_i32 * offset.u - 3_i32 * offset.v,
                        v: base.v + 3_i32 * offset.u + 4_i32 * offset.v,
                    },
                );
            }
            2 => {}
            1 => {
                break 'c_19449;
            }
            _ => {
                abort();
            }
        }
        rs_mark_used_area(
            board,
            vector {
                u: base.u + 2_i32 * offset.u - 1_i32 * offset.v,
                v: base.v + 1_i32 * offset.u + 3_i32 * offset.v,
            },
        );
        rs_mark_used_area(
            board,
            vector {
                u: base.u + 1_i32 * offset.u - 1_i32 * offset.v,
                v: base.v + 1_i32 * offset.u + 2_i32 * offset.v,
            },
        );
        rs_mark_used_area(
            board,
            vector {
                u: base.u + 1_i32 * offset.u - 2_i32 * offset.v,
                v: base.v + 2_i32 * offset.u + 3_i32 * offset.v,
            },
        );
    };
}
pub(crate) unsafe extern "C" fn rs_reserve_movement_index(board: *mut board) -> usize {
    if (*board).movements.length as u64 >= 1_u64.wrapping_neg() >> 29_i32 {
        abort();
    }
    let mut capacity: usize = (*board).movements.capacity;
    while (*board).movements.length >= capacity {
        capacity = 4_usize
            .wrapping_mul(capacity.wrapping_add(12_usize))
            .wrapping_div(3_usize);
    }
    if capacity != (*board).movements.capacity {
        let elems: *mut movement = realloc(
            (*board).movements.movements as *mut c_void,
            ::core::mem::size_of::<movement>().wrapping_mul(capacity),
        ) as *mut movement;
        if elems.is_null() {
            abort();
        }
        (*board).movements.movements = elems;
        (*board).movements.capacity = capacity;
    }
    let c2rust_fresh10 = (*board).movements.length;
    (*board).movements.length = (*board).movements.length.wrapping_add(1);
    c2rust_fresh10
}
pub(crate) unsafe extern "C" fn rs_movements_equal(a: movement, b: movement) -> bool {
    if a.rotation == 0_i32 && b.rotation == 0_i32 {
        return rs_vectors_equal(a.translation, b.translation);
    }
    if a.rotation != b.rotation {
        return false;
    }
    let a_center: vector = if a.r#type == movement_type::SWING_MOVEMENT.0 as i32 as u32 {
        a.base
    } else {
        a.absolute_grab_position
    };
    let b_center: vector = if b.r#type == movement_type::SWING_MOVEMENT.0 as i32 as u32 {
        b.base
    } else {
        b.absolute_grab_position
    };
    rs_vectors_equal(a_center, b_center)
}
pub(crate) unsafe extern "C" fn rs_move_atom(
    board: *mut board,
    a: *mut atom,
    position: vector,
    movement_index: usize,
    chain_atom_list: *mut u32,
) {
    let mut capacity: usize = (*board).moving_atoms.capacity;
    while (*board).moving_atoms.length >= capacity {
        capacity = 4_usize
            .wrapping_mul(capacity.wrapping_add(12_usize))
            .wrapping_div(3_usize);
    }
    if capacity != (*board).moving_atoms.capacity {
        let elems: *mut atom_at_position = realloc(
            (*board).moving_atoms.atoms_at_positions as *mut c_void,
            ::core::mem::size_of::<atom_at_position>().wrapping_mul(capacity),
        ) as *mut atom_at_position;
        if elems.is_null() {
            abort();
        }
        (*board).moving_atoms.atoms_at_positions = elems;
        (*board).moving_atoms.capacity = capacity;
    }
    let c2rust_fresh11 = (*board).moving_atoms.length;
    (*board).moving_atoms.length = (*board).moving_atoms.length.wrapping_add(1);
    *(*board)
        .moving_atoms
        .atoms_at_positions
        .offset(c2rust_fresh11 as isize) = atom_at_position { position, atom: *a };
    if *a & IS_CHAIN_ATOM != 0 {
        rs_move_chain_atom_to_list(
            board,
            rs_lookup_chain_atom(board, position),
            chain_atom_list,
        );
    }
    if rs_remove_atom(board, atom_ref_at_position { atom: a, position }) {
        *a = (*a | (MOVED | ((movement_index as atom) << 29_i32))) as atom;
    }
}
pub(crate) unsafe extern "C" fn rs_move_atoms(
    board: *mut board,
    a: *mut atom,
    mut movement: movement,
) {
    if *a & VALID != 0 && *a & REMOVED != 0 && *a & MOVED != 0 {
        if !rs_movements_equal(
            movement,
            *(*board).movements.movements.offset((*a >> 29_i32) as isize),
        ) {
            rs_report_collision(
                board,
                movement.absolute_grab_position,
                b"atom moved in two directions simultaneously\0".as_ptr() as *const c_char,
            );
        }
        return;
    }
    movement.first_chain_atom = 4294967295_u32;
    movement.first_atom_index = (*board).moving_atoms.cursor;
    let movement_index: usize = rs_reserve_movement_index(board);
    rs_move_atom(
        board,
        a,
        movement.absolute_grab_position,
        movement_index,
        &raw mut movement.first_chain_atom,
    );
    while (*board).moving_atoms.cursor < (*board).moving_atoms.length {
        let m: atom_at_position = *(*board)
            .moving_atoms
            .atoms_at_positions
            .offset((*board).moving_atoms.cursor as isize);
        let mut rs_bond_direction: i32 = 0_i32;
        while rs_bond_direction < 6_i32 {
            if m.atom
                & (RECENT_BOND0
                    | NORMAL_BOND0
                    | TRIPLEX_R_BOND0
                    | TRIPLEX_Y_BOND0
                    | TRIPLEX_K_BOND0)
                    << rs_bond_direction
                & !(0x3f_u64 << 29_i32)
                != 0
            {
                let mut p: vector = m.position;
                let d: vector = rs_u_offset_for_direction(rs_bond_direction);
                p.u += d.u;
                p.v += d.v;
                let b: *mut atom = rs_lookup_atom(board, p);
                if !(*b & VALID == 0 || *b & REMOVED != 0) {
                    rs_move_atom(
                        board,
                        b,
                        p,
                        movement_index,
                        &raw mut movement.first_chain_atom,
                    );
                }
            }
            rs_bond_direction += 1;
        }
        if m.atom & HAS_DISJOINT_BOND != 0 {
            let bond: disjoint_bond =
                rs_lookup_disjoint_bond(&raw mut (*board).disjoint_bond_table, m.position);
            rs_remove_disjoint_bond(&raw mut (*board).disjoint_bond_table, m.position);
            rs_insert_disjoint_bond(&raw mut movement.disjoint_bonds, bond);
            let b_0: *mut atom = rs_lookup_atom(board, bond.to_position);
            if *b_0 & VALID != 0 && *b_0 & REMOVED == 0 {
                rs_move_atom(
                    board,
                    b_0,
                    bond.to_position,
                    movement_index,
                    &raw mut movement.first_chain_atom,
                );
            }
        }
        (*board).moving_atoms.cursor = (*board).moving_atoms.cursor.wrapping_add(1);
    }
    movement.number_of_atoms = (*board)
        .moving_atoms
        .cursor
        .wrapping_sub(movement.first_atom_index);
    *(*board).movements.movements.offset(movement_index as isize) = movement;
}
pub(crate) unsafe extern "C" fn rs_compare_movements_by_number_of_atoms(
    a: *const c_void,
    b: *const c_void,
) -> i32 {
    (*(a as *const movement)).number_of_atoms as i32
        - (*(b as *const movement)).number_of_atoms as i32
}
pub(crate) unsafe extern "C" fn rs_perform_arm_instructions(
    solution: *mut solution,
    board: *mut board,
) {
    (*board).moving_atoms.length = 0_usize;
    (*board).moving_atoms.cursor = 0_usize;
    (*board).movements.length = 0_usize;
    (*board).movements.cursor = 0_usize;
    let n: u32 = (*solution).number_of_arms as u32;
    let mut i: u32 = 0_u32;
    while i < n {
        let m: *mut mechanism = (*solution).arms.offset(i as isize);
        (*m).movement = rs_zero_vector;
        (*m).r#type = ((*m).r#type as c_ulong & !(MOVED_GRABBED_ATOMS)) as u32;
        's_18: {
            if (*board).cycle >= *(*solution).arm_tape_start_cycle.offset(i as isize) as u64 {
                let mut index: usize = ((*board).cycle as usize)
                    .wrapping_sub(*(*solution).arm_tape_start_cycle.offset(i as isize) as usize);
                if *(*solution).arm_tape_halt_index.offset(i as isize) == 18446744073709551615_usize
                    && (*solution).tape_period != 0_u64
                {
                    index = (index as c_ulong).wrapping_rem((*solution).tape_period as c_ulong)
                        as usize;
                }
                if !(index >= *(*solution).arm_tape_length.offset(i as isize)
                    || index >= *(*solution).arm_tape_halt_index.offset(i as isize))
                {
                    let inst: c_char =
                        *(*(*solution).arm_tape.offset(i as isize)).offset(index as isize);
                    if !(inst as i32 == ' ' as i32 || inst as i32 == '\0' as i32) {
                        if ((*board).half_cycle == 1_i32) as i32
                            == (inst as i32 == 'f' as i32 || inst as i32 == 'r' as i32) as i32
                        {
                            let mut track_motion: vector = vector { u: 0_i32, v: 0_i32 };
                            let arm_length: i32 = abs((*m).direction_u.u) | abs((*m).direction_u.v);
                            if inst as i32 == 'f' as i32 {
                                if (*m).r#type as c_ulong & GRABBING != 0
                                    || (*m).r#type & rs_ANY_WHEEL != 0
                                {
                                    break 's_18;
                                }
                            } else if inst as i32 == 'r' as i32 {
                                if (*m).r#type as c_ulong & GRABBING == 0
                                    || (*m).r#type & rs_ANY_WHEEL != 0
                                {
                                    break 's_18;
                                }
                            } else if (inst as i32 == 'w' as i32 || inst as i32 == 's' as i32)
                                && (*m).r#type as c_ulong & PISTON == 0
                            {
                                rs_report_collision(
                                    board,
                                    (*m).position,
                                    b"trying to extend/retract a non-piston arm\0".as_ptr()
                                        as *const c_char,
                                );
                                break 's_18;
                            } else if inst as i32 == 'w' as i32 {
                                if arm_length == 3_i32 {
                                    break 's_18;
                                }
                            } else if inst as i32 == 's' as i32 {
                                if arm_length == 1_i32 {
                                    break 's_18;
                                }
                            } else if inst as i32 == 't' as i32 || inst as i32 == 'g' as i32 {
                                let mut index_0: u32 = 0;
                                if !rs_lookup_track(solution, (*m).position, &raw mut index_0) {
                                    rs_report_collision(
                                        board,
                                        (*m).position,
                                        b"trying to move an arm along a track that isn't on a track\0"
                                            .as_ptr() as *const c_char,
                                    );
                                    break 's_18;
                                } else {
                                    if inst as i32 == 't' as i32 {
                                        track_motion = *(*solution)
                                            .track_minus_motions
                                            .offset(index_0 as isize);
                                    } else if inst as i32 == 'g' as i32 {
                                        track_motion = *(*solution)
                                            .track_plus_motions
                                            .offset(index_0 as isize);
                                    }
                                    if track_motion.u == 0_i32 && track_motion.v == 0_i32 {
                                        break 's_18;
                                    }
                                }
                            }
                            (*m).r#type = ((*m).r#type as c_ulong | MOVED_GRABBED_ATOMS) as u32;
                            let step: i32 = rs_angular_distance_between_grabbers((*m).r#type);
                            let mut direction: i32 = 0_i32;
                            while direction < 6_i32 {
                                let offset: vector = rs_u_offset_for_direction(direction);
                                let p: vector =
                                    rs_mechanism_relative_position(*m, offset.u, offset.v, 1_i32);
                                if inst as i32 == 'a' as i32 {
                                    rs_record_swing_area(solution, board, (*m).position, p, 1_i32);
                                }
                                if inst as i32 == 'd' as i32 {
                                    rs_record_swing_area(solution, board, (*m).position, p, -1_i32);
                                }
                                let a: atom_ref_at_position =
                                    rs_get_atom(board, *m, offset.u, offset.v);
                                if *a.atom != 0 {
                                    if inst as i32 == 'f' as i32 && *a.atom & REMOVED == 0 {
                                        let mut i_0: i32 = 0_i32;
                                        while i_0 < 16_i32 {
                                            if *a.atom & 1_u64 << i_0 + 1_i32 != 0 {
                                                (*board).atom_grabs[i_0 as usize] =
                                                    (*board).atom_grabs[i_0 as usize]
                                                        .wrapping_add(1);
                                                break;
                                            } else {
                                                i_0 += 1;
                                            }
                                        }
                                        let grabs: atom = (*a.atom
                                            & 0x1f_u64.wrapping_mul(GRABBED_ONCE))
                                        .wrapping_div(GRABBED_ONCE)
                                            as atom;
                                        *a.atom = (*a.atom & !0x1f_u64.wrapping_mul(GRABBED_ONCE))
                                            as atom;
                                        *a.atom = (*a.atom
                                            | (grabs.wrapping_add(1 as atom) as u64)
                                                .wrapping_mul(GRABBED_ONCE))
                                            as atom;
                                        (*m).r#type = ((*m).r#type as c_ulong
                                            | (GRABBING_LOW_BIT) << direction)
                                            as u32;
                                    } else if !((*m).r#type as c_ulong
                                        & (GRABBING_LOW_BIT) << direction
                                        == 0
                                        || *a.atom & REMOVED == 0
                                            && *a.atom & 0x1f_u64.wrapping_mul(GRABBED_ONCE) == 0)
                                    {
                                        if inst as i32 == 'r' as i32
                                            && *a.atom & REMOVED == 0
                                            && *a.atom & 0x1f_u64.wrapping_mul(GRABBED_ONCE) != 0
                                        {
                                            let grabs_0: atom = (*a.atom
                                                & 0x1f_u64.wrapping_mul(GRABBED_ONCE))
                                            .wrapping_div(GRABBED_ONCE)
                                                as atom;
                                            *a.atom = (*a.atom
                                                & !0x1f_u64.wrapping_mul(GRABBED_ONCE))
                                                as atom;
                                            *a.atom = (*a.atom
                                                | (grabs_0.wrapping_sub(1 as atom) as u64)
                                                    .wrapping_mul(GRABBED_ONCE))
                                                as atom;
                                            *a.atom = (*a.atom | BEING_DROPPED) as atom;
                                            rs_schedule_flag_reset_if_needed(board, a.atom);
                                        } else {
                                            let mut movement: movement = movement {
                                                number_of_atoms: 0,
                                                first_atom_index: 0,
                                                r#type: (if (*m).r#type as c_ulong & PISTON != 0 {
                                                    movement_type::IS_PISTON.0 as i32
                                                } else {
                                                    0_i32
                                                })
                                                    as u32,
                                                base: (*m).position,
                                                grabber_offset: offset,
                                                absolute_grab_position: a.position,
                                                translation: vector { u: 0, v: 0 },
                                                base_rotation: (*m).arm_rotation,
                                                rotation: 0,
                                                piston_extension: 0,
                                                first_chain_atom: 0,
                                                disjoint_bonds: disjoint_bond_table {
                                                    bonds: ::core::ptr::null_mut::<disjoint_bond>(),
                                                    size: 0,
                                                    count: 0,
                                                },
                                            };
                                            movement.grabber_offset.u *= arm_length;
                                            movement.grabber_offset.v *= arm_length;
                                            match inst as i32 {
                                                113 => {
                                                    movement.r#type |=
                                                        movement_type::PIVOT_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.rotation = 1_i32;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                101 => {
                                                    movement.r#type |=
                                                        movement_type::PIVOT_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.rotation = -1_i32;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                97 => {
                                                    movement.r#type |=
                                                        movement_type::SWING_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.rotation = 1_i32;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                100 => {
                                                    movement.r#type |=
                                                        movement_type::SWING_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.rotation = -1_i32;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                119 => {
                                                    movement.r#type |=
                                                        movement_type::PISTON_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.piston_extension = 1_i32;
                                                    movement.translation =
                                                        rs_normalize_axis((*m).direction_u);
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                115 => {
                                                    movement.r#type |=
                                                        movement_type::PISTON_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.piston_extension = -1_i32;
                                                    movement.translation =
                                                        rs_normalize_axis((*m).direction_u);
                                                    movement.translation.u =
                                                        -movement.translation.u;
                                                    movement.translation.v =
                                                        -movement.translation.v;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                116 | 103 => {
                                                    movement.r#type |=
                                                        movement_type::TRACK_MOVEMENT.0 as i32
                                                            as u32;
                                                    movement.translation = track_motion;
                                                    rs_move_atoms(board, a.atom, movement);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                }
                                direction += step;
                            }
                            match inst as i32 {
                                113 | 101 => {
                                    (*m).pivot_parity = ((*m).pivot_parity == 0) as i32;
                                }
                                97 => {
                                    let u: vector = rs_u_offset_for_direction(1_i32);
                                    let v: vector = rs_v_offset_for_direction(1_i32);
                                    let nu: vector = vector {
                                        u: (*m).direction_u.u * u.u + (*m).direction_v.u * u.v,
                                        v: (*m).direction_u.v * u.u + (*m).direction_v.v * u.v,
                                    };
                                    let nv: vector = vector {
                                        u: (*m).direction_u.u * v.u + (*m).direction_v.u * v.v,
                                        v: (*m).direction_u.v * v.u + (*m).direction_v.v * v.v,
                                    };
                                    (*m).direction_u = nu;
                                    (*m).direction_v = nv;
                                    (*m).arm_rotation += 1;
                                    (*m).arm_rotation = (*m).arm_rotation % 6_i32;
                                }
                                100 => {
                                    let u_0: vector = rs_u_offset_for_direction(-1_i32);
                                    let v_0: vector = rs_v_offset_for_direction(-1_i32);
                                    let nu_0: vector = vector {
                                        u: (*m).direction_u.u * u_0.u + (*m).direction_v.u * u_0.v,
                                        v: (*m).direction_u.v * u_0.u + (*m).direction_v.v * u_0.v,
                                    };
                                    let nv_0: vector = vector {
                                        u: (*m).direction_u.u * v_0.u + (*m).direction_v.u * v_0.v,
                                        v: (*m).direction_u.v * v_0.u + (*m).direction_v.v * v_0.v,
                                    };
                                    (*m).direction_u = nu_0;
                                    (*m).direction_v = nv_0;
                                    (*m).arm_rotation -= 1;
                                    (*m).arm_rotation = (*m).arm_rotation % 6_i32;
                                }
                                119 => {
                                    rs_adjust_axis_magnitude(&raw mut (*m).direction_u, 1_i32);
                                    rs_adjust_axis_magnitude(&raw mut (*m).direction_v, 1_i32);
                                }
                                115 => {
                                    rs_adjust_axis_magnitude(&raw mut (*m).direction_u, -1_i32);
                                    rs_adjust_axis_magnitude(&raw mut (*m).direction_v, -1_i32);
                                }
                                116 | 103 => {
                                    (*m).position.u += track_motion.u;
                                    (*m).position.v += track_motion.v;
                                    (*m).movement = track_motion;
                                }
                                102 => {
                                    (*m).r#type = ((*m).r#type as c_ulong | GRABBING) as u32;
                                }
                                114 => {
                                    (*m).r#type &= !rs_GRABBING_EVERYTHING;
                                    (*m).r#type = ((*m).r#type as c_ulong & !(GRABBING)) as u32;
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if (*board).half_cycle == 2_i32 {
        let mut i_1: u32 = 0_u32;
        while i_1 < n {
            let m_0: *mut mechanism = (*solution).arms.offset(i_1 as isize);
            if (*board).cycle >= *(*solution).arm_tape_start_cycle.offset(i_1 as isize) as u64 {
                if (*m_0).r#type as c_ulong & MOVED_GRABBED_ATOMS == 0 {
                    let step_0: i32 = rs_angular_distance_between_grabbers((*m_0).r#type);
                    let mut direction_0: i32 = 0_i32;
                    while direction_0 < 6_i32 {
                        if (*m_0).r#type as c_ulong & (GRABBING_LOW_BIT) << direction_0 != 0 {
                            let offset_0: vector = rs_u_offset_for_direction(direction_0);
                            let pos: vector =
                                rs_mechanism_relative_position(*m_0, offset_0.u, offset_0.v, 1_i32);
                            let a_0: *mut atom = rs_lookup_atom(board, pos);
                            if *a_0 & VALID != 0 && *a_0 & REMOVED != 0 && *a_0 & MOVED != 0 {
                                rs_report_collision(
                                    board,
                                    pos,
                                    b"atom moved while being held stationary by another arm\0"
                                        .as_ptr()
                                        as *const c_char,
                                );
                            }
                        }
                        direction_0 += step_0;
                    }
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
        let mut atom_index: usize = 0_usize;
        qsort(
            (*board).movements.movements as *mut c_void,
            (*board).movements.length,
            ::core::mem::size_of::<movement>(),
            Some(
                rs_compare_movements_by_number_of_atoms
                    as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
            ),
        );
        let offset_1: usize = (*board).moving_atoms.length;
        if offset_1.wrapping_mul(2_usize) > (*board).moving_atoms.capacity {
            (*board).moving_atoms.capacity = offset_1.wrapping_mul(2_usize);
            (*board).moving_atoms.atoms_at_positions = realloc(
                (*board).moving_atoms.atoms_at_positions as *mut c_void,
                ::core::mem::size_of::<atom_at_position>()
                    .wrapping_mul((*board).moving_atoms.capacity),
            ) as *mut atom_at_position;
        }
        memcpy(
            (*board)
                .moving_atoms
                .atoms_at_positions
                .offset(offset_1 as isize) as *mut c_void,
            (*board).moving_atoms.atoms_at_positions as *const c_void,
            ::core::mem::size_of::<atom_at_position>().wrapping_mul(offset_1),
        );
        let mut i_2: usize = 0_usize;
        while i_2 < (*board).movements.length {
            let m_1: *mut movement = (*board).movements.movements.offset(i_2 as isize);
            memcpy(
                (*board)
                    .moving_atoms
                    .atoms_at_positions
                    .offset(atom_index as isize) as *mut c_void,
                (*board)
                    .moving_atoms
                    .atoms_at_positions
                    .offset(offset_1 as isize)
                    .offset((*m_1).first_atom_index as isize) as *const c_void,
                (*m_1)
                    .number_of_atoms
                    .wrapping_mul(::core::mem::size_of::<atom_at_position>()),
            );
            (*m_1).first_atom_index = atom_index;
            atom_index = atom_index.wrapping_add((*m_1).number_of_atoms);
            i_2 = i_2.wrapping_add(1);
        }
        let mut maximum_rotation_distance: i32 = 1_i32;
        atom_index = 0_usize;
        let mut i_3: usize = 0_usize;
        while i_3 < (*board).movements.length {
            let m_2: *mut movement = (*board).movements.movements.offset(i_3 as isize);
            if (*m_2).first_chain_atom != 4294967295_u32 {
                (*(*board)
                    .chain_atoms
                    .offset((*m_2).first_chain_atom as isize))
                .prev_in_list = &raw mut (*m_2).first_chain_atom;
            }
            let base: vector =
                if (*m_2).r#type & 3_u32 == movement_type::SWING_MOVEMENT.0 as i32 as u32 {
                    (*m_2).base
                } else {
                    (*m_2).absolute_grab_position
                };
            let u_1: vector = rs_u_offset_for_direction((*m_2).rotation);
            let v_1: vector = rs_v_offset_for_direction((*m_2).rotation);
            let mut j: usize = 0_usize;
            while j < (*m_2).number_of_atoms {
                let c2rust_fresh8 = atom_index;
                atom_index = atom_index.wrapping_add(1);
                let ap: *mut atom_at_position = (*board)
                    .moving_atoms
                    .atoms_at_positions
                    .offset(c2rust_fresh8 as isize);
                rs_apply_movement_to_position(
                    base,
                    (*m_2).translation,
                    u_1,
                    v_1,
                    &raw mut (*ap).position,
                );
                if (*m_2).rotation != 0_i32 {
                    let mut delta: vector = (*ap).position;
                    delta.u -= base.u;
                    delta.v -= base.v;
                    let distance: i32 = rs_vector_hexicab_length(delta) as i32;
                    if distance > maximum_rotation_distance {
                        maximum_rotation_distance = distance;
                    }
                    rs_rotate_bonds(&raw mut (*ap).atom, (*m_2).rotation);
                }
                j = j.wrapping_add(1);
            }
            let mut chain: u32 = (*m_2).first_chain_atom;
            while chain != 4294967295_u32 {
                let ca: *mut chain_atom = (*board).chain_atoms.offset(chain as isize);
                rs_apply_movement_to_position(
                    base,
                    (*m_2).translation,
                    u_1,
                    v_1,
                    &raw mut (*ca).current_position,
                );
                if (*board).chain_mode.0 == chain_mode::EXTEND_CHAIN.0 {
                    rs_apply_movement_to_position(
                        base,
                        (*m_2).translation,
                        u_1,
                        v_1,
                        &raw mut (*ca).original_position,
                    );
                }
                let rotation: i32 = ((*ca).flags & 7_u32) as i32;
                (*ca).flags = (*ca).flags & !7_u32;
                (*ca).flags |= rs_normalize_direction(rotation + (*m_2).rotation) as u32;
                if (*m_2).rotation > 0_i32 {
                    let mut sextants: u32 =
                        ((*ca).flags & 63_u32 << 4_i32) >> 1_i32 & 63_u32 << 4_i32;
                    sextants = sextants | 1_u32 << 4_i32 + 5_i32;
                    (*ca).flags = (*ca).flags & !(63_u32 << 4_i32);
                    (*ca).flags |= sextants;
                } else if (*m_2).rotation < 0_i32 {
                    let mut sextants_0: u32 =
                        ((*ca).flags & 63_u32 << 4_i32) << 1_i32 & 63_u32 << 4_i32;
                    sextants_0 = sextants_0 | 1_u32 << 4_i32;
                    (*ca).flags = (*ca).flags & !(63_u32 << 4_i32);
                    (*ca).flags |= sextants_0;
                }
                chain = (*ca).next_in_list;
            }
            let mut j_0: u32 = 0_u32;
            while j_0 < (*m_2).disjoint_bonds.size {
                let bond: *mut disjoint_bond = (*m_2).disjoint_bonds.bonds.offset(j_0 as isize);
                rs_apply_movement_to_position(
                    base,
                    (*m_2).translation,
                    u_1,
                    v_1,
                    &raw mut (*bond).from_position,
                );
                rs_apply_movement_to_position(
                    base,
                    (*m_2).translation,
                    u_1,
                    v_1,
                    &raw mut (*bond).to_position,
                );
                j_0 = j_0.wrapping_add(1);
            }
            if (*m_2).r#type & 3_u32 == movement_type::TRACK_MOVEMENT.0 as i32 as u32 {
                (*m_2).base.u += (*m_2).translation.u;
                (*m_2).base.v += (*m_2).translation.v;
            } else if (*m_2).r#type & 3_u32 == movement_type::PISTON_MOVEMENT.0 as i32 as u32 {
                let g: vector = rs_normalize_axis((*m_2).grabber_offset);
                (*m_2).grabber_offset.u = if g.u != 0 {
                    g.u * ((*m_2).grabber_offset.u / g.u + (*m_2).piston_extension)
                } else {
                    0_i32
                };
                (*m_2).grabber_offset.v = if g.v != 0 {
                    g.v * ((*m_2).grabber_offset.v / g.v + (*m_2).piston_extension)
                } else {
                    0_i32
                };
            }
            rs_apply_movement_to_position(
                base,
                (*m_2).translation,
                u_1,
                v_1,
                &raw mut (*m_2).absolute_grab_position,
            );
            if (*m_2).r#type & 3_u32 == movement_type::SWING_MOVEMENT.0 as i32 as u32 {
                (*m_2).base_rotation += (*m_2).rotation;
                (*m_2).base_rotation = (*m_2).base_rotation % 6_i32;
            }
            i_3 = i_3.wrapping_add(1);
        }
        let mut collision_increment: f64 = 0.25f64
            / pow(
                2_f64,
                round(log(maximum_rotation_distance as f64) / log(2_f64)),
            );
        if !(collision_increment <= 0.125f64) {
            collision_increment = 0.125f64;
        }
        let mut collision_location: vector = vector { u: 0, v: 0 };
        if crate::collision::rs_collision(
            solution,
            board,
            collision_increment as f32,
            (&raw mut collision_location).cast(),
        ) {
            rs_report_collision(
                board,
                collision_location,
                b"collision during motion phase\0".as_ptr() as *const c_char,
            );
        }
        if (*board).collision_check_limit > 0_u64
            && (*board).collision_checks > (*board).collision_check_limit
        {
            rs_report_collision(
                board,
                rs_zero_vector,
                b"solution reached limit for maximum number of collision checks\0".as_ptr()
                    as *const c_char,
            );
        }
        atom_index = 0_usize;
        let mut i_4: usize = 0_usize;
        while i_4 < (*board).movements.length {
            let mut m_3: movement = *(*board).movements.movements.offset(i_4 as isize);
            let mut j_1: usize = 0_usize;
            while j_1 < m_3.number_of_atoms {
                let c2rust_fresh9 = atom_index;
                atom_index = atom_index.wrapping_add(1);
                let mut ap_0: atom_at_position = *(*board)
                    .moving_atoms
                    .atoms_at_positions
                    .offset(c2rust_fresh9 as isize);
                if rs_position_may_be_visible_to_solution(solution, ap_0.position) {
                    ap_0.atom = (ap_0.atom & !(IS_CHAIN_ATOM)) as atom;
                }
                rs_insert_atom(board, ap_0.position, (VALID | ap_0.atom) as atom);
                j_1 = j_1.wrapping_add(1);
            }
            let mut chain_0: u32 = m_3.first_chain_atom;
            while chain_0 != 4294967295_u32 {
                let ca_0: chain_atom = *(*board).chain_atoms.offset(chain_0 as isize);
                if rs_position_may_be_visible_to_solution(solution, ca_0.current_position) {
                    if (*board).chain_mode.0 == chain_mode::EXTEND_CHAIN.0 {
                        (*board).chain_will_become_visible = true;
                    }
                    rs_move_chain_atom_to_list(board, chain_0, ::core::ptr::null_mut::<u32>());
                } else if (*board).chain_mode.0 == chain_mode::EXTEND_CHAIN.0
                    && rs_position_may_eventually_be_visible_to_solution(
                        solution,
                        ca_0.original_position,
                        ca_0.current_position,
                    ) as i32
                        != 0
                {
                    (*board).chain_will_become_visible = true;
                    let c2rust_lvalue_ptr = &raw mut *rs_lookup_atom_in_grid(
                        &raw mut (*board).grid,
                        ca_0.current_position,
                    );
                    *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr & !(IS_CHAIN_ATOM)) as atom;
                    rs_move_chain_atom_to_list(board, chain_0, ::core::ptr::null_mut::<u32>());
                } else {
                    rs_add_chain_atom_to_table(board, chain_0);
                }
                chain_0 = ca_0.next_in_list;
            }
            let mut j_2: u32 = 0_u32;
            while j_2 < m_3.disjoint_bonds.size {
                let bond_0: disjoint_bond = *m_3.disjoint_bonds.bonds.offset(j_2 as isize);
                if !rs_vectors_equal(bond_0.from_position, bond_0.to_position) {
                    rs_insert_disjoint_bond(&raw mut (*board).disjoint_bond_table, bond_0);
                }
                j_2 = j_2.wrapping_add(1);
            }
            free(m_3.disjoint_bonds.bonds as *mut c_void);
            m_3.disjoint_bonds.size = 0_u32;
            i_4 = i_4.wrapping_add(1);
        }
    }
}
pub(crate) unsafe extern "C" fn rs_mark_arm_area(solution: *mut solution, board: *mut board) {
    let mut i: usize = 0_usize;
    while i < (*solution).number_of_arms {
        let m: *mut mechanism = (*solution).arms.offset(i as isize);
        let step: i32 = rs_angular_distance_between_grabbers((*m).r#type);
        let mut direction: i32 = 0_i32;
        while direction < 6_i32 {
            let offset: vector = rs_u_offset_for_direction(direction);
            if (*solution).production as i32 != 0
                && rs_cabinet_for_position(
                    solution,
                    rs_mechanism_relative_position(*m, offset.u, offset.v, 1_i32),
                ) as i32
                    == 255_i32
            {
                rs_report_collision(
                    board,
                    rs_mechanism_relative_position(*m, offset.u, offset.v, 1_i32),
                    b"grabber went outside cabinet wall\0".as_ptr() as *const c_char,
                );
            }
            let saved_u: vector = (*m).direction_u;
            let saved_v: vector = (*m).direction_v;
            loop {
                let p: vector = rs_mechanism_relative_position(*m, offset.u, offset.v, 1_i32);
                rs_mark_used_area(board, p);
                if rs_vectors_equal((*m).direction_u, rs_zero_vector) {
                    break;
                }
                rs_adjust_axis_magnitude(&raw mut (*m).direction_u, -1_i32);
                rs_adjust_axis_magnitude(&raw mut (*m).direction_v, -1_i32);
            }
            (*m).direction_u = saved_u;
            (*m).direction_v = saved_v;
            direction += step;
        }
        i = i.wrapping_add(1);
    }
}
