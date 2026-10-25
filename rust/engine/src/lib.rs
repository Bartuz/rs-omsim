//! Native Rust omsim engine, with profiled optimizations over the C2Rust baseline.
//! No omsim C code is linked. This is not a memory-safety rewrite.
//! All inter-module omsim symbols use `rs_`; only system libc/libm are external.
#![allow(clippy::all, deprecated, unused_variables)]
#![allow(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals
)]
#![allow(unsafe_op_in_unsafe_fn, unused_assignments, unused_mut)]
#![allow(improper_ctypes, unused_parens, unused_unsafe)]
#![allow(clashing_extern_declarations)]
mod board_index;
mod glyph_plan;
mod trig_memo;
pub mod collision;
mod collision_math;
pub mod decode;
pub mod ffwd;
pub mod flags;
pub mod sweep;
pub mod parse;
pub mod sim;
pub mod steady_state;
pub mod verifier;

pub mod types;
