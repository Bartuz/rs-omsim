//! Atoms on the board: access as mechanisms see them, removal, bonds, molecules (flood
//! fill) and overlap stacks.
use super::*;

#[inline]
unsafe fn rs_output_cell_free(a: *mut atom) -> bool {
    *a & VALID == 0 || *a & REMOVED != 0 || *a & WHEEL_ATOM != 0
}
pub(crate) unsafe extern "C" fn rs_add_produced_atom_collider(board: *mut board, pos: vector) {
    if (*board).number_of_atoms_being_produced >= (*board).atoms_being_produced_capacity {
        (*board).atoms_being_produced_capacity = (*board)
            .atoms_being_produced_capacity
            .wrapping_add(3_u32)
            .wrapping_mul(4_u32)
            .wrapping_div(3_u32);
        (*board).atoms_being_produced = realloc(
            (*board).atoms_being_produced as *mut c_void,
            ((*board).atoms_being_produced_capacity as usize)
                .wrapping_mul(::core::mem::size_of::<vector>()),
        ) as *mut vector;
    }
    let c2rust_fresh6 = (*board).number_of_atoms_being_produced;
    (*board).number_of_atoms_being_produced =
        (*board).number_of_atoms_being_produced.wrapping_add(1);
    *(*board).atoms_being_produced.offset(c2rust_fresh6 as isize) = pos;
}
pub(crate) unsafe extern "C" fn rs_produce_atom(
    board: *mut board,
    m: mechanism,
    du: i32,
    dv: i32,
    a: atom,
) {
    let pos: vector = rs_mechanism_relative_position(m, du, dv, 1_i32);
    if (*board).half_cycle == 1_i32 {
        rs_add_produced_atom_collider(board, pos);
    } else {
        rs_insert_atom(board, pos, (a | VALID) as atom);
    };
}
pub(crate) unsafe extern "C" fn rs_get_atom(
    board: *mut board,
    m: mechanism,
    du: i32,
    dv: i32,
) -> atom_ref_at_position {
    let pos: vector = rs_mechanism_relative_position(m, du, dv, 1_i32);
    rs_visible_atom(rs_lookup_atom(board, pos), pos, m.r#type)
}
/// The atom at `a` as a mechanism of `mechanism_type` sees it (C: the rest of get_atom).
#[inline]
unsafe fn rs_visible_atom(a: *mut atom, pos: vector, mechanism_type: u32) -> atom_ref_at_position {
    static mut rs_empty: atom = 0;
    if *a & VALID == 0 {
        return atom_ref_at_position {
            atom: &raw const rs_empty as *mut atom,
            position: pos,
        };
    }
    if mechanism_type & rs_ANY_ARM != 0 && *a & REMOVED != 0 && *a & MOVED != 0 {
        return atom_ref_at_position {
            atom: a,
            position: pos,
        };
    }
    if *a & REMOVED != 0 {
        return atom_ref_at_position {
            atom: &raw const rs_empty as *mut atom,
            position: pos,
        };
    }
    if mechanism_type as c_ulong & (DUPLICATION | PROLIFERATION | rs_ANY_WHEEL as c_ulong) == 0
        && *a & WHEEL_ATOM != 0
    {
        return atom_ref_at_position {
            atom: &raw const rs_empty as *mut atom,
            position: pos,
        };
    }
    atom_ref_at_position {
        atom: a,
        position: pos,
    }
}
/// A conversion consumes only atoms that are unbonded and not held (C: conversion_input).
#[inline]
unsafe fn rs_unbound_input(input: atom_ref_at_position) -> atom_ref_at_position {
    static mut rs_empty: atom = 0;
    if *input.atom
        & (0x3f_u64.wrapping_mul(
            RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0,
        ) | HAS_DISJOINT_BOND
            | 0x1f_u64.wrapping_mul(GRABBED_ONCE))
        != 0
    {
        return atom_ref_at_position {
            atom: &raw const rs_empty as *mut atom,
            position: input.position,
        };
    }
    input
}
/// A glyph read through the board's glyph plan: the slot rs_lookup_atom would return,
/// without hashing once the cell is linked.
#[inline]
pub(crate) unsafe fn rs_glyph_slot(
    board: *mut board,
    glyph: *mut crate::glyph_plan::Glyph,
    du: i32,
    dv: i32,
) -> atom_ref_at_position {
    debug_assert!((-1..=1).contains(&du) && (-1..=1).contains(&dv));
    let cell = (*glyph).cells.as_mut_ptr().add(((du + 1) * 3 + dv + 1) as usize);
    let a = if (*cell).slot == usize::MAX {
        rs_link_glyph_cell(board, cell)
    } else {
        (*board).grid.atoms_at_positions.add((*cell).slot)
    };
    atom_ref_at_position {
        atom: &raw mut (*a).atom,
        position: (*cell).position,
    }
}
/// Looks the cell up, and caches the slot once it belongs to this position until the next
/// rehash: always in the fixed array; in the hash table once it is labelled. A label is
/// never cleared (removal leaves VALID|REMOVED), so a repeated lookup takes the same
/// probes to the same slot and cannot trigger a rehash. An empty hash slot is not
/// reserved: another position may take it.
#[cold]
#[inline(never)]
unsafe fn rs_link_glyph_cell(
    board: *mut board,
    cell: *mut crate::glyph_plan::Cell,
) -> *mut atom_at_position {
    let a = rs_lookup_atom_at_position(&raw mut (*board).grid, (*cell).position);
    let slot = a.offset_from((*board).grid.atoms_at_positions) as usize;
    if slot < 4096 || (*a).atom != 0 {
        (*cell).slot = slot;
    }
    a
}
#[inline]
pub(crate) unsafe fn rs_get_glyph_atom(
    board: *mut board,
    glyph: *mut crate::glyph_plan::Glyph,
    mechanism_type: u32,
    du: i32,
    dv: i32,
) -> atom_ref_at_position {
    let a = rs_glyph_slot(board, glyph, du, dv);
    rs_visible_atom(a.atom, a.position, mechanism_type)
}
#[inline]
pub(crate) unsafe fn rs_glyph_conversion_input(
    board: *mut board,
    glyph: *mut crate::glyph_plan::Glyph,
    mechanism_type: u32,
    du: i32,
    dv: i32,
) -> atom_ref_at_position {
    rs_unbound_input(rs_get_glyph_atom(board, glyph, mechanism_type, du, dv))
}
#[inline]
pub(crate) unsafe fn rs_glyph_conversion_output(
    board: *mut board,
    glyph: *mut crate::glyph_plan::Glyph,
    du: i32,
    dv: i32,
) -> bool {
    rs_output_cell_free(rs_glyph_slot(board, glyph, du, dv).atom)
}
#[inline(never)]
pub(crate) unsafe extern "C" fn rs_remove_overlapping_atom(
    board: *mut board,
    a: atom_ref_at_position,
) {
    let mut i: u32 = 0_u32;
    while i < (*board).number_of_overlapped_atoms {
        if rs_vectors_equal(
            (*(*board).overlapped_atoms.offset(i as isize)).position,
            a.position,
        ) {
            *a.atom = (*(*board).overlapped_atoms.offset(i as isize)).atom;
            crate::board_index::changed(board, a.atom);
            rs_schedule_flag_reset_if_needed(board, a.atom);
            memmove(
                (*board).overlapped_atoms.offset(i as isize) as *mut c_void,
                (*board).overlapped_atoms.offset(i as isize).offset(1_isize) as *const c_void,
                ((*board)
                    .number_of_overlapped_atoms
                    .wrapping_sub(i)
                    .wrapping_sub(1_u32) as usize)
                    .wrapping_mul(::core::mem::size_of::<atom_at_position>()),
            );
            (*board).number_of_overlapped_atoms =
                (*board).number_of_overlapped_atoms.wrapping_sub(1);
            return;
        }
        i = i.wrapping_add(1);
    }
}
#[inline]
pub(crate) unsafe extern "C" fn rs_remove_atom(board: *mut board, a: atom_ref_at_position) -> bool {
    if *a.atom & OVERLAPS_ATOMS != 0 {
        rs_remove_overlapping_atom(board, a);
        false
    } else {
        *a.atom = (VALID | REMOVED) as atom;
        crate::board_index::changed(board, a.atom);
        true
    }
}
#[inline]
pub(crate) unsafe extern "C" fn rs_transform_atom(a: *mut atom, new_type: atom) {
    *a = (*a
        & !(SALT
            | (WATER | FIRE | EARTH | AIR)
            | QUICKSILVER
            | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
            | VITAE
            | MORS
            | QUINTESSENCE)) as atom;
    *a |= new_type;
}
pub(crate) unsafe extern "C" fn rs_add_bond(
    m: mechanism,
    a: *mut atom,
    b: *mut atom,
    u: i32,
    v: i32,
    bond: atom,
    unless: atom,
) {
    let ab: atom = rs_bond_direction(m, u, v);
    let ba: atom = rs_bond_direction(m, -u, -v);
    if *a & ab & unless != 0 || *b & ba & unless != 0 {
        return;
    }
    *a |= ab & bond;
    *b |= ba & bond;
}
pub(crate) unsafe extern "C" fn rs_get_bond_height(
    board: *mut board,
    a: atom_ref_at_position,
    ab: atom,
) -> i32 {
    let lowest: *mut atom = rs_lookup_atom(board, a.position);
    if lowest == a.atom {
        return 0_i32;
    }
    let mut height: i32 = 0_i32;
    if *lowest & ab != 0 {
        height += 1;
    }
    let mut i: u32 = 0_u32;
    while i < (*board).number_of_overlapped_atoms {
        let overlap: *mut atom_at_position = (*board).overlapped_atoms.offset(i as isize);
        if &raw mut (*overlap).atom == a.atom {
            break;
        }
        if rs_vectors_equal((*overlap).position, a.position) as i32 != 0
            && (*overlap).atom & ab != 0
        {
            height += 1;
        }
        i = i.wrapping_add(1);
    }
    height
}
pub(crate) unsafe extern "C" fn rs_lookup_atom_at_height(
    board: *mut board,
    b: atom_ref_at_position,
    ba: atom,
    mut height: i32,
) -> *mut atom {
    if *b.atom & ba != 0 {
        height -= 1;
        if height < 0_i32 {
            return b.atom;
        }
    }
    let mut i: u32 = 0_u32;
    while i < (*board).number_of_overlapped_atoms {
        let overlap: *mut atom_at_position = (*board).overlapped_atoms.offset(i as isize);
        if rs_vectors_equal((*overlap).position, b.position) as i32 != 0
            && (*overlap).atom & ba != 0
        {
            height -= 1;
            if height < 0_i32 {
                return &raw mut (*overlap).atom;
            }
        }
        i = i.wrapping_add(1);
    }
    if (*board).rs_collision as i32 != 0 {
    } else {
        __assert_fail(
            b"board->collision\0".as_ptr() as *const c_char,
            b"sim.c\0".as_ptr()
                as *const c_char,
            394_u32,
            b"atom *lookup_atom_at_height(struct board *, struct atom_ref_at_position, atom, int)\0"
                .as_ptr() as *const c_char,
        );
    };
    ::core::ptr::null_mut::<atom>()
}
pub(crate) unsafe extern "C" fn rs_add_atom_to_molecule(
    board: *mut board,
    molecule: *mut molecule,
    a: atom_ref_at_position,
) {
    if (*molecule).size >= (*molecule).capacity {
        (*molecule).capacity = if (*molecule).capacity != 0 {
            (*molecule)
                .capacity
                .wrapping_mul(3_usize)
                .wrapping_div(2_usize)
        } else {
            8_usize
        };
        (*molecule).atoms = realloc(
            (*molecule).atoms as *mut c_void,
            (*molecule)
                .capacity
                .wrapping_mul(::core::mem::size_of::<atom_ref_at_position>()),
        ) as *mut atom_ref_at_position;
    }
    let c2rust_fresh3 = (*molecule).size;
    (*molecule).size = (*molecule).size.wrapping_add(1);
    *(*molecule).atoms.offset(c2rust_fresh3 as isize) = a as atom_ref_at_position;
}
pub(crate) unsafe extern "C" fn rs_get_molecule(
    board: *mut board,
    mut a: atom_ref_at_position,
) -> *mut molecule {
    (*board).molecule.cursor = 0_usize;
    (*board).molecule.size = 0_usize;
    if *a.atom
        & (SALT
            | (WATER | FIRE | EARTH | AIR)
            | QUICKSILVER
            | (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
            | VITAE
            | MORS
            | QUINTESSENCE)
        == 0
    {
        return &raw mut (*board).molecule;
    }
    let c2rust_lvalue_ptr = &raw mut *rs_lookup_atom(board, a.position);
    *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr | VISITED) as atom;
    rs_add_atom_to_molecule(board, &raw mut (*board).molecule, a);
    while (*board).molecule.cursor < (*board).molecule.size {
        let c2rust_fresh2 = (*board).molecule.cursor;
        (*board).molecule.cursor = (*board).molecule.cursor.wrapping_add(1);
        a = *(*board).molecule.atoms.offset(c2rust_fresh2 as isize) as atom_ref_at_position;
        let mut rs_bond_direction: i32 = 0_i32;
        while rs_bond_direction < 6_i32 {
            let ab: atom = (((RECENT_BOND0
                | NORMAL_BOND0
                | TRIPLEX_R_BOND0
                | TRIPLEX_Y_BOND0
                | TRIPLEX_K_BOND0)
                & !(0x3f_u64 << 29_i32))
                << rs_bond_direction) as atom;
            if *a.atom & ab != 0 {
                let d: vector = rs_u_offset_for_direction(rs_bond_direction);
                let p: vector = vector {
                    u: a.position.u + d.u,
                    v: a.position.v + d.v,
                };
                let mut b: atom_ref_at_position = atom_ref_at_position {
                    atom: rs_lookup_atom(board, p),
                    position: p,
                };
                if *b.atom & VISITED == 0 {
                    *b.atom = (*b.atom | VISITED) as atom;
                    let height: i32 = rs_get_bond_height(board, a, ab);
                    let opposite_direction: i32 = rs_bond_direction
                        + (if rs_bond_direction < 3_i32 {
                            3_i32
                        } else {
                            -3_i32
                        });
                    let ba: atom = (((RECENT_BOND0
                        | NORMAL_BOND0
                        | TRIPLEX_R_BOND0
                        | TRIPLEX_Y_BOND0
                        | TRIPLEX_K_BOND0)
                        & !(0x3f_u64 << 29_i32))
                        << opposite_direction) as atom;
                    b.atom = rs_lookup_atom_at_height(board, b, ba, height);
                    if !b.atom.is_null() {
                        rs_add_atom_to_molecule(board, &raw mut (*board).molecule, b);
                    }
                }
            }
            rs_bond_direction += 1;
        }
        if *a.atom & HAS_DISJOINT_BOND != 0 {
            let bond: disjoint_bond =
                rs_lookup_disjoint_bond(&raw mut (*board).disjoint_bond_table, a.position);
            let b_0: atom_ref_at_position = atom_ref_at_position {
                atom: rs_lookup_atom(board, bond.to_position),
                position: bond.to_position,
            };
            if *b_0.atom & VISITED == 0 {
                *b_0.atom = (*b_0.atom | VISITED) as atom;
                rs_add_atom_to_molecule(board, &raw mut (*board).molecule, b_0);
            }
        }
    }
    let mut i: u32 = 0_u32;
    while (i as usize) < (*board).molecule.size {
        let c2rust_lvalue_ptr_0 = &raw mut *rs_lookup_atom(
            board,
            (*(*board).molecule.atoms.offset(i as isize)).position,
        );
        *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0 & !(VISITED)) as atom;
        i = i.wrapping_add(1);
    }
    &raw mut (*board).molecule
}
pub(crate) unsafe extern "C" fn rs_remove_molecule(board: *mut board, molecule: *mut molecule) {
    let mut i: u32 = 0_u32;
    while (i as usize) < (*molecule).size {
        let a: atom_ref_at_position = *(*molecule).atoms.offset(i as isize) as atom_ref_at_position;
        let mut bot: *mut atom = rs_lookup_atom(board, a.position);
        if a.atom == bot {
            rs_remove_atom(board, a);
        } else {
            let mut j: u32 = 0_u32;
            while j < (*board).number_of_overlapped_atoms {
                if &raw mut (*(*board).overlapped_atoms.offset(j as isize)).atom == a.atom {
                    if *a.atom & OVERLAPS_ATOMS == 0 {
                        *bot = (*bot & !(OVERLAPS_ATOMS)) as atom;
                    }
                    memmove(
                        (*board).overlapped_atoms.offset(j as isize) as *mut c_void,
                        (*board).overlapped_atoms.offset(j as isize).offset(1_isize)
                            as *const c_void,
                        ((*board)
                            .number_of_overlapped_atoms
                            .wrapping_sub(j)
                            .wrapping_sub(1_u32) as usize)
                            .wrapping_mul(::core::mem::size_of::<atom_at_position>()),
                    );
                    (*board).number_of_overlapped_atoms =
                        (*board).number_of_overlapped_atoms.wrapping_sub(1);
                    break;
                } else {
                    if rs_vectors_equal(
                        (*(*board).overlapped_atoms.offset(j as isize)).position,
                        a.position,
                    ) {
                        bot = &raw mut (*(*board).overlapped_atoms.offset(j as isize)).atom;
                    }
                    j = j.wrapping_add(1);
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
pub(crate) unsafe extern "C" fn rs_molecule_was_recently_unbonded(
    board: *mut board,
    a: *mut atom,
    pos: vector,
) -> bool {
    let molecule: *mut molecule = rs_get_molecule(
        board,
        atom_ref_at_position {
            atom: a,
            position: pos,
        },
    );
    let mut i: u32 = 0_u32;
    while (i as usize) < (*molecule).size {
        if *(*(*molecule).atoms.offset(i as isize)).atom & 0x3f_u64 << 29_i32 != 0 {
            return true;
        }
        i = i.wrapping_add(1);
    }
    false
}
pub(crate) unsafe extern "C" fn rs_compare_overlapped_atoms(
    a: *const c_void,
    b: *const c_void,
) -> i32 {
    let oa: atom = (*(a as *mut atom) & OVERLAPS_ATOMS) as atom;
    let ob: atom = (*(b as *mut atom) & OVERLAPS_ATOMS) as atom;
    if oa != ob {
        return (oa > ob) as i32 - (oa < ob) as i32;
    }
    (a > b) as i32 - (a < b) as i32
}
pub(crate) unsafe extern "C" fn rs_raise_overlapped_atoms_slow_path(
    board: *mut board,
    pos: vector,
) {
    let mut atoms: [*mut atom; 6] = [::core::ptr::null_mut::<atom>(); 6];
    let mut atoms_copy: [atom; 6] = [0; 6];
    let mut count: i32 = 0_i32;
    let mut any_unbonded: bool = false;
    let c2rust_fresh4 = count;
    count += 1;
    atoms[c2rust_fresh4 as usize] = rs_lookup_atom(board, pos);
    let mut i: u32 = 0_u32;
    while i < (*board).number_of_overlapped_atoms {
        if rs_vectors_equal(
            (*(*board).overlapped_atoms.offset(i as isize)).position,
            pos,
        ) {
            if count >= 6_i32 {
                rs_report_collision(
                    board,
                    pos,
                    b"too many atoms on the same hex\0".as_ptr() as *const c_char,
                );
                return;
            }
            let c2rust_fresh5 = count;
            count += 1;
            atoms[c2rust_fresh5 as usize] =
                &raw mut (*(*board).overlapped_atoms.offset(i as isize)).atom;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0_u32;
    while i_0 < count as u32 {
        *atoms[i_0 as usize] = (*atoms[i_0 as usize] as u64 & !(OVERLAPS_ATOMS)) as atom;
        if rs_molecule_was_recently_unbonded(board, atoms[i_0 as usize], pos) {
            if any_unbonded as i32 != 0 && *atoms[i_0 as usize] as u64 & DISJOINT_SAFE == 0 {
                rs_report_collision(
                    board,
                    pos,
                    b"disjoint bonds are not supported\0".as_ptr() as *const c_char,
                );
            }
            any_unbonded = true;
            *atoms[i_0 as usize] = (*atoms[i_0 as usize] as u64 | OVERLAPS_ATOMS) as atom;
        }
        i_0 = i_0.wrapping_add(1);
    }
    if !any_unbonded {
        return;
    }
    let mut i_1: i32 = 0_i32;
    while i_1 < count {
        atoms_copy[i_1 as usize] = *atoms[i_1 as usize];
        i_1 += 1;
    }
    qsort(
        &raw mut atoms_copy as *mut atom as *mut c_void,
        count as usize,
        ::core::mem::size_of::<*mut atom>(),
        Some(
            rs_compare_overlapped_atoms
                as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
        ),
    );
    let mut i_2: u32 = 0_u32;
    while i_2 < count as u32 {
        let mut a: atom = (atoms_copy[i_2 as usize] & !(OVERLAPS_ATOMS)) as atom;
        if i_2 < (count - 1_i32) as u32 {
            a = (a as u64 | OVERLAPS_ATOMS) as atom;
        }
        if i_2 > 1_u32 {
            a = (a as u64 | VISITED) as atom;
        }
        *atoms[i_2 as usize] = a;
        crate::board_index::changed(board, atoms[i_2 as usize]);
        i_2 = i_2.wrapping_add(1);
    }
}
pub(crate) unsafe extern "C" fn rs_raise_overlapped_atoms(board: *mut board) {
    let mut i: u32 = 0_u32;
    while i < (*board).number_of_overlapped_atoms {
        let curr: *mut atom = &raw mut (*(*board).overlapped_atoms.offset(i as isize)).atom;
        let pos: vector = (*(*board).overlapped_atoms.offset(i as isize)).position;
        if *curr & VISITED != 0 {
            *curr = (*curr & !(VISITED)) as atom;
        } else if *curr & OVERLAPS_ATOMS != 0 {
            rs_raise_overlapped_atoms_slow_path(board, pos);
        } else {
            let bot: *mut atom = rs_lookup_atom(board, pos);
            if rs_molecule_was_recently_unbonded(board, bot, pos) {
                if rs_molecule_was_recently_unbonded(board, curr, pos) {
                    if *curr & DISJOINT_SAFE == 0 {
                        rs_report_collision(
                            board,
                            pos,
                            b"disjoint bonds are not supported\0".as_ptr() as *const c_char,
                        );
                    }
                } else {
                    let tmp: atom = *bot;
                    *bot = (*curr | OVERLAPS_ATOMS) as atom;
                    *curr = (tmp as u64 & !(OVERLAPS_ATOMS)) as atom;
                    crate::board_index::changed(board, bot);
                }
            }
        }
        i = i.wrapping_add(1);
    }
}
