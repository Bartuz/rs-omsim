//! One cycle of the simulation (C: sim.c). The phases live in submodules; this file
//! holds the run loop, shared statics and the extern declarations.
pub use crate::types::*;
#[allow(unused_imports)]
use core::ffi::{c_char, c_double, c_float, c_int, c_uint, c_ulong, c_ulonglong, c_void};
mod arms;
mod atoms;
mod geometry;
mod glyphs;
mod grid;
mod io;
mod setup;
pub(crate) use self::arms::*;
pub(crate) use self::atoms::*;
pub use self::geometry::*;
pub(crate) use self::glyphs::*;
pub use self::grid::*;
pub use self::io::*;
pub use self::setup::*;
// Generated from pinned omsim by scripts/regenerate_engine.sh.
#[allow(unused_imports)]
use crate::flags::*;
unsafe extern "C" {
    unsafe fn calloc(__nmemb: usize, __size: usize) -> *mut c_void;
    unsafe fn realloc(__ptr: *mut c_void, __size: usize) -> *mut c_void;
    unsafe fn free(__ptr: *mut c_void);
    unsafe fn abort() -> !;
    unsafe fn qsort(__base: *mut c_void, __nmemb: usize, __size: usize, __compar: __compar_fn_t);
    unsafe fn abs(__x: i32) -> i32;
    unsafe fn memcpy(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
    unsafe fn memmove(__dest: *mut c_void, __src: *const c_void, __n: usize) -> *mut c_void;
    unsafe fn memset(__s: *mut c_void, __c: i32, __n: usize) -> *mut c_void;
    unsafe fn rs_collision(
        solution: *mut solution,
        board: *mut board,
        increment: f32,
        collision_location: *mut vector,
    ) -> bool;
    unsafe fn __assert_fail(
        __assertion: *const c_char,
        __file: *const c_char,
        __line: u32,
        __function: *const c_char,
    ) -> !;
    unsafe fn log(__x: f64) -> f64;
    unsafe fn pow(__x: f64, __y: f64) -> f64;
    unsafe fn round(__x: f64) -> f64;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct footprint {
    pub r#type: u32,
    pub hexes: *const vector,
}
static mut rs_GRABBING_EVERYTHING: u32 = 0;
static mut rs_ANY_GLYPH: u32 = (CALCIFICATION
    | ANIMISMUS
    | PROJECTION
    | DISPERSION
    | PURIFICATION
    | DUPLICATION
    | UNIFICATION
    | BONDING
    | UNBONDING
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
static mut rs_ANY_WHEEL: u32 = (VAN_BERLO | RAVARI) as u32;
static mut rs_zero_vector: vector = vector { u: 0, v: 0 };
unsafe extern "C" fn rs_report_collision(board: *mut board, p: vector, reason: *const c_char) {
    if (*board).rs_collision {
        return;
    }
    (*board).rs_collision = true;
    (*board).collision_location = p;
    (*board).collision_reason = reason;
}
unsafe extern "C" fn rs_reset_temporary_flags(board: *mut board) {
    let mut i: u32 = 0_u32;
    while i < (*board).flag_reset_length {
        **(*board).flag_reset.offset(i as isize) = (**(*board).flag_reset.offset(i as isize)
            & !(BEING_DROPPED | 0x3f_u64 << 29_i32))
            as atom;
        i = i.wrapping_add(1);
    }
    (*board).flag_reset_length = 0_u32;
    let mut i_0: u32 = 0_u32;
    while i_0 < (*board).number_of_overlapped_atoms {
        (*(*board).overlapped_atoms.offset(i_0 as isize)).atom =
            ((*(*board).overlapped_atoms.offset(i_0 as isize)).atom
                & !(BEING_DROPPED | 0x3f_u64 << 29_i32)) as atom;
        i_0 = i_0.wrapping_add(1);
    }
}
unsafe extern "C" fn rs_check_completion(solution: *mut solution, board: *mut board) {
    let mut min: u64 = 18446744073709551615_u64;
    let mut i: usize = 0_usize;
    while i < (*solution).number_of_inputs_and_outputs {
        let io: *mut input_output = (*solution).inputs_and_outputs.offset(i as isize);
        if (*io).r#type.0 & input_output_type::OUTPUT.0 != 0 {
            let count: u64 = (*io).number_of_outputs;
            if count < min {
                min = count;
            }
            if !((*io).output_cycles_end != -1_i32 as u64
                && (*board).cycle > (*io).output_cycles_end)
            {
                if count > (*io).output_cycles_capacity {
                    (*io).output_cycles_capacity = count.wrapping_mul(3_u64).wrapping_div(2_u64);
                    if (*io).output_cycles_capacity < 6_u64 {
                        (*io).output_cycles_capacity = 6_u64;
                    }
                    (*io).output_cycles = realloc(
                        (*io).output_cycles as *mut c_void,
                        ((*io).output_cycles_capacity as usize)
                            .wrapping_mul(::core::mem::size_of::<u64>()),
                    ) as *mut u64;
                }
                let mut j: u64 = (*io).number_of_output_cycles;
                while j < count {
                    *(*io).output_cycles.offset(j as isize) = (*board).cycle;
                    j = j.wrapping_add(1);
                }
                (*io).number_of_output_cycles = count;
            }
        }
        i = i.wrapping_add(1);
    }
    (*board).complete = min >= (*solution).target_number_of_outputs;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_run(solution: *mut solution, board: *mut board) {
    while (*board).half_cycle <= 2_i32 {
        if (*board).half_cycle == 2_i32 && (*board).number_of_overlapped_atoms > 0_u32 {
            rs_report_collision(
                board,
                (*(*board).overlapped_atoms.offset(0isize)).position,
                b"overlapping atoms before motion phase\0".as_ptr() as *const c_char,
            );
        }
        rs_perform_arm_instructions(solution, board);
        if (*board).half_cycle == 2_i32 && (*board).number_of_overlapped_atoms > 0_u32 {
            rs_report_collision(
                board,
                (*(*board).overlapped_atoms.offset(0isize)).position,
                b"overlapping atoms after motion phase\0".as_ptr() as *const c_char,
            );
        }
        rs_mark_arm_area(solution, board);
        rs_fill_conduits(solution, board);
        rs_flag_blocked_inputs(solution, board);
        rs_spawn_inputs(solution, board);
        rs_reset_temporary_flags(board);
        rs_apply_glyphs(solution, board);
        if (*board).number_of_overlapped_atoms > 0_u32 {
            rs_raise_overlapped_atoms(board);
        }
        rs_consume_outputs(solution, board);
        (*board).half_cycle += 1;
    }
    (*board).cycle = (*board).cycle.wrapping_add(1);
    (*board).half_cycle = 1_i32;
    (*board).number_of_atoms_being_produced = 0_u32;
    rs_check_completion(solution, board);
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    rs_GRABBING_EVERYTHING = 0x3f_u64.wrapping_mul(GRABBING_LOW_BIT) as u32;
}
#[used]
#[cfg_attr(target_os = "linux", unsafe(link_section = ".init_array"))]
#[cfg_attr(target_os = "windows", unsafe(link_section = ".CRT$XIB"))]
#[cfg_attr(target_os = "macos", unsafe(link_section = "__DATA,__mod_init_func"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
