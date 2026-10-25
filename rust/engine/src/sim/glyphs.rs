//! Glyphs and conduits.
use super::*;

pub(crate) unsafe extern "C" fn rs_apply_conduit(
    solution: *mut solution,
    board: *mut board,
    m: mechanism,
) {
    let mut conduit: *mut conduit = (*solution).conduits.offset(m.conduit_index as isize);
    let other_side: mechanism = *(*solution)
        .glyphs
        .offset((*conduit).other_side_glyph_index as isize);
    let mut base: u32 = 0_u32;
    if (*board).half_cycle == 1_i32 {
        let mut j: u32 = 0_u32;
        while j < (*conduit).number_of_molecules {
            let length: u32 = *(*conduit).molecule_lengths.offset(j as isize);
            let mut valid: bool = true;
            let mut consume: bool = true;
            let mut k: u32 = 0_u32;
            while k < length {
                let p: vector = (*(*conduit).atoms.offset(base.wrapping_add(k) as isize)).position;
                let a: *mut atom =
                    rs_lookup_atom(board, rs_mechanism_relative_position(m, p.u, p.v, 1_i32));
                if *a & VALID == 0 || *a & REMOVED != 0 {
                    valid = false;
                    break;
                } else {
                    *a = (*a | CONDUIT_SHAPE) as atom;
                    k = k.wrapping_add(1);
                }
            }
            if valid {
                let mut k_0: u32 = 0_u32;
                while k_0 < length && consume as i32 != 0 {
                    let p_0: vector =
                        (*(*conduit).atoms.offset(base.wrapping_add(k_0) as isize)).position;
                    let a_0: atom = *rs_lookup_atom(
                        board,
                        rs_mechanism_relative_position(m, p_0.u, p_0.v, 1_i32),
                    );
                    let mut rs_bond_direction: i32 = 0_i32;
                    while rs_bond_direction < 6_i32 && consume as i32 != 0 {
                        if a_0 as u64
                            & (RECENT_BOND0
                                | NORMAL_BOND0
                                | TRIPLEX_R_BOND0
                                | TRIPLEX_Y_BOND0
                                | TRIPLEX_K_BOND0)
                                << rs_normalize_direction(
                                    rs_bond_direction + rs_direction_for_offset(m.direction_u),
                                )
                            != 0
                        {
                            let d: vector = rs_u_offset_for_direction(rs_bond_direction);
                            let b: *mut atom = rs_lookup_atom(
                                board,
                                rs_mechanism_relative_position(m, p_0.u + d.u, p_0.v + d.v, 1_i32),
                            );
                            if *b & VALID == 0 || *b & REMOVED != 0 || *b & CONDUIT_SHAPE == 0 {
                                consume = false;
                            }
                        }
                        rs_bond_direction += 1;
                    }
                    k_0 = k_0.wrapping_add(1);
                }
            }
            let mut k_1: u32 = 0_u32;
            while k_1 < length {
                let p_1: vector =
                    (*(*conduit).atoms.offset(base.wrapping_add(k_1) as isize)).position;
                let a_1: *mut atom = rs_lookup_atom(
                    board,
                    rs_mechanism_relative_position(m, p_1.u, p_1.v, 1_i32),
                );
                *a_1 = (*a_1 & !(CONDUIT_SHAPE)) as atom;
                k_1 = k_1.wrapping_add(1);
            }
            if !valid {
                memmove(
                    (*conduit).atoms.offset(base as isize) as *mut c_void,
                    (*conduit)
                        .atoms
                        .offset(base as isize)
                        .offset(length as isize) as *const c_void,
                    ((*conduit)
                        .number_of_positions
                        .wrapping_sub(base)
                        .wrapping_sub(length) as usize)
                        .wrapping_mul(::core::mem::size_of::<atom_at_position>()),
                );
                memmove(
                    (*conduit).molecule_lengths.offset(j as isize) as *mut c_void,
                    (*conduit)
                        .molecule_lengths
                        .offset(j as isize)
                        .offset(1_isize) as *const c_void,
                    ((*conduit)
                        .number_of_molecules
                        .wrapping_sub(j)
                        .wrapping_sub(1_u32) as usize)
                        .wrapping_mul(::core::mem::size_of::<u32>()),
                );
                (*conduit).number_of_molecules = (*conduit).number_of_molecules.wrapping_sub(1);
                j = j.wrapping_sub(1);
            } else {
                let mut k_2: u32 = 0_u32;
                while k_2 < length {
                    let delta: vector =
                        (*(*conduit).atoms.offset(base.wrapping_add(k_2) as isize)).position;
                    let p_2: vector = rs_mechanism_relative_position(m, delta.u, delta.v, 1_i32);
                    let a_2: *mut atom = rs_lookup_atom(board, p_2);
                    if *a_2 & HAS_DISJOINT_BOND != 0 {
                        let mut bond: disjoint_bond =
                            rs_lookup_disjoint_bond(&raw mut (*board).disjoint_bond_table, p_2);
                        rs_remove_disjoint_bond(&raw mut (*board).disjoint_bond_table, p_2);
                        bond.from_position =
                            rs_mechanism_relative_position(other_side, delta.u, delta.v, 1_i32);
                        let to_delta: vector = rs_position_within_mechanism(
                            m,
                            bond.to_position.u,
                            bond.to_position.v,
                            1_i32,
                        );
                        bond.to_position = rs_mechanism_relative_position(
                            other_side, to_delta.u, to_delta.v, 1_i32,
                        );
                        rs_insert_disjoint_bond(&raw mut (*conduit).disjoint_bond_table, bond);
                    }
                    if consume {
                        (*(*conduit).atoms.offset(base.wrapping_add(k_2) as isize)).atom =
                            (*a_2 & !(OVERLAPS_ATOMS)) as atom;
                        rs_remove_atom(
                            board,
                            atom_ref_at_position {
                                atom: a_2,
                                position: p_2,
                            },
                        );
                    } else {
                        *a_2 = (*a_2
                            & !(0x3f_u64.wrapping_mul(
                                RECENT_BOND0
                                    | NORMAL_BOND0
                                    | TRIPLEX_R_BOND0
                                    | TRIPLEX_Y_BOND0
                                    | TRIPLEX_K_BOND0,
                            ) & !(0x3f_u64 << 29_i32)
                                & (*(*conduit).atoms.offset(base.wrapping_add(k_2) as isize)).atom))
                            as atom;
                    }
                    rs_add_produced_atom_collider(
                        board,
                        rs_mechanism_relative_position(other_side, delta.u, delta.v, 1_i32),
                    );
                    k_2 = k_2.wrapping_add(1);
                }
                base = base.wrapping_add(length);
            }
            j = j.wrapping_add(1);
        }
    } else {
        conduit = (*solution)
            .conduits
            .offset(other_side.conduit_index as isize);
        let rotation: i32 = rs_direction_for_offset(m.direction_u)
            - rs_direction_for_offset(other_side.direction_u);
        let mut j_0: u32 = 0_u32;
        while j_0 < (*conduit).number_of_molecules {
            let length_0: u32 = *(*conduit).molecule_lengths.offset(j_0 as isize);
            let mut k_3: u32 = 0_u32;
            while k_3 < length_0 {
                let delta_0: vector =
                    (*(*conduit).atoms.offset(base.wrapping_add(k_3) as isize)).position;
                let mut a_3: atom =
                    (*(*conduit).atoms.offset(base.wrapping_add(k_3) as isize)).atom;
                if a_3 as u64
                    & 0x23_u64.wrapping_mul(
                        RECENT_BOND0
                            | NORMAL_BOND0
                            | TRIPLEX_R_BOND0
                            | TRIPLEX_Y_BOND0
                            | TRIPLEX_K_BOND0,
                    )
                    & !(0x3f_u64 << 29_i32)
                    == 0
                {
                    a_3 = (a_3 as u64 | DISJOINT_SAFE) as atom;
                }
                rs_rotate_bonds(&raw mut a_3, rotation);
                rs_produce_atom(board, m, delta_0.u, delta_0.v, a_3);
                k_3 = k_3.wrapping_add(1);
            }
            base = base.wrapping_add(length_0);
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: u32 = 0_u32;
        while j_1 < (*conduit).disjoint_bond_table.size {
            let bond_0: disjoint_bond = *(*conduit).disjoint_bond_table.bonds.offset(j_1 as isize);
            if !rs_vectors_equal(bond_0.from_position, bond_0.to_position) {
                rs_insert_disjoint_bond(&raw mut (*board).disjoint_bond_table, bond_0);
            }
            *(*conduit).disjoint_bond_table.bonds.offset(j_1 as isize) =
                rs_uninitialized_disjoint_bond;
            j_1 = j_1.wrapping_add(1);
        }
        (*conduit).disjoint_bond_table.count = 0_u32;
    };
}
/// Glyph reads at (du, dv), through the board's glyph plan (glyph_plan.rs).
#[derive(Clone, Copy)]
struct GlyphReads {
    board: *mut board,
    glyph: *mut crate::glyph_plan::Glyph,
    m: mechanism,
}
impl GlyphReads {
    #[inline(always)]
    unsafe fn get(self, du: i32, dv: i32) -> atom_ref_at_position {
        rs_get_glyph_atom(self.board, self.glyph, self.m.r#type, du, dv)
    }
    #[inline(always)]
    unsafe fn input(self, du: i32, dv: i32) -> atom_ref_at_position {
        rs_glyph_conversion_input(self.board, self.glyph, self.m.r#type, du, dv)
    }
    #[inline(always)]
    unsafe fn output(self, du: i32, dv: i32) -> bool {
        rs_glyph_conversion_output(self.board, self.glyph, du, dv)
    }
}
pub(crate) unsafe extern "C" fn rs_apply_glyphs(solution: *mut solution, board: *mut board) {
    let n: usize = (*solution).number_of_glyphs;
    let plans = crate::glyph_plan::plans(solution, board);
    let mut i: usize = 0_usize;
    while i < n {
        let m: mechanism = *(*solution).glyphs.offset(i as isize);
        let conversion: *mut atom = &raw mut (*(*solution).glyphs.offset(i as isize)).conversion;
        let r = GlyphReads {
            board,
            glyph: plans.add(i),
            m,
        };
        match m.r#type & rs_ANY_GLYPH {
            1 => {
                let a: atom_ref_at_position = r.get(0_i32, 0_i32);
                if *a.atom & (WATER | FIRE | EARTH | AIR) != 0 {
                    rs_transform_atom(a.atom, (SALT) as atom);
                }
            }
            2 => {
                if (*board).half_cycle == 1_i32 {
                    let a_0: atom_ref_at_position = r.input(0_i32, 0_i32);
                    let b: atom_ref_at_position = r.input(1_i32, 0_i32);
                    if (*a_0.atom & *b.atom) & SALT != 0
                        && r.output(0_i32, 1_i32) as i32 != 0
                        && r.output(1_i32, -1_i32) as i32 != 0
                    {
                        rs_remove_atom(board, a_0);
                        rs_remove_atom(board, b);
                        *conversion = 1 as atom;
                    }
                }
                if *conversion != 0 {
                    rs_produce_atom(board, m, 0_i32, 1_i32, (VITAE) as atom);
                    rs_produce_atom(board, m, 1_i32, -1_i32, (MORS) as atom);
                }
            }
            4 => {
                let q: atom_ref_at_position = r.get(0_i32, 0_i32);
                let a_1: atom_ref_at_position = r.get(1_i32, 0_i32);
                let metal: atom =
                    (*a_1.atom & (LEAD | TIN | IRON | COPPER | SILVER | GOLD) & !(GOLD)) as atom;
                if metal != 0
                    && *q.atom
                        & 0x3f_u64.wrapping_mul(
                            RECENT_BOND0
                                | NORMAL_BOND0
                                | TRIPLEX_R_BOND0
                                | TRIPLEX_Y_BOND0
                                | TRIPLEX_K_BOND0,
                        )
                        == 0
                    && *q.atom & HAS_DISJOINT_BOND == 0
                    && *q.atom & 0x1f_u64.wrapping_mul(GRABBED_ONCE) == 0
                    && *q.atom & QUICKSILVER != 0
                {
                    rs_remove_atom(board, q);
                    rs_transform_atom(a_1.atom, metal >> 1_i32);
                }
            }
            8 => {
                if (*board).half_cycle == 1_i32 {
                    let a_2: atom_ref_at_position = r.input(0_i32, 0_i32);
                    if *a_2.atom & QUINTESSENCE != 0
                        && r.output(1_i32, 0_i32) as i32 != 0
                        && r.output(1_i32, -1_i32) as i32 != 0
                        && r.output(0_i32, -1_i32) as i32 != 0
                        && r.output(-1_i32, 0_i32) as i32 != 0
                    {
                        rs_remove_atom(board, a_2);
                        *conversion = 1 as atom;
                    }
                }
                if *conversion != 0 {
                    rs_produce_atom(board, m, 1_i32, 0_i32, (EARTH) as atom);
                    rs_produce_atom(board, m, 1_i32, -1_i32, (WATER) as atom);
                    rs_produce_atom(board, m, 0_i32, -1_i32, (FIRE) as atom);
                    rs_produce_atom(board, m, -1_i32, 0_i32, (AIR) as atom);
                }
            }
            16 => {
                if (*board).half_cycle == 1_i32 {
                    let a_3: atom_ref_at_position = r.input(0_i32, 0_i32);
                    let b_0: atom_ref_at_position = r.input(1_i32, 0_i32);
                    let metal_0: atom = ((*a_3.atom & *b_0.atom)
                        & (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                        & !(GOLD)) as atom;
                    if metal_0 != 0 && r.output(0_i32, 1_i32) as i32 != 0 {
                        rs_remove_atom(board, a_3);
                        rs_remove_atom(board, b_0);
                        *conversion = metal_0 >> 1_i32;
                    }
                }
                if *conversion != 0 {
                    rs_produce_atom(board, m, 0_i32, 1_i32, *conversion);
                }
            }
            32 => {
                let a_4: atom_ref_at_position = r.get(0_i32, 0_i32);
                let b_1: atom_ref_at_position = r.get(1_i32, 0_i32);
                let elemental: atom = (*a_4.atom & (WATER | FIRE | EARTH | AIR)) as atom;
                if elemental != 0 && *b_1.atom & SALT != 0 && *b_1.atom & WHEEL_ATOM == 0 {
                    rs_transform_atom(b_1.atom, elemental);
                }
            }
            64 => {
                if (*board).half_cycle == 1_i32 {
                    let a_5: atom_ref_at_position = r.input(0_i32, 1_i32);
                    let b_2: atom_ref_at_position = r.input(-1_i32, 1_i32);
                    let c: atom_ref_at_position = r.input(0_i32, -1_i32);
                    let d: atom_ref_at_position = r.input(1_i32, -1_i32);
                    if (*a_5.atom | *b_2.atom | *c.atom | *d.atom) & (WATER | FIRE | EARTH | AIR)
                        == WATER | FIRE | EARTH | AIR
                        && r.output(0_i32, 0_i32) as i32 != 0
                    {
                        rs_remove_atom(board, a_5);
                        rs_remove_atom(board, b_2);
                        rs_remove_atom(board, c);
                        rs_remove_atom(board, d);
                        *conversion = 1 as atom;
                    }
                }
                if *conversion != 0 {
                    rs_produce_atom(board, m, 0_i32, 0_i32, (QUINTESSENCE) as atom);
                }
            }
            128 => {
                let a_6: atom_ref_at_position = r.get(0_i32, 0_i32);
                let b_3: atom_ref_at_position = r.get(1_i32, 0_i32);
                if *a_6.atom != 0 && *b_3.atom != 0 {
                    rs_add_bond(
                        m,
                        a_6.atom,
                        b_3.atom,
                        1_i32,
                        0_i32,
                        (0x3f_u64 << 35_i32) as atom,
                        (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32) as atom,
                    );
                }
            }
            256 => {
                let a_7: atom_ref_at_position = r.get(0_i32, 0_i32);
                let b_4: atom_ref_at_position = r.get(1_i32, 0_i32);
                if *a_7.atom != 0 && *b_4.atom != 0 {
                    let ab: atom = rs_bond_direction(m, 1_i32, 0_i32);
                    let ba: atom = rs_bond_direction(m, -1_i32, 0_i32);
                    if (*a_7.atom & ab) as u64 & !(0x3f_u64 << 29_i32) != 0
                        && (*b_4.atom & ba) as u64 & !(0x3f_u64 << 29_i32) != 0
                    {
                        if (*board).disjoint_bond_table.count > 0_u32 {
                            let molecule: *mut molecule = rs_get_molecule(board, a_7);
                            let mut i_0: u32 = 0_u32;
                            while (i_0 as usize) < (*molecule).size {
                                let p: atom_ref_at_position =
                                    *(*molecule).atoms.offset(i_0 as isize) as atom_ref_at_position;
                                if *p.atom & HAS_DISJOINT_BOND != 0 {
                                    rs_remove_disjoint_bond(
                                        &raw mut (*board).disjoint_bond_table,
                                        p.position,
                                    );
                                    *p.atom = (*p.atom & !(HAS_DISJOINT_BOND)) as atom;
                                }
                                i_0 = i_0.wrapping_add(1);
                            }
                        }
                        *a_7.atom &= !ab;
                        *a_7.atom = (*a_7.atom | ab as u64 & 0x3f_u64 << 29_i32) as atom;
                        rs_schedule_flag_reset_if_needed(board, a_7.atom);
                        *b_4.atom &= !ba;
                        *b_4.atom = (*b_4.atom | ba as u64 & 0x3f_u64 << 29_i32) as atom;
                        rs_schedule_flag_reset_if_needed(board, b_4.atom);
                    }
                }
            }
            512 => {
                let ky: atom_ref_at_position = r.get(0_i32, 0_i32);
                let yr: atom_ref_at_position = r.get(0_i32, 1_i32);
                let rk: atom_ref_at_position = r.get(1_i32, 0_i32);
                if *ky.atom != 0 && *yr.atom != 0 && (*ky.atom & *yr.atom) & FIRE != 0 {
                    rs_add_bond(
                        m,
                        ky.atom,
                        yr.atom,
                        0_i32,
                        1_i32,
                        (0x3f_u64 << 47_i32) as atom,
                        (0x3f_u64 << 35_i32) as atom,
                    );
                }
                if *yr.atom != 0 && *rk.atom != 0 && (*yr.atom & *rk.atom) & FIRE != 0 {
                    rs_add_bond(
                        m,
                        yr.atom,
                        rk.atom,
                        1_i32,
                        -1_i32,
                        (0x3f_u64 << 41_i32) as atom,
                        (0x3f_u64 << 35_i32) as atom,
                    );
                }
                if *rk.atom != 0 && *ky.atom != 0 && (*rk.atom & *ky.atom) & FIRE != 0 {
                    rs_add_bond(
                        m,
                        rk.atom,
                        ky.atom,
                        -1_i32,
                        0_i32,
                        (0x3f_u64 << 53_i32) as atom,
                        (0x3f_u64 << 35_i32) as atom,
                    );
                }
            }
            1024 => {
                let center: atom_ref_at_position = r.get(0_i32, 0_i32);
                let a_8: atom_ref_at_position = r.get(1_i32, 0_i32);
                let b_5: atom_ref_at_position = r.get(0_i32, -1_i32);
                let c_0: atom_ref_at_position = r.get(-1_i32, 1_i32);
                if *center.atom != 0 && *a_8.atom != 0 {
                    rs_add_bond(
                        m,
                        a_8.atom,
                        center.atom,
                        -1_i32,
                        0_i32,
                        (0x3f_u64 << 35_i32) as atom,
                        (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32) as atom,
                    );
                }
                if *center.atom != 0 && *b_5.atom != 0 {
                    rs_add_bond(
                        m,
                        b_5.atom,
                        center.atom,
                        0_i32,
                        1_i32,
                        (0x3f_u64 << 35_i32) as atom,
                        (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32) as atom,
                    );
                }
                if *center.atom != 0 && *c_0.atom != 0 {
                    rs_add_bond(
                        m,
                        c_0.atom,
                        center.atom,
                        1_i32,
                        -1_i32,
                        (0x3f_u64 << 35_i32) as atom,
                        (0x3f_u64 << 41_i32 | 0x3f_u64 << 47_i32 | 0x3f_u64 << 53_i32) as atom,
                    );
                }
            }
            2048 => {
                let a_9: atom_ref_at_position = r.get(0_i32, 0_i32);
                if *a_9.atom != 0
                    && *a_9.atom
                        & 0x3f_u64.wrapping_mul(
                            RECENT_BOND0
                                | NORMAL_BOND0
                                | TRIPLEX_R_BOND0
                                | TRIPLEX_Y_BOND0
                                | TRIPLEX_K_BOND0,
                        )
                        == 0
                    && *a_9.atom & HAS_DISJOINT_BOND == 0
                    && *a_9.atom & 0x1f_u64.wrapping_mul(GRABBED_ONCE) == 0
                {
                    rs_remove_atom(board, a_9);
                }
            }
            2147483648 => {
                let a_10: atom_ref_at_position = r.get(0_i32, 0_i32);
                let metal_1: atom =
                    (*a_10.atom & (LEAD | TIN | IRON | COPPER | SILVER | GOLD) & !(LEAD)) as atom;
                if metal_1 != 0 && r.output(1_i32, 0_i32) as i32 != 0 {
                    rs_insert_atom(
                        board,
                        rs_mechanism_relative_position(m, 1_i32, 0_i32, 1_i32),
                        (QUICKSILVER | VALID) as atom,
                    );
                    rs_transform_atom(a_10.atom, metal_1 << 1_i32);
                }
            }
            1073741824 => {
                if (*board).half_cycle == 1_i32 {
                    let a_11: atom_ref_at_position = r.input(0_i32, 0_i32);
                    let metal_2: atom = (*a_11.atom
                        & (LEAD | TIN | IRON | COPPER | SILVER | GOLD)
                        & !(LEAD)) as atom;
                    if metal_2 != 0
                        && r.output(1_i32, 0_i32) as i32 != 0
                        && r.output(-1_i32, 0_i32) as i32 != 0
                    {
                        rs_remove_atom(board, a_11);
                        *conversion = metal_2;
                    }
                }
                if *conversion != 0 {
                    let mut a_12: atom = *conversion << 1_i32;
                    let mut b_6: atom = (LEAD) as atom;
                    while a_12 < b_6 {
                        a_12 <<= 1_i32;
                        b_6 >>= 1_i32;
                    }
                    rs_produce_atom(board, m, 1_i32, 0_i32, a_12);
                    rs_produce_atom(board, m, -1_i32, 0_i32, b_6);
                }
            }
            536870912 => {
                if (*board).half_cycle == 1_i32 {
                    let a_13: atom_ref_at_position = r.get(-1_i32, 1_i32);
                    let q_0: atom_ref_at_position = r.input(1_i32, 1_i32);
                    let metal_3: atom =
                        (*a_13.atom & (LEAD | TIN | IRON | COPPER | SILVER | GOLD)) as atom;
                    if metal_3 != 0
                        && *q_0.atom & QUICKSILVER != 0
                        && r.output(1_i32, -1_i32) as i32 != 0
                    {
                        rs_remove_atom(board, q_0);
                        *conversion = metal_3;
                    }
                }
                if *conversion != 0 {
                    rs_produce_atom(board, m, 1_i32, -1_i32, *conversion);
                }
            }
            8192 => {
                rs_apply_conduit(solution, board, m);
            }
            4096 | _ => {}
        }
        if *conversion != 0 && (*board).half_cycle == 2_i32 {
            *conversion = 0 as atom;
        }
        i = i.wrapping_add(1);
    }
}
pub(crate) unsafe extern "C" fn rs_fill_conduit_molecule(
    solution: *mut solution,
    board: *mut board,
    conduit: *mut conduit,
    atom_next: *mut u32,
    atom_cursor: *mut u32,
) -> bool {
    let m: mechanism = *(*solution).glyphs.offset((*conduit).glyph_index as isize);
    let rotation: i32 = rs_direction_for_offset(m.direction_u);
    while *atom_cursor < *atom_next {
        let ap: atom_at_position = *(*conduit).atoms.offset(*atom_cursor as isize);
        let mut rs_bond_direction: i32 = 0_i32;
        while rs_bond_direction < 6_i32 {
            if ap.atom
                & (RECENT_BOND0
                    | NORMAL_BOND0
                    | TRIPLEX_R_BOND0
                    | TRIPLEX_Y_BOND0
                    | TRIPLEX_K_BOND0)
                    << rs_normalize_direction(rs_bond_direction + rotation)
                & !(0x3f_u64 << 29_i32)
                != 0
            {
                let mut p: vector = ap.position;
                let d: vector = rs_u_offset_for_direction(rs_bond_direction);
                p.u += d.u;
                p.v += d.v;
                let a: *mut atom =
                    rs_lookup_atom(board, rs_mechanism_relative_position(m, p.u, p.v, 1_i32));
                if !(*a & VALID == 0 || *a & REMOVED != 0 || *a & VISITED != 0) {
                    if *a & CONDUIT_SHAPE == 0 || *a & 0x1f_u64.wrapping_mul(GRABBED_ONCE) != 0 {
                        return false;
                    }
                    (*(*conduit).atoms.offset(*atom_next as isize)).atom =
                        (*a & !(BEING_DROPPED) & !(CONDUIT_SHAPE)) as atom;
                    (*(*conduit).atoms.offset(*atom_next as isize)).position = p;
                    *atom_next = (*atom_next).wrapping_add(1);
                    *a = (*a | VISITED) as atom;
                }
            }
            rs_bond_direction += 1;
        }
        if ap.atom & HAS_DISJOINT_BOND != 0 {
            let bond: disjoint_bond = rs_lookup_disjoint_bond(
                &raw mut (*board).disjoint_bond_table,
                rs_mechanism_relative_position(m, ap.position.u, ap.position.v, 1_i32),
            );
            if !rs_vectors_equal(bond.from_position, bond.to_position) {
                let a_0: *mut atom = rs_lookup_atom(board, bond.to_position);
                if *a_0 & VALID != 0 && *a_0 & REMOVED == 0 && *a_0 & VISITED == 0 {
                    if *a_0 & CONDUIT_SHAPE == 0 || *a_0 & 0x1f_u64.wrapping_mul(GRABBED_ONCE) != 0
                    {
                        return false;
                    }
                    (*(*conduit).atoms.offset(*atom_next as isize)).atom =
                        (*a_0 & !(BEING_DROPPED) & !(CONDUIT_SHAPE)) as atom;
                    let p_0: vector = rs_position_within_mechanism(
                        m,
                        bond.to_position.u,
                        bond.to_position.v,
                        1_i32,
                    );
                    (*(*conduit).atoms.offset(*atom_next as isize)).position = p_0;
                    *atom_next = (*atom_next).wrapping_add(1);
                    *a_0 = (*a_0 | VISITED) as atom;
                }
            }
        }
        *atom_cursor = (*atom_cursor).wrapping_add(1);
    }
    true
}
pub(crate) unsafe extern "C" fn rs_fill_conduits(solution: *mut solution, board: *mut board) {
    if (*board).half_cycle != 1_i32 {
        return;
    }
    let mut i: u32 = 0_u32;
    while (i as usize) < (*solution).number_of_conduits {
        let conduit: *mut conduit = (*solution).conduits.offset(i as isize);
        let m: mechanism = *(*solution).glyphs.offset((*conduit).glyph_index as isize);
        let mut j: u32 = 0_u32;
        while j < (*conduit).number_of_positions {
            let p: vector = *(*conduit).positions.offset(j as isize);
            let a: *mut atom =
                rs_lookup_atom(board, rs_mechanism_relative_position(m, p.u, p.v, 1_i32));
            if *a & VALID != 0 && *a & REMOVED == 0 {
                *a = (*a | CONDUIT_SHAPE) as atom;
            }
            j = j.wrapping_add(1);
        }
        (*conduit).number_of_molecules = 0_u32;
        let mut atom_next: u32 = 0_u32;
        let mut atom_cursor: u32 = 0_u32;
        let mut j_0: u32 = 0_u32;
        while j_0 < (*conduit).number_of_positions {
            let p_0: vector = *(*conduit).positions.offset(j_0 as isize);
            let a_0: *mut atom = rs_lookup_atom(
                board,
                rs_mechanism_relative_position(m, p_0.u, p_0.v, 1_i32),
            );
            if *a_0 & VALID != 0
                && *a_0 & REMOVED == 0
                && *a_0 & BEING_DROPPED != 0
                && *a_0 & 0x1f_u64.wrapping_mul(GRABBED_ONCE) == 0
            {
                (*(*conduit).atoms.offset(atom_next as isize)).atom =
                    (*a_0 & !(BEING_DROPPED) & !(CONDUIT_SHAPE)) as atom;
                (*(*conduit).atoms.offset(atom_next as isize)).position = p_0;
                *a_0 = (*a_0 | VISITED) as atom;
                atom_next = atom_next.wrapping_add(1);
                let saved_cursor: u32 = atom_cursor;
                if rs_fill_conduit_molecule(
                    solution,
                    board,
                    conduit,
                    &raw mut atom_next,
                    &raw mut atom_cursor,
                ) {
                    let mut k: u32 = saved_cursor;
                    while k < atom_cursor {
                        let mut p_1: vector = (*(*conduit).atoms.offset(k as isize)).position;
                        p_1 = rs_mechanism_relative_position(m, p_1.u, p_1.v, 1_i32);
                        let c2rust_lvalue_ptr = &raw mut *rs_lookup_atom(board, p_1);
                        *c2rust_lvalue_ptr = (*c2rust_lvalue_ptr & !(BEING_DROPPED)) as atom;
                        k = k.wrapping_add(1);
                    }
                    let c2rust_fresh7 = (*conduit).number_of_molecules;
                    (*conduit).number_of_molecules = (*conduit).number_of_molecules.wrapping_add(1);
                    *(*conduit).molecule_lengths.offset(c2rust_fresh7 as isize) =
                        atom_cursor.wrapping_sub(saved_cursor);
                } else {
                    let mut k_0: u32 = saved_cursor;
                    while k_0 < atom_next {
                        let mut p_2: vector = (*(*conduit).atoms.offset(k_0 as isize)).position;
                        p_2 = rs_mechanism_relative_position(m, p_2.u, p_2.v, 1_i32);
                        let c2rust_lvalue_ptr_0 = &raw mut *rs_lookup_atom(board, p_2);
                        *c2rust_lvalue_ptr_0 = (*c2rust_lvalue_ptr_0 & !(VISITED)) as atom;
                        k_0 = k_0.wrapping_add(1);
                    }
                    atom_next = saved_cursor;
                    atom_cursor = saved_cursor;
                }
            }
            j_0 = j_0.wrapping_add(1);
        }
        let mut j_1: u32 = 0_u32;
        while j_1 < (*conduit).number_of_positions {
            let p_3: vector = *(*conduit).positions.offset(j_1 as isize);
            let a_1: *mut atom = rs_lookup_atom(
                board,
                rs_mechanism_relative_position(m, p_3.u, p_3.v, 1_i32),
            );
            *a_1 = (*a_1 & (!(CONDUIT_SHAPE) & !(VISITED))) as atom;
            j_1 = j_1.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
}
