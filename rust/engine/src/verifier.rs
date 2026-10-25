#[allow(unused_imports)]
use crate::flags::*;
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_long, c_ulong, c_void};
unsafe extern "C" {
    unsafe fn rs_decode_solution(
        solution: *mut solution,
        pf: *mut puzzle_file,
        sf: *mut solution_file,
        error: *mut *const c_char,
    ) -> bool;
    unsafe fn rs_solution_file_cost(sf: *mut solution_file) -> u64;
    unsafe fn rs_solution_instructions(solution: *mut solution) -> u64;
    unsafe fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
    unsafe fn memcmp(__s1: *const c_void, __s2: *const c_void, __n: usize) -> i32;
    unsafe fn strcmp(__s1: *const c_char, __s2: *const c_char) -> i32;
    unsafe fn strncmp(__s1: *const c_char, __s2: *const c_char, __n: usize) -> i32;
    unsafe fn strlen(__s: *const c_char) -> usize;
    unsafe fn rs_parse_puzzle_file(path: *const c_char) -> *mut puzzle_file;
    unsafe fn rs_parse_puzzle_byte_string(b: byte_string) -> *mut puzzle_file;
    unsafe fn rs_free_puzzle_file(puzzle: *mut puzzle_file);
    unsafe fn rs_parse_solution_file(path: *const c_char) -> *mut solution_file;
    unsafe fn rs_parse_solution_byte_string(b: byte_string) -> *mut solution_file;
    unsafe fn rs_free_solution_file(solution: *mut solution_file);
    unsafe fn strtol(__nptr: *const c_char, __endptr: *mut *mut c_char, __base: i32) -> c_long;
    unsafe fn malloc(__size: usize) -> *mut c_void;
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
    unsafe fn llabs(__x: i64) -> i64;
    unsafe fn rs_initial_setup(solution: *mut solution, board: *mut board, intial_board_size: u32);
    unsafe fn rs_run(solution: *mut solution, board: *mut board);
    unsafe fn rs_destroy(solution: *mut solution, board: *mut board);
    unsafe fn rs_lookup_atom(board: *mut board, query: vector) -> *mut atom;
    unsafe fn rs_used_area(board: *mut board) -> u32;
    unsafe fn rs_u_offset_for_direction(direction: i32) -> vector;
    unsafe fn rs_v_offset_for_direction(direction: i32) -> vector;
    unsafe fn rs_run_until_steady_state(
        solution: *mut solution,
        board: *mut board,
        cycle_limit: u64,
    ) -> steady_state;
    unsafe fn tolower(__c: i32) -> i32;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct verifier {
    pub pf: *mut puzzle_file,
    pub sf: *mut solution_file,
    pub throughput_measurements: throughput_measurements,
    pub cycle_limit: u64,
    pub fails_on_wrong_output_mask: u64,
    pub fails_on_wrong_output_bonds_mask: u64,
    pub wrong_output_index: i32,
    pub wrong_output_board: board,
    pub wrong_output_origin: vector,
    pub wrong_output_basis_u: vector,
    pub wrong_output_basis_v: vector,
    pub completion: per_cycle_measurements,
    pub output_intervals_for_output: *mut output_intervals,
    pub output_intervals: output_intervals,
    pub collision_check_limit: u64,
    pub disable_limits: bool,
    pub error: error,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct error {
    pub description: *const c_char,
    pub source: *const c_char,
    pub cycle: i32,
    pub location_u: i32,
    pub location_v: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output_intervals {
    pub intervals: *mut i32,
    pub number_of_intervals: i32,
    pub intervals_repeat_after: i32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct per_cycle_measurements {
    pub cycles: i32,
    pub area: i32,
    pub has_max_direction: [bool; 12],
    pub max_direction: [i32; 12],
    pub executed_instructions: i32,
    pub instruction_executions: [i32; 10],
    pub atom_grabs: [i32; 16],
    pub number_of_atoms: [i32; 16],
    pub error: error,
    pub valid: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct throughput_measurements {
    pub throughput_cycles: i64,
    pub throughput_outputs: i64,
    pub throughput_linear_area: i64,
    pub throughput_quadratic_area: f64,
    pub throughput_height: f64,
    pub throughput_width: f64,
    pub throughput_minimum_hexagon: f64,
    pub area_growth_order: growth_order,
    pub throughput_waste: i32,
    pub pivot_parity: i32,
    pub steady_state_start_cycle: i32,
    pub steady_state: per_cycle_measurements,
    pub error: error,
    pub valid: bool,
}
#[inline]
unsafe extern "C" fn rs_byte_string_is(b: byte_string, str: *const c_char) -> bool {
    let len: usize = strlen(str);
    b.length == len && memcmp(b.bytes as *const c_void, str as *const c_void, len) == 0
}
#[inline]
unsafe extern "C" fn rs_cycle(solution: *mut solution, board: *mut board) {
    rs_run(solution, board);
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
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_find_puzzle_name_in_solution_bytes(
    solution_bytes: *const c_char,
    solution_length: i32,
    name_length: *mut i32,
) -> *const c_char {
    let bytes: byte_string = byte_string {
        bytes: solution_bytes as *mut u8,
        length: solution_length as usize,
    };
    let sf: *mut solution_file = rs_parse_solution_byte_string(bytes) as *mut solution_file;
    if sf.is_null() {
        return ::core::ptr::null::<c_char>();
    }
    if !name_length.is_null() {
        *name_length = (*sf).puzzle.length as i32;
    }
    let name: *const c_char = (*sf).puzzle.bytes as *const c_char;
    rs_free_solution_file(sf as *mut solution_file);
    name
}
unsafe extern "C" fn rs_instruction_index(instruction: c_char) -> i32 {
    match instruction as i32 {
        97 => 1_i32,
        100 => 0_i32,
        101 => 6_i32,
        102 => 5_i32,
        103 => 8_i32,
        113 => 7_i32,
        114 => 4_i32,
        115 => 3_i32,
        116 => 9_i32,
        119 => 2_i32,
        _ => -1_i32,
    }
}
unsafe extern "C" fn rs_max(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}
unsafe extern "C" fn rs_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0_i32 {
        let t: i32 = b;
        b = a % b;
        a = t;
    }
    a
}
unsafe extern "C" fn rs_lcm(a: i32, b: i32) -> i32 {
    if a == 0_i32 || b == 0_i32 {
        return 0_i32;
    }
    a * b / rs_gcd(a, b)
}
unsafe extern "C" fn rs_normalize_output_intervals(oi: *mut output_intervals) {
    let cycle_start: i32 = (*oi).intervals_repeat_after;
    let mut n: i32 = (*oi).number_of_intervals - cycle_start;
    if (*oi).intervals_repeat_after <= 0_i32 || n == 0_i32 {
        return;
    }
    let mut i: i32 = 1_i32;
    while i <= n / 2_i32 {
        if n % i == 0_i32 {
            let mut repeating: bool = true;
            let mut j: i32 = 0_i32;
            while j < n {
                if *(*oi).intervals.offset((cycle_start + j) as isize)
                    != *(*oi).intervals.offset((cycle_start + j % i) as isize)
                {
                    repeating = false;
                    break;
                } else {
                    j += 1;
                }
            }
            if repeating {
                (*oi).number_of_intervals = cycle_start + i;
                n = i;
                break;
            }
        }
        i += 1;
    }
    while (*oi).number_of_intervals > 0_i32
        && (*oi).intervals_repeat_after > 0_i32
        && *(*oi)
            .intervals
            .offset(((*oi).number_of_intervals - 1_i32) as isize)
            == *(*oi)
                .intervals
                .offset(((*oi).intervals_repeat_after - 1_i32) as isize)
    {
        (*oi).intervals_repeat_after -= 1;
        (*oi).number_of_intervals -= 1;
    }
}
unsafe extern "C" fn rs_output_interval_at(oi: *mut output_intervals, index: i32) -> i32 {
    let repeat_after: i32 = if (*oi).intervals_repeat_after < 0_i32 {
        (*oi).number_of_intervals
    } else {
        (*oi).intervals_repeat_after
    };
    if index < repeat_after {
        return *(*oi).intervals.offset(index as isize);
    }
    if repeat_after == (*oi).number_of_intervals {
        return -1_i32;
    }
    *(*oi).intervals.offset(
        (repeat_after + (index - repeat_after) % ((*oi).number_of_intervals - repeat_after))
            as isize,
    )
}
unsafe extern "C" fn rs_combine_output_intervals(
    dst: *mut output_intervals,
    src: *mut output_intervals,
) {
    let dst_repeat_after: i32 = if (*dst).intervals_repeat_after < 0_i32 {
        (*dst).number_of_intervals
    } else {
        (*dst).intervals_repeat_after
    };
    let src_repeat_after: i32 = if (*src).intervals_repeat_after < 0_i32 {
        (*src).number_of_intervals
    } else {
        (*src).intervals_repeat_after
    };
    let mut initial_length: i32 = rs_max(dst_repeat_after, src_repeat_after);
    let lcm_repeat: i32 = rs_lcm(
        (*dst).number_of_intervals - dst_repeat_after,
        (*src).number_of_intervals - src_repeat_after,
    );
    let mut dst_total: i32 = 0_i32;
    let mut src_total: i32 = 0_i32;
    let mut i: i32 = 0_i32;
    while i < lcm_repeat {
        dst_total += rs_output_interval_at(dst, initial_length + i);
        src_total += rs_output_interval_at(src, initial_length + i);
        i += 1;
    }
    let mut dst_output_cycle: i32 = 0_i32;
    let mut src_output_cycle: i32 = 0_i32;
    let mut repeats_to_overtake: i32 = 0_i32;
    let mut i_0: i32 = 0_i32;
    while i_0 < initial_length + lcm_repeat {
        let dst_interval: i32 = rs_output_interval_at(dst, i_0);
        let src_interval: i32 = rs_output_interval_at(src, i_0);
        if dst_interval < 0_i32 || src_interval < 0_i32 {
            initial_length = i_0;
            break;
        } else {
            dst_output_cycle += dst_interval;
            src_output_cycle += src_interval;
            if i_0 >= initial_length {
                if dst_total > src_total {
                    repeats_to_overtake = rs_max(
                        repeats_to_overtake,
                        (src_output_cycle - dst_output_cycle + dst_total - src_total - 1_i32)
                            / (dst_total - src_total)
                            + 1_i32,
                    );
                } else if src_total > dst_total {
                    repeats_to_overtake = rs_max(
                        repeats_to_overtake,
                        (dst_output_cycle - src_output_cycle + src_total - dst_total - 1_i32)
                            / (src_total - dst_total)
                            + 1_i32,
                    );
                }
            }
            i_0 += 1;
        }
    }
    dst_output_cycle = 0_i32;
    src_output_cycle = 0_i32;
    let number_of_result_intervals: u64 =
        (initial_length + lcm_repeat * (repeats_to_overtake + 1_i32)) as u64;
    let result_intervals: *mut i32 = calloc(
        number_of_result_intervals as usize,
        ::core::mem::size_of::<i32>(),
    ) as *mut i32;
    let mut last_max: i32 = 0_i32;
    let mut i_1: i32 = 0_i32;
    while (i_1 as u64) < number_of_result_intervals {
        dst_output_cycle += rs_output_interval_at(dst, i_1);
        src_output_cycle += rs_output_interval_at(src, i_1);
        *result_intervals.offset(i_1 as isize) =
            rs_max(dst_output_cycle, src_output_cycle) - last_max;
        last_max += *result_intervals.offset(i_1 as isize);
        i_1 += 1;
    }
    free((*dst).intervals as *mut c_void);
    (*dst).intervals = result_intervals;
    (*dst).number_of_intervals = number_of_result_intervals as i32;
    (*dst).intervals_repeat_after = initial_length + lcm_repeat * repeats_to_overtake;
}
unsafe extern "C" fn rs_verifier_create_empty() -> *mut c_void {
    let v: *mut verifier = calloc(::core::mem::size_of::<verifier>(), 1_usize) as *mut verifier;
    (*v).cycle_limit = 150000_u64;
    (*v).collision_check_limit = 20000000000_u64;
    (*v).wrong_output_index = -1_i32;
    v as *mut c_void
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_create(
    puzzle_filename: *const c_char,
    solution_filename: *const c_char,
) -> *mut c_void {
    let v: *mut verifier = rs_verifier_create_empty() as *mut verifier;
    (*v).pf = rs_parse_puzzle_file(puzzle_filename) as *mut puzzle_file;
    if (*v).pf.is_null() {
        (*v).error.description = b"invalid puzzle file\0".as_ptr() as *const c_char;
        (*v).error.source = rs_verifier_error_source_puzzle_file;
        return v as *mut c_void;
    }
    (*v).sf = rs_parse_solution_file(solution_filename) as *mut solution_file;
    if (*v).sf.is_null() {
        (*v).error.description = b"invalid solution file\0".as_ptr() as *const c_char;
        (*v).error.source = rs_verifier_error_source_solution_file;
        return v as *mut c_void;
    }
    v as *mut c_void
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_create_from_bytes(
    puzzle_bytes: *const c_char,
    puzzle_length: i32,
    solution_bytes: *const c_char,
    solution_length: i32,
) -> *mut c_void {
    let puzzle_copy: *mut c_char = malloc(puzzle_length as usize) as *mut c_char;
    memcpy(
        puzzle_copy as *mut c_void,
        puzzle_bytes as *const c_void,
        puzzle_length as usize,
    );
    let solution_copy: *mut c_char = malloc(solution_length as usize) as *mut c_char;
    memcpy(
        solution_copy as *mut c_void,
        solution_bytes as *const c_void,
        solution_length as usize,
    );
    let v: *mut verifier = rs_verifier_create_from_bytes_without_copying(
        puzzle_copy,
        puzzle_length,
        solution_copy,
        solution_length,
    ) as *mut verifier;
    if !(*v).pf.is_null() {
        (*(*v).pf).owns_bytes = true;
    } else {
        free(puzzle_copy as *mut c_void);
    }
    if !(*v).sf.is_null() {
        (*(*v).sf).owns_bytes = true;
    } else {
        free(solution_copy as *mut c_void);
    }
    v as *mut c_void
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_create_from_bytes_without_copying(
    puzzle_bytes: *const c_char,
    puzzle_length: i32,
    solution_bytes: *const c_char,
    solution_length: i32,
) -> *mut c_void {
    let v: *mut verifier = rs_verifier_create_empty() as *mut verifier;
    (*v).pf = rs_parse_puzzle_byte_string(byte_string {
        bytes: puzzle_bytes as *mut u8,
        length: puzzle_length as usize,
    }) as *mut puzzle_file;
    if (*v).pf.is_null() {
        (*v).error.description = b"invalid puzzle file\0".as_ptr() as *const c_char;
        (*v).error.source = rs_verifier_error_source_puzzle_file;
        return v as *mut c_void;
    }
    (*v).sf = rs_parse_solution_byte_string(byte_string {
        bytes: solution_bytes as *mut u8,
        length: solution_length as usize,
    }) as *mut solution_file;
    if (*v).sf.is_null() {
        (*v).error.description = b"invalid solution file\0".as_ptr() as *const c_char;
        (*v).error.source = rs_verifier_error_source_solution_file;
        return v as *mut c_void;
    }
    v as *mut c_void
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error(verifier: *mut c_void) -> *const c_char {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error.description
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error_cycle(verifier: *mut c_void) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error.cycle
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error_location_u(verifier: *mut c_void) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error.location_u
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error_location_v(verifier: *mut c_void) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error.location_v
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error_source(verifier: *mut c_void) -> *const c_char {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error.source
}
#[unsafe(no_mangle)]
pub static mut rs_verifier_error_source_puzzle_file: *const c_char =
    b"puzzle file\0".as_ptr() as *const c_char;
#[unsafe(no_mangle)]
pub static mut rs_verifier_error_source_solution_file: *const c_char =
    b"solution file\0".as_ptr() as *const c_char;
#[unsafe(no_mangle)]
pub static mut rs_verifier_error_source_metric: *const c_char =
    b"metric\0".as_ptr() as *const c_char;
#[unsafe(no_mangle)]
pub static mut rs_verifier_error_source_simulation: *const c_char =
    b"simulation\0".as_ptr() as *const c_char;
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_error_clear(verifier: *mut c_void) {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).error = error {
        description: ::core::ptr::null::<c_char>(),
        source: ::core::ptr::null::<c_char>(),
        cycle: 0,
        location_u: 0,
        location_v: 0,
    };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_destroy(verifier: *mut c_void) {
    let v: *mut verifier = verifier as *mut verifier;
    if !(*v).output_intervals_for_output.is_null() && !(*v).pf.is_null() {
        let mut i: u32 = 0_u32;
        while i < (*(*v).pf).number_of_outputs {
            free((*(*v).output_intervals_for_output.add(i as usize)).intervals as *mut c_void);
            i = i.wrapping_add(1);
        }
    }
    free((*v).output_intervals_for_output as *mut c_void);
    free((*v).output_intervals.intervals as *mut c_void);
    rs_free_puzzle_file((*v).pf as *mut puzzle_file);
    rs_free_solution_file((*v).sf as *mut solution_file);
    rs_verifier_wrong_output_clear(v as *mut c_void);
    free(v as *mut c_void);
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_set_cycle_limit(verifier: *mut c_void, mut cycle_limit: i32) {
    let v: *mut verifier = verifier as *mut verifier;
    if cycle_limit < 0_i32 {
        cycle_limit = 0_i32;
    }
    (*v).cycle_limit = cycle_limit as u64;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_disable_limits(verifier: *mut c_void) {
    let v: *mut verifier = verifier as *mut verifier;
    if v.is_null() {
        return;
    }
    (*v).disable_limits = true;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_set_fails_on_wrong_output(
    verifier: *mut c_void,
    output_index: i32,
    fails_on_wrong_output: i32,
) {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).fails_on_wrong_output_mask =
        (*v).fails_on_wrong_output_mask & !(1_u64 << output_index as u64);
    (*v).fails_on_wrong_output_mask = (*v).fails_on_wrong_output_mask
        | if fails_on_wrong_output != 0 {
            1_u64 << output_index as u64
        } else {
            0_u64
        };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_set_fails_on_wrong_output_bonds(
    verifier: *mut c_void,
    output_index: i32,
    fails_on_wrong_output_bonds: i32,
) {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).fails_on_wrong_output_bonds_mask =
        (*v).fails_on_wrong_output_bonds_mask & !(1_u64 << output_index as u64);
    (*v).fails_on_wrong_output_bonds_mask = (*v).fails_on_wrong_output_bonds_mask
        | if fails_on_wrong_output_bonds != 0 {
            1_u64 << output_index as u64
        } else {
            0_u64
        };
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_wrong_output_index(verifier: *mut c_void) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).wrong_output_index
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_wrong_output_atom(
    verifier: *mut c_void,
    offset_u: i32,
    offset_v: i32,
) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    if (*v).wrong_output_index < 0_i32 {
        return -1_i32;
    }
    let mut p: vector = (*v).wrong_output_origin;
    p.u = p.u + ((*v).wrong_output_basis_u.u * offset_u + (*v).wrong_output_basis_v.u * offset_v);
    p.v = p.v + ((*v).wrong_output_basis_u.v * offset_u + (*v).wrong_output_basis_v.v * offset_v);
    let a: atom = *rs_lookup_atom(&raw mut (*v).wrong_output_board, p);
    let mut i: i32 = 1_i32;
    while i <= 16_i32 {
        if a as u64 & 1_u64 << i != 0 {
            return i;
        }
        i += 1;
    }
    -1_i32
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_wrong_output_clear(verifier: *mut c_void) {
    let v: *mut verifier = verifier as *mut verifier;
    (*v).wrong_output_index = -1_i32;
    rs_destroy(
        ::core::ptr::null_mut::<solution>(),
        &raw mut (*v).wrong_output_board,
    );
}
unsafe extern "C" fn rs_check_wrong_output_and_destroy(
    v: *mut verifier,
    solution: *mut solution,
    board: *mut board,
) {
    if (*board).wrong_output_index < (*solution).number_of_inputs_and_outputs
        && (*v).wrong_output_index < 0_i32
    {
        let io: *mut input_output = (*solution)
            .inputs_and_outputs
            .add((*board).wrong_output_index);
        (*v).wrong_output_index = (*io).puzzle_index as i32;
        (*v).wrong_output_board = *board;
        let part: *mut solution_part =
            (*(*v).sf).parts.add((*io).solution_index as usize) as *mut solution_part;
        (*v).wrong_output_origin = vector {
            u: (*part).position[0usize],
            v: (*part).position[1usize],
        };
        (*v).wrong_output_basis_u = rs_u_offset_for_direction((*part).rotation);
        (*v).wrong_output_basis_v = rs_v_offset_for_direction((*part).rotation);
        rs_destroy(solution, ::core::ptr::null_mut::<board>());
    } else {
        rs_destroy(solution, board);
    };
}
unsafe extern "C" fn rs_dot2(p: vector, q: vector) -> i32 {
    p.u * (2_i32 * q.u + q.v) + p.v * (2_i32 * q.v + q.u)
}
static mut rs_unset_measurements: per_cycle_measurements = per_cycle_measurements {
    cycles: -1_i32,
    area: -1_i32,
    has_max_direction: [false; 12],
    max_direction: [
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
        -1_i32,
    ],
    executed_instructions: -1_i32,
    instruction_executions: [
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
    ],
    atom_grabs: [
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
    ],
    number_of_atoms: [
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
        -1_i32, -1_i32, -1_i32, -1_i32, -1_i32,
    ],
    error: error {
        description: ::core::ptr::null::<c_char>(),
        source: ::core::ptr::null::<c_char>(),
        cycle: 0,
        location_u: 0,
        location_v: 0,
    },
    valid: true,
};
unsafe extern "C" fn rs_measure_at_current_cycle(
    v: *mut verifier,
    solution: *mut solution,
    board: *mut board,
    check_completion: bool,
) -> per_cycle_measurements {
    let mut error_measurements: per_cycle_measurements = rs_unset_measurements;
    if (*board).rs_collision {
        error_measurements.error.description = (*board).collision_reason;
        error_measurements.error.cycle = (*board).cycle as i32;
        error_measurements.error.location_u = (*board).collision_location.u;
        error_measurements.error.location_v = (*board).collision_location.v;
        error_measurements.error.source = rs_verifier_error_source_simulation;
        return error_measurements;
    } else if !(*board).complete && check_completion as i32 != 0 {
        error_measurements.error.description =
            b"solution did not complete within cycle limit\0".as_ptr() as *const c_char;
        error_measurements.error.source = rs_verifier_error_source_simulation;
        return error_measurements;
    } else if (*board).cycle > 2147483647_u64 {
        error_measurements.error.description =
            b"solution did not complete within cycle integer limit\0".as_ptr() as *const c_char;
        error_measurements.error.source = rs_verifier_error_source_simulation;
        return error_measurements;
    }
    let mut m: per_cycle_measurements = per_cycle_measurements {
        cycles: (*board).cycle as i32,
        area: rs_used_area(board) as i32,
        has_max_direction: [
            true, true, true, true, true, true, true, true, true, true, true, true,
        ],
        max_direction: [
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
            -2147483647_i32 - 1_i32,
        ],
        executed_instructions: 0_i32,
        instruction_executions: [0; 10],
        atom_grabs: [0; 16],
        number_of_atoms: [0; 16],
        error: error {
            description: ::core::ptr::null::<c_char>(),
            source: ::core::ptr::null::<c_char>(),
            cycle: 0,
            location_u: 0,
            location_v: 0,
        },
        valid: true,
    };
    let mut i: u32 = 0_u32;
    while i
        < (((32_i32 - -32_i32) * (32_i32 - -32_i32)) as u32)
            .wrapping_add((*board).grid.hash_capacity)
    {
        let a: atom = (*(*board).grid.atoms_at_positions.add(i as usize)).atom;
        if a as u64 & VALID != 0 {
            let p: vector = (*(*board).grid.atoms_at_positions.add(i as usize)).position;
            let mut j: i32 = 0_i32;
            while j < 12_i32 {
                m.max_direction[j as usize] = rs_max(
                    m.max_direction[j as usize],
                    rs_dot2(rs_growth_directions[j as usize], p),
                );
                j += 1;
            }
            if a as u64 & REMOVED == 0 && a as u64 & WHEEL_ATOM == 0 {
                let mut j_0: i32 = 0_i32;
                while j_0 < 16_i32 {
                    if (*(*board).grid.atoms_at_positions.add(i as usize)).atom
                        & 1_u64 << j_0 + 1_i32
                        != 0
                    {
                        m.number_of_atoms[j_0 as usize] += 1;
                    }
                    j_0 += 1;
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: i32 = 0_i32;
    while i_0 < 16_i32 {
        m.atom_grabs[i_0 as usize] = (*board).atom_grabs[i_0 as usize] as i32;
        i_0 += 1;
    }
    if !solution.is_null() {
        let mut i_1: u32 = 0_u32;
        while (i_1 as usize) < (*solution).number_of_arms {
            if (*board).cycle >= *(*solution).arm_tape_start_cycle.add(i_1 as usize) as u64 {
                if *(*solution).arm_tape_length.add(i_1 as usize) > 0_usize {
                    let mut number_of_times_through_tape: i32 = 0;
                    let mut progress_through_tape: i32 = 0;
                    if *(*solution).arm_tape_halt_index.add(i_1 as usize)
                        != 18446744073709551615_usize
                    {
                        number_of_times_through_tape = 0_i32;
                        progress_through_tape = (*board).cycle.wrapping_sub(
                            *(*solution).arm_tape_start_cycle.add(i_1 as usize) as u64,
                        ) as i32;
                    } else {
                        number_of_times_through_tape = (*board)
                            .cycle
                            .wrapping_sub(
                                *(*solution).arm_tape_start_cycle.add(i_1 as usize) as u64
                            )
                            .wrapping_div((*solution).tape_period)
                            as i32;
                        progress_through_tape = (*board)
                            .cycle
                            .wrapping_sub(
                                *(*solution).arm_tape_start_cycle.add(i_1 as usize) as u64
                            )
                            .wrapping_rem((*solution).tape_period)
                            as i32;
                    }
                    let mut j_1: usize = 0_usize;
                    while j_1 < *(*solution).arm_tape_length.add(i_1 as usize) {
                        if !(*(*(*solution).arm_tape.add(i_1 as usize)).add(j_1)
                            as i32
                            == ' ' as i32
                            || *(*(*solution).arm_tape.add(i_1 as usize)).add(j_1)
                                as i32
                                == '\0' as i32)
                        {
                            if *(*(*solution).arm_tape.add(i_1 as usize)).add(j_1)
                                as i32
                                == 'b' as i32
                            {
                                break;
                            }
                            if j_1
                                < ((*board).cycle as usize).wrapping_sub(
                                    *(*solution).arm_tape_start_cycle.add(i_1 as usize) as usize,
                                )
                            {
                                m.executed_instructions += 1;
                            }
                            m.instruction_executions[rs_instruction_index(
                                *(*(*solution).arm_tape.add(i_1 as usize)).add(j_1),
                            ) as usize] += if j_1 < progress_through_tape as usize {
                                number_of_times_through_tape + 1_i32
                            } else {
                                number_of_times_through_tape
                            };
                        }
                        j_1 = j_1.wrapping_add(1);
                    }
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
    } else {
        m.executed_instructions = -1_i32;
    }
    m
}
unsafe extern "C" fn rs_name_for_atom_type(atom_type: i32) -> *const c_char {
    match atom_type {
        0 => b"salt\0".as_ptr() as *const c_char,
        1 => b"air\0".as_ptr() as *const c_char,
        2 => b"earth\0".as_ptr() as *const c_char,
        3 => b"fire\0".as_ptr() as *const c_char,
        4 => b"water\0".as_ptr() as *const c_char,
        5 => b"quicksilver\0".as_ptr() as *const c_char,
        6 => b"gold\0".as_ptr() as *const c_char,
        7 => b"silver\0".as_ptr() as *const c_char,
        8 => b"copper\0".as_ptr() as *const c_char,
        9 => b"iron\0".as_ptr() as *const c_char,
        10 => b"tin\0".as_ptr() as *const c_char,
        11 => b"lead\0".as_ptr() as *const c_char,
        12 => b"vitae\0".as_ptr() as *const c_char,
        13 => b"mors\0".as_ptr() as *const c_char,
        14 => ::core::ptr::null::<c_char>(),
        15 => b"quintessence\0".as_ptr() as *const c_char,
        _ => ::core::ptr::null::<c_char>(),
    }
}
unsafe extern "C" fn rs_lookup_per_cycle_metric(
    measurements: *const per_cycle_measurements,
    mut metric: *const c_char,
    error: *mut error,
) -> i32 {
    if !(*measurements).valid {
        return -1_i32;
    }
    if strcmp(metric, b"cycles\0".as_ptr() as *const c_char) == 0 {
        (*measurements).cycles
    } else if strcmp(metric, b"area (approximate)\0".as_ptr() as *const c_char) == 0
        || strcmp(metric, b"area\0".as_ptr() as *const c_char) == 0
    {
        (*measurements).area
    } else if strcmp(metric, b"height\0".as_ptr() as *const c_char) == 0 {
        let mut height: i32 = -1_i32;
        let mut i: i32 = 1_i32;
        while i < 6_i32 {
            if !(!(*measurements).has_max_direction[i as usize]
                || !(*measurements).has_max_direction[(i + 6_i32) as usize])
            {
                let height_in_direction: i32 = ((*measurements).max_direction[i as usize]
                    + (*measurements).max_direction[(i + 6_i32) as usize])
                    / 3_i32
                    + 1_i32;
                if height < 0_i32 || height_in_direction < height {
                    height = height_in_direction;
                }
            }
            i += 2_i32;
        }
        height
    } else if strcmp(metric, b"width*2\0".as_ptr() as *const c_char) == 0 {
        let mut width2: i32 = -1_i32;
        let mut i_0: i32 = 0_i32;
        while i_0 < 6_i32 {
            if !(!(*measurements).has_max_direction[i_0 as usize]
                || !(*measurements).has_max_direction[(i_0 + 6_i32) as usize])
            {
                let width2_in_direction: i32 = (*measurements).max_direction[i_0 as usize]
                    + (*measurements).max_direction[(i_0 + 6_i32) as usize]
                    + 2_i32;
                if width2 < 0_i32 || width2_in_direction < width2 {
                    width2 = width2_in_direction;
                }
            }
            i_0 += 2_i32;
        }
        width2
    } else if strcmp(metric, b"minimum hexagon\0".as_ptr() as *const c_char) == 0 {
        let mut minimum_hexagon: i32 = -1_i32;
        let mut i_1: i32 = 1_i32;
        while i_1 < 6_i32 {
            if !(*measurements).has_max_direction[i_1 as usize]
                || !(*measurements).has_max_direction[(i_1 + 6_i32) as usize]
            {
                return -1_i32;
            }
            minimum_hexagon = rs_max(
                minimum_hexagon,
                (((*measurements).max_direction[i_1 as usize]
                    + (*measurements).max_direction[(i_1 + 6_i32) as usize])
                    / 3_i32
                    + 1_i32)
                    / 2_i32,
            );
            i_1 += 2_i32;
        }
        minimum_hexagon = rs_max(
            minimum_hexagon,
            (((*measurements).max_direction[1usize]
                + (*measurements).max_direction[5usize]
                + (*measurements).max_direction[9usize])
                / 3_i32
                + 2_i32)
                / 3_i32,
        );
        minimum_hexagon = rs_max(
            minimum_hexagon,
            (((*measurements).max_direction[3usize]
                + (*measurements).max_direction[7usize]
                + (*measurements).max_direction[11usize])
                / 3_i32
                + 2_i32)
                / 3_i32,
        );
        if minimum_hexagon >= 0_i32 {
            minimum_hexagon += 1_i32;
        }
        minimum_hexagon
    } else if strcmp(metric, b"executed instructions\0".as_ptr() as *const c_char) == 0 {
        (*measurements).executed_instructions
    } else if strcmp(
        metric,
        b"instruction executions\0".as_ptr() as *const c_char,
    ) == 0
    {
        let mut value: i32 = 0_i32;
        let mut i_2: i32 = 0_i32;
        while i_2 < 10_i32 {
            value += (*measurements).instruction_executions[i_2 as usize];
            i_2 += 1;
        }
        value
    } else if strncmp(
        b"instruction executions with hotkey \0".as_ptr() as *const c_char,
        metric,
        strlen(b"instruction executions with hotkey \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric = metric.offset(strlen(
            b"instruction executions with hotkey \0".as_ptr() as *const c_char
        ) as isize);
        if *metric == 0 {
            *error = error {
                description:
                    b"no hotkeys specified in 'instruction executions with hotkey' metric\0"
                        .as_ptr() as *const c_char,
                source: rs_verifier_error_source_metric,
                cycle: 0,
                location_u: 0,
                location_v: 0,
            };
            return -1_i32;
        }
        let mut value_0: i32 = 0_i32;
        while *metric != 0 {
            let idx: i32 = rs_instruction_index(tolower(*metric as i32) as c_char);
            if idx >= 0_i32 {
                value_0 += (*measurements).instruction_executions[idx as usize];
            } else {
                *error = error {
                    description: b"invalid instruction hotkey in 'instruction executions with hotkey' metric\0"
                        .as_ptr() as *const c_char,
                    source: rs_verifier_error_source_metric,
                    cycle: 0,
                    location_u: 0,
                    location_v: 0,
                };
                return -1_i32;
            }
            metric = metric.offset(1);
        }
        value_0
    } else if strcmp(metric, b"atoms grabbed\0".as_ptr() as *const c_char) == 0 {
        let mut value_1: i32 = 0_i32;
        let mut i_3: i32 = 0_i32;
        while i_3 < 16_i32 {
            value_1 += (*measurements).atom_grabs[i_3 as usize];
            i_3 += 1;
        }
        value_1
    } else if strncmp(
        b"atoms grabbed of type \0".as_ptr() as *const c_char,
        metric,
        strlen(b"atoms grabbed of type \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric =
            metric.offset(strlen(b"atoms grabbed of type \0".as_ptr() as *const c_char) as isize);
        let mut i_4: i32 = 0_i32;
        while i_4 < 16_i32 {
            let name: *const c_char = rs_name_for_atom_type(i_4);
            if !name.is_null() && strcmp(metric, name) == 0 {
                return (*measurements).atom_grabs[i_4 as usize];
            }
            i_4 += 1;
        }
        *error = error {
            description: b"unknown atom type\0".as_ptr() as *const c_char,
            source: rs_verifier_error_source_metric,
            cycle: 0,
            location_u: 0,
            location_v: 0,
        };
        -1_i32
    } else if strcmp(metric, b"number of atoms\0".as_ptr() as *const c_char) == 0 {
        let mut value_2: i32 = -1_i32;
        let mut i_5: i32 = 0_i32;
        while i_5 < 16_i32 {
            if (*measurements).number_of_atoms[i_5 as usize] >= 0_i32 {
                if value_2 < 0_i32 {
                    value_2 = 0_i32;
                }
                value_2 += (*measurements).number_of_atoms[i_5 as usize];
            }
            i_5 += 1;
        }
        value_2
    } else if strncmp(
        b"number of atoms of type \0".as_ptr() as *const c_char,
        metric,
        strlen(b"number of atoms of type \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric =
            metric.offset(strlen(b"number of atoms of type \0".as_ptr() as *const c_char) as isize);
        let mut i_6: i32 = 0_i32;
        while i_6 < 16_i32 {
            let name_0: *const c_char = rs_name_for_atom_type(i_6);
            if !name_0.is_null() && strcmp(metric, name_0) == 0 {
                return (*measurements).number_of_atoms[i_6 as usize];
            }
            i_6 += 1;
        }
        *error = error {
            description: b"unknown atom type\0".as_ptr() as *const c_char,
            source: rs_verifier_error_source_metric,
            cycle: 0,
            location_u: 0,
            location_v: 0,
        };
        -1_i32
    } else {
        *error = error {
            description: b"unknown metric\0".as_ptr() as *const c_char,
            source: rs_verifier_error_source_metric,
            cycle: 0,
            location_u: 0,
            location_v: 0,
        };
        -1_i32
    }
}
unsafe extern "C" fn rs_measure_throughput(v: *mut verifier) -> throughput_measurements {
    let mut m: throughput_measurements = throughput_measurements {
        throughput_cycles: -1_i64,
        throughput_outputs: -1_i64,
        throughput_linear_area: -1_i64,
        throughput_quadratic_area: -1_f64,
        throughput_height: 0.,
        throughput_width: 0.,
        throughput_minimum_hexagon: 0.,
        area_growth_order: growth_order::GROWTH_NONE,
        throughput_waste: -1_i32,
        pivot_parity: 0,
        steady_state_start_cycle: 0,
        steady_state: per_cycle_measurements {
            cycles: 0,
            area: 0,
            has_max_direction: [false; 12],
            max_direction: [0; 12],
            executed_instructions: 0,
            instruction_executions: [0; 10],
            atom_grabs: [0; 16],
            number_of_atoms: [0; 16],
            error: error {
                description: ::core::ptr::null::<c_char>(),
                source: ::core::ptr::null::<c_char>(),
                cycle: 0,
                location_u: 0,
                location_v: 0,
            },
            valid: false,
        },
        error: error {
            description: ::core::ptr::null::<c_char>(),
            source: ::core::ptr::null::<c_char>(),
            cycle: 0,
            location_u: 0,
            location_v: 0,
        },
        valid: true,
    };
    if (*v).output_intervals_for_output.is_null() {
        (*v).output_intervals_for_output = calloc(
            (*(*v).pf).number_of_outputs as usize,
            ::core::mem::size_of::<output_intervals>(),
        ) as *mut output_intervals;
    }
    let mut i: u32 = 0_u32;
    while i < (*(*v).pf).number_of_outputs {
        *(*v).output_intervals_for_output.add(i as usize) = output_intervals {
            intervals: ::core::ptr::null_mut::<i32>(),
            number_of_intervals: 0,
            intervals_repeat_after: -1_i32,
        };
        i = i.wrapping_add(1);
    }
    (*v).output_intervals = output_intervals {
        intervals: ::core::ptr::null_mut::<i32>(),
        number_of_intervals: 0,
        intervals_repeat_after: -1_i32,
    };
    let mut solution: solution = solution {
        glyphs: ::core::ptr::null_mut::<mechanism>(),
        number_of_glyphs: 0,
        arms: ::core::ptr::null_mut::<mechanism>(),
        arm_tape: ::core::ptr::null_mut::<*mut c_char>(),
        arm_tape_length: ::core::ptr::null_mut::<usize>(),
        arm_tape_start_cycle: ::core::ptr::null_mut::<i64>(),
        arm_tape_halt_index: ::core::ptr::null_mut::<usize>(),
        number_of_arms: 0,
        tape_period: 0,
        track_positions: ::core::ptr::null_mut::<vector>(),
        track_plus_motions: ::core::ptr::null_mut::<vector>(),
        track_minus_motions: ::core::ptr::null_mut::<vector>(),
        track_table_size: 0,
        track_overlap: 0,
        track_self_overlap: 0,
        number_of_track_loops: 0,
        conduits: ::core::ptr::null_mut::<conduit>(),
        number_of_conduits: 0,
        production: false,
        cabinet_walls: ::core::ptr::null_mut::<vector>(),
        number_of_cabinet_walls: 0,
        cabinet_map: [[0; 32]; 32],
        cabinet_violations: 0,
        inputs_and_outputs: ::core::ptr::null_mut::<input_output>(),
        number_of_inputs_and_outputs: 0,
        target_number_of_outputs: 0,
        min_visible_u: 0,
        max_visible_u: 0,
        min_visible_v: 0,
        max_visible_v: 0,
    };
    let mut board: board = board {
        grid: atom_grid {
            atoms_at_positions: ::core::ptr::null_mut::<atom_at_position>(),
            hash_capacity: 0,
            board: ::core::ptr::null_mut::<board>(),
        },
        area: 0,
        flag_reset: ::core::ptr::null_mut::<*mut atom>(),
        flag_reset_capacity: 0,
        flag_reset_length: 0,
        cycle: 0,
        half_cycle: 0,
        movements: movement_list {
            movements: ::core::ptr::null_mut::<movement>(),
            capacity: 0,
            length: 0,
            cursor: 0,
        },
        moving_atoms: moving_atoms {
            atoms_at_positions: ::core::ptr::null_mut::<atom_at_position>(),
            capacity: 0,
            length: 0,
            cursor: 0,
        },
        overlapped_atoms: ::core::ptr::null_mut::<atom_at_position>(),
        number_of_overlapped_atoms: 0,
        overlapped_atoms_capacity: 0,
        atoms_being_produced: ::core::ptr::null_mut::<vector>(),
        number_of_atoms_being_produced: 0,
        atoms_being_produced_capacity: 0,
        molecule: molecule {
            atoms: ::core::ptr::null_mut::<atom_ref_at_position>(),
            cursor: 0,
            size: 0,
            capacity: 0,
        },
        rs_collision: false,
        collision_location: vector { u: 0, v: 0 },
        collision_reason: ::core::ptr::null::<c_char>(),
        atom_grabs: [0; 16],
        fails_on_wrong_output_mask: 0,
        fails_on_wrong_output_bonds_mask: 0,
        wrong_output_index: 0,
        overlap: 0,
        chain_mode: chain_mode::DISCOVER_CHAIN,
        chain_atoms: ::core::ptr::null_mut::<chain_atom>(),
        number_of_chain_atoms: 0,
        chain_atom_table: ::core::ptr::null_mut::<u32>(),
        chain_atom_table_size: 0,
        chain_will_become_visible: false,
        area_growth_order: growth_order::GROWTH_NONE,
        area_directions: ::core::ptr::null_mut::<linear_area_direction>(),
        number_of_area_directions: 0,
        disjoint_bond_table: disjoint_bond_table {
            bonds: ::core::ptr::null_mut::<disjoint_bond>(),
            size: 0,
            count: 0,
        },
        collision_checks: 0,
        collision_check_limit: 0,
        complete: false,
        index: core::ptr::null_mut(),
        glyph_plan: core::ptr::null_mut(),
        written_bbox: [i32::MAX, i32::MIN, i32::MAX, i32::MIN],
    };
    if !rs_decode_solution(
        &raw mut solution,
        (*v).pf as *mut puzzle_file,
        (*v).sf as *mut solution_file,
        &raw mut m.error.description,
    ) {
        m.error.source = rs_verifier_error_source_solution_file;
        return m;
    }
    rs_initial_setup(&raw mut solution, &raw mut board, (*(*v).sf).area);
    if !(*v).disable_limits {
        board.collision_check_limit = (*v).collision_check_limit;
    }
    board.fails_on_wrong_output_mask = (*v).fails_on_wrong_output_mask;
    board.fails_on_wrong_output_bonds_mask = (*v).fails_on_wrong_output_bonds_mask;
    let steady_state: steady_state = rs_run_until_steady_state(
        &raw mut solution,
        &raw mut board,
        if (*v).disable_limits as i32 != 0 {
            18446744073709551615_u64
        } else {
            (*v).cycle_limit
        },
    );
    if board.rs_collision {
        m.error.description = board.collision_reason;
        m.error.cycle = board.cycle as i32;
        m.error.location_u = board.collision_location.u as i32;
        m.error.location_v = board.collision_location.v as i32;
        m.error.source = rs_verifier_error_source_simulation;
    } else if steady_state.eventual_behavior.0
        == eventual_behavior::EVENTUALLY_ENTERS_STEADY_STATE.0
    {
        m.throughput_cycles = steady_state.number_of_cycles as i64;
        m.throughput_outputs = steady_state.number_of_outputs as i64;
        m.throughput_linear_area = steady_state.linear_area_growth as i64;
        m.throughput_quadratic_area = steady_state.quadratic_area_growth;
        m.area_growth_order = steady_state.area_growth_order;
        m.throughput_height = 1e100f64;
        m.throughput_width = 1e100f64;
        m.throughput_minimum_hexagon = 0 as f64;
        let mut i_0: i32 = 0_i32;
        while i_0 < 6_i32 {
            let width_growth: f64 = steady_state.directional_growth[i_0 as usize]
                + steady_state.directional_growth[(i_0 + 6_i32) as usize];
            if width_growth < m.throughput_width {
                m.throughput_width = width_growth;
            }
            let height_growth: f64 = (steady_state.directional_growth[(i_0 + 1_i32) as usize]
                + steady_state.directional_growth[(i_0 + 7_i32) as usize])
                / 1.5f64;
            if height_growth < m.throughput_height {
                m.throughput_height = height_growth;
            }
            if height_growth / 2_f64 > m.throughput_minimum_hexagon {
                m.throughput_minimum_hexagon = height_growth / 2_f64;
            }
            i_0 += 2_i32;
        }
        let triangle1: f64 = (steady_state.directional_growth[1usize]
            + steady_state.directional_growth[5usize]
            + steady_state.directional_growth[9usize])
            / 4.5f64;
        let triangle2: f64 = (steady_state.directional_growth[3usize]
            + steady_state.directional_growth[7usize]
            + steady_state.directional_growth[11usize])
            / 4.5f64;
        if triangle1 > m.throughput_minimum_hexagon {
            m.throughput_minimum_hexagon = triangle1;
        }
        if triangle2 > m.throughput_minimum_hexagon {
            m.throughput_minimum_hexagon = triangle2;
        }
        m.pivot_parity = steady_state.pivot_parity as i32;
        m.steady_state_start_cycle = steady_state.outputs_repeat_after_cycle as i32;
        m.throughput_waste = 0_i32;
        m.steady_state = rs_measure_at_current_cycle(v, &raw mut solution, &raw mut board, false);
        m.steady_state.cycles = -1_i32;
        let mut i_1: i32 = 0_i32;
        while i_1 < 12_i32 {
            if steady_state.directional_growth[i_1 as usize] > 0 as f64 {
                m.steady_state.has_max_direction[i_1 as usize] = false;
            }
            i_1 += 1;
        }
        if m.area_growth_order.0 != growth_order::GROWTH_NONE.0 {
            m.steady_state.area = -1_i32;
            m.throughput_waste = 1_i32;
            let mut i_2: i32 = 0_i32;
            while i_2 < 16_i32 {
                m.steady_state.number_of_atoms[i_2 as usize] = -1_i32;
                i_2 += 1;
            }
        }
        m.steady_state.executed_instructions = rs_solution_instructions(&raw mut solution) as i32;
        let mut i_3: u32 = 0_u32;
        while (i_3 as usize) < solution.number_of_arms {
            let mut j: usize = 0_usize;
            while j < *solution.arm_tape_length.add(i_3 as usize) {
                if !(*(*solution.arm_tape.add(i_3 as usize)).add(j) as i32
                    == ' ' as i32
                    || *(*solution.arm_tape.add(i_3 as usize)).add(j) as i32
                        == '\0' as i32
                    || *(*solution.arm_tape.add(i_3 as usize)).add(j) as i32
                        == 'b' as i32)
                {
                    m.steady_state.instruction_executions[rs_instruction_index(
                        *(*solution.arm_tape.add(i_3 as usize)).add(j),
                    ) as usize] = -1_i32;
                }
                j = j.wrapping_add(1);
            }
            i_3 = i_3.wrapping_add(1);
        }
        let mut i_4: i32 = 0_i32;
        while i_4 < 16_i32 {
            m.steady_state.atom_grabs[i_4 as usize] = -1_i32;
            i_4 += 1;
        }
    } else {
        m.error.description =
            b"solution did not converge on a throughput\0".as_ptr() as *const c_char;
        m.error.source = rs_verifier_error_source_simulation;
    }
    let mut i_5: usize = 0_usize;
    while i_5 < solution.number_of_inputs_and_outputs {
        let io: *mut input_output = solution.inputs_and_outputs.add(i_5);
        if (*io).r#type.0 & input_output_type::OUTPUT.0 != 0 {
            let oi: *mut output_intervals = (*v)
                .output_intervals_for_output
                .add((*io).puzzle_index as usize);
            (*oi).intervals = calloc(
                (*io).number_of_output_cycles as usize,
                ::core::mem::size_of::<i32>(),
            ) as *mut i32;
            let mut j_0: u64 = 0_u64;
            while j_0 < (*io).number_of_output_cycles {
                *(*oi).intervals.add(j_0 as usize) =
                    *(*io).output_cycles.add(j_0 as usize) as i32;
                j_0 = j_0.wrapping_add(1);
            }
            (*oi).number_of_intervals = (*io).number_of_output_cycles as i32;
            if steady_state.eventual_behavior.0
                == eventual_behavior::EVENTUALLY_ENTERS_STEADY_STATE.0
            {
                let mut j_1: usize = 0_usize;
                while j_1 < (*oi).number_of_intervals as usize {
                    if *(*oi).intervals.add(j_1) as u64
                        > steady_state.outputs_repeat_after_cycle
                    {
                        (*oi).intervals_repeat_after = j_1 as i32;
                        break;
                    } else {
                        j_1 = j_1.wrapping_add(1);
                    }
                }
            }
            let mut last: i32 = 0_i32;
            let mut j_2: i32 = 0_i32;
            while j_2 < (*oi).number_of_intervals {
                *(*oi).intervals.offset(j_2 as isize) -= last;
                last += *(*oi).intervals.offset(j_2 as isize);
                j_2 += 1;
            }
            rs_normalize_output_intervals(oi);
        }
        i_5 = i_5.wrapping_add(1);
    }
    rs_check_wrong_output_and_destroy(v, &raw mut solution, &raw mut board);
    m
}
unsafe extern "C" fn rs_measure_output_intervals(
    v: *mut verifier,
    output: i32,
) -> *mut output_intervals {
    if v.is_null() || (*v).sf.is_null() || (*v).pf.is_null() {
        return ::core::ptr::null_mut::<output_intervals>();
    }
    if output >= 0_i32 && output as u32 >= (*(*v).pf).number_of_outputs {
        return ::core::ptr::null_mut::<output_intervals>();
    }
    if !(*v).throughput_measurements.valid {
        (*v).throughput_measurements = rs_measure_throughput(v);
    }
    if output < 0_i32 {
        if (*v).output_intervals.intervals.is_null() {
            (*v).output_intervals.intervals =
                calloc(1_usize, ::core::mem::size_of::<i32>()) as *mut i32;
            (*v).output_intervals.number_of_intervals = 1_i32;
            (*v).output_intervals.intervals_repeat_after = 0_i32;
            let mut i: u32 = 0_u32;
            while i < (*(*v).pf).number_of_outputs {
                rs_combine_output_intervals(
                    &raw mut (*v).output_intervals,
                    (*v).output_intervals_for_output.add(i as usize),
                );
                rs_normalize_output_intervals(&raw mut (*v).output_intervals);
                i = i.wrapping_add(1);
            }
        }
        return &raw mut (*v).output_intervals;
    }
    (*v).output_intervals_for_output.offset(output as isize)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_number_of_output_intervals(verifier: *mut c_void) -> i32 {
    rs_verifier_number_of_output_intervals_for_output(verifier, -1_i32)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_output_interval(
    verifier: *mut c_void,
    which_interval: i32,
) -> i32 {
    rs_verifier_output_interval_for_output(verifier, -1_i32, which_interval)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_output_intervals_repeat_after(verifier: *mut c_void) -> i32 {
    rs_verifier_output_intervals_for_output_repeat_after(verifier, -1_i32)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_number_of_output_intervals_for_output(
    verifier: *mut c_void,
    output: i32,
) -> i32 {
    let oi: *mut output_intervals = rs_measure_output_intervals(verifier as *mut verifier, output);
    if oi.is_null() {
        return -1_i32;
    }
    (*oi).number_of_intervals
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_output_interval_for_output(
    verifier: *mut c_void,
    output: i32,
    which_interval: i32,
) -> i32 {
    let oi: *mut output_intervals = rs_measure_output_intervals(verifier as *mut verifier, output);
    if oi.is_null() || which_interval < 0_i32 || which_interval >= (*oi).number_of_intervals {
        return -1_i32;
    }
    *(*oi).intervals.offset(which_interval as isize)
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_output_intervals_for_output_repeat_after(
    verifier: *mut c_void,
    output: i32,
) -> i32 {
    let oi: *mut output_intervals = rs_measure_output_intervals(verifier as *mut verifier, output);
    if oi.is_null() {
        return -1_i32;
    }
    (*oi).intervals_repeat_after
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_number_of_outputs(verifier: *mut c_void) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    if (*v).pf.is_null() {
        return -1_i32;
    }
    (*(*v).pf).number_of_outputs as i32
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_evaluate_metric(
    verifier: *mut c_void,
    mut metric: *const c_char,
) -> i32 {
    let v: *mut verifier = verifier as *mut verifier;
    if (*v).sf.is_null() {
        return -1_i32;
    }
    if strcmp(metric, b"parsed cycles\0".as_ptr() as *const c_char) == 0 {
        return (*(*v).sf).cycles as i32;
    } else if strcmp(metric, b"parsed cost\0".as_ptr() as *const c_char) == 0 {
        return (*(*v).sf).cost as i32;
    } else if strcmp(metric, b"parsed area\0".as_ptr() as *const c_char) == 0 {
        return (*(*v).sf).area as i32;
    } else if strcmp(metric, b"parsed instructions\0".as_ptr() as *const c_char) == 0 {
        return (*(*v).sf).instructions as i32;
    } else if strcmp(
        metric,
        b"number of track segments\0".as_ptr() as *const c_char,
    ) == 0
    {
        let mut value: i32 = 0_i32;
        let mut i: u32 = 0_u32;
        while i < (*(*v).sf).number_of_parts {
            value = (value as u32)
                .wrapping_add((*(*(*v).sf).parts.add(i as usize)).number_of_track_hexes)
                as i32;
            i = i.wrapping_add(1);
        }
        return value;
    } else if strncmp(
        b"parts of type \0".as_ptr() as *const c_char,
        metric,
        strlen(b"parts of type \0".as_ptr() as *const c_char),
    ) == 0
    {
        let mut value_0: i32 = 0_i32;
        let part_name: *const c_char =
            metric.offset(strlen(b"parts of type \0".as_ptr() as *const c_char) as isize);
        let mut i_0: u32 = 0_u32;
        while i_0 < (*(*v).sf).number_of_parts {
            if rs_byte_string_is((*(*(*v).sf).parts.add(i_0 as usize)).name, part_name) {
                value_0 += 1;
            }
            i_0 = i_0.wrapping_add(1);
        }
        return value_0;
    } else if strcmp(metric, b"cost\0".as_ptr() as *const c_char) == 0 {
        return rs_solution_file_cost((*v).sf as *mut solution_file) as i32;
    } else if strcmp(metric, b"duplicate reagents\0".as_ptr() as *const c_char) == 0
        || strcmp(metric, b"duplicate products\0".as_ptr() as *const c_char) == 0
    {
        let input: bool = strcmp(metric, b"duplicate reagents\0".as_ptr() as *const c_char) == 0;
        let n: u32 = if input as i32 != 0 {
            (*(*v).pf).number_of_inputs
        } else {
            (*(*v).pf).number_of_outputs
        };
        let mut duplicates: i32 = 0_i32;
        let seen: *mut i32 = calloc(n as usize, ::core::mem::size_of::<i32>()) as *mut i32;
        let mut i_1: u32 = 0_u32;
        while i_1 < (*(*v).sf).number_of_parts {
            if !(input as i32 != 0
                && !rs_byte_string_is(
                    (*(*(*v).sf).parts.add(i_1 as usize)).name,
                    b"input\0".as_ptr() as *const c_char,
                ))
            {
                if !(!input
                    && !rs_byte_string_is(
                        (*(*(*v).sf).parts.add(i_1 as usize)).name,
                        b"out-std\0".as_ptr() as *const c_char,
                    )
                    && !rs_byte_string_is(
                        (*(*(*v).sf).parts.add(i_1 as usize)).name,
                        b"out-rep\0".as_ptr() as *const c_char,
                    ))
                {
                    let index: u32 = (*(*(*v).sf).parts.add(i_1 as usize)).which_input_or_output;
                    if index >= n {
                        duplicates = -1_i32;
                        (*v).error.description = b"solution refers to a reagent or product that doesn't exist in the puzzle\0"
                            .as_ptr() as *const c_char;
                        (*v).error.source = rs_verifier_error_source_solution_file;
                        break;
                    } else {
                        if *seen.add(index as usize) != 0 {
                            duplicates += 1;
                        }
                        *seen.add(index as usize) = 1_i32;
                    }
                }
            }
            i_1 = i_1.wrapping_add(1);
        }
        free(seen as *mut c_void);
        return duplicates;
    } else if strcmp(metric, b"maximum track gap^2\0".as_ptr() as *const c_char) == 0 {
        let mut gap2: i64 = 0_i64;
        let mut i_2: u32 = 0_u32;
        while i_2 < (*(*v).sf).number_of_parts {
            let mut j: u32 = 1_u32;
            while j < (*(*(*v).sf).parts.add(i_2 as usize)).number_of_track_hexes {
                let a: solution_hex_offset = *(*(*(*v).sf).parts.add(i_2 as usize))
                    .track_hexes
                    .offset(j.wrapping_sub(1_u32) as isize)
                    as solution_hex_offset;
                let b: solution_hex_offset = *(*(*(*v).sf).parts.add(i_2 as usize))
                    .track_hexes
                    .add(j as usize)
                    as solution_hex_offset;
                let du: i32 = b.offset[0usize] - a.offset[0usize];
                let dv: i32 = b.offset[1usize] - a.offset[1usize];
                let g: i64 = (du * du + du * dv + dv * dv) as i64;
                if g > gap2 {
                    gap2 = g;
                }
                j = j.wrapping_add(1);
            }
            i_2 = i_2.wrapping_add(1);
        }
        return if gap2 > 2147483647_i64 {
            2147483647_i32
        } else {
            gap2 as i32
        };
    } else if strcmp(
        metric,
        b"maximum absolute part coordinate\0".as_ptr() as *const c_char,
    ) == 0
    {
        let mut max_distance: i64 = 0_i64;
        let mut i_3: u32 = 0_u32;
        while i_3 < (*(*v).sf).number_of_parts {
            let p: solution_part = *(*(*v).sf).parts.add(i_3 as usize) as solution_part;
            let q: i64 = llabs(p.position[0usize] as i64);
            let r: i64 = llabs(p.position[1usize] as i64);
            let s: i64 = llabs(-(p.position[0usize] as i64) - p.position[1usize] as i64);
            if q > max_distance {
                max_distance = q;
            }
            if r > max_distance {
                max_distance = r;
            }
            if s > max_distance {
                max_distance = s;
            }
            i_3 = i_3.wrapping_add(1);
        }
        return if max_distance > 2147483647_i64 {
            2147483647_i32
        } else {
            max_distance as i32
        };
    }
    if (*v).pf.is_null() {
        (*v).error.description = b"invalid puzzle file\0".as_ptr() as *const c_char;
        (*v).error.source = rs_verifier_error_source_puzzle_file;
        return -1_i32;
    }
    if strcmp(metric, b"reaches steady state\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        if (*v).throughput_measurements.throughput_cycles < 0_i64 {
            return 0_i32;
        } else {
            return 1_i32;
        }
    } else if strcmp(metric, b"per repetition cycles\0".as_ptr() as *const c_char) == 0
        || strcmp(metric, b"throughput cycles\0".as_ptr() as *const c_char) == 0
    {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        return (*v).throughput_measurements.throughput_cycles as i32;
    } else if strcmp(
        metric,
        b"per repetition outputs\0".as_ptr() as *const c_char,
    ) == 0
        || strcmp(metric, b"throughput outputs\0".as_ptr() as *const c_char) == 0
    {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        return (*v).throughput_measurements.throughput_outputs as i32;
    } else if strcmp(metric, b"per repetition area\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        if (*v).throughput_measurements.area_growth_order.0 == growth_order::GROWTH_QUADRATIC.0 {
            (*v).error = error {
                description: b"metric doesn't have a consistent per-repetition value\0".as_ptr()
                    as *const c_char,
                source: rs_verifier_error_source_simulation,
                cycle: 0,
                location_u: 0,
                location_v: 0,
            };
            return -1_i32;
        }
        return (*v).throughput_measurements.throughput_linear_area as i32;
    } else if strcmp(metric, b"throughput waste\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        return (*v).throughput_measurements.throughput_waste;
    } else if strcmp(
        metric,
        b"visual loop start cycle\0".as_ptr() as *const c_char,
    ) == 0
    {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        return (*v).throughput_measurements.steady_state_start_cycle;
    } else if strcmp(metric, b"visual loop end cycle\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        let start: i32 = (*v).throughput_measurements.steady_state_start_cycle;
        let period: i32 = (*v).throughput_measurements.throughput_cycles as i32;
        if (*v).throughput_measurements.pivot_parity != 0 {
            return start + 2_i32 * period;
        } else {
            return start + period;
        }
    }
    let mut solution: solution = solution {
        glyphs: ::core::ptr::null_mut::<mechanism>(),
        number_of_glyphs: 0,
        arms: ::core::ptr::null_mut::<mechanism>(),
        arm_tape: ::core::ptr::null_mut::<*mut c_char>(),
        arm_tape_length: ::core::ptr::null_mut::<usize>(),
        arm_tape_start_cycle: ::core::ptr::null_mut::<i64>(),
        arm_tape_halt_index: ::core::ptr::null_mut::<usize>(),
        number_of_arms: 0,
        tape_period: 0,
        track_positions: ::core::ptr::null_mut::<vector>(),
        track_plus_motions: ::core::ptr::null_mut::<vector>(),
        track_minus_motions: ::core::ptr::null_mut::<vector>(),
        track_table_size: 0,
        track_overlap: 0,
        track_self_overlap: 0,
        number_of_track_loops: 0,
        conduits: ::core::ptr::null_mut::<conduit>(),
        number_of_conduits: 0,
        production: false,
        cabinet_walls: ::core::ptr::null_mut::<vector>(),
        number_of_cabinet_walls: 0,
        cabinet_map: [[0; 32]; 32],
        cabinet_violations: 0,
        inputs_and_outputs: ::core::ptr::null_mut::<input_output>(),
        number_of_inputs_and_outputs: 0,
        target_number_of_outputs: 0,
        min_visible_u: 0,
        max_visible_u: 0,
        min_visible_v: 0,
        max_visible_v: 0,
    };
    let mut board: board = board {
        grid: atom_grid {
            atoms_at_positions: ::core::ptr::null_mut::<atom_at_position>(),
            hash_capacity: 0,
            board: ::core::ptr::null_mut::<board>(),
        },
        area: 0,
        flag_reset: ::core::ptr::null_mut::<*mut atom>(),
        flag_reset_capacity: 0,
        flag_reset_length: 0,
        cycle: 0,
        half_cycle: 0,
        movements: movement_list {
            movements: ::core::ptr::null_mut::<movement>(),
            capacity: 0,
            length: 0,
            cursor: 0,
        },
        moving_atoms: moving_atoms {
            atoms_at_positions: ::core::ptr::null_mut::<atom_at_position>(),
            capacity: 0,
            length: 0,
            cursor: 0,
        },
        overlapped_atoms: ::core::ptr::null_mut::<atom_at_position>(),
        number_of_overlapped_atoms: 0,
        overlapped_atoms_capacity: 0,
        atoms_being_produced: ::core::ptr::null_mut::<vector>(),
        number_of_atoms_being_produced: 0,
        atoms_being_produced_capacity: 0,
        molecule: molecule {
            atoms: ::core::ptr::null_mut::<atom_ref_at_position>(),
            cursor: 0,
            size: 0,
            capacity: 0,
        },
        rs_collision: false,
        collision_location: vector { u: 0, v: 0 },
        collision_reason: ::core::ptr::null::<c_char>(),
        atom_grabs: [0; 16],
        fails_on_wrong_output_mask: 0,
        fails_on_wrong_output_bonds_mask: 0,
        wrong_output_index: 0,
        overlap: 0,
        chain_mode: chain_mode::DISCOVER_CHAIN,
        chain_atoms: ::core::ptr::null_mut::<chain_atom>(),
        number_of_chain_atoms: 0,
        chain_atom_table: ::core::ptr::null_mut::<u32>(),
        chain_atom_table_size: 0,
        chain_will_become_visible: false,
        area_growth_order: growth_order::GROWTH_NONE,
        area_directions: ::core::ptr::null_mut::<linear_area_direction>(),
        number_of_area_directions: 0,
        disjoint_bond_table: disjoint_bond_table {
            bonds: ::core::ptr::null_mut::<disjoint_bond>(),
            size: 0,
            count: 0,
        },
        collision_checks: 0,
        collision_check_limit: 0,
        complete: false,
        index: core::ptr::null_mut(),
        glyph_plan: core::ptr::null_mut(),
        written_bbox: [i32::MAX, i32::MIN, i32::MAX, i32::MIN],
    };
    if !rs_decode_solution(
        &raw mut solution,
        (*v).pf as *mut puzzle_file,
        (*v).sf as *mut solution_file,
        &raw mut (*v).error.description,
    ) {
        (*v).error.source = rs_verifier_error_source_solution_file;
        return -1_i32;
    }
    if strcmp(metric, b"instructions\0".as_ptr() as *const c_char) == 0 {
        let instructions: i32 = rs_solution_instructions(&raw mut solution) as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        return instructions;
    } else if strncmp(
        b"instructions with hotkey \0".as_ptr() as *const c_char,
        metric,
        strlen(b"instructions with hotkey \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric = metric
            .offset(strlen(b"instructions with hotkey \0".as_ptr() as *const c_char) as isize);
        let mut value_1: i32 = 0_i32;
        if *metric == 0 {
            value_1 = -1_i32;
            (*v).error.description = b"no hotkeys specified in 'instructions with hotkey' metric\0"
                .as_ptr() as *const c_char;
            (*v).error.source = rs_verifier_error_source_metric;
        }
        while *metric != 0 {
            match tolower(*metric as i32) {
                97 | 100 | 101 | 102 | 103 | 113 | 114 | 115 | 116 | 119 | 98 => {}
                _ => {
                    (*v).error.description =
                        b"invalid instruction hotkey in 'instructions with hotkey' metric\0"
                            .as_ptr() as *const c_char;
                    (*v).error.source = rs_verifier_error_source_metric;
                    rs_destroy(&raw mut solution, &raw mut board);
                    return -1_i32;
                }
            }
            let mut i_4: u32 = 0_u32;
            while (i_4 as usize) < solution.number_of_arms {
                let mut j_0: usize = 0_usize;
                while j_0 < *solution.arm_tape_length.add(i_4 as usize) {
                    if *(*solution.arm_tape.add(i_4 as usize)).add(j_0) as i32
                        == tolower(*metric as i32)
                    {
                        value_1 += 1;
                    }
                    j_0 = j_0.wrapping_add(1);
                }
                i_4 = i_4.wrapping_add(1);
            }
            metric = metric.offset(1);
        }
        rs_destroy(&raw mut solution, &raw mut board);
        return value_1;
    } else if strcmp(
        metric,
        b"instruction tape period\0".as_ptr() as *const c_char,
    ) == 0
    {
        let period_0: i32 = solution.tape_period as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        return period_0;
    } else if strcmp(metric, b"number of arms\0".as_ptr() as *const c_char) == 0 {
        let arms: i32 = solution.number_of_arms as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        return arms;
    } else if strcmp(metric, b"cabinet violations\0".as_ptr() as *const c_char) == 0 {
        let cabinet_violations: i32 = solution.cabinet_violations as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        if cabinet_violations == 0_i32 {
            return 0_i32;
        } else {
            return 1_i32;
        }
    } else if strcmp(metric, b"conduit violations\0".as_ptr() as *const c_char) == 0 {
        let conduit_violations: i32 = (solution.cabinet_violations as c_ulong
            & ((1 as c_ulong) << 7_i32 | (1 as c_ulong) << 8_i32))
            as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        if conduit_violations == 0_i32 {
            return 0_i32;
        } else {
            return 1_i32;
        }
    } else if strcmp(metric, b"track self-overlap\0".as_ptr() as *const c_char) == 0 {
        let track_self_overlap: i32 = solution.track_self_overlap as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        return track_self_overlap;
    } else if strcmp(metric, b"track loops\0".as_ptr() as *const c_char) == 0 {
        let loops: i32 = solution.number_of_track_loops as i32;
        rs_destroy(&raw mut solution, &raw mut board);
        return loops;
    }
    rs_initial_setup(&raw mut solution, &raw mut board, (*(*v).sf).area);
    if !(*v).disable_limits {
        board.collision_check_limit = (*v).collision_check_limit;
    }
    board.fails_on_wrong_output_mask = (*v).fails_on_wrong_output_mask;
    board.fails_on_wrong_output_bonds_mask = (*v).fails_on_wrong_output_bonds_mask;
    if strcmp(metric, b"overlap\0".as_ptr() as *const c_char) == 0 {
        let mut overlap: i32 = 2147483647_i32;
        if board.overlap < 2147483647_u64 {
            overlap = board.overlap as i32;
        }
        rs_destroy(&raw mut solution, &raw mut board);
        return overlap;
    }
    if strncmp(
        b"product \0".as_ptr() as *const c_char,
        metric,
        strlen(b"product \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric = metric.offset(strlen(b"product \0".as_ptr() as *const c_char) as isize);
        let mut endptr: *mut c_char = ::core::ptr::null_mut::<c_char>();
        let product_count: c_long = strtol(metric, &raw mut endptr, 10_i32);
        if product_count < 0 as c_long || endptr == metric as *mut c_char {
            (*v).error.description = b"invalid product count\0".as_ptr() as *const c_char;
            (*v).error.source = rs_verifier_error_source_metric;
            rs_destroy(&raw mut solution, &raw mut board);
            return -1_i32;
        }
        if *endptr as i32 != ' ' as i32 {
            (*v).error.description =
                b"product count must be followed by a metric\0".as_ptr() as *const c_char;
            (*v).error.source = rs_verifier_error_source_metric;
            rs_destroy(&raw mut solution, &raw mut board);
            return -1_i32;
        }
        metric = endptr.add(1) as *const c_char;
        solution.target_number_of_outputs = product_count as u64;
        if product_count <= 0 as c_long {
            board.complete = true;
        }
        crate::ffwd::run(
            (&raw mut solution).cast(),
            (&raw mut board).cast(),
            (!(*v).disable_limits).then_some((*v).cycle_limit),
        );
        let mut m: per_cycle_measurements =
            rs_measure_at_current_cycle(v, &raw mut solution, &raw mut board, true);
        rs_check_wrong_output_and_destroy(v, &raw mut solution, &raw mut board);
        (*v).error = m.error;
        rs_lookup_per_cycle_metric(&raw mut m, metric, &raw mut (*v).error)
    } else if strncmp(
        b"cycle \0".as_ptr() as *const c_char,
        metric,
        strlen(b"cycle \0".as_ptr() as *const c_char),
    ) == 0
    {
        metric = metric.offset(strlen(b"cycle \0".as_ptr() as *const c_char) as isize);
        let mut endptr_0: *mut c_char = ::core::ptr::null_mut::<c_char>();
        let cycle_count: c_long = strtol(metric, &raw mut endptr_0, 10_i32);
        if cycle_count < 0 as c_long || endptr_0 == metric as *mut c_char {
            (*v).error.description = b"invalid cycle count\0".as_ptr() as *const c_char;
            (*v).error.source = rs_verifier_error_source_metric;
            rs_destroy(&raw mut solution, &raw mut board);
            return -1_i32;
        }
        if *endptr_0 as i32 != ' ' as i32 {
            (*v).error.description =
                b"cycle count must be followed by a metric\0".as_ptr() as *const c_char;
            (*v).error.source = rs_verifier_error_source_metric;
            rs_destroy(&raw mut solution, &raw mut board);
            return -1_i32;
        }
        metric = endptr_0.add(1) as *const c_char;
        solution.target_number_of_outputs = 18446744073709551615 as c_ulong;
        crate::ffwd::run(
            (&raw mut solution).cast(),
            (&raw mut board).cast(),
            Some(cycle_count as u64),
        );
        board.complete = true;
        let mut m_0: per_cycle_measurements =
            rs_measure_at_current_cycle(v, &raw mut solution, &raw mut board, true);
        rs_check_wrong_output_and_destroy(v, &raw mut solution, &raw mut board);
        (*v).error = m_0.error;
        rs_lookup_per_cycle_metric(&raw mut m_0, metric, &raw mut (*v).error)
    } else if strncmp(
        b"steady state \0".as_ptr() as *const c_char,
        metric,
        strlen(b"steady state \0".as_ptr() as *const c_char),
    ) == 0
    {
        rs_destroy(&raw mut solution, &raw mut board);
        metric = metric.offset(strlen(b"steady state \0".as_ptr() as *const c_char) as isize);
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        if (*v).throughput_measurements.throughput_cycles < 0_i64 {
            (*v).error = (*v).throughput_measurements.error;
            return -1_i32;
        }
        (*v).error = (*v).throughput_measurements.steady_state.error;
        let value_2: i32 = rs_lookup_per_cycle_metric(
            &raw mut (*v).throughput_measurements.steady_state,
            metric,
            &raw mut (*v).error,
        );
        if value_2 < 0_i32 && (*v).error.description.is_null() {
            (*v).error = error {
                description: b"metric doesn't reach a steady state\0".as_ptr() as *const c_char,
                source: rs_verifier_error_source_simulation,
                cycle: 0,
                location_u: 0,
                location_v: 0,
            };
        }
        value_2
    } else {
        let mut error: error = error {
            description: ::core::ptr::null::<c_char>(),
            source: ::core::ptr::null::<c_char>(),
            cycle: 0,
            location_u: 0,
            location_v: 0,
        };
        rs_lookup_per_cycle_metric(&raw const rs_unset_measurements, metric, &raw mut error);
        if !error.description.is_null() {
            (*v).error = error;
            return -1_i32;
        }
        if !(*v).completion.valid {
            crate::ffwd::run(
                (&raw mut solution).cast(),
                (&raw mut board).cast(),
                (!(*v).disable_limits).then_some((*v).cycle_limit),
            );
            (*v).completion =
                rs_measure_at_current_cycle(v, &raw mut solution, &raw mut board, true);
        }
        rs_check_wrong_output_and_destroy(v, &raw mut solution, &raw mut board);
        (*v).error = (*v).completion.error;
        rs_lookup_per_cycle_metric(&raw mut (*v).completion, metric, &raw mut (*v).error)
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_verifier_evaluate_approximate_metric(
    verifier: *mut c_void,
    metric: *const c_char,
) -> f64 {
    let v: *mut verifier = verifier as *mut verifier;
    if (*v).sf.is_null() {
        return -1_f64;
    }
    if strcmp(metric, b"per repetition^2 area\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        (*v).throughput_measurements.throughput_quadratic_area
    } else if strcmp(metric, b"width\0".as_ptr() as *const c_char) == 0 {
        rs_verifier_evaluate_metric(verifier, b"width*2\0".as_ptr() as *const c_char) as f64
            / 2.0f64
    } else if strcmp(metric, b"steady state width\0".as_ptr() as *const c_char) == 0 {
        rs_verifier_evaluate_metric(
            verifier,
            b"steady state width*2\0".as_ptr() as *const c_char,
        ) as f64
            / 2.0f64
    } else if strcmp(metric, b"per repetition height\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        (*v).throughput_measurements.throughput_height
    } else if strcmp(metric, b"per repetition width\0".as_ptr() as *const c_char) == 0 {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        (*v).throughput_measurements.throughput_width
    } else if strcmp(
        metric,
        b"per repetition minimum hexagon\0".as_ptr() as *const c_char,
    ) == 0
    {
        if !(*v).throughput_measurements.valid {
            (*v).throughput_measurements = rs_measure_throughput(v);
        }
        (*v).error = (*v).throughput_measurements.error;
        (*v).throughput_measurements.throughput_minimum_hexagon
    } else {
        rs_verifier_evaluate_metric(verifier, metric) as f64
    }
}
