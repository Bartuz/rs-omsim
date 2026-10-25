//! omsim in Rust. The simulator is the `omsim-native` crate in `engine/`; this crate adds
//! a checked file parser, a safe verifier handle, and the `omsim` and `run-tests`
//! binaries (ports of `main.c` and `run-tests.c`). With the `reference` feature the same
//! handle also drives the C at the root of the repository, for differential tests.
pub mod parse;
pub mod verifier;

pub use verifier::{Engine, Verifier};
