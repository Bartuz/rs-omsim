#[allow(unused_imports)]
use crate::flags::*;
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_int, c_schar, c_uchar, c_ulong, c_ulonglong, c_void};
unsafe extern "C" {
    unsafe fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
    unsafe fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> i32;
    unsafe fn strlen(__s: *const c_char) -> usize;
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
    unsafe fn abort() -> !;
    unsafe fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
    unsafe fn abs(__x: i32) -> i32;
    unsafe fn rs_destroy(solution: *mut solution, board: *mut board);
    unsafe fn rs_lookup_track(solution: *mut solution, query: vector, index: *mut u32) -> bool;
    unsafe fn rs_cabinet_for_position(solution: *mut solution, position: vector) -> u8;
    unsafe fn rs_glyph_footprint(mechanism_type: u32) -> *const vector;
    unsafe fn rs_u_offset_for_direction(direction: i32) -> vector;
    unsafe fn rs_v_offset_for_direction(direction: i32) -> vector;
    unsafe fn rs_direction_for_offset(d: vector) -> i32;
    unsafe fn rs_angular_distance_between_grabbers(mechanism_type: u32) -> i32;
    unsafe fn rs_mechanism_relative_position(m: mechanism, du: i32, dv: i32, w: i32) -> vector;
    unsafe fn rs_bond_direction(m: mechanism, du: i32, dv: i32) -> atom;
    unsafe fn rs_rotate_bond_bits(bond_bits: atom, rotation: i32) -> atom;
    unsafe fn rs_polymer_position_from_global_position(io: *mut input_output, p: vector) -> vector;
}
#[inline]
unsafe extern "C" fn rs_byte_string_is(b: byte_string, str: *const c_char) -> bool {
    let len: usize = strlen(str);
    b.length == len && memcmp(b.bytes as *const c_void, str as *const c_void, len) == 0
}
static mut rs_ANY_GLYPH: u32 = ((1 as c_ulong) << 0_i32
    | (1 as c_ulong) << 1_i32
    | (1 as c_ulong) << 2_i32
    | (1 as c_ulong) << 3_i32
    | (1 as c_ulong) << 4_i32
    | (1 as c_ulong) << 5_i32
    | (1 as c_ulong) << 6_i32
    | (1 as c_ulong) << 7_i32
    | (1 as c_ulong) << 8_i32
    | TRIPLEX_BONDING
    | MULTI_BONDING
    | DISPOSAL
    | EQUILIBRIUM
    | PROLIFERATION
    | DIVISION
    | REJECTION
    | CONDUIT) as u32;
static mut rs_ANY_ARM: u32 =
    (ARM | TWO_ARM | THREE_ARM | SIX_ARM | PISTON | VAN_BERLO | RAVARI) as u32;
