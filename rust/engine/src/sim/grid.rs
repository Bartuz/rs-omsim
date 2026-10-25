//! The board: 64x64 array plus open-addressing hash table of atoms and used-area markers,
//! area marking, track lookup, and the disjoint-bond and chain-atom side tables.
use super::*;

pub(crate) static mut rs_uninitialized_disjoint_bond: disjoint_bond = disjoint_bond {
    from_position: vector { u: 0, v: 0 },
    to_position: vector { u: 0, v: 0 },
};
pub(crate) static mut rs_removed_disjoint_bond: disjoint_bond = disjoint_bond {
    from_position: vector { u: 0_i32, v: 1_i32 },
    to_position: vector { u: 0_i32, v: 1_i32 },
};
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_used_area(board: *mut board) -> u32 {
    (*board).area
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_lookup_track(
    solution: *mut solution,
    query: vector,
    index: *mut u32,
) -> bool {
    if (*solution).track_table_size == 0_u32 {
        return false;
    }
    let hash: u32 = rs_vector_hash(query);
    let mask: u32 = (*solution).track_table_size.wrapping_sub(1_u32);
    *index = hash & mask;
    loop {
        let p: vector = *(*solution).track_positions.offset(*index as isize);
        if p.u == -2147483647_i32 - 1_i32 && p.v == -2147483647_i32 - 1_i32 {
            return false;
        }
        if rs_vectors_equal(p, query) {
            return true;
        }
        *index = (*index).wrapping_add(1_u32) & mask;
        if *index == hash & mask {
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_move_chain_atom_to_list(
    board: *mut board,
    chain_atom_index: u32,
    list: *mut u32,
) {
    if chain_atom_index == 4294967295_u32 {
        return;
    }
    let m: *mut chain_atom = (*board).chain_atoms.offset(chain_atom_index as isize);
    if (*m).next_in_list != 4294967295_u32 {
        (*(*board).chain_atoms.offset((*m).next_in_list as isize)).prev_in_list = (*m).prev_in_list;
    }
    if !(*m).prev_in_list.is_null() {
        *(*m).prev_in_list = (*m).next_in_list;
    }
    if !list.is_null() {
        if *list == chain_atom_index {
            abort();
        }
        (*m).prev_in_list = list;
        (*m).next_in_list = *list;
        if *list != 4294967295_u32 {
            (*(*board).chain_atoms.offset(*list as isize)).prev_in_list =
                &raw mut (*m).next_in_list;
        }
        *list = chain_atom_index;
    } else {
        (*m).prev_in_list = ::core::ptr::null_mut::<u32>();
        (*m).next_in_list = 4294967295_u32;
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_add_chain_atom_to_table(board: *mut board, chain_atom_index: u32) {
    if (*board).chain_atom_table_size == 0_u32 {
        return;
    }
    let p: vector = (*(*board).chain_atoms.offset(chain_atom_index as isize)).current_position;
    let hash: u32 = rs_vector_hash(p);
    let mask: u32 = (*board).chain_atom_table_size.wrapping_sub(1_u32);
    rs_move_chain_atom_to_list(
        board,
        chain_atom_index,
        (*board).chain_atom_table.offset((hash & mask) as isize),
    );
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_lookup_chain_atom(board: *mut board, query: vector) -> u32 {
    if (*board).chain_atom_table_size == 0_u32 {
        return 4294967295_u32;
    }
    let hash: u32 = rs_vector_hash(query);
    let mask: u32 = (*board).chain_atom_table_size.wrapping_sub(1_u32);
    let mut index: u32 = *(*board).chain_atom_table.offset((hash & mask) as isize);
    while index != 4294967295_u32 {
        if rs_vectors_equal(
            query,
            (*(*board).chain_atoms.offset(index as isize)).current_position,
        ) {
            return index;
        }
        index = (*(*board).chain_atoms.offset(index as isize)).next_in_list;
    }
    index
}
pub(crate) unsafe extern "C" fn rs_disjoint_bonds_equal(
    a: disjoint_bond,
    b: disjoint_bond,
) -> bool {
    rs_vectors_equal(a.from_position, b.from_position) as i32 != 0
        && rs_vectors_equal(a.to_position, b.to_position) as i32 != 0
}
pub(crate) unsafe extern "C" fn rs_lookup_disjoint_bond_slot(
    table: *mut disjoint_bond_table,
    query: vector,
    for_insertion: bool,
) -> *mut disjoint_bond {
    if (*table).size == 0_u32 {
        return &raw const rs_uninitialized_disjoint_bond as *mut disjoint_bond;
    }
    let hash: u32 = rs_vector_hash(query);
    let mask: u32 = (*table).size.wrapping_sub(1_u32);
    let mut index: u32 = hash & mask;
    loop {
        let bond: *mut disjoint_bond = (*table).bonds.offset(index as isize);
        if rs_disjoint_bonds_equal(*bond, rs_uninitialized_disjoint_bond) {
            if for_insertion {
                (*table).count = (*table).count.wrapping_add(1);
            }
            return bond;
        }
        if rs_disjoint_bonds_equal(*bond, rs_removed_disjoint_bond) {
            if for_insertion {
                return bond;
            }
        } else if rs_vectors_equal((*bond).from_position, query) {
            return bond;
        }
        index = index.wrapping_add(1_u32) & mask;
    }
}
pub(crate) unsafe extern "C" fn rs_lookup_disjoint_bond(
    table: *mut disjoint_bond_table,
    query: vector,
) -> disjoint_bond {
    *rs_lookup_disjoint_bond_slot(table, query, false)
}
pub(crate) unsafe extern "C" fn rs_insert_disjoint_bond(
    table: *mut disjoint_bond_table,
    bond: disjoint_bond,
) {
    if (*table).count >= (*table).size.wrapping_div(2_u32) {
        let old_size: u32 = (*table).size;
        (*table).size = (*table).size.wrapping_mul(2_u32);
        if (*table).size < 16_u32 {
            (*table).size = 16_u32;
        }
        let old: *mut disjoint_bond = (*table).bonds;
        (*table).bonds = calloc(
            (*table).size as usize,
            ::core::mem::size_of::<disjoint_bond>(),
        ) as *mut disjoint_bond;
        (*table).count = 0_u32;
        let mut i: u32 = 0_u32;
        while i < old_size {
            let old_bond: disjoint_bond = *old.offset(i as isize);
            if !rs_vectors_equal(old_bond.from_position, old_bond.to_position) {
                (*table).count = (*table).count.wrapping_add(1);
                *rs_lookup_disjoint_bond_slot(table, old_bond.from_position, true) = old_bond;
            }
            i = i.wrapping_add(1);
        }
        free(old as *mut c_void);
    }
    *rs_lookup_disjoint_bond_slot(table, bond.from_position, true) = bond;
}
pub(crate) unsafe extern "C" fn rs_remove_disjoint_bond(
    table: *mut disjoint_bond_table,
    position: vector,
) {
    *rs_lookup_disjoint_bond_slot(table, position, false) = rs_removed_disjoint_bond;
}
pub(crate) unsafe extern "C" fn rs_lookup_atom_at_position_in_array(
    grid: *mut atom_grid,
    query: vector,
) -> *mut atom_at_position {
    (*grid)
        .atoms_at_positions
        .offset(((query.u - -32_i32) * (32_i32 - -32_i32) + query.v - -32_i32) as isize)
}
#[inline(never)]
pub(crate) unsafe extern "C" fn rs_lookup_atom_at_position_in_hashtable(
    grid: *mut atom_grid,
    query: vector,
) -> *mut atom_at_position {
    let hash: u32 = rs_vector_hash(query);
    loop {
        let mask: u32 = (*grid).hash_capacity.wrapping_sub(1_u32);
        let mut index: u32 = hash & mask;
        let mut misses: u32 = 0_u32;
        loop {
            let a: *mut atom_at_position = (*grid).atoms_at_positions.offset(
                (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32).wrapping_add(index) as isize,
            );
            if (*a).atom == 0 {
                return a;
            }
            if rs_vectors_equal((*a).position, query) {
                return a;
            }
            misses = misses.wrapping_add(1);
            if misses == 6_u32 {
                rs_rehash(grid, (*grid).hash_capacity.wrapping_mul(2_u32));
                break;
            } else {
                index = index.wrapping_add(1_u32) & mask;
            }
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_lookup_atom_at_position(
    grid: *mut atom_grid,
    query: vector,
) -> *mut atom_at_position {
    if (*grid).hash_capacity == 0_u32 {
        rs_rehash(grid, 1_u32);
    }
    if query.u >= -32_i32 && query.u < 32_i32 && query.v >= -32_i32 && query.v < 32_i32 {
        rs_lookup_atom_at_position_in_array(grid, query)
    } else {
        rs_lookup_atom_at_position_in_hashtable(grid, query)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_lookup_atom(board: *mut board, query: vector) -> *mut atom {
    &raw mut (*(rs_lookup_atom_at_position
        as unsafe extern "C" fn(*mut atom_grid, vector) -> *mut atom_at_position)(
        &raw mut (*board).grid,
        query,
    ))
    .atom
}
pub(crate) unsafe extern "C" fn rs_lookup_topmost_atom(
    board: *mut board,
    a: *mut atom_at_position,
) -> *mut atom {
    if (*a).atom & OVERLAPS_ATOMS == 0 {
        return &raw mut (*a).atom;
    }
    let mut i: i32 = (*board).number_of_overlapped_atoms.wrapping_sub(1_u32) as i32;
    while i >= 0_i32 {
        if rs_vectors_equal(
            (*(*board).overlapped_atoms.offset(i as isize)).position,
            (*a).position,
        ) {
            return &raw mut (*(*board).overlapped_atoms.offset(i as isize)).atom;
        }
        i -= 1;
    }
    if (*board).rs_collision as i32 != 0 {
    } else {
        __assert_fail(
            b"board->collision\0".as_ptr() as *const c_char,
            b"sim.c\0".as_ptr()
                as *const c_char,
            2365_u32,
            b"atom *lookup_topmost_atom(struct board *, struct atom_at_position *)\0".as_ptr()
                as *const c_char,
        );
    };
    &raw mut (*a).atom
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_lookup_atom_in_grid(grid: *mut atom_grid, query: vector) -> *mut atom {
    &raw mut (*(rs_lookup_atom_at_position
        as unsafe extern "C" fn(*mut atom_grid, vector) -> *mut atom_at_position)(
        grid, query
    ))
    .atom
}
pub(crate) unsafe extern "C" fn rs_mark_used_area_with_overlap(
    board: *mut board,
    point: vector,
    overlap: *mut u64,
) -> atom {
    let a: *mut atom_at_position = rs_lookup_atom_at_position(&raw mut (*board).grid, point);
    if (*a).atom & VALID != 0 {
        if !overlap.is_null() && (*a).atom & VISITED == 0 {
            *overlap = (*overlap).wrapping_add(1);
        }
        return (*a).atom;
    }
    (*a).position = point;
    (*a).atom = (VALID | REMOVED) as atom;
    (*board).area = (*board).area.wrapping_add(1);
    note_written(board, point);
    (*a).atom
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_mark_used_area(board: *mut board, point: vector) -> atom {
    rs_mark_used_area_with_overlap(board, point, ::core::ptr::null_mut::<u64>())
}
#[inline(never)]
pub(crate) unsafe extern "C" fn rs_insert_overlapped_atom(
    board: *mut board,
    existing_atom: *mut atom_at_position,
    atom: atom,
) {
    let mut capacity: usize = (*board).overlapped_atoms_capacity as usize;
    while ((*board).number_of_overlapped_atoms as usize) >= capacity {
        capacity = 4_usize
            .wrapping_mul(capacity.wrapping_add(12_usize))
            .wrapping_div(3_usize);
    }
    if capacity != (*board).overlapped_atoms_capacity as usize {
        let atoms_at_positions: *mut atom_at_position = realloc(
            (*board).overlapped_atoms as *mut c_void,
            ::core::mem::size_of::<atom_at_position>().wrapping_mul(capacity),
        ) as *mut atom_at_position;
        if atoms_at_positions.is_null() {
            abort();
        }
        (*board).overlapped_atoms = atoms_at_positions;
        (*board).overlapped_atoms_capacity = capacity as u32;
    }
    let c2rust_lvalue_ptr = &raw mut *rs_lookup_topmost_atom(board, existing_atom);
    *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr | OVERLAPS_ATOMS) as atom;
    let c2rust_fresh1 = (*board).number_of_overlapped_atoms;
    (*board).number_of_overlapped_atoms = (*board).number_of_overlapped_atoms.wrapping_add(1);
    *(*board).overlapped_atoms.offset(c2rust_fresh1 as isize) = atom_at_position {
        position: (*existing_atom).position,
        atom,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_insert_atom(board: *mut board, query: vector, atom: atom) {
    let a: *mut atom_at_position = rs_lookup_atom_at_position(&raw mut (*board).grid, query);
    if (*a).atom & VALID != 0 && (*a).atom & REMOVED == 0 {
        rs_insert_overlapped_atom(board, a, atom);
    } else {
        (*a).position = query;
        if (*a).atom & REMOVED == 0 {
            (*board).area = (*board).area.wrapping_add(1);
        }
        (*a).atom = atom;
        crate::board_index::inserted(board, a);
        note_written(board, query);
    };
}
#[inline(always)]
pub(crate) unsafe fn note_written(board: *mut board, p: vector) {
    let b = &mut (*board).written_bbox;
    b[0] = b[0].min(p.u);
    b[1] = b[1].max(p.u);
    b[2] = b[2].min(p.v);
    b[3] = b[3].max(p.v);
}
#[inline(never)]
pub(crate) unsafe extern "C" fn rs_rehash(grid: *mut atom_grid, size: u32) {
    let mut n: u32 = (*grid).hash_capacity;
    if n < 16_u32 {
        n = 16_u32;
    }
    while n < size {
        n = n.wrapping_mul(2_u32);
    }
    if n == (*grid).hash_capacity {
        return;
    }
    let old: atom_grid = *grid;
    (*grid).atoms_at_positions = calloc(
        (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32).wrapping_add(n) as usize,
        ::core::mem::size_of::<atom_at_position>(),
    ) as *mut atom_at_position;
    (*grid).hash_capacity = n;
    if !(*grid).board.is_null() {
        (*(*grid).board).flag_reset_length = 0_u32;
        crate::board_index::reset_after_rehash((*grid).board, 4096 + n as usize);
        crate::glyph_plan::reset_after_rehash((*grid).board);
    }
    if old.atoms_at_positions.is_null() {
        return;
    }
    let mut i: u32 = 0_u32;
    while i < ((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32 {
        let a: atom_at_position = *old.atoms_at_positions.offset(i as isize);
        if a.atom & VALID != 0 {
            let b: *mut atom_at_position = rs_lookup_atom_at_position_in_array(grid, a.position);
            *b = a;
            if !(*grid).board.is_null() {
                crate::board_index::inserted((*grid).board, b);
                rs_schedule_flag_reset_if_needed((*grid).board as *mut board, &raw mut (*b).atom);
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0_u32;
    while i_0 < old.hash_capacity {
        let a_0: atom_at_position = *old
            .atoms_at_positions
            .offset((((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32).wrapping_add(i_0) as isize);
        if a_0.atom & VALID != 0 {
            let b_0: *mut atom_at_position =
                rs_lookup_atom_at_position_in_hashtable(grid, a_0.position);
            *b_0 = a_0;
            if !(*grid).board.is_null() {
                crate::board_index::inserted((*grid).board, b_0);
                rs_schedule_flag_reset_if_needed((*grid).board as *mut board, &raw mut (*b_0).atom);
            }
        }
        i_0 = i_0.wrapping_add(1);
    }
    free(old.atoms_at_positions as *mut c_void);
}
pub(crate) unsafe extern "C" fn rs_schedule_flag_reset_if_needed(board: *mut board, a: *mut atom) {
    if *a & (BEING_DROPPED | 0x3f_u64 << 29_i32) == 0 {
        return;
    }
    if (*board).flag_reset_length >= (*board).flag_reset_capacity {
        let n: u32 = (*board)
            .flag_reset_length
            .wrapping_add(1_u32)
            .wrapping_mul(2_u32);
        let flag_reset: *mut *mut atom = realloc(
            (*board).flag_reset as *mut c_void,
            (n as usize).wrapping_mul(::core::mem::size_of::<*mut atom>()),
        ) as *mut *mut atom;
        if flag_reset.is_null() {
            abort();
        }
        (*board).flag_reset = flag_reset;
        (*board).flag_reset_capacity = n;
    }
    let c2rust_fresh0 = (*board).flag_reset_length;
    (*board).flag_reset_length = (*board).flag_reset_length.wrapping_add(1);
    *(*board).flag_reset.offset(c2rust_fresh0 as isize) = a;
}
#[cfg(test)]
pub(crate) mod index_tests {
    use super::*;

    #[test]
    fn occupied_index_survives_overlap_removal_motion_and_rehash() {
        unsafe {
            let mut board: board = core::mem::zeroed();
            board.grid.board = &mut board;
            rs_rehash(&mut board.grid, 16);
            let p = vector { u: 50, v: -40 };
            rs_insert_atom(&mut board, p, 3);
            let check = |board: *mut board| {
                let indexed = crate::board_index::occupied_slots(board);
                let scanned: Vec<_> = (0..4096 + (*board).grid.hash_capacity as usize)
                    .filter(|&i| {
                        (*(*board).grid.atoms_at_positions.add(i)).atom & (1 | 1 << 27) == 1
                    })
                    .collect();
                assert_eq!(indexed, scanned);
            };
            check(&mut board); // lazy initialization
            rs_insert_atom(&mut board, p, 5); // two atoms in one grid slot
            let base = rs_lookup_atom(&mut board, p);
            let top = rs_lookup_topmost_atom(&mut board, (base as *mut u8).sub(8).cast());
            assert!(rs_remove_atom(
                &mut board,
                atom_ref_at_position {
                    atom: top,
                    position: p
                }
            ));
            // Removing a stack entry must not clear the occupied base grid slot.
            check(&mut board);
            // Removing the base promotes the stack entry, even when it was
            // just marked removed. This must update the grid slot's bit too.
            assert!(!rs_remove_atom(
                &mut board,
                atom_ref_at_position {
                    atom: base,
                    position: p,
                },
            ));
            assert_eq!(board.number_of_overlapped_atoms, 0);
            check(&mut board);
            for i in 0..600 {
                rs_insert_atom(
                    &mut board,
                    vector {
                        u: 100 + i,
                        v: i * 7,
                    },
                    3,
                );
            }
            assert!(board.grid.hash_capacity > 16);
            check(&mut board);
            for i in (0..600).step_by(3) {
                let position = vector {
                    u: 100 + i,
                    v: i * 7,
                };
                let atom = rs_lookup_atom(&mut board, position);
                rs_remove_atom(&mut board, atom_ref_at_position { atom, position });
            }
            check(&mut board);
            rs_destroy(core::ptr::null_mut(), &mut board);
            assert!(board.index.is_null());
        }
    }

    #[test]
    fn glyph_plan_reads_follow_values_rehash_and_empty_slot_reuse() {
        unsafe {
            let mut board: board = core::mem::zeroed();
            board.grid.board = &mut board;
            rs_rehash(&mut board.grid, 16);
            let mut m: mechanism = core::mem::zeroed();
            m.r#type = CALCIFICATION as u32;
            m.direction_u = vector { u: 1, v: 0 };
            m.direction_v = vector { u: 0, v: 1 };
            let hashed = vector { u: 60, v: -40 };
            let far = vector { u: 7000, v: -7000 };
            let mut glyphs = [
                mechanism { position: hashed, ..m },
                mechanism { position: far, ..m },
                // In the fixed array, rotated: geometry must match mechanism_relative_position.
                mechanism {
                    position: vector { u: 3, v: -2 },
                    direction_u: vector { u: 0, v: 1 },
                    direction_v: vector { u: -1, v: 1 },
                    ..m
                },
            ];
            let mut solution: solution = core::mem::zeroed();
            solution.glyphs = glyphs.as_mut_ptr();
            solution.number_of_glyphs = glyphs.len();
            let plans = crate::glyph_plan::plans(&mut solution, &mut board);
            let agree = |board: *mut board| {
                for (i, g) in glyphs.iter().enumerate() {
                    for du in -1..=1 {
                        for dv in -1..=1 {
                            let planned = rs_get_glyph_atom(board, plans.add(i), g.r#type, du, dv);
                            let looked_up = rs_get_atom(board, *g, du, dv);
                            assert!(rs_vectors_equal(planned.position, looked_up.position));
                            assert_eq!(*planned.atom, *looked_up.atom);
                        }
                    }
                }
            };
            agree(&mut board);

            rs_insert_atom(&mut board, hashed, VALID | SALT);
            assert_eq!(*rs_get_glyph_atom(&mut board, plans, m.r#type, 0, 0).atom, VALID | SALT);
            let a = rs_lookup_atom(&mut board, hashed);
            rs_remove_atom(&mut board, atom_ref_at_position { atom: a, position: hashed });
            assert_eq!(*rs_get_glyph_atom(&mut board, plans, m.r#type, 0, 0).atom, 0);
            rs_insert_atom(&mut board, hashed, VALID | WATER);
            let capacity = board.grid.hash_capacity;
            for i in 0..600 {
                rs_insert_atom(&mut board, vector { u: 100 + i, v: i * 7 }, VALID | SALT);
            }
            assert!(board.grid.hash_capacity > capacity);
            assert_eq!(rs_glyph_slot(&mut board, plans, 0, 0).atom, rs_lookup_atom(&mut board, hashed));
            assert_eq!(*rs_get_glyph_atom(&mut board, plans, m.r#type, 0, 0).atom, VALID | WATER);
            agree(&mut board);

            // An empty hash slot read by a glyph is not reserved: another position may take it.
            assert_eq!(*rs_get_glyph_atom(&mut board, plans.add(1), m.r#type, 0, 0).atom, 0);
            let empty = rs_lookup_atom_at_position(&mut board.grid, far);
            let slot = empty.offset_from(board.grid.atoms_at_positions) as u32 - 4096;
            let mask = board.grid.hash_capacity - 1;
            let q = (10000..1_000_000)
                .map(|u| vector { u, v: u * 11 })
                .find(|&q| rs_vector_hash(q) & mask == slot)
                .unwrap();
            rs_insert_atom(&mut board, q, VALID | FIRE);
            assert_eq!((*empty).atom, VALID | FIRE);
            assert_eq!(*rs_get_glyph_atom(&mut board, plans.add(1), m.r#type, 0, 0).atom, 0);
            agree(&mut board);

            // Fixed-array cells are pinned to their position, so they are cached while empty.
            let fixed = plans.add(2);
            rs_get_glyph_atom(&mut board, fixed, m.r#type, 1, -1);
            assert_ne!((*fixed).cells[6].slot, usize::MAX); // (du, dv) = (1, -1)
            let p = rs_mechanism_relative_position(glyphs[2], 1, -1, 1);
            rs_insert_atom(&mut board, p, VALID | AIR);
            assert_eq!(*rs_get_glyph_atom(&mut board, fixed, m.r#type, 1, -1).atom, VALID | AIR);
            agree(&mut board);

            rs_destroy(core::ptr::null_mut(), &mut board);
            assert!(board.glyph_plan.is_null());
        }
    }
}
