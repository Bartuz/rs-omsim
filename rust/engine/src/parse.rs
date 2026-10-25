// Generated from pinned omsim by scripts/regenerate_engine.sh.
#[allow(unused_imports)]
use crate::flags::*;
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_int, c_long, c_schar, c_uchar, c_ushort, c_void};
#[repr(C)]
pub struct _IO_wide_data {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_codecvt {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct _IO_marker {
    _opaque: [u8; 0],
}
use ::c2rust_bitfields;
unsafe extern "C" {
    unsafe fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> i32;
    unsafe fn strlen(__s: *const c_char) -> usize;
    unsafe fn fclose(__stream: *mut FILE) -> i32;
    unsafe fn fopen(__filename: *const c_char, __modes: *const c_char) -> *mut FILE;
    unsafe fn fread(__ptr: *mut c_void, __size: usize, __n: usize, __stream: *mut FILE) -> usize;
    unsafe fn fseek(__stream: *mut FILE, __off: c_long, __whence: i32) -> i32;
    unsafe fn ftell(__stream: *mut FILE) -> c_long;
    unsafe fn malloc(__size: usize) -> *mut c_void;
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
}
pub type __uint64_t = u64;
pub type __off_t = c_long;
pub type __off64_t = c_long;
pub type FILE = _IO_FILE;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: i32,
    pub _IO_read_ptr: *mut c_char,
    pub _IO_read_end: *mut c_char,
    pub _IO_read_base: *mut c_char,
    pub _IO_write_base: *mut c_char,
    pub _IO_write_ptr: *mut c_char,
    pub _IO_write_end: *mut c_char,
    pub _IO_buf_base: *mut c_char,
    pub _IO_buf_end: *mut c_char,
    pub _IO_save_base: *mut c_char,
    pub _IO_backup_base: *mut c_char,
    pub _IO_save_end: *mut c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: i32,
    #[bitfield(name = "_flags2", ty = "c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: u16,
    pub _vtable_offset: i8,
    pub _shortbuf: [c_char; 1],
    pub _lock: *mut c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: i32,
    pub _unused3: i32,
    pub _total_written: __uint64_t,
    pub _unused2: [c_char; 8],
}
pub type _IO_lock_t = ();
#[inline]
unsafe extern "C" fn rs_byte_string_is(b: byte_string, str: *const c_char) -> bool {
    let len: usize = strlen(str);
    b.length == len && memcmp(b.bytes as *const c_void, str as *const c_void, len) == 0
}
unsafe extern "C" fn rs_read_file(filename: *const c_char) -> byte_string {
    let mut contents: byte_string = byte_string {
        bytes: ::core::ptr::null_mut::<u8>(),
        length: 0,
    };
    let f: *mut FILE = fopen(filename, b"rb\0".as_ptr() as *const c_char);
    if f.is_null() {
        return byte_string {
            bytes: ::core::ptr::null_mut::<u8>(),
            length: 0_usize,
        };
    }
    fseek(f, 0 as c_long, 2_i32);
    contents.length = ftell(f) as usize;
    contents.bytes = malloc(contents.length) as *mut u8;
    fseek(f, 0 as c_long, 0_i32);
    fread(contents.bytes as *mut c_void, contents.length, 1_usize, f);
    fclose(f);
    contents
}
unsafe extern "C" fn rs_read_byte(contents: *mut byte_string) -> u8 {
    if (*contents).length < 1_usize {
        return 0_u8;
    }
    let value: u8 = *(*contents).bytes.offset(0isize);
    (*contents).length = (*contents).length.wrapping_sub(1);
    (*contents).bytes = (*contents).bytes.offset(1);
    value
}
unsafe extern "C" fn rs_read_signed_byte(contents: *mut byte_string) -> i8 {
    let value: u8 = rs_read_byte(contents);
    if value as i32 <= 127_i32 {
        return value as i8;
    }
    (-(!(value as i32) as u8 as i8 as i32) - 1_i32) as i8
}
unsafe extern "C" fn rs_read_uint32(contents: *mut byte_string) -> u32 {
    if (*contents).length < 4_usize {
        return 0_u32;
    }
    let value: u32 = *(*contents).bytes.offset(0isize) as u32
        | (*(*contents).bytes.offset(1isize) as u32) << 8_i32
        | (*(*contents).bytes.offset(2isize) as u32) << 16_i32
        | (*(*contents).bytes.offset(3isize) as u32) << 24_i32;
    (*contents).length = (*contents).length.wrapping_sub(4_usize);
    (*contents).bytes = (*contents).bytes.offset(4_isize);
    value
}
unsafe extern "C" fn rs_read_int32(contents: *mut byte_string) -> i32 {
    let value: u32 = rs_read_uint32(contents);
    if value <= 2147483647_u32 {
        return value as i32;
    }
    -(!value as i32) - 1_i32
}
unsafe extern "C" fn rs_read_uint64(contents: *mut byte_string) -> i32 {
    let lo: u32 = rs_read_uint32(contents);
    let hi: u32 = rs_read_uint32(contents);
    (lo as u64 | (hi as u64) << 32_i32) as i32
}
unsafe extern "C" fn rs_read_string(contents: *mut byte_string) -> byte_string {
    let mut len: usize = 0_usize;
    let mut shift: i32 = 0_i32;
    while (*contents).length > 0_usize {
        let byte: u8 = rs_read_byte(contents);
        len |= ((byte as i32 & 0x7f_i32) << shift) as usize;
        shift += 7_i32;
        if byte as i32 & 0x80_i32 == 0 {
            break;
        }
    }
    if (*contents).length < len {
        len = (*contents).length;
    }
    let value: byte_string = byte_string {
        bytes: (*contents).bytes,
        length: len,
    };
    (*contents).length = (*contents).length.wrapping_sub(len);
    (*contents).bytes = (*contents).bytes.offset(len as isize);
    value
}
unsafe extern "C" fn rs_parse_puzzle_molecule(b: *mut byte_string, molecule: *mut puzzle_molecule) {
    (*molecule).number_of_atoms = rs_read_uint32(b);
    (*molecule).atoms = calloc(
        (*molecule).number_of_atoms as usize,
        ::core::mem::size_of::<puzzle_atom>(),
    ) as *mut puzzle_atom;
    let mut j: u32 = 0_u32;
    while j < (*molecule).number_of_atoms {
        (*(*molecule).atoms.offset(j as isize)).r#type = rs_read_byte(b);
        (*(*molecule).atoms.offset(j as isize)).offset[0usize] = rs_read_signed_byte(b);
        (*(*molecule).atoms.offset(j as isize)).offset[1usize] = rs_read_signed_byte(b);
        j = j.wrapping_add(1);
    }
    (*molecule).number_of_bonds = rs_read_uint32(b);
    (*molecule).bonds = calloc(
        (*molecule).number_of_bonds as usize,
        ::core::mem::size_of::<puzzle_bond>(),
    ) as *mut puzzle_bond;
    let mut j_0: u32 = 0_u32;
    while j_0 < (*molecule).number_of_bonds {
        (*(*molecule).bonds.offset(j_0 as isize)).r#type = rs_read_byte(b);
        (*(*molecule).bonds.offset(j_0 as isize)).from[0usize] = rs_read_signed_byte(b);
        (*(*molecule).bonds.offset(j_0 as isize)).from[1usize] = rs_read_signed_byte(b);
        (*(*molecule).bonds.offset(j_0 as isize)).to[0usize] = rs_read_signed_byte(b);
        (*(*molecule).bonds.offset(j_0 as isize)).to[1usize] = rs_read_signed_byte(b);
        j_0 = j_0.wrapping_add(1);
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_parse_puzzle_byte_string(mut b: byte_string) -> *mut puzzle_file {
    let puzzle: *mut puzzle_file =
        calloc(1_usize, ::core::mem::size_of::<puzzle_file>()) as *mut puzzle_file;
    (*puzzle).bytes = b.bytes as *mut c_void;
    if rs_read_uint32(&raw mut b) != 3_u32 {
        rs_free_puzzle_file(puzzle);
        return ::core::ptr::null_mut::<puzzle_file>();
    }
    (*puzzle).name = rs_read_string(&raw mut b);
    (*puzzle).creator = rs_read_uint64(&raw mut b) as u64;
    (*puzzle).parts_available = rs_read_uint64(&raw mut b) as u64;
    (*puzzle).number_of_inputs = rs_read_uint32(&raw mut b);
    (*puzzle).inputs = calloc(
        (*puzzle).number_of_inputs as usize,
        ::core::mem::size_of::<puzzle_molecule>(),
    ) as *mut puzzle_molecule;
    let mut i: u32 = 0_u32;
    while i < (*puzzle).number_of_inputs {
        rs_parse_puzzle_molecule(
            &raw mut b,
            (*puzzle).inputs.offset(i as isize) as *mut puzzle_molecule,
        );
        i = i.wrapping_add(1);
    }
    (*puzzle).number_of_outputs = rs_read_uint32(&raw mut b);
    (*puzzle).outputs = calloc(
        (*puzzle).number_of_outputs as usize,
        ::core::mem::size_of::<puzzle_molecule>(),
    ) as *mut puzzle_molecule;
    let mut i_0: u32 = 0_u32;
    while i_0 < (*puzzle).number_of_outputs {
        rs_parse_puzzle_molecule(
            &raw mut b,
            (*puzzle).outputs.offset(i_0 as isize) as *mut puzzle_molecule,
        );
        i_0 = i_0.wrapping_add(1);
    }
    (*puzzle).output_scale = rs_read_uint32(&raw mut b);
    if rs_read_byte(&raw mut b) != 0 {
        let info: *mut puzzle_production_info =
            calloc(1_usize, ::core::mem::size_of::<puzzle_production_info>())
                as *mut puzzle_production_info;
        (*puzzle).production_info = info as *mut puzzle_production_info;
        (*info).shrink_left = rs_read_byte(&raw mut b) != 0;
        (*info).shrink_right = rs_read_byte(&raw mut b) != 0;
        (*info).isolate_inputs_from_outputs = rs_read_byte(&raw mut b) != 0;
        (*info).number_of_cabinets = rs_read_uint32(&raw mut b);
        (*info).cabinets = calloc(
            (*info).number_of_cabinets as usize,
            ::core::mem::size_of::<puzzle_cabinet>(),
        ) as *mut puzzle_cabinet;
        let mut i_1: u32 = 0_u32;
        while i_1 < (*info).number_of_cabinets {
            (*(*info).cabinets.offset(i_1 as isize)).position[0usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).cabinets.offset(i_1 as isize)).position[1usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).cabinets.offset(i_1 as isize)).r#type = rs_read_string(&raw mut b);
            i_1 = i_1.wrapping_add(1);
        }
        (*info).number_of_conduits = rs_read_uint32(&raw mut b);
        (*info).conduits = calloc(
            (*info).number_of_conduits as usize,
            ::core::mem::size_of::<puzzle_conduit>(),
        ) as *mut puzzle_conduit;
        let mut i_2: u32 = 0_u32;
        while i_2 < (*info).number_of_conduits {
            (*(*info).conduits.offset(i_2 as isize)).starting_position_a[0usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).conduits.offset(i_2 as isize)).starting_position_a[1usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).conduits.offset(i_2 as isize)).starting_position_b[0usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).conduits.offset(i_2 as isize)).starting_position_b[1usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).conduits.offset(i_2 as isize)).number_of_hexes = rs_read_uint32(&raw mut b);
            (*(*info).conduits.offset(i_2 as isize)).hexes = calloc(
                (*(*info).conduits.offset(i_2 as isize)).number_of_hexes as usize,
                ::core::mem::size_of::<puzzle_conduit_hex>(),
            )
                as *mut puzzle_conduit_hex;
            let mut j: u32 = 0_u32;
            while j < (*(*info).conduits.offset(i_2 as isize)).number_of_hexes {
                (*(*(*info).conduits.offset(i_2 as isize))
                    .hexes
                    .offset(j as isize))
                .offset[0usize] = rs_read_signed_byte(&raw mut b);
                (*(*(*info).conduits.offset(i_2 as isize))
                    .hexes
                    .offset(j as isize))
                .offset[1usize] = rs_read_signed_byte(&raw mut b);
                j = j.wrapping_add(1);
            }
            i_2 = i_2.wrapping_add(1);
        }
        (*info).number_of_vials = rs_read_uint32(&raw mut b);
        (*info).vials = calloc(
            (*info).number_of_vials as usize,
            ::core::mem::size_of::<puzzle_vial>(),
        ) as *mut puzzle_vial;
        let mut i_3: u32 = 0_u32;
        while i_3 < (*info).number_of_vials {
            (*(*info).vials.offset(i_3 as isize)).position[0usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).vials.offset(i_3 as isize)).position[1usize] =
                rs_read_signed_byte(&raw mut b);
            (*(*info).vials.offset(i_3 as isize)).style = rs_read_byte(&raw mut b);
            (*(*info).vials.offset(i_3 as isize)).count = rs_read_uint32(&raw mut b);
            i_3 = i_3.wrapping_add(1);
        }
    }
    puzzle as *mut puzzle_file
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_parse_puzzle_file(path: *const c_char) -> *mut puzzle_file {
    let b: byte_string = rs_read_file(path);
    let puzzle: *mut puzzle_file = rs_parse_puzzle_byte_string(b) as *mut puzzle_file;
    if puzzle.is_null() {
        free(b.bytes as *mut c_void);
        return ::core::ptr::null_mut::<puzzle_file>();
    }
    (*puzzle).owns_bytes = true;
    puzzle as *mut puzzle_file
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_free_puzzle_file(puzzle: *mut puzzle_file) {
    if puzzle.is_null() {
        return;
    }
    let mut i: u32 = 0_u32;
    while i < (*puzzle).number_of_inputs {
        free((*(*puzzle).inputs.offset(i as isize)).atoms as *mut c_void);
        free((*(*puzzle).inputs.offset(i as isize)).bonds as *mut c_void);
        i = i.wrapping_add(1);
    }
    free((*puzzle).inputs as *mut c_void);
    let mut i_0: u32 = 0_u32;
    while i_0 < (*puzzle).number_of_outputs {
        free((*(*puzzle).outputs.offset(i_0 as isize)).atoms as *mut c_void);
        free((*(*puzzle).outputs.offset(i_0 as isize)).bonds as *mut c_void);
        i_0 = i_0.wrapping_add(1);
    }
    free((*puzzle).outputs as *mut c_void);
    if !(*puzzle).production_info.is_null() {
        free((*(*puzzle).production_info).cabinets as *mut c_void);
        let mut i_1: u32 = 0_u32;
        while i_1 < (*(*puzzle).production_info).number_of_conduits {
            free(
                (*(*(*puzzle).production_info).conduits.offset(i_1 as isize)).hexes as *mut c_void,
            );
            i_1 = i_1.wrapping_add(1);
        }
        free((*(*puzzle).production_info).conduits as *mut c_void);
        free((*(*puzzle).production_info).vials as *mut c_void);
    }
    free((*puzzle).production_info as *mut c_void);
    if (*puzzle).owns_bytes {
        free((*puzzle).bytes);
    }
    free(puzzle as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_parse_solution_byte_string(mut b: byte_string) -> *mut solution_file {
    let solution: *mut solution_file =
        calloc(1_usize, ::core::mem::size_of::<solution_file>()) as *mut solution_file;
    (*solution).bytes = b.bytes as *mut c_void;
    if rs_read_uint32(&raw mut b) != 7_u32 {
        rs_free_solution_file(solution);
        return ::core::ptr::null_mut::<solution_file>();
    }
    (*solution).puzzle = rs_read_string(&raw mut b);
    (*solution).name = rs_read_string(&raw mut b);
    if rs_read_uint32(&raw mut b) != 0 {
        (*solution).solved = true;
        let zero: u32 = rs_read_uint32(&raw mut b);
        (*solution).cycles = rs_read_uint32(&raw mut b);
        let one: u32 = rs_read_uint32(&raw mut b);
        (*solution).cost = rs_read_uint32(&raw mut b);
        let two: u32 = rs_read_uint32(&raw mut b);
        (*solution).area = rs_read_uint32(&raw mut b);
        let three: u32 = rs_read_uint32(&raw mut b);
        (*solution).instructions = rs_read_uint32(&raw mut b);
        if zero != 0_u32 || one != 1_u32 || two != 2_u32 || three != 3_u32 {
            rs_free_solution_file(solution);
            return ::core::ptr::null_mut::<solution_file>();
        }
    }
    (*solution).number_of_parts = rs_read_uint32(&raw mut b);
    if (*solution).number_of_parts > 9999_u32 {
        (*solution).number_of_parts = 0_u32;
        rs_free_solution_file(solution);
        return ::core::ptr::null_mut::<solution_file>();
    }
    (*solution).parts = calloc(
        (*solution).number_of_parts as usize,
        ::core::mem::size_of::<solution_part>(),
    ) as *mut solution_part;
    let mut i: u32 = 0_u32;
    while i < (*solution).number_of_parts {
        let part: *mut solution_part = (*solution).parts.offset(i as isize) as *mut solution_part;
        (*part).name = rs_read_string(&raw mut b);
        if rs_read_byte(&raw mut b) as i32 != 1_i32 {
            rs_free_solution_file(solution);
            return ::core::ptr::null_mut::<solution_file>();
        }
        (*part).position[0usize] = rs_read_int32(&raw mut b);
        (*part).position[1usize] = rs_read_int32(&raw mut b);
        (*part).size = rs_read_uint32(&raw mut b);
        (*part).rotation = rs_read_int32(&raw mut b);
        (*part).which_input_or_output = rs_read_uint32(&raw mut b);
        (*part).number_of_instructions = rs_read_uint32(&raw mut b);
        if (*part).number_of_instructions > 99999_u32 {
            rs_free_solution_file(solution);
            return ::core::ptr::null_mut::<solution_file>();
        }
        (*part).instructions = calloc(
            (*part).number_of_instructions as usize,
            ::core::mem::size_of::<solution_instruction>(),
        ) as *mut solution_instruction;
        let mut j: u32 = 0_u32;
        while j < (*part).number_of_instructions {
            (*(*part).instructions.offset(j as isize)).index = rs_read_int32(&raw mut b);
            (*(*part).instructions.offset(j as isize)).instruction =
                rs_read_byte(&raw mut b) as c_char;
            j = j.wrapping_add(1);
        }
        if rs_byte_string_is((*part).name, b"track\0".as_ptr() as *const c_char) {
            (*part).number_of_track_hexes = rs_read_uint32(&raw mut b);
            if (*part).number_of_track_hexes > 9999_u32 {
                rs_free_solution_file(solution);
                return ::core::ptr::null_mut::<solution_file>();
            }
            (*part).track_hexes = calloc(
                (*part).number_of_track_hexes as usize,
                ::core::mem::size_of::<solution_hex_offset>(),
            ) as *mut solution_hex_offset;
            let mut j_0: u32 = 0_u32;
            while j_0 < (*part).number_of_track_hexes {
                (*(*part).track_hexes.offset(j_0 as isize)).offset[0usize] =
                    rs_read_int32(&raw mut b);
                (*(*part).track_hexes.offset(j_0 as isize)).offset[1usize] =
                    rs_read_int32(&raw mut b);
                j_0 = j_0.wrapping_add(1);
            }
        }
        (*part).arm_number = rs_read_uint32(&raw mut b);
        if rs_byte_string_is((*part).name, b"pipe\0".as_ptr() as *const c_char) {
            (*part).conduit_id = rs_read_uint32(&raw mut b);
            (*part).number_of_conduit_hexes = rs_read_uint32(&raw mut b);
            if (*part).number_of_conduit_hexes > 9999_u32 {
                rs_free_solution_file(solution);
                return ::core::ptr::null_mut::<solution_file>();
            }
            (*part).conduit_hexes = calloc(
                (*part).number_of_conduit_hexes as usize,
                ::core::mem::size_of::<solution_hex_offset>(),
            ) as *mut solution_hex_offset;
            let mut j_1: u32 = 0_u32;
            while j_1 < (*part).number_of_conduit_hexes {
                (*(*part).conduit_hexes.offset(j_1 as isize)).offset[0usize] =
                    rs_read_int32(&raw mut b);
                (*(*part).conduit_hexes.offset(j_1 as isize)).offset[1usize] =
                    rs_read_int32(&raw mut b);
                j_1 = j_1.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    solution as *mut solution_file
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_parse_solution_file(path: *const c_char) -> *mut solution_file {
    let b: byte_string = rs_read_file(path);
    let solution: *mut solution_file = rs_parse_solution_byte_string(b) as *mut solution_file;
    if solution.is_null() {
        free(b.bytes as *mut c_void);
        return ::core::ptr::null_mut::<solution_file>();
    }
    (*solution).owns_bytes = true;
    solution as *mut solution_file
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_free_solution_file(solution: *mut solution_file) {
    if solution.is_null() {
        return;
    }
    let mut i: u32 = 0_u32;
    while i < (*solution).number_of_parts {
        free((*(*solution).parts.offset(i as isize)).instructions as *mut c_void);
        free((*(*solution).parts.offset(i as isize)).track_hexes as *mut c_void);
        free((*(*solution).parts.offset(i as isize)).conduit_hexes as *mut c_void);
        i = i.wrapping_add(1);
    }
    free((*solution).parts as *mut c_void);
    if (*solution).owns_bytes {
        free((*solution).bytes);
    }
    free(solution as *mut c_void);
}