static mut rs_zero_vector: vector = vector { u: 0, v: 0 };
#[inline]
unsafe extern "C" fn rs_vectors_equal(a: vector, b: vector) -> bool {
    a.u == b.u && a.v == b.v
}
unsafe extern "C" fn rs_decode_instruction(inst: c_char) -> c_char {
    match inst as i32 {
        82 => 'd' as c_char,
        114 => 'a' as c_char,
        69 => 'w' as c_char,
        101 => 's' as c_char,
        71 => 'f' as c_char,
        103 => 'r' as c_char,
        80 => 'e' as c_char,
        112 => 'q' as c_char,
        65 => 'g' as c_char,
        97 => 't' as c_char,
        67 => {
            abort();
        }
        88 => {
            abort();
        }
        66 => 'b' as c_char,
        79 => ' ' as c_char,
        32 => ' ' as c_char,
        _ => ' ' as c_char,
    }
}
unsafe extern "C" fn rs_decode_atom(atom: u32) -> atom {
    (1_u64 << atom) as atom
}
unsafe extern "C" fn rs_decode_bond_type(bond_type: u8) -> atom {
    let mut bonds: atom = 0 as atom;
    if bond_type as i32 & 1_i32 != 0 {
        bonds = (bonds as u64 | 0x3f_u64 << 35_i32) as atom;
    }
    if bond_type as i32 & 2_i32 != 0 {
        bonds = (bonds as u64 | 0x3f_u64 << 41_i32) as atom;
    }
    if bond_type as i32 & 4_i32 != 0 {
        bonds = (bonds as u64 | 0x3f_u64 << 53_i32) as atom;
    }
    if bond_type as i32 & 8_i32 != 0 {
        bonds = (bonds as u64 | 0x3f_u64 << 47_i32) as atom;
    }
    bonds
}
unsafe extern "C" fn rs_decode_mechanism_type(part_name: byte_string) -> u32 {
    if rs_byte_string_is(
        part_name,
        b"glyph-calcification\0".as_ptr() as *const c_char,
    ) {
        ((1 as c_ulong) << 0_i32) as u32
    } else if rs_byte_string_is(
        part_name,
        b"glyph-life-and-death\0".as_ptr() as *const c_char,
    ) {
        ((1 as c_ulong) << 1_i32) as u32
    } else if rs_byte_string_is(part_name, b"glyph-projection\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 2_i32) as u32
    } else if rs_byte_string_is(part_name, b"glyph-dispersion\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 3_i32) as u32
    } else if rs_byte_string_is(part_name, b"glyph-purification\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 4_i32) as u32
    } else if rs_byte_string_is(part_name, b"glyph-duplication\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 5_i32) as u32
    } else if rs_byte_string_is(part_name, b"glyph-unification\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 6_i32) as u32
    } else if rs_byte_string_is(part_name, b"bonder\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 7_i32) as u32
    } else if rs_byte_string_is(part_name, b"unbonder\0".as_ptr() as *const c_char) {
        ((1 as c_ulong) << 8_i32) as u32
    } else if rs_byte_string_is(part_name, b"bonder-prisma\0".as_ptr() as *const c_char) {
        (TRIPLEX_BONDING) as u32
    } else if rs_byte_string_is(part_name, b"bonder-speed\0".as_ptr() as *const c_char) {
        (MULTI_BONDING) as u32
    } else if rs_byte_string_is(part_name, b"glyph-disposal\0".as_ptr() as *const c_char) {
        (DISPOSAL) as u32
    } else if rs_byte_string_is(part_name, b"glyph-marker\0".as_ptr() as *const c_char) {
        (EQUILIBRIUM) as u32
    } else if rs_byte_string_is(part_name, b"arm1\0".as_ptr() as *const c_char) {
        (ARM) as u32
    } else if rs_byte_string_is(part_name, b"arm2\0".as_ptr() as *const c_char) {
        (TWO_ARM) as u32
    } else if rs_byte_string_is(part_name, b"arm3\0".as_ptr() as *const c_char) {
        (THREE_ARM) as u32
    } else if rs_byte_string_is(part_name, b"arm6\0".as_ptr() as *const c_char) {
        (SIX_ARM) as u32
    } else if rs_byte_string_is(part_name, b"piston\0".as_ptr() as *const c_char) {
        (PISTON) as u32
    } else if rs_byte_string_is(part_name, b"baron\0".as_ptr() as *const c_char) {
        (VAN_BERLO) as u32
    } else if rs_byte_string_is(part_name, b"glyph-rejection\0".as_ptr() as *const c_char) {
        (REJECTION) as u32
    } else if rs_byte_string_is(part_name, b"glyph-division\0".as_ptr() as *const c_char) {
        (DIVISION) as u32
    } else if rs_byte_string_is(
        part_name,
        b"glyph-proliferation\0".as_ptr() as *const c_char,
    ) {
        (PROLIFERATION) as u32
    } else if rs_byte_string_is(part_name, b"ravari\0".as_ptr() as *const c_char) {
        (RAVARI) as u32
    } else {
        0_u32
    }
}
unsafe extern "C" fn rs_parts_available_bits_for_part_name(part_name: byte_string) -> u64 {
    if rs_byte_string_is(
        part_name,
        b"glyph-calcification\0".as_ptr() as *const c_char,
    ) {
        1_u64 << 12_i32
    } else if rs_byte_string_is(
        part_name,
        b"glyph-life-and-death\0".as_ptr() as *const c_char,
    ) {
        1_u64 << 16_i32
    } else if rs_byte_string_is(part_name, b"glyph-projection\0".as_ptr() as *const c_char) {
        1_u64 << 14_i32
    } else if rs_byte_string_is(part_name, b"glyph-dispersion\0".as_ptr() as *const c_char) {
        1_u64 << 18_i32
    } else if rs_byte_string_is(part_name, b"glyph-purification\0".as_ptr() as *const c_char) {
        1_u64 << 15_i32
    } else if rs_byte_string_is(part_name, b"glyph-duplication\0".as_ptr() as *const c_char) {
        1_u64 << 13_i32
    } else if rs_byte_string_is(part_name, b"glyph-unification\0".as_ptr() as *const c_char) {
        1_u64 << 18_i32
    } else if rs_byte_string_is(part_name, b"bonder\0".as_ptr() as *const c_char) {
        1_u64 << 8_i32
    } else if rs_byte_string_is(part_name, b"unbonder\0".as_ptr() as *const c_char) {
        1_u64 << 9_i32
    } else if rs_byte_string_is(part_name, b"bonder-prisma\0".as_ptr() as *const c_char) {
        1_u64 << 11_i32
    } else if rs_byte_string_is(part_name, b"bonder-speed\0".as_ptr() as *const c_char) {
        1_u64 << 10_i32
    } else if rs_byte_string_is(part_name, b"glyph-disposal\0".as_ptr() as *const c_char) {
        1_u64 << 17_i32
    } else if rs_byte_string_is(part_name, b"glyph-marker\0".as_ptr() as *const c_char) {
        1_u64 << 1_i32
    } else if rs_byte_string_is(part_name, b"arm1\0".as_ptr() as *const c_char) {
        1_u64 << 0_i32
    } else if rs_byte_string_is(part_name, b"arm2\0".as_ptr() as *const c_char) {
        1_u64 << 1_i32
    } else if rs_byte_string_is(part_name, b"arm3\0".as_ptr() as *const c_char) {
        1_u64 << 1_i32
    } else if rs_byte_string_is(part_name, b"arm6\0".as_ptr() as *const c_char) {
        1_u64 << 1_i32
    } else if rs_byte_string_is(part_name, b"piston\0".as_ptr() as *const c_char) {
        1_u64 << 2_i32
    } else if rs_byte_string_is(part_name, b"baron\0".as_ptr() as *const c_char) {
        1_u64 << 28_i32
    } else if rs_byte_string_is(part_name, b"track\0".as_ptr() as *const c_char) {
        1_u64 << 3_i32
    } else if rs_byte_string_is(part_name, b"glyph-rejection\0".as_ptr() as *const c_char) {
        1_u64 << 19_i32
    } else if rs_byte_string_is(part_name, b"glyph-division\0".as_ptr() as *const c_char) {
        1_u64 << 20_i32
    } else if rs_byte_string_is(
        part_name,
        b"glyph-proliferation\0".as_ptr() as *const c_char,
    ) {
        1_u64 << 21_i32
    } else if rs_byte_string_is(part_name, b"ravari\0".as_ptr() as *const c_char) {
        1_u64 << 29_i32
    } else {
        0_u64
    }
}
unsafe extern "C" fn rs_parts_available_bits_for_instruction(inst: c_char) -> u64 {
    match inst as i32 {
        82 => 1_u64 << 22_i32,
        114 => 1_u64 << 22_i32,
        69 => 1_u64 << 2_i32,
        101 => 1_u64 << 2_i32,
        71 => 1_u64 << 23_i32,
        103 => 1_u64 << 22_i32,
        80 => 1_u64 << 26_i32,
        112 => 1_u64 << 26_i32,
        65 => 1_u64 << 3_i32,
        97 => 1_u64 << 3_i32,
        67 => 1_u64 << 25_i32,
        88 => 1_u64 << 24_i32,
        79 => 1_u64 << 25_i32,
        _ => 0_u64,
    }
}
unsafe extern "C" fn rs_compare_instructions_by_index(aa: *const c_void, bb: *const c_void) -> i32 {
    let a: *const solution_instruction = aa as *const solution_instruction;
    let b: *const solution_instruction = bb as *const solution_instruction;
    if (*a).index < (*b).index {
        return -1_i32;
    }
    if (*a).index > (*b).index {
        return 1_i32;
    }
    0_i32
}
unsafe extern "C" fn rs_compare_conduits_by_id(aa: *const c_void, bb: *const c_void) -> i32 {
    let a: *const conduit = aa as *const conduit;
    let b: *const conduit = bb as *const conduit;
    if (*a).id < (*b).id {
        return -1_i32;
    }
    if (*a).id > (*b).id {
        return 1_i32;
    }
    if (*a).glyph_index < (*b).glyph_index {
        return -1_i32;
    }
    if (*a).glyph_index > (*b).glyph_index {
        return 1_i32;
    }
    0_i32
}
unsafe extern "C" fn rs_create_disjoint_bonds(io: *mut input_output) {
    let disjoint_bond_path: *mut u32 = calloc(
        (*io).number_of_atoms as usize,
        ::core::mem::size_of::<u32>(),
    ) as *mut u32;
    let stack: *mut u32 = calloc(
        (*io).number_of_atoms as usize,
        ::core::mem::size_of::<u32>(),
    ) as *mut u32;
    let mut stack_depth: u32 = 0_u32;
    let mut i: u32 = 0_u32;
    while i < (*io).number_of_atoms {
        let a: *mut atom_at_position = (*io).atoms.add(i as usize);
        if (*a).atom & VISITED == 0 {
            (*a).atom = ((*a).atom | VISITED) as atom;
            let slot = stack_depth;
            stack_depth = stack_depth.wrapping_add(1);
            *stack.add(slot as usize) = i;
            let c2rust_fresh14 = (*io).number_of_disjoint_bonds;
            (*io).number_of_disjoint_bonds = (*io).number_of_disjoint_bonds.wrapping_add(1);
            *disjoint_bond_path.add(c2rust_fresh14 as usize) = i;
            while stack_depth > 0_u32 {
                stack_depth = stack_depth.wrapping_sub(1);
                let a_0: *mut atom_at_position = (*io)
                    .atoms
                    .offset(*stack.offset(stack_depth as isize) as isize);
                let mut bond_direction_0: i32 = 0_i32;
                while bond_direction_0 < 6_i32 {
                    let ab: atom = (((RECENT_BOND0
                        | NORMAL_BOND0
                        | TRIPLEX_R_BOND0
                        | TRIPLEX_Y_BOND0
                        | TRIPLEX_K_BOND0)
                        & !(0x3f_u64 << 29_i32))
                        << bond_direction_0) as atom;
                    if (*a_0).atom & ab != 0 {
                        let d: vector = rs_u_offset_for_direction(bond_direction_0);
                        let p: vector = vector {
                            u: (*a_0).position.u + d.u,
                            v: (*a_0).position.v + d.v,
                        };
                        let mut j: u32 = 0_u32;
                        while j < (*io).number_of_atoms {
                            let b: *mut atom_at_position = (*io).atoms.add(j as usize);
                            if rs_vectors_equal((*b).position, p) {
                                if (*b).atom & VISITED == 0 {
                                    (*b).atom = ((*b).atom | VISITED) as atom;
                                    let index = stack_depth;
                                    stack_depth = stack_depth.wrapping_add(1);
                                    *stack.add(index as usize) = j;
                                }
                                break;
                            } else {
                                j = j.wrapping_add(1);
                            }
                        }
                    }
                    bond_direction_0 += 1;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    if (*io).number_of_disjoint_bonds == 1_u32 {
        (*io).number_of_disjoint_bonds = 0_u32;
    }
    (*io).disjoint_bonds = calloc(
        (*io).number_of_disjoint_bonds as usize,
        ::core::mem::size_of::<disjoint_bond>(),
    ) as *mut disjoint_bond;
    let mut i_0: u32 = 0_u32;
    while i_0 < (*io).number_of_disjoint_bonds {
        (*(*io).disjoint_bonds.add(i_0 as usize)).from_position = (*(*io)
            .atoms
            .offset(*disjoint_bond_path.offset(i_0 as isize) as isize))
        .position;
        (*(*io)
            .atoms
            .offset(*disjoint_bond_path.offset(i_0 as isize) as isize))
        .atom = ((*(*io)
            .atoms
            .offset(*disjoint_bond_path.offset(i_0 as isize) as isize))
        .atom as u64
            | HAS_DISJOINT_BOND) as atom;
        if i_0 == (*io).number_of_disjoint_bonds.wrapping_sub(1_u32) {
            (*(*io).disjoint_bonds.add(i_0 as usize)).to_position = (*(*io)
                .atoms
                .add((*disjoint_bond_path.offset(0isize)) as usize))
            .position;
        } else {
            (*(*io).disjoint_bonds.add(i_0 as usize)).to_position = (*(*io)
                .atoms
                .offset(*disjoint_bond_path.offset(i_0.wrapping_add(1_u32) as isize) as isize))
            .position;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: u32 = 0_u32;
    while i_1 < (*io).number_of_atoms {
        (*(*io).atoms.add(i_1 as usize)).atom =
            ((*(*io).atoms.add(i_1 as usize)).atom & !(VISITED)) as atom;
        i_1 = i_1.wrapping_add(1);
    }
    free(disjoint_bond_path as *mut c_void);
    free(stack as *mut c_void);
}
unsafe extern "C" fn rs_decode_molecule(c: puzzle_molecule, m: mechanism, io: *mut input_output) {
    (*io).atoms = calloc(
        c.number_of_atoms as usize,
        ::core::mem::size_of::<atom_at_position>(),
    ) as *mut atom_at_position;
    (*io).number_of_atoms = c.number_of_atoms;
    let mut i: u32 = 0_u32;
    while i < c.number_of_atoms {
        (*(*io).atoms.add(i as usize)).atom =
            rs_decode_atom((*c.atoms.add(i as usize)).r#type as u32);
        (*(*io).atoms.add(i as usize)).position = rs_mechanism_relative_position(
            m,
            (*c.atoms.add(i as usize)).offset[0usize] as i32,
            (*c.atoms.add(i as usize)).offset[1usize] as i32,
            1_i32,
        );
        if (*c.atoms.add(i as usize)).offset[0usize] as i32 == 0_i32
            && (*c.atoms.add(i as usize)).offset[1usize] as i32 == 0_i32
        {
            (*io).center_atom_index = i;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: u32 = 0_u32;
    while i_0 < c.number_of_bonds {
        let b: puzzle_bond = *c.bonds.add(i_0 as usize) as puzzle_bond;
        let bonds: atom = rs_decode_bond_type((*c.bonds.add(i_0 as usize)).r#type);
        let p1: vector =
            rs_mechanism_relative_position(m, b.from[0usize] as i32, b.from[1usize] as i32, 1_i32);
        let p2: vector =
            rs_mechanism_relative_position(m, b.to[0usize] as i32, b.to[1usize] as i32, 1_i32);
        let b1: atom = bonds
            & rs_bond_direction(
                m,
                b.to[0usize] as i32 - b.from[0usize] as i32,
                b.to[1usize] as i32 - b.from[1usize] as i32,
            );
        let b2: atom = bonds
            & rs_bond_direction(
                m,
                b.from[0usize] as i32 - b.to[0usize] as i32,
                b.from[1usize] as i32 - b.to[1usize] as i32,
            );
        let mut j: u32 = 0_u32;
        while j < c.number_of_atoms {
            let p: vector = (*(*io).atoms.add(j as usize)).position;
            if p.u == p1.u && p.v == p1.v {
                (*(*io).atoms.add(j as usize)).atom |= b1;
            }
            if p.u == p2.u && p.v == p2.v {
                (*(*io).atoms.add(j as usize)).atom |= b2;
            }
            j = j.wrapping_add(1);
        }
        i_0 = i_0.wrapping_add(1);
    }
    let rightward_bonds: atom = rs_bond_direction(m, 1_i32, 0_i32)
        | rs_bond_direction(m, 0_i32, 1_i32)
        | rs_bond_direction(m, 1_i32, -1_i32);
    if (*io).r#type.0 & input_output_type::INPUT.0 != 0 {
        let mut i_1: u32 = 0_u32;
        while i_1 < (*io).number_of_atoms {
            if (*(*io).atoms.add(i_1 as usize)).atom & rightward_bonds == 0 {
                (*(*io).atoms.add(i_1 as usize)).atom =
                    ((*(*io).atoms.add(i_1 as usize)).atom | DISJOINT_SAFE) as atom;
            }
            i_1 = i_1.wrapping_add(1);
        }
    }
}
static mut rs_cabinet_walls_Small: [vector; 12] = [
    vector { u: 2_i32, v: 0_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 0_i32, v: 2_i32 },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: -1_i32,
    },
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_walls_SmallWide: [vector; 14] = [
    vector { u: 3_i32, v: 0_i32 },
    vector { u: 2_i32, v: 1_i32 },
    vector { u: 1_i32, v: 2_i32 },
    vector { u: 0_i32, v: 2_i32 },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: -1_i32,
    },
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_walls_SmallWider: [vector; 16] = [
    vector { u: 4_i32, v: 0_i32 },
    vector { u: 3_i32, v: 1_i32 },
    vector { u: 2_i32, v: 2_i32 },
    vector { u: 1_i32, v: 2_i32 },
    vector { u: 0_i32, v: 2_i32 },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: -1_i32,
    },
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -2_i32,
    },
    vector {
        u: 4_i32,
        v: -2_i32,
    },
    vector {
        u: 4_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_walls_Medium: [vector; 18] = [
    vector { u: 3_i32, v: 0_i32 },
    vector { u: 2_i32, v: 1_i32 },
    vector { u: 1_i32, v: 2_i32 },
    vector { u: 0_i32, v: 3_i32 },
    vector {
        u: -1_i32,
        v: 3_i32,
    },
    vector {
        u: -2_i32,
        v: 3_i32,
    },
    vector {
        u: -3_i32,
        v: 3_i32,
    },
    vector {
        u: -3_i32,
        v: 2_i32,
    },
    vector {
        u: -3_i32,
        v: 1_i32,
    },
    vector {
        u: -3_i32,
        v: 0_i32,
    },
    vector {
        u: -2_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: -2_i32,
    },
    vector {
        u: 0_i32,
        v: -3_i32,
    },
    vector {
        u: 1_i32,
        v: -3_i32,
    },
    vector {
        u: 2_i32,
        v: -3_i32,
    },
    vector {
        u: 3_i32,
        v: -3_i32,
    },
    vector {
        u: 3_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_walls_MediumWide: [vector; 20] = [
    vector { u: 4_i32, v: 0_i32 },
    vector { u: 3_i32, v: 1_i32 },
    vector { u: 2_i32, v: 2_i32 },
    vector { u: 1_i32, v: 3_i32 },
    vector { u: 0_i32, v: 3_i32 },
    vector {
        u: -1_i32,
        v: 3_i32,
    },
    vector {
        u: -2_i32,
        v: 3_i32,
    },
    vector {
        u: -3_i32,
        v: 3_i32,
    },
    vector {
        u: -3_i32,
        v: 2_i32,
    },
    vector {
        u: -3_i32,
        v: 1_i32,
    },
    vector {
        u: -3_i32,
        v: 0_i32,
    },
    vector {
        u: -2_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: -2_i32,
    },
    vector {
        u: 0_i32,
        v: -3_i32,
    },
    vector {
        u: 1_i32,
        v: -3_i32,
    },
    vector {
        u: 2_i32,
        v: -3_i32,
    },
    vector {
        u: 3_i32,
        v: -3_i32,
    },
    vector {
        u: 4_i32,
        v: -3_i32,
    },
    vector {
        u: 4_i32,
        v: -2_i32,
    },
    vector {
        u: 4_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_walls_Large: [vector; 24] = [
    vector { u: 4_i32, v: 0_i32 },
    vector { u: 3_i32, v: 1_i32 },
    vector { u: 2_i32, v: 2_i32 },
    vector { u: 1_i32, v: 3_i32 },
    vector { u: 0_i32, v: 4_i32 },
    vector {
        u: -1_i32,
        v: 4_i32,
    },
    vector {
        u: -2_i32,
        v: 4_i32,
    },
    vector {
        u: -3_i32,
        v: 4_i32,
    },
    vector {
        u: -4_i32,
        v: 4_i32,
    },
    vector {
        u: -4_i32,
        v: 3_i32,
    },
    vector {
        u: -4_i32,
        v: 2_i32,
    },
    vector {
        u: -4_i32,
        v: 1_i32,
    },
    vector {
        u: -4_i32,
        v: 0_i32,
    },
    vector {
        u: -3_i32,
        v: -1_i32,
    },
    vector {
        u: -2_i32,
        v: -2_i32,
    },
    vector {
        u: -1_i32,
        v: -3_i32,
    },
    vector {
        u: 0_i32,
        v: -4_i32,
    },
    vector {
        u: 1_i32,
        v: -4_i32,
    },
    vector {
        u: 2_i32,
        v: -4_i32,
    },
    vector {
        u: 3_i32,
        v: -4_i32,
    },
    vector {
        u: 4_i32,
        v: -4_i32,
    },
    vector {
        u: 4_i32,
        v: -3_i32,
    },
    vector {
        u: 4_i32,
        v: -2_i32,
    },
    vector {
        u: 4_i32,
        v: -1_i32,
    },
];
static mut rs_cabinet_insides_Small: [vector; 7] = [
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
];
static mut rs_cabinet_insides_SmallWide: [vector; 10] = [
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 2_i32, v: 0_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 1_i32 },
];
static mut rs_cabinet_insides_SmallWider: [vector; 13] = [
    vector {
        u: 0_i32,
        v: -1_i32,
    },
    vector {
        u: 1_i32,
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
    vector {
        u: 3_i32,
        v: -1_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 2_i32, v: 0_i32 },
    vector { u: 3_i32, v: 0_i32 },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 2_i32, v: 1_i32 },
];
static mut rs_cabinet_insides_Medium: [vector; 19] = [
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
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
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 2_i32, v: 0_i32 },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector { u: 0_i32, v: 2_i32 },
];
static mut rs_cabinet_insides_MediumWide: [vector; 24] = [
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -2_i32,
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
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
    vector {
        u: 3_i32,
        v: -1_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 2_i32, v: 0_i32 },
    vector { u: 3_i32, v: 0_i32 },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 2_i32, v: 1_i32 },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector { u: 0_i32, v: 2_i32 },
    vector { u: 1_i32, v: 2_i32 },
];
static mut rs_cabinet_insides_Large: [vector; 37] = [
    vector {
        u: 0_i32,
        v: -3_i32,
    },
    vector {
        u: 1_i32,
        v: -3_i32,
    },
    vector {
        u: 2_i32,
        v: -3_i32,
    },
    vector {
        u: 3_i32,
        v: -3_i32,
    },
    vector {
        u: -1_i32,
        v: -2_i32,
    },
    vector {
        u: 0_i32,
        v: -2_i32,
    },
    vector {
        u: 1_i32,
        v: -2_i32,
    },
    vector {
        u: 2_i32,
        v: -2_i32,
    },
    vector {
        u: 3_i32,
        v: -2_i32,
    },
    vector {
        u: -2_i32,
        v: -1_i32,
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
        v: -1_i32,
    },
    vector {
        u: 2_i32,
        v: -1_i32,
    },
    vector {
        u: 3_i32,
        v: -1_i32,
    },
    vector {
        u: -3_i32,
        v: 0_i32,
    },
    vector {
        u: -2_i32,
        v: 0_i32,
    },
    vector {
        u: -1_i32,
        v: 0_i32,
    },
    vector { u: 0_i32, v: 0_i32 },
    vector { u: 1_i32, v: 0_i32 },
    vector { u: 2_i32, v: 0_i32 },
    vector { u: 3_i32, v: 0_i32 },
    vector {
        u: -3_i32,
        v: 1_i32,
    },
    vector {
        u: -2_i32,
        v: 1_i32,
    },
    vector {
        u: -1_i32,
        v: 1_i32,
    },
    vector { u: 0_i32, v: 1_i32 },
    vector { u: 1_i32, v: 1_i32 },
    vector { u: 2_i32, v: 1_i32 },
    vector {
        u: -3_i32,
        v: 2_i32,
    },
    vector {
        u: -2_i32,
        v: 2_i32,
    },
    vector {
        u: -1_i32,
        v: 2_i32,
    },
    vector { u: 0_i32, v: 2_i32 },
    vector { u: 1_i32, v: 2_i32 },
    vector {
        u: -3_i32,
        v: 3_i32,
    },
    vector {
        u: -2_i32,
        v: 3_i32,
    },
    vector {
        u: -1_i32,
        v: 3_i32,
    },
    vector { u: 0_i32, v: 3_i32 },
];
unsafe extern "C" fn rs_number_of_walls_for_cabinet_type(r#type: byte_string) -> usize {
    if rs_byte_string_is(r#type, b"Small\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 12]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"SmallWide\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 14]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"SmallWider\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 16]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"Medium\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 18]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"MediumWide\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 20]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"Large\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 24]>().wrapping_div(::core::mem::size_of::<vector>())
    } else {
        0_usize
    }
}
unsafe extern "C" fn rs_get_walls_for_cabinet_type(r#type: byte_string) -> *mut vector {
    if rs_byte_string_is(r#type, b"Small\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_Small as *mut vector
    } else if rs_byte_string_is(r#type, b"SmallWide\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_SmallWide as *mut vector
    } else if rs_byte_string_is(r#type, b"SmallWider\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_SmallWider as *mut vector
    } else if rs_byte_string_is(r#type, b"Medium\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_Medium as *mut vector
    } else if rs_byte_string_is(r#type, b"MediumWide\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_MediumWide as *mut vector
    } else if rs_byte_string_is(r#type, b"Large\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_walls_Large as *mut vector
    } else {
        ::core::ptr::null_mut::<vector>()
    }
}
unsafe extern "C" fn rs_copy_walls_for_cabinet_type(
    r#type: byte_string,
    dest: *mut vector,
    u: i32,
    v: i32,
) -> usize {
    let mut src: *mut vector = ::core::ptr::null_mut::<vector>();
    if rs_byte_string_is(r#type, b"Small\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_Small as *mut vector;
    } else if rs_byte_string_is(r#type, b"SmallWide\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_SmallWide as *mut vector;
    } else if rs_byte_string_is(r#type, b"SmallWider\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_SmallWider as *mut vector;
    } else if rs_byte_string_is(r#type, b"Medium\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_Medium as *mut vector;
    } else if rs_byte_string_is(r#type, b"MediumWide\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_MediumWide as *mut vector;
    } else if rs_byte_string_is(r#type, b"Large\0".as_ptr() as *const c_char) {
        src = &raw mut rs_cabinet_walls_Large as *mut vector;
    } else {
        return 0_usize;
    }
    let n: usize = rs_number_of_walls_for_cabinet_type(r#type);
    let mut i: usize = 0_usize;
    while i < n {
        (*dest.add(i)).u = (*src.add(i)).u + u;
        (*dest.add(i)).v = (*src.add(i)).v + v;
        i = i.wrapping_add(1);
    }
    n
}
unsafe extern "C" fn rs_number_of_insides_for_cabinet_type(r#type: byte_string) -> usize {
    if rs_byte_string_is(r#type, b"Small\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 7]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"SmallWide\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 10]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"SmallWider\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 13]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"Medium\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 19]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"MediumWide\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 24]>().wrapping_div(::core::mem::size_of::<vector>())
    } else if rs_byte_string_is(r#type, b"Large\0".as_ptr() as *const c_char) {
        ::core::mem::size_of::<[vector; 37]>().wrapping_div(::core::mem::size_of::<vector>())
    } else {
        0_usize
    }
}
unsafe extern "C" fn rs_get_insides_for_cabinet_type(r#type: byte_string) -> *mut vector {
    if rs_byte_string_is(r#type, b"Small\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_Small as *mut vector
    } else if rs_byte_string_is(r#type, b"SmallWide\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_SmallWide as *mut vector
    } else if rs_byte_string_is(r#type, b"SmallWider\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_SmallWider as *mut vector
    } else if rs_byte_string_is(r#type, b"Medium\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_Medium as *mut vector
    } else if rs_byte_string_is(r#type, b"MediumWide\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_MediumWide as *mut vector
    } else if rs_byte_string_is(r#type, b"Large\0".as_ptr() as *const c_char) {
        &raw mut rs_cabinet_insides_Large as *mut vector
    } else {
        ::core::ptr::null_mut::<vector>()
    }
}
unsafe extern "C" fn rs_mark_visible_region(solution: *mut solution, p: vector, hex_radius: i32) {
    if p.u < -2147483647_i32 - 1_i32 + hex_radius {
        (*solution).min_visible_u = -2147483647_i32 - 1_i32;
    } else if p.u - hex_radius < (*solution).min_visible_u {
        (*solution).min_visible_u = p.u - hex_radius;
    }
    if p.u > 2147483647_i32 - hex_radius {
        (*solution).max_visible_u = 2147483647_i32;
    } else if p.u + hex_radius > (*solution).max_visible_u {
        (*solution).max_visible_u = p.u + hex_radius;
    }
    if p.v < -2147483647_i32 - 1_i32 + hex_radius {
        (*solution).min_visible_v = -2147483647_i32 - 1_i32;
    } else if p.v - hex_radius < (*solution).min_visible_v {
        (*solution).min_visible_v = p.v - hex_radius;
    }
    if p.v > 2147483647_i32 - hex_radius {
        (*solution).max_visible_v = 2147483647_i32;
    } else if p.v + hex_radius > (*solution).max_visible_v {
        (*solution).max_visible_v = p.v + hex_radius;
    }
}
unsafe extern "C" fn rs_mark_visible_input_output(solution: *mut solution, io: *mut input_output) {
    if (*io).r#type.0 & input_output_type::REPEATING_OUTPUT.0 != 0 {
        let mut i: u32 = 0_u32;
        while i < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
            rs_mark_visible_region(solution, (*(*io).atoms.add(i as usize)).position, 0_i32);
            i = i.wrapping_add(1);
        }
    } else {
        let mut i_0: u32 = 0_u32;
        while i_0 < (*io).number_of_atoms {
            rs_mark_visible_region(
                solution,
                (*(*io).atoms.add(i_0 as usize)).position,
                0_i32,
            );
            i_0 = i_0.wrapping_add(1);
        }
    };
}
unsafe extern "C" fn rs_decode_position(position: *mut i8) -> vector {
    vector {
        u: *position.add(0) as i32,
        v: *position.add(1) as i32,
    }
}
unsafe extern "C" fn rs_check_production_constraints(
    solution: *mut solution,
    info: *mut puzzle_production_info,
) {
    let mut i: u32 = 0_u32;
    while i < (*info).number_of_cabinets {
        let mut src: *mut vector =
            rs_get_insides_for_cabinet_type((*(*info).cabinets.add(i as usize)).r#type);
        let mut n: usize =
            rs_number_of_insides_for_cabinet_type((*(*info).cabinets.add(i as usize)).r#type);
        let mut j: usize = 0_usize;
        while j < n {
            let u: i32 = (*(*info).cabinets.add(i as usize)).position[0usize] as i32
                + (*src.add(j)).u
                + (32_i32 >> 1_i32);
            let v: i32 = (*(*info).cabinets.add(i as usize)).position[1usize] as i32
                + (*src.add(j)).v
                + (32_i32 >> 1_i32);
            if u >= 0_i32 && u < 32_i32 && v >= 0_i32 && v < 32_i32 {
                (*solution).cabinet_map[u as usize][v as usize] = i.wrapping_add(1_u32) as u8;
            }
            j = j.wrapping_add(1);
        }
        src = rs_get_walls_for_cabinet_type((*(*info).cabinets.add(i as usize)).r#type);
        n = rs_number_of_walls_for_cabinet_type((*(*info).cabinets.add(i as usize)).r#type);
        let mut j_0: usize = 0_usize;
        while j_0 < n {
            let u_0: i32 = (*(*info).cabinets.add(i as usize)).position[0usize] as i32
                + (*src.add(j_0)).u
                + (32_i32 >> 1_i32);
            let v_0: i32 = (*(*info).cabinets.add(i as usize)).position[1usize] as i32
                + (*src.add(j_0)).v
                + (32_i32 >> 1_i32);
            if u_0 >= 0_i32 && u_0 < 32_i32 && v_0 >= 0_i32 && v_0 < 32_i32 {
                (*solution).cabinet_map[u_0 as usize][v_0 as usize] = 255_u8;
            }
            j_0 = j_0.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    let mut swapped: bool = false;
    if (*solution).number_of_conduits != 2_u32.wrapping_mul((*info).number_of_conduits) as usize {
        (*solution).cabinet_violations =
            ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 7_i32) as u32;
    } else {
        let mut i_0: u32 = 0_u32;
        while (i_0 as usize) < (*solution).number_of_conduits {
            let conduit: *mut conduit = (*solution).conduits.add(i_0 as usize);
            let conduit_index: u32 = (*conduit).id.wrapping_sub(100_u32);
            if conduit_index >= (*info).number_of_conduits {
                (*solution).cabinet_violations =
                    ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 7_i32) as u32;
                break;
            } else {
                let puzzle_conduit: *mut puzzle_conduit =
                    (*info).conduits.add(conduit_index as usize) as *mut puzzle_conduit;
                if (*conduit).number_of_positions != (*puzzle_conduit).number_of_hexes {
                    (*solution).cabinet_violations = ((*solution).cabinet_violations as c_ulong
                        | (1 as c_ulong) << 7_i32)
                        as u32;
                    break;
                } else {
                    let starting_position: *mut i8 = if i_0 & 1_u32 != 0 {
                        &raw mut (*puzzle_conduit).starting_position_b as *mut i8
                    } else {
                        &raw mut (*puzzle_conduit).starting_position_a as *mut i8
                    };
                    let expected_cabinet: u8 = rs_cabinet_for_position(
                        solution,
                        rs_decode_position(starting_position as *mut i8),
                    );
                    if i_0 & 1_u32 == 0_u32 {
                        let center: vector =
                            (*(*solution).glyphs.add((*conduit).glyph_index as usize)).position;
                        swapped = rs_cabinet_for_position(solution, center) as i32
                            != expected_cabinet as i32;
                    }
                    let m: *mut mechanism = (*solution).glyphs.offset(
                        (if swapped as i32 != 0 {
                            (*conduit).other_side_glyph_index
                        } else {
                            (*conduit).glyph_index
                        }) as isize,
                    );
                    if rs_cabinet_for_position(solution, (*m).position) as i32
                        != expected_cabinet as i32
                    {
                        (*solution).cabinet_violations = ((*solution).cabinet_violations as c_ulong
                            | (1 as c_ulong) << 6_i32)
                            as u32;
                        break;
                    } else {
                        let mut j_1: u32 = 0_u32;
                        while j_1 < (*conduit).number_of_positions {
                            let pos: vector = *(*conduit).positions.add(j_1 as usize);
                            if !rs_vectors_equal(
                                pos,
                                rs_decode_position(
                                    &raw mut (*(*puzzle_conduit).hexes.add(j_1 as usize)).offset
                                        as *mut i8,
                                ),
                            ) {
                                (*solution).cabinet_violations = ((*solution).cabinet_violations
                                    as c_ulong
                                    | (1 as c_ulong) << 7_i32)
                                    as u32;
                                break;
                            } else {
                                j_1 = j_1.wrapping_add(1);
                            }
                        }
                        i_0 = i_0.wrapping_add(1);
                    }
                }
            }
        }
    }
    let mut i_1: u32 = 0_u32;
    while (i_1 as usize) < (*solution).number_of_glyphs {
        let m_0: *mut mechanism = (*solution).glyphs.add(i_1 as usize);
        if (*m_0).r#type as c_ulong != CONDUIT {
            let footprint: *const vector = rs_glyph_footprint((*m_0).r#type);
            let mut j_2: i32 = 0_i32;
            loop {
                let pos_0: vector = rs_mechanism_relative_position(
                    *m_0,
                    (*footprint.offset(j_2 as isize)).u,
                    (*footprint.offset(j_2 as isize)).v,
                    1_i32,
                );
                let cabinet: u8 = rs_cabinet_for_position(solution, pos_0);
                if cabinet as i32 == 0_i32 || cabinet as i32 == 255_i32 {
                    (*solution).cabinet_violations = ((*solution).cabinet_violations as c_ulong
                        | (1 as c_ulong) << 0_i32)
                        as u32;
                    break;
                } else {
                    if rs_vectors_equal(*footprint.offset(j_2 as isize), rs_zero_vector) {
                        break;
                    }
                    j_2 += 1;
                }
            }
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: u32 = 0_u32;
    while (i_2 as usize) < (*solution).number_of_arms {
        let m_1: *mut mechanism = (*solution).arms.add(i_2 as usize);
        let base_cabinet: u8 = rs_cabinet_for_position(solution, (*m_1).position);
        if base_cabinet as i32 == 0_i32 || base_cabinet as i32 == 255_i32 {
            (*solution).cabinet_violations =
                ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 1_i32) as u32;
            break;
        } else {
            let step: i32 = rs_angular_distance_between_grabbers((*m_1).r#type);
            let mut direction: i32 = 0_i32;
            while direction < 6_i32 {
                let offset: vector = rs_u_offset_for_direction(direction);
                let pos_1: vector = rs_mechanism_relative_position(*m_1, offset.u, offset.v, 1_i32);
                let cabinet_0: u8 = rs_cabinet_for_position(solution, pos_1);
                if cabinet_0 as i32 == 0_i32 || cabinet_0 as i32 == 255_i32 {
                    (*solution).cabinet_violations = ((*solution).cabinet_violations as c_ulong
                        | (1 as c_ulong) << 1_i32)
                        as u32;
                    break;
                } else if cabinet_0 as i32 != base_cabinet as i32 {
                    (*solution).cabinet_violations = ((*solution).cabinet_violations as c_ulong
                        | (1 as c_ulong) << 2_i32)
                        as u32;
                    break;
                } else {
                    direction += step;
                }
            }
            i_2 = i_2.wrapping_add(1);
        }
    }
    let mut i_3: u32 = 0_u32;
    while i_3 < (*solution).track_table_size {
        let position: vector = *(*solution).track_positions.add(i_3 as usize);
        if !(position.u == -2147483647_i32 - 1_i32 && position.v == -2147483647_i32 - 1_i32) {
            let cabinet_1: u8 = rs_cabinet_for_position(solution, position);
            if cabinet_1 as i32 == 0_i32 || cabinet_1 as i32 == 255_i32 {
                (*solution).cabinet_violations =
                    ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 4_i32) as u32;
                break;
            }
        }
        i_3 = i_3.wrapping_add(1);
    }
    let mut i_4: u32 = 0_u32;
    while (i_4 as usize) < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.add(i_4 as usize);
        let mut j_3: u32 = 0_u32;
        while j_3 < (*io).number_of_atoms {
            let cabinet_2: u8 =
                rs_cabinet_for_position(solution, (*(*io).atoms.add(j_3 as usize)).position);
            if cabinet_2 as i32 == 0_i32 || cabinet_2 as i32 == 255_i32 {
                (*solution).cabinet_violations =
                    ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 3_i32) as u32;
                break;
            } else {
                j_3 = j_3.wrapping_add(1);
            }
        }
        i_4 = i_4.wrapping_add(1);
    }
    if (*info).isolate_inputs_from_outputs {
        let isolation_data: *mut input_output_type = calloc(
            (*info).number_of_cabinets.wrapping_add(1_u32) as usize,
            ::core::mem::size_of::<input_output_type>(),
        ) as *mut input_output_type;
        let mut i_5: u32 = 0_u32;
        while (i_5 as usize) < (*solution).number_of_inputs_and_outputs {
            let io_0: *mut input_output = (*solution).inputs_and_outputs.add(i_5 as usize);
            let cabinet_3: u8 = rs_cabinet_for_position(solution, (*(*io_0).atoms).position);
            *isolation_data.add(cabinet_3 as usize) = input_output_type(
                (*isolation_data.add(cabinet_3 as usize)).0 | (*io_0).r#type.0,
            );
            if (*isolation_data.add(cabinet_3 as usize)).0 & input_output_type::INPUT.0 != 0
                && (*isolation_data.add(cabinet_3 as usize)).0 & input_output_type::OUTPUT.0 != 0
            {
                (*solution).cabinet_violations =
                    ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 5_i32) as u32;
                break;
            } else {
                i_5 = i_5.wrapping_add(1);
            }
        }
        free(isolation_data as *mut c_void);
    }
}
unsafe extern "C" fn rs_set_tape(tape: *mut c_char, n: i32, inst: i32, error: *mut *const c_char) {
    if *tape.offset(n as isize) != 0 {
        *error = b"solution contains an instruction conflict\0".as_ptr() as *const c_char;
    }
    *tape.offset(n as isize) = inst as c_char;
}
unsafe extern "C" fn rs_compare_vectors(aa: *const c_void, bb: *const c_void) -> i32 {
    let a: vector = *(aa as *const vector);
    let b: vector = *(bb as *const vector);
    if a.u < b.u {
        return -1_i32;
    } else if a.u > b.u {
        return 1_i32;
    } else if a.v < b.v {
        return -1_i32;
    } else if a.v > b.v {
        return 1_i32;
    }
    0_i32
}
unsafe extern "C" fn rs_repeat_molecule(io: *mut input_output, error: *mut *const c_char) -> bool {
    if (*io).number_of_original_atoms == 0_u32 {
        *error = b"puzzle contains an empty infinite product\0".as_ptr() as *const c_char;
        return false;
    }
    let mut offset: vector = (*(*io)
        .original_atoms
        .offset((*io).number_of_original_atoms.wrapping_sub(1_u32) as isize))
    .position;
    (*io).monomer_width = rs_polymer_position_from_global_position(io, offset).u;
    offset.u -= (*io).repetition_origin.u;
    offset.v -= (*io).repetition_origin.v;
    let mut number_of_placeholders: u32 = 0_u32;
    let placeholders: *mut vector = calloc(
        (*io).number_of_original_atoms.wrapping_mul(6_u32) as usize,
        ::core::mem::size_of::<vector>(),
    ) as *mut vector;
    let mut i: u32 = 0_u32;
    while i < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
        let p: vector = (*(*io).original_atoms.add(i as usize)).position;
        let mut j: i32 = 0_i32;
        while j < 6_i32 {
            if (*(*io).original_atoms.add(i as usize)).atom
                & (RECENT_BOND0
                    | NORMAL_BOND0
                    | TRIPLEX_R_BOND0
                    | TRIPLEX_Y_BOND0
                    | TRIPLEX_K_BOND0)
                    << j
                != 0
            {
                let bond_offset: vector = rs_u_offset_for_direction(j);
                let target_position: vector = vector {
                    u: p.u + bond_offset.u - offset.u,
                    v: p.v + bond_offset.v - offset.v,
                };
                let mut k: u32 = 0_u32;
                while k < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
                    let mut q: vector = (*(*io).original_atoms.add(k as usize)).position;
                    if rs_vectors_equal(target_position, q) {
                        q.u = q.u + (offset.u * 6_i32);
                        q.v = q.v + (offset.v * 6_i32);
                        let index = number_of_placeholders;
                        number_of_placeholders = number_of_placeholders.wrapping_add(1);
                        *placeholders.add(index as usize) = q;
                        break;
                    } else {
                        k = k.wrapping_add(1);
                    }
                }
            }
            j += 1;
        }
        i = i.wrapping_add(1);
    }
    if number_of_placeholders == 0_u32 {
        let mut placeholder_position: vector = (*(*io)
            .original_atoms
            .offset((*io).number_of_original_atoms.wrapping_sub(1_u32) as isize))
        .position;
        placeholder_position.u = placeholder_position.u + (offset.u * (6_i32 - 1_i32));
        placeholder_position.v = placeholder_position.v + (offset.v * (6_i32 - 1_i32));
        *placeholders.add(0) = placeholder_position;
        number_of_placeholders = 1_u32;
    }
    qsort(
        placeholders as *mut c_void,
        number_of_placeholders as usize,
        ::core::mem::size_of::<vector>(),
        Some(rs_compare_vectors as unsafe extern "C" fn(*const c_void, *const c_void) -> i32),
    );
    let mut removed_duplicates: u32 = 0_u32;
    let mut i_0: u32 = 0_u32;
    while i_0 < number_of_placeholders {
        if i_0 > 0_u32
            && rs_vectors_equal(
                *placeholders.offset(i_0.wrapping_sub(1_u32) as isize),
                *placeholders.add(i_0 as usize),
            ) as i32
                != 0
        {
            removed_duplicates = removed_duplicates.wrapping_add(1);
        } else {
            *placeholders.offset(i_0.wrapping_sub(removed_duplicates) as isize) =
                *placeholders.add(i_0 as usize);
        }
        i_0 = i_0.wrapping_add(1);
    }
    number_of_placeholders = number_of_placeholders.wrapping_sub(removed_duplicates);
    let atoms: *mut atom_at_position = calloc(
        (*io)
            .number_of_original_atoms
            .wrapping_sub(1_u32)
            .wrapping_mul(6_u32)
            .wrapping_add(number_of_placeholders) as usize,
        ::core::mem::size_of::<atom_at_position>(),
    ) as *mut atom_at_position;
    let mut i_1: u32 = 0_u32;
    while i_1 < 6_u32 {
        let mut j_0: u32 = 0_u32;
        while j_0 < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
            let a: *mut atom_at_position = atoms.offset(
                (*io)
                    .number_of_original_atoms
                    .wrapping_sub(1_u32)
                    .wrapping_mul(i_1)
                    .wrapping_add(j_0) as isize,
            );
            *a = *(*io).original_atoms.add(j_0 as usize);
            (*a).position.u =
                ((*a).position.u as u32).wrapping_add(i_1.wrapping_mul(offset.u as u32)) as i32;
            (*a).position.v =
                ((*a).position.v as u32).wrapping_add(i_1.wrapping_mul(offset.v as u32)) as i32;
            (*a).atom = (*(*io).original_atoms.add(j_0 as usize)).atom;
            j_0 = j_0.wrapping_add(1);
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: u32 = 0_u32;
    while i_2 < number_of_placeholders {
        let a_0: *mut atom_at_position = atoms.offset(
            (*io)
                .number_of_original_atoms
                .wrapping_sub(1_u32)
                .wrapping_mul(6_u32)
                .wrapping_add(i_2) as isize,
        );
        (*a_0).atom = (REPEATING_OUTPUT_PLACEHOLDER) as atom;
        (*a_0).position = *placeholders.add(i_2 as usize);
        i_2 = i_2.wrapping_add(1);
    }
    free(placeholders as *mut c_void);
    free((*io).atoms as *mut c_void);
    (*io).atoms = atoms;
    (*io).number_of_atoms = (*io)
        .number_of_original_atoms
        .wrapping_sub(1_u32)
        .wrapping_mul(6_u32)
        .wrapping_add(number_of_placeholders);
    (*io).min_v = 2147483647_i32;
    (*io).max_v = -2147483647_i32 - 1_i32;
    let mut i_3: u32 = 0_u32;
    while i_3 < (*io).number_of_atoms {
        if (*(*io).atoms.add(i_3 as usize)).atom & REPEATING_OUTPUT_PLACEHOLDER == 0 {
            let p_0: vector = rs_polymer_position_from_global_position(
                io,
                (*(*io).atoms.add(i_3 as usize)).position,
            );
            if p_0.v < (*io).min_v {
                (*io).min_v = p_0.v;
            }
            if p_0.v > (*io).max_v {
                (*io).max_v = p_0.v;
            }
        }
        i_3 = i_3.wrapping_add(1);
    }
    let rows: usize = ((*io).max_v - (*io).min_v + 1_i32) as usize;
    (*io).row_min_u = realloc(
        (*io).row_min_u as *mut c_void,
        rows.wrapping_mul(::core::mem::size_of::<i32>()),
    ) as *mut i32;
    (*io).row_max_u = realloc(
        (*io).row_max_u as *mut c_void,
        rows.wrapping_mul(::core::mem::size_of::<i32>()),
    ) as *mut i32;
    let mut i_4: usize = 0_usize;
    while i_4 < rows {
        *(*io).row_min_u.add(i_4) = 2147483647_i32;
        *(*io).row_max_u.add(i_4) = -2147483647_i32 - 1_i32;
        i_4 = i_4.wrapping_add(1);
    }
    let mut i_5: u32 = 0_u32;
    while i_5 < (*io).number_of_atoms {
        if (*(*io).atoms.add(i_5 as usize)).atom & REPEATING_OUTPUT_PLACEHOLDER == 0 {
            let mut j_1: u32 = i_5.wrapping_add(1_u32);
            while j_1 < (*io).number_of_atoms {
                if (*(*io).atoms.add(j_1 as usize)).atom & REPEATING_OUTPUT_PLACEHOLDER == 0 {
                    let d: vector = vector {
                        u: (*(*io).atoms.add(j_1 as usize)).position.u
                            - (*(*io).atoms.add(i_5 as usize)).position.u,
                        v: (*(*io).atoms.add(j_1 as usize)).position.v
                            - (*(*io).atoms.add(i_5 as usize)).position.v,
                    };
                    let direction: i32 = rs_direction_for_offset(d);
                    if direction >= 0_i32 {
                        (*(*io).atoms.add(j_1 as usize)).atom |= rs_rotate_bond_bits(
                            ((*(*io).atoms.add(i_5 as usize)).atom
                                & (RECENT_BOND0
                                    | NORMAL_BOND0
                                    | TRIPLEX_R_BOND0
                                    | TRIPLEX_Y_BOND0
                                    | TRIPLEX_K_BOND0)
                                    << direction) as atom,
                            3_i32,
                        );
                    }
                }
                j_1 = j_1.wrapping_add(1);
            }
        }
        i_5 = i_5.wrapping_add(1);
    }
    let mut i_6: u32 = 0_u32;
    while i_6 < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
        if (*(*io).original_atoms.add(i_6 as usize)).atom & REPEATING_OUTPUT_PLACEHOLDER == 0 {
            let mut j_2: u32 = i_6.wrapping_add(1_u32);
            while j_2 < (*io).number_of_original_atoms.wrapping_sub(1_u32) {
                if (*(*io).original_atoms.add(j_2 as usize)).atom & REPEATING_OUTPUT_PLACEHOLDER
                    == 0
                {
                    let d_0: vector = vector {
                        u: (*(*io).original_atoms.add(j_2 as usize)).position.u
                            - (*(*io).original_atoms.add(i_6 as usize)).position.u,
                        v: (*(*io).original_atoms.add(j_2 as usize)).position.v
                            - (*(*io).original_atoms.add(i_6 as usize)).position.v,
                    };
                    let direction_0: i32 = rs_direction_for_offset(d_0);
                    if direction_0 >= 0_i32 {
                        let mut p_1: vector = rs_polymer_position_from_global_position(
                            io,
                            (*(*io).original_atoms.add(i_6 as usize)).position,
                        );
                        let mut q_0: vector = rs_polymer_position_from_global_position(
                            io,
                            (*(*io).original_atoms.add(j_2 as usize)).position,
                        );
                        if p_1.v > q_0.v {
                            let tmp: vector = p_1;
                            p_1 = q_0;
                            q_0 = tmp;
                        }
                        let mut v: i32 = p_1.v + 1_i32;
                        while v < q_0.v {
                            let row: usize = (v - (*io).min_v) as usize;
                            let u: i32 = p_1.u + (q_0.u - p_1.u) * (v - p_1.v) / (q_0.v - p_1.v);
                            if u < *(*io).row_min_u.add(row) {
                                *(*io).row_min_u.add(row) = u;
                            }
                            if u > *(*io).row_max_u.add(row) {
                                *(*io).row_max_u.add(row) = u;
                            }
                            v += 1;
                        }
                    }
                }
                j_2 = j_2.wrapping_add(1);
            }
            let p_2: vector = rs_polymer_position_from_global_position(
                io,
                (*(*io).original_atoms.add(i_6 as usize)).position,
            );
            let row_0: usize = (p_2.v - (*io).min_v) as usize;
            if p_2.u < *(*io).row_min_u.add(row_0) {
                *(*io).row_min_u.add(row_0) = p_2.u;
            }
            if p_2.u > *(*io).row_max_u.add(row_0) {
                *(*io).row_max_u.add(row_0) = p_2.u;
            }
        }
        i_6 = i_6.wrapping_add(1);
    }
    let mut i_7: usize = 0_usize;
    while i_7 < rows {
        if *(*io).row_min_u.add(i_7) == 2147483647_i32
            && *(*io).row_max_u.add(i_7) == -2147483647_i32 - 1_i32
        {
            *(*io).row_min_u.add(i_7) = -2147483647_i32 - 1_i32;
            *(*io).row_max_u.add(i_7) = 2147483647_i32;
        }
        i_7 = i_7.wrapping_add(1);
    }
    true
}
unsafe extern "C" fn rs_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0_i32 {
        let t: i32 = b;
        b = a % b;
        a = t;
    }
    a
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_decode_solution(
    solution: *mut solution,
    pf: *mut puzzle_file,
    sf: *mut solution_file,
    mut error: *mut *const c_char,
) -> bool {
    let mut ignored_error: *const c_char = ::core::ptr::null::<c_char>();
    if error.is_null() {
        error = &raw mut ignored_error;
    }
    *error = ::core::ptr::null::<c_char>();
    let mut i: u32 = 0_u32;
    while i < (*pf).number_of_inputs {
        if (*(*pf).inputs.add(i as usize)).number_of_atoms == 0_u32 {
            *error = b"puzzle file contains a reagent with no atoms\0".as_ptr() as *const c_char;
            return false;
        }
        i = i.wrapping_add(1);
    }
    let mut number_of_unplaced_outputs: i64 = 0_i64;
    let mut i_0: u32 = 0_u32;
    while i_0 < (*pf).number_of_outputs {
        number_of_unplaced_outputs += 1;
        if (*(*pf).outputs.add(i_0 as usize)).number_of_atoms == 0_u32 {
            *error = b"puzzle file contains a product with no atoms\0".as_ptr() as *const c_char;
            return false;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut number_of_arms: usize = 0_usize;
    let mut number_of_glyphs: usize = 0_usize;
    let mut number_of_conduits: usize = 0_usize;
    let mut number_of_track_hexes: usize = 0_usize;
    let mut number_of_inputs_and_outputs: usize = 0_usize;
    let mut i_1: u32 = 0_u32;
    while i_1 < (*sf).number_of_parts {
        let parts_available: u64 =
            rs_parts_available_bits_for_part_name((*(*sf).parts.add(i_1 as usize)).name);
        if parts_available & (*pf).parts_available != parts_available {
            *error = b"solution contains a part that has been disabled in the puzzle file\0"
                .as_ptr() as *const c_char;
            return false;
        }
        let mechanism_type: u32 =
            rs_decode_mechanism_type((*(*sf).parts.add(i_1 as usize)).name);
        if mechanism_type & rs_ANY_ARM != 0 {
            if (*(*sf).parts.add(i_1 as usize)).size > 3_u32 {
                (*(*sf).parts.add(i_1 as usize)).size = 3_u32;
            } else if (*(*sf).parts.add(i_1 as usize)).size < 1_u32 {
                (*(*sf).parts.add(i_1 as usize)).size = 1_u32;
            }
            number_of_arms = number_of_arms.wrapping_add(1);
        } else if mechanism_type & rs_ANY_GLYPH != 0 {
            number_of_glyphs = number_of_glyphs.wrapping_add(1);
        } else if rs_byte_string_is(
            (*(*sf).parts.add(i_1 as usize)).name,
            b"track\0".as_ptr() as *const c_char,
        ) {
            number_of_track_hexes = number_of_track_hexes
                .wrapping_add((*(*sf).parts.add(i_1 as usize)).number_of_track_hexes as usize);
        } else if rs_byte_string_is(
            (*(*sf).parts.add(i_1 as usize)).name,
            b"input\0".as_ptr() as *const c_char,
        ) {
            if (*(*sf).parts.add(i_1 as usize)).which_input_or_output >= (*pf).number_of_inputs {
                *error = b"solution refers to an input that doesn't exist in the puzzle file\0"
                    .as_ptr() as *const c_char;
                return false;
            }
            number_of_inputs_and_outputs = number_of_inputs_and_outputs.wrapping_add(1);
        } else if rs_byte_string_is(
            (*(*sf).parts.add(i_1 as usize)).name,
            b"out-std\0".as_ptr() as *const c_char,
        ) {
            if (*(*sf).parts.add(i_1 as usize)).which_input_or_output >= (*pf).number_of_outputs
            {
                *error = b"solution refers to an output that doesn't exist in the puzzle file\0"
                    .as_ptr() as *const c_char;
                return false;
            }
            number_of_inputs_and_outputs = number_of_inputs_and_outputs.wrapping_add(1);
            number_of_unplaced_outputs -= 1;
        } else if rs_byte_string_is(
            (*(*sf).parts.add(i_1 as usize)).name,
            b"out-rep\0".as_ptr() as *const c_char,
        ) {
            let which_output: u32 = (*(*sf).parts.add(i_1 as usize)).which_input_or_output;
            if which_output >= (*pf).number_of_outputs {
                *error = b"solution refers to an output that doesn't exist in the puzzle file\0"
                    .as_ptr() as *const c_char;
                return false;
            }
            number_of_inputs_and_outputs = number_of_inputs_and_outputs.wrapping_add(1);
            number_of_unplaced_outputs -= 1;
        } else if rs_byte_string_is(
            (*(*sf).parts.add(i_1 as usize)).name,
            b"pipe\0".as_ptr() as *const c_char,
        ) {
            number_of_glyphs = number_of_glyphs.wrapping_add(1);
            number_of_conduits = number_of_conduits.wrapping_add(1);
        }
        i_1 = i_1.wrapping_add(1);
    }
    if number_of_unplaced_outputs != 0_i64 {
        *error = b"all products in the puzzle must be placed in the solution\0".as_ptr()
            as *const c_char;
        rs_destroy(solution, ::core::ptr::null_mut::<board>());
        return false;
    }
    (*solution).number_of_arms = number_of_arms;
    (*solution).number_of_glyphs = number_of_glyphs;
    (*solution).number_of_conduits = number_of_conduits;
    (*solution).number_of_inputs_and_outputs = number_of_inputs_and_outputs;
    let mut number_of_cabinet_walls: usize = 0_usize;
    let mut i_2: u32 = 0_u32;
    while !(*pf).production_info.is_null() && i_2 < (*(*pf).production_info).number_of_cabinets {
        number_of_cabinet_walls =
            number_of_cabinet_walls.wrapping_add(rs_number_of_walls_for_cabinet_type(
                (*(*(*pf).production_info).cabinets.add(i_2 as usize)).r#type,
            ));
        i_2 = i_2.wrapping_add(1);
    }
    (*solution).number_of_cabinet_walls = number_of_cabinet_walls;
    (*solution).cabinet_walls =
        calloc(number_of_cabinet_walls, ::core::mem::size_of::<vector>()) as *mut vector;
    number_of_cabinet_walls = 0_usize;
    let mut i_3: u32 = 0_u32;
    while !(*pf).production_info.is_null() && i_3 < (*(*pf).production_info).number_of_cabinets {
        number_of_cabinet_walls =
            number_of_cabinet_walls.wrapping_add(rs_copy_walls_for_cabinet_type(
                (*(*(*pf).production_info).cabinets.add(i_3 as usize)).r#type,
                (*solution)
                    .cabinet_walls
                    .add(number_of_cabinet_walls),
                (*(*(*pf).production_info).cabinets.add(i_3 as usize)).position[0usize] as i32,
                (*(*(*pf).production_info).cabinets.add(i_3 as usize)).position[1usize] as i32,
            ));
        i_3 = i_3.wrapping_add(1);
    }
    (*solution).glyphs = calloc(
        (*solution).number_of_glyphs,
        ::core::mem::size_of::<mechanism>(),
    ) as *mut mechanism;
    (*solution).arms = calloc(
        (*solution).number_of_arms,
        ::core::mem::size_of::<mechanism>(),
    ) as *mut mechanism;
    (*solution).arm_tape = calloc(
        (*solution).number_of_arms,
        ::core::mem::size_of::<*mut c_char>(),
    ) as *mut *mut c_char;
    (*solution).arm_tape_length =
        calloc((*solution).number_of_arms, ::core::mem::size_of::<usize>()) as *mut usize;
    (*solution).arm_tape_start_cycle =
        calloc((*solution).number_of_arms, ::core::mem::size_of::<i64>()) as *mut i64;
    (*solution).arm_tape_halt_index =
        calloc((*solution).number_of_arms, ::core::mem::size_of::<usize>()) as *mut usize;
    (*solution).conduits = calloc(
        (*solution).number_of_conduits,
        ::core::mem::size_of::<conduit>(),
    ) as *mut conduit;
    (*solution).inputs_and_outputs = calloc(
        (*solution).number_of_inputs_and_outputs,
        ::core::mem::size_of::<input_output>(),
    ) as *mut input_output;
    (*solution).track_table_size = 1_u32;
    while (2_u32.wrapping_mul((*solution).track_table_size) as usize)
        < 3_usize.wrapping_mul(number_of_track_hexes)
    {
        (*solution).track_table_size = (*solution).track_table_size.wrapping_mul(2_u32);
    }
    (*solution).track_positions = calloc(
        (*solution).track_table_size as usize,
        ::core::mem::size_of::<vector>(),
    ) as *mut vector;
    let mut i_4: i32 = 0_i32;
    while (i_4 as u32) < (*solution).track_table_size {
        *(*solution).track_positions.offset(i_4 as isize) = vector {
            u: -2147483647_i32 - 1_i32,
            v: -2147483647_i32 - 1_i32,
        };
        i_4 += 1;
    }
    (*solution).track_plus_motions = calloc(
        (*solution).track_table_size as usize,
        ::core::mem::size_of::<vector>(),
    ) as *mut vector;
    (*solution).track_minus_motions = calloc(
        (*solution).track_table_size as usize,
        ::core::mem::size_of::<vector>(),
    ) as *mut vector;
    (*solution).min_visible_u = 2147483647_i32;
    (*solution).max_visible_u = -2147483647_i32 - 1_i32;
    (*solution).min_visible_v = 2147483647_i32;
    (*solution).max_visible_v = -2147483647_i32 - 1_i32;
    let mut arm_index: u32 = (*solution).number_of_arms.wrapping_sub(1_usize) as u32;
    let mut glyph_index: u32 = (*solution).number_of_glyphs.wrapping_sub(1_usize) as u32;
    let mut conduit_index: u32 = (*solution).number_of_conduits.wrapping_sub(1_usize) as u32;
    let mut io_index: usize = (*solution)
        .number_of_inputs_and_outputs
        .wrapping_sub(1_usize);
    let mut i_5: u32 = (*sf).number_of_parts.wrapping_sub(1_u32);
    while i_5 < (*sf).number_of_parts {
        let mut part: solution_part = *(*sf).parts.add(i_5 as usize) as solution_part;
        if rs_byte_string_is(part.name, b"track\0".as_ptr() as *const c_char) {
            part.rotation = 0_i32;
        } else {
            part.rotation = part.rotation % 6_i32;
        }
        let mut m: mechanism = mechanism {
            r#type: rs_decode_mechanism_type(part.name),
            position: vector {
                u: part.position[0usize],
                v: part.position[1usize],
            },
            direction_u: rs_u_offset_for_direction(part.rotation),
            direction_v: rs_v_offset_for_direction(part.rotation),
            conversion: 0,
            conduit_index: 0,
            movement: vector { u: 0, v: 0 },
            arm_rotation: part.rotation,
            pivot_parity: 0,
        };
        if m.r#type & rs_ANY_ARM != 0 {
            m.direction_u.u = (m.direction_u.u as u32).wrapping_mul(part.size) as i32;
            m.direction_u.v = (m.direction_u.v as u32).wrapping_mul(part.size) as i32;
            m.direction_v.u = (m.direction_v.u as u32).wrapping_mul(part.size) as i32;
            m.direction_v.v = (m.direction_v.v as u32).wrapping_mul(part.size) as i32;
            let c2rust_fresh0 = arm_index;
            arm_index = arm_index.wrapping_sub(1);
            *(*solution).arms.add(c2rust_fresh0 as usize) = m;
            rs_mark_visible_region(
                solution,
                m.position,
                (if m.r#type as c_ulong & PISTON != 0 {
                    3_u32
                } else {
                    part.size
                }) as i32,
            );
        } else if m.r#type & rs_ANY_GLYPH != 0 {
            let c2rust_fresh1 = glyph_index;
            glyph_index = glyph_index.wrapping_sub(1);
            *(*solution).glyphs.add(c2rust_fresh1 as usize) = m;
            if m.r#type as c_ulong & EQUILIBRIUM == 0 {
                let footprint: *const vector = rs_glyph_footprint(m.r#type);
                let mut j: i32 = 0_i32;
                loop {
                    let p: vector = *footprint.offset(j as isize);
                    rs_mark_visible_region(
                        solution,
                        rs_mechanism_relative_position(m, p.u, p.v, 1_i32),
                        0_i32,
                    );
                    if rs_vectors_equal(p, rs_zero_vector) {
                        break;
                    }
                    j += 1;
                }
            }
        } else if rs_byte_string_is(part.name, b"track\0".as_ptr() as *const c_char) {
            let mut last_position: vector = m.position;
            let mut end_overlapped: bool = false;
            let mut j_0: i32 = part.number_of_track_hexes.wrapping_sub(1_u32) as i32;
            while part.number_of_track_hexes > 0_u32 && j_0 >= -1_i32 {
                let mut hex: solution_hex_offset = solution_hex_offset { offset: [0; 2] };
                if j_0 == -1_i32 {
                    hex = *part
                        .track_hexes
                        .offset(part.number_of_track_hexes.wrapping_sub(1_u32) as isize)
                        as solution_hex_offset;
                    let du: i32 =
                        hex.offset[0usize] - (*part.track_hexes.add(0)).offset[0usize];
                    let dv: i32 =
                        hex.offset[1usize] - (*part.track_hexes.add(0)).offset[1usize];
                    if part.number_of_track_hexes <= 2_u32
                        || rs_direction_for_offset(vector { u: du, v: dv }) < 0_i32
                    {
                        let mut index: u32 = 0;
                        rs_lookup_track(solution, last_position, &raw mut index);
                        *(*solution).track_minus_motions.add(index as usize) =
                            vector { u: 0_i32, v: 0_i32 };
                        break;
                    } else {
                        (*solution).number_of_track_loops =
                            (*solution).number_of_track_loops.wrapping_add(1);
                    }
                } else {
                    hex = *part.track_hexes.offset(j_0 as isize) as solution_hex_offset;
                    let mut k: i32 = j_0 + 1_i32;
                    while (k as u32) < part.number_of_track_hexes {
                        if hex.offset[0usize]
                            == (*part.track_hexes.offset(k as isize)).offset[0usize]
                            && hex.offset[1usize]
                                == (*part.track_hexes.offset(k as isize)).offset[1usize]
                        {
                            (*solution).track_self_overlap =
                                (*solution).track_self_overlap.wrapping_add(1);
                        }
                        k += 1;
                    }
                }
                let p_0: vector = rs_mechanism_relative_position(
                    m,
                    hex.offset[0usize],
                    hex.offset[1usize],
                    1_i32,
                );
                if j_0 as u32 == part.number_of_track_hexes.wrapping_sub(1_u32) {
                    last_position = p_0;
                }
                let mut index_0: u32 = 0;
                if j_0 != -1_i32 || !end_overlapped {
                    rs_lookup_track(solution, p_0, &raw mut index_0);
                    if j_0 >= 0_i32
                        && ((*(*solution).track_positions.add(index_0 as usize)).u
                            != -2147483647_i32 - 1_i32
                            || (*(*solution).track_positions.add(index_0 as usize)).v
                                != -2147483647_i32 - 1_i32)
                    {
                        (*solution).track_overlap = (*solution).track_overlap.wrapping_add(1);
                    }
                    *(*solution).track_positions.add(index_0 as usize) = p_0;
                    *(*solution).track_plus_motions.add(index_0 as usize) = vector {
                        u: last_position.u - p_0.u,
                        v: last_position.v - p_0.v,
                    };
                }
                rs_lookup_track(solution, last_position, &raw mut index_0);
                *(*solution).track_minus_motions.add(index_0 as usize) = vector {
                    u: p_0.u - last_position.u,
                    v: p_0.v - last_position.v,
                };
                last_position = p_0;
                rs_mark_visible_region(solution, p_0, 3_i32);
                if j_0 as u32 != part.number_of_track_hexes.wrapping_sub(1_u32)
                    && hex.offset[0usize]
                        == (*part
                            .track_hexes
                            .offset(part.number_of_track_hexes.wrapping_sub(1_u32) as isize))
                        .offset[0usize]
                    && hex.offset[1usize]
                        == (*part
                            .track_hexes
                            .offset(part.number_of_track_hexes.wrapping_sub(1_u32) as isize))
                        .offset[1usize]
                {
                    end_overlapped = true;
                }
                j_0 -= 1;
            }
        } else if rs_byte_string_is(part.name, b"pipe\0".as_ptr() as *const c_char) {
            m.r#type = (CONDUIT) as u32;
            let conduit: conduit = conduit {
                glyph_index,
                other_side_glyph_index: 0,
                id: part.conduit_id,
                positions: calloc(
                    ::core::mem::size_of::<vector>(),
                    part.number_of_conduit_hexes as usize,
                ) as *mut vector,
                number_of_positions: part.number_of_conduit_hexes,
                atoms: calloc(
                    ::core::mem::size_of::<atom_at_position>(),
                    part.number_of_conduit_hexes as usize,
                ) as *mut atom_at_position,
                disjoint_bond_table: disjoint_bond_table {
                    bonds: ::core::ptr::null_mut::<disjoint_bond>(),
                    size: 0,
                    count: 0,
                },
                molecule_lengths: calloc(
                    ::core::mem::size_of::<u32>(),
                    part.number_of_conduit_hexes as usize,
                ) as *mut u32,
                number_of_molecules: 0,
            };
            let mut j_1: u32 = 0_u32;
            while j_1 < part.number_of_conduit_hexes {
                (*conduit.positions.add(j_1 as usize)).u =
                    (*part.conduit_hexes.add(j_1 as usize)).offset[0usize];
                (*conduit.positions.add(j_1 as usize)).v =
                    (*part.conduit_hexes.add(j_1 as usize)).offset[1usize];
                rs_mark_visible_region(
                    solution,
                    rs_mechanism_relative_position(
                        m,
                        (*conduit.positions.add(j_1 as usize)).u,
                        (*conduit.positions.add(j_1 as usize)).v,
                        1_i32,
                    ),
                    0_i32,
                );
                j_1 = j_1.wrapping_add(1);
            }
            let c2rust_fresh2 = glyph_index;
            glyph_index = glyph_index.wrapping_sub(1);
            *(*solution).glyphs.add(c2rust_fresh2 as usize) = m;
            let c2rust_fresh3 = conduit_index;
            conduit_index = conduit_index.wrapping_sub(1);
            *(*solution).conduits.add(c2rust_fresh3 as usize) = conduit;
        } else if rs_byte_string_is(part.name, b"input\0".as_ptr() as *const c_char) {
            let c: puzzle_molecule =
                *(*pf).inputs.add(part.which_input_or_output as usize) as puzzle_molecule;
            let io: *mut input_output = (*solution).inputs_and_outputs.add(io_index);
            (*io).r#type = input_output_type::INPUT;
            (*io).puzzle_index = part.which_input_or_output;
            (*io).solution_index = i_5;
            rs_decode_molecule(c, m, io);
            rs_create_disjoint_bonds(io);
            io_index = io_index.wrapping_sub(1);
            rs_mark_visible_input_output(solution, io);
        } else if rs_byte_string_is(part.name, b"out-std\0".as_ptr() as *const c_char) {
            let c_0: puzzle_molecule =
                *(*pf).outputs.add(part.which_input_or_output as usize) as puzzle_molecule;
            let io_0: *mut input_output = (*solution).inputs_and_outputs.add(io_index);
            (*io_0).r#type = input_output_type::SINGLE_OUTPUT;
            (*io_0).puzzle_index = part.which_input_or_output;
            (*io_0).solution_index = i_5;
            (*io_0).output_cycles_end = -1_i32 as u64;
            rs_decode_molecule(c_0, m, io_0);
            io_index = io_index.wrapping_sub(1);
            rs_mark_visible_input_output(solution, io_0);
        } else if rs_byte_string_is(part.name, b"out-rep\0".as_ptr() as *const c_char) {
            let c_1: puzzle_molecule =
                *(*pf).outputs.add(part.which_input_or_output as usize) as puzzle_molecule;
            let io_1: *mut input_output = (*solution).inputs_and_outputs.add(io_index);
            (*io_1).r#type = input_output_type::REPEATING_OUTPUT;
            (*io_1).puzzle_index = part.which_input_or_output;
            (*io_1).solution_index = i_5;
            (*io_1).output_cycles_end = -1_i32 as u64;
            rs_decode_molecule(c_1, m, io_1);
            (*io_1).original_atoms = (*io_1).atoms;
            (*io_1).number_of_original_atoms = (*io_1).number_of_atoms;
            if (*io_1).number_of_original_atoms == 0_u32 {
                *error = b"solution contains an empty infinite product\0".as_ptr() as *const c_char;
                rs_destroy(solution, ::core::ptr::null_mut::<board>());
                return false;
            }
            let placeholder: *mut atom_at_position = (*io_1)
                .original_atoms
                .offset((*io_1).number_of_original_atoms.wrapping_sub(1_u32) as isize);
            let mut i_6: u32 = 0_u32;
            while i_6 < (*io_1).number_of_original_atoms {
                if (*(*io_1).original_atoms.add(i_6 as usize)).atom
                    & REPEATING_OUTPUT_PLACEHOLDER
                    == 0
                {
                    i_6 = i_6.wrapping_add(1);
                } else {
                    let a: atom_at_position = *(*io_1).original_atoms.add(i_6 as usize);
                    *(*io_1).original_atoms.add(i_6 as usize) = *placeholder;
                    *placeholder = a;
                    break;
                }
            }
            if (*placeholder).atom & REPEATING_OUTPUT_PLACEHOLDER == 0 {
                *error = b"solution contains an infinite product without a repetition placeholder\0"
                    .as_ptr() as *const c_char;
                rs_destroy(solution, ::core::ptr::null_mut::<board>());
                return false;
            }
            (*io_1).atoms = ::core::ptr::null_mut::<atom_at_position>();
            (*io_1).number_of_atoms = 0_u32;
            (*io_1).repetition_origin = m.position;
            (*io_1).repetition_direction_u = vector {
                u: (*placeholder).position.u - m.position.u,
                v: (*placeholder).position.v - m.position.v,
            };
            let divisor: i32 = abs(rs_gcd(
                (*io_1).repetition_direction_u.u,
                (*io_1).repetition_direction_u.v,
            ));
            if divisor == 0_i32 {
                *error = b"solution contains a repetition placeholder at the origin\0".as_ptr()
                    as *const c_char;
                rs_destroy(solution, ::core::ptr::null_mut::<board>());
                return false;
            }
            (*io_1).repetition_direction_u.u /= divisor;
            (*io_1).repetition_direction_u.v /= divisor;
            (*io_1).repetition_direction_v = vector {
                u: -(*io_1).repetition_direction_u.v,
                v: (*io_1).repetition_direction_u.u + (*io_1).repetition_direction_u.v,
            };
            (*io_1).outputs_per_repetition = (*pf).output_scale;
            if !rs_repeat_molecule(io_1, error) {
                rs_destroy(solution, ::core::ptr::null_mut::<board>());
                return false;
            }
            io_index = io_index.wrapping_sub(1);
            rs_mark_visible_input_output(solution, io_1);
        }
        i_5 = i_5.wrapping_sub(1);
    }
    qsort(
        (*solution).conduits as *mut c_void,
        (*solution).number_of_conduits,
        ::core::mem::size_of::<conduit>(),
        Some(
            rs_compare_conduits_by_id as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
        ),
    );
    let mut destination: *mut conduit = ::core::ptr::null_mut::<conduit>();
    let mut i_7: u32 = 0_u32;
    while (i_7 as usize) < (*solution).number_of_conduits {
        let conduit_0: *mut conduit = (*solution).conduits.add(i_7 as usize);
        let mut next: *mut conduit = ::core::ptr::null_mut::<conduit>();
        if (i_7.wrapping_add(1_u32) as usize) < (*solution).number_of_conduits {
            next = (*solution)
                .conduits
                .add(i_7.wrapping_add(1_u32) as usize);
        }
        if !destination.is_null() && (*destination).id == (*conduit_0).id {
            (*conduit_0).other_side_glyph_index = (*destination).glyph_index;
        } else if next.is_null() || (*next).id != (*conduit_0).id {
            *error = b"solution contains an unpaired conduit\0".as_ptr() as *const c_char;
            rs_destroy(solution, ::core::ptr::null_mut::<board>());
            return false;
        } else {
            destination = conduit_0;
            (*conduit_0).other_side_glyph_index = (*next).glyph_index;
        }
        (*(*solution).glyphs.add((*conduit_0).glyph_index as usize)).conduit_index = i_7;
        i_7 = i_7.wrapping_add(1);
    }
    if !(*pf).production_info.is_null() {
        (*solution).production = true;
        rs_check_production_constraints(
            solution,
            (*pf).production_info as *mut puzzle_production_info,
        );
    } else if (*solution).number_of_conduits > 0_usize {
        (*solution).cabinet_violations =
            ((*solution).cabinet_violations as c_ulong | (1 as c_ulong) << 8_i32) as u32;
    }
    arm_index = 0_u32;
    let mut i_8: u32 = 0_u32;
    while i_8 < (*sf).number_of_parts {
        let part_0: solution_part = *(*sf).parts.add(i_8 as usize) as solution_part;
        if rs_decode_mechanism_type(part_0.name) & rs_ANY_ARM != 0 {
            if part_0.number_of_instructions == 0 {
                arm_index = arm_index.wrapping_add(1);
            } else {
                *(*solution).arm_tape_halt_index.add(arm_index as usize) =
                    18446744073709551615 as c_ulong as usize;
                qsort(
                    part_0.instructions as *mut c_void,
                    part_0.number_of_instructions as usize,
                    ::core::mem::size_of::<solution_instruction>(),
                    Some(
                        rs_compare_instructions_by_index
                            as unsafe extern "C" fn(*const c_void, *const c_void) -> i32,
                    ),
                );
                let min_tape: i32 = (*part_0.instructions.add(0)).index;
                let max_tape: i32 = (*part_0
                    .instructions
                    .offset(part_0.number_of_instructions.wrapping_sub(1_u32) as isize))
                .index;
                if max_tape as i64 - min_tape as i64 > 99999_i64 {
                    *error = b"solution has an arm with an instruction tape that's too long\0"
                        .as_ptr() as *const c_char;
                    rs_destroy(solution, ::core::ptr::null_mut::<board>());
                    return false;
                }
                let mut tape_length: i32 = max_tape - min_tape + 1_i32;
                if min_tape as i64 + tape_length as i64 * 2_i64 > 2147483647_i64 {
                    *error =
                        b"solution has an arm with a potential instruction tape index overflow\0"
                            .as_ptr() as *const c_char;
                    rs_destroy(solution, ::core::ptr::null_mut::<board>());
                    return false;
                }
                *(*solution).arm_tape.add(arm_index as usize) =
                    calloc((tape_length * 2_i32) as usize, 1_usize) as *mut c_char;
                *(*solution).arm_tape_start_cycle.add(arm_index as usize) = min_tape as i64;
                let mut last_end: i32 = 0_i32;
                let mut last_repeat: i32 = 0_i32;
                let mut reset_from: i32 = 0_i32;
                let tape: *mut c_char = *(*solution).arm_tape.add(arm_index as usize);
                let mut j_2: u32 = 0_u32;
                while j_2 < part_0.number_of_instructions {
                    let inst: solution_instruction =
                        *part_0.instructions.add(j_2 as usize) as solution_instruction;
                    if j_2 > 0_u32
                        && inst.index
                            == (*part_0.instructions.offset(j_2.wrapping_sub(1_u32) as isize)).index
                    {
                        *error = b"solution contains an arm with two instructions that have the same index\0"
                            .as_ptr() as *const c_char;
                        rs_destroy(solution, ::core::ptr::null_mut::<board>());
                        return false;
                    }
                    let parts_available_0: u64 =
                        rs_parts_available_bits_for_instruction(inst.instruction);
                    if parts_available_0 & (*pf).parts_available != parts_available_0 {
                        *error = b"solution contains an instruction that has been disabled in the puzzle file\0"
                            .as_ptr() as *const c_char;
                        rs_destroy(solution, ::core::ptr::null_mut::<board>());
                        return false;
                    }
                    let mut n: i32 = inst.index - min_tape;
                    if inst.instruction as i32 == 'B' as i32
                        && *(*solution).arm_tape_halt_index.add(arm_index as usize)
                            == 18446744073709551615_usize
                    {
                        *(*solution).arm_tape_halt_index.add(arm_index as usize) = n as usize;
                    }
                    if inst.instruction as i32 == 'C' as i32 {
                        if last_repeat < -min_tape {
                            last_repeat = -min_tape;
                        }
                        while j_2 < part_0.number_of_instructions
                            && (*part_0.instructions.add(j_2 as usize)).instruction as i32
                                == 'C' as i32
                        {
                            if last_end
                                > (*part_0.instructions.add(j_2 as usize)).index - min_tape
                            {
                                *error = b"solution contains a repeat instruction that overlaps with a reset instruction\0"
                                    .as_ptr() as *const c_char;
                                rs_destroy(solution, ::core::ptr::null_mut::<board>());
                                return false;
                            }
                            if last_end > last_repeat {
                                memcpy(
                                    tape.offset(
                                        (*part_0.instructions.add(j_2 as usize)).index as isize,
                                    )
                                    .offset(-(min_tape as isize))
                                        as *mut c_void,
                                    tape.offset(last_repeat as isize) as *const c_void,
                                    (last_end - last_repeat) as usize,
                                );
                                let m_0: i32 = (*part_0.instructions.add(j_2 as usize)).index
                                    - min_tape
                                    + last_end
                                    - last_repeat;
                                if m_0 > tape_length {
                                    tape_length = m_0;
                                }
                            }
                            j_2 = j_2.wrapping_add(1);
                        }
                        if j_2 < part_0.number_of_instructions {
                            last_repeat =
                                (*part_0.instructions.add(j_2 as usize)).index - min_tape;
                            reset_from = last_repeat;
                        }
                        j_2 = j_2.wrapping_sub(1);
                    } else if inst.instruction as i32 == 'X' as i32 {
                        let position: vector =
                            (*(*solution).arms.add(arm_index as usize)).position;
                        let mut track: vector = position;
                        let mut track_steps: i32 = 0_i32;
                        let mut track_looping_steps: i32 = 0_i32;
                        let mut rotation: i32 = 0_i32;
                        let mut piston: i32 = part_0.size as i32;
                        let mut grab: i32 = 0_i32;
                        let mut k_0: u32 = reset_from as u32;
                        while k_0 < n as u32 {
                            let mut motion: vector = vector { u: 0_i32, v: 0_i32 };
                            if *tape.add(k_0 as usize) as i32 == 'a' as i32 {
                                rotation += 1;
                            } else if *tape.add(k_0 as usize) as i32 == 'd' as i32 {
                                rotation -= 1;
                            } else if *tape.add(k_0 as usize) as i32 == 'w' as i32 {
                                if piston < 3_i32 {
                                    piston += 1;
                                }
                            } else if *tape.add(k_0 as usize) as i32 == 's' as i32 {
                                if piston > 1_i32 {
                                    piston -= 1;
                                }
                            } else if *tape.add(k_0 as usize) as i32 == 'g' as i32 {
                                let mut index_1: u32 = 0;
                                if rs_lookup_track(solution, track, &raw mut index_1) {
                                    motion =
                                        *(*solution).track_plus_motions.add(index_1 as usize);
                                    if motion.u != 0_i32 || motion.v != 0_i32 {
                                        track_steps += 1;
                                    }
                                }
                            } else if *tape.add(k_0 as usize) as i32 == 't' as i32 {
                                let mut index_2: u32 = 0;
                                if rs_lookup_track(solution, track, &raw mut index_2) {
                                    motion =
                                        *(*solution).track_minus_motions.add(index_2 as usize);
                                    if motion.u != 0_i32 || motion.v != 0_i32 {
                                        track_steps -= 1;
                                    }
                                }
                            } else if *tape.add(k_0 as usize) as i32 == 'f' as i32 {
                                grab = 1_i32;
                            } else if *tape.add(k_0 as usize) as i32 == 'r' as i32 {
                                grab = 0_i32;
                            }
                            track.u += motion.u;
                            track.v += motion.v;
                            if track.u == position.u && track.v == position.v {
                                track_looping_steps += track_steps;
                                track_steps = 0_i32;
                            }
                            k_0 = k_0.wrapping_add(1);
                        }
                        if grab > 0_i32 {
                            let c2rust_fresh4 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh4, 'r' as i32, error);
                        }
                        while piston as u32 > part_0.size {
                            let c2rust_fresh5 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh5, 's' as i32, error);
                            piston -= 1;
                        }
                        while rotation > 3_i32 {
                            rotation -= 6_i32;
                        }
                        while rotation < -3_i32 {
                            rotation += 6_i32;
                        }
                        while rotation > 0_i32 {
                            let c2rust_fresh6 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh6, 'd' as i32, error);
                            rotation -= 1;
                        }
                        while rotation < 0_i32 {
                            let c2rust_fresh7 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh7, 'a' as i32, error);
                            rotation += 1;
                        }
                        if track_steps != 0_i32 {
                            let mut search_depth: i32 = track_steps;
                            let mut direction: i32 = 1_i32;
                            let mut forward: *mut vector = (*solution).track_plus_motions;
                            if search_depth < 0_i32 {
                                search_depth = -search_depth;
                                direction = -1_i32;
                                forward = (*solution).track_minus_motions;
                            }
                            if track_steps * (track_steps + track_looping_steps) < 0_i32 {
                                search_depth += 1;
                            }
                            let mut p_1: vector = track;
                            let mut i_9: i32 = 0_i32;
                            while i_9 < search_depth {
                                if p_1.u == position.u && p_1.v == position.v {
                                    track_steps = -i_9 * direction;
                                    break;
                                } else {
                                    let mut index_3: u32 = 0;
                                    if !rs_lookup_track(solution, p_1, &raw mut index_3) {
                                        break;
                                    }
                                    p_1.u += (*forward.add(index_3 as usize)).u;
                                    p_1.v += (*forward.add(index_3 as usize)).v;
                                    i_9 += 1;
                                }
                            }
                        }
                        while track_steps > 0_i32 {
                            let c2rust_fresh8 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh8, 't' as i32, error);
                            track_steps -= 1;
                        }
                        while track_steps < 0_i32 {
                            let c2rust_fresh9 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh9, 'g' as i32, error);
                            track_steps += 1;
                        }
                        while (piston as u32) < part_0.size {
                            let c2rust_fresh10 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh10, 'w' as i32, error);
                            piston += 1;
                        }
                        if n == inst.index - min_tape {
                            let c2rust_fresh11 = n;
                            n += 1;
                            rs_set_tape(tape, c2rust_fresh11, ' ' as i32, error);
                        }
                        reset_from = n;
                        if n > tape_length {
                            tape_length = n;
                        }
                        last_end = n;
                    } else {
                        rs_set_tape(
                            tape,
                            n,
                            rs_decode_instruction(inst.instruction) as i32,
                            error,
                        );
                        last_end = n + 1_i32;
                    }
                    if !(*error).is_null() {
                        rs_destroy(solution, ::core::ptr::null_mut::<board>());
                        return false;
                    }
                    j_2 = j_2.wrapping_add(1);
                }
                *(*solution).arm_tape_length.add(arm_index as usize) = tape_length as usize;
                if *(*solution).arm_tape_halt_index.add(arm_index as usize)
                    == 18446744073709551615_usize
                    && tape_length as u64 > (*solution).tape_period
                {
                    (*solution).tape_period = tape_length as u64;
                }
                arm_index = arm_index.wrapping_add(1);
            }
        }
        i_8 = i_8.wrapping_add(1);
    }
    let mut tape_start_cycle: i64 = 9223372036854775807_i64;
    let mut i_10: u32 = 0_u32;
    while (i_10 as usize) < (*solution).number_of_arms {
        if *(*solution).arm_tape_start_cycle.add(i_10 as usize) < tape_start_cycle {
            tape_start_cycle = *(*solution).arm_tape_start_cycle.add(i_10 as usize);
        }
        i_10 = i_10.wrapping_add(1);
    }
    let mut i_11: u32 = 0_u32;
    while (i_11 as usize) < (*solution).number_of_arms {
        *(*solution).arm_tape_start_cycle.add(i_11 as usize) -= tape_start_cycle;
        i_11 = i_11.wrapping_add(1);
    }
    (*solution).target_number_of_outputs = 6_u32.wrapping_mul((*pf).output_scale) as u64;
    true
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_solution_file_cost(sf: *mut solution_file) -> u64 {
    let mut cost: u64 = 0_u64;
    let mut i: u32 = 0_u32;
    while i < (*sf).number_of_parts {
        let part_name: byte_string = (*(*sf).parts.add(i as usize)).name;
        if rs_byte_string_is(
            part_name,
            b"glyph-calcification\0".as_ptr() as *const c_char,
        ) {
            cost = cost.wrapping_add(10_u64);
        } else if rs_byte_string_is(
            part_name,
            b"glyph-life-and-death\0".as_ptr() as *const c_char,
        ) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-projection\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-dispersion\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-purification\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-duplication\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-unification\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"bonder\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(10_u64);
        } else if rs_byte_string_is(part_name, b"unbonder\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(10_u64);
        } else if rs_byte_string_is(part_name, b"bonder-prisma\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"bonder-speed\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        } else if rs_byte_string_is(part_name, b"arm1\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"arm2\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        } else if rs_byte_string_is(part_name, b"arm3\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        } else if rs_byte_string_is(part_name, b"arm6\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        } else if rs_byte_string_is(part_name, b"piston\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(40_u64);
        } else if rs_byte_string_is(part_name, b"baron\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        } else if rs_byte_string_is(part_name, b"track\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(
                (*(*sf).parts.add(i as usize))
                    .number_of_track_hexes
                    .wrapping_mul(5_u32) as u64,
            );
        } else if rs_byte_string_is(part_name, b"glyph-rejection\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(part_name, b"glyph-division\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(20_u64);
        } else if rs_byte_string_is(
            part_name,
            b"glyph-proliferation\0".as_ptr() as *const c_char,
        ) {
            cost = cost.wrapping_add(40_u64);
        } else if rs_byte_string_is(part_name, b"ravari\0".as_ptr() as *const c_char) {
            cost = cost.wrapping_add(30_u64);
        }
        i = i.wrapping_add(1);
    }
    cost
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_solution_instructions(solution: *mut solution) -> u64 {
    let mut instructions: u64 = 0_u64;
    let mut i: u32 = 0_u32;
    while (i as usize) < (*solution).number_of_arms {
        let mut j: u32 = 0_u32;
        while (j as usize) < *(*solution).arm_tape_length.add(i as usize) {
            if *(*(*solution).arm_tape.add(i as usize)).add(j as usize) as i32 != ' ' as i32
                && *(*(*solution).arm_tape.add(i as usize)).add(j as usize) as i32
                    != '\0' as i32
                && *(*(*solution).arm_tape.add(i as usize)).add(j as usize) as i32
                    != 'b' as i32
            {
                instructions = instructions.wrapping_add(1);
            }
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    instructions
}
