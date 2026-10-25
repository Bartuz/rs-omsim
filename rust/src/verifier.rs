//! A safe handle over the `verifier_*` API (see `verifier.h`), for either engine.
use std::{
    ffi::{CStr, CString, c_char, c_double, c_int, c_void},
    marker::PhantomData,
    ptr::NonNull,
    rc::Rc,
};

/// The `verifier_*` entry points of one engine.
pub struct Engine {
    pub name: &'static str,
    create: unsafe extern "C" fn(*const c_char, c_int, *const c_char, c_int) -> *mut c_void,
    destroy: unsafe extern "C" fn(*mut c_void),
    error: unsafe extern "C" fn(*mut c_void) -> *const c_char,
    error_cycle: unsafe extern "C" fn(*mut c_void) -> c_int,
    error_location_u: unsafe extern "C" fn(*mut c_void) -> c_int,
    error_location_v: unsafe extern "C" fn(*mut c_void) -> c_int,
    error_clear: unsafe extern "C" fn(*mut c_void),
    set_cycle_limit: unsafe extern "C" fn(*mut c_void, c_int),
    evaluate_metric: unsafe extern "C" fn(*mut c_void, *const c_char) -> c_int,
    evaluate_approximate_metric: unsafe extern "C" fn(*mut c_void, *const c_char) -> c_double,
}

/// The Rust engine in `engine/`.
pub static RUST: Engine = {
    use omsim_native::verifier::*;
    Engine {
        name: "rust",
        create: rs_verifier_create_from_bytes,
        destroy: rs_verifier_destroy,
        error: rs_verifier_error,
        error_cycle: rs_verifier_error_cycle,
        error_location_u: rs_verifier_error_location_u,
        error_location_v: rs_verifier_error_location_v,
        error_clear: rs_verifier_error_clear,
        set_cycle_limit: rs_verifier_set_cycle_limit,
        evaluate_metric: rs_verifier_evaluate_metric,
        evaluate_approximate_metric: rs_verifier_evaluate_approximate_metric,
    }
};

#[cfg(feature = "reference")]
mod c {
    use super::*;
    unsafe extern "C" {
        pub fn verifier_create_from_bytes(
            p: *const c_char,
            pn: c_int,
            s: *const c_char,
            sn: c_int,
        ) -> *mut c_void;
        pub fn verifier_destroy(v: *mut c_void);
        pub fn verifier_error(v: *mut c_void) -> *const c_char;
        pub fn verifier_error_cycle(v: *mut c_void) -> c_int;
        pub fn verifier_error_location_u(v: *mut c_void) -> c_int;
        pub fn verifier_error_location_v(v: *mut c_void) -> c_int;
        pub fn verifier_error_clear(v: *mut c_void);
        pub fn verifier_set_cycle_limit(v: *mut c_void, limit: c_int);
        pub fn verifier_evaluate_metric(v: *mut c_void, metric: *const c_char) -> c_int;
        pub fn verifier_evaluate_approximate_metric(
            v: *mut c_void,
            metric: *const c_char,
        ) -> c_double;
    }
}

/// The C at the root of the repository, compiled in by `build.rs`.
#[cfg(feature = "reference")]
pub static C: Engine = {
    use c::*;
    Engine {
        name: "c",
        create: verifier_create_from_bytes,
        destroy: verifier_destroy,
        error: verifier_error,
        error_cycle: verifier_error_cycle,
        error_location_u: verifier_error_location_u,
        error_location_v: verifier_error_location_v,
        error_clear: verifier_error_clear,
        set_cycle_limit: verifier_set_cycle_limit,
        evaluate_metric: verifier_evaluate_metric,
        evaluate_approximate_metric: verifier_evaluate_approximate_metric,
    }
};

/// An error, formatted as `text @cycle (u,v)` so two engines' errors compare as strings.
pub type Error = String;

/// Owns one verifier. Neither Send nor Sync: omsim has not been audited for concurrent
/// calls. The engine copies the input buffers.
pub struct Verifier {
    engine: &'static Engine,
    ptr: NonNull<c_void>,
    _not_send_sync: PhantomData<Rc<()>>,
}

impl Verifier {
    /// Checks both files with the parser in `parse.rs` before the engine sees them. This
    /// guards the engine's own parser, not the decoder or the simulation.
    pub fn new(engine: &'static Engine, puzzle: &[u8], solution: &[u8]) -> Result<Self, Error> {
        crate::parse::puzzle(puzzle).map_err(|e| format!("puzzle: {e}"))?;
        crate::parse::solution(solution).map_err(|e| format!("solution: {e}"))?;
        let pn = c_int::try_from(puzzle.len()).map_err(|_| "puzzle too large")?;
        let sn = c_int::try_from(solution.len()).map_err(|_| "solution too large")?;
        // SAFETY: the buffers are valid for the call, and the engine copies them.
        let ptr = unsafe {
            (engine.create)(puzzle.as_ptr().cast(), pn, solution.as_ptr().cast(), sn)
        };
        let verifier = Self {
            engine,
            ptr: NonNull::new(ptr).ok_or("verifier allocation failed")?,
            _not_send_sync: PhantomData,
        };
        verifier.check()?;
        Ok(verifier)
    }

    fn check(&self) -> Result<(), Error> {
        let (e, p) = (self.engine, self.ptr.as_ptr());
        // SAFETY: p is live until Drop. The error is NULL or a NUL-terminated string the
        // engine owns; the other calls read plain fields of the current error.
        unsafe {
            let error = (e.error)(p);
            if error.is_null() {
                return Ok(());
            }
            Err(format!(
                "{} @{} ({},{})",
                CStr::from_ptr(error).to_string_lossy(),
                (e.error_cycle)(p),
                (e.error_location_u)(p),
                (e.error_location_v)(p)
            ))
        }
    }

    pub fn set_cycle_limit(&mut self, limit: i32) -> Result<(), Error> {
        if limit <= 0 {
            return Err("cycle limit must be positive".into());
        }
        // SAFETY: uniquely borrowed, live verifier.
        unsafe { (self.engine.set_cycle_limit)(self.ptr.as_ptr(), limit) };
        Ok(())
    }

    /// Evaluates one metric. A failed metric leaves no error behind for the next one.
    pub fn metric(&mut self, name: &str) -> Result<i32, Error> {
        let name = CString::new(name).map_err(|_| "metric contains a NUL byte")?;
        // SAFETY: live verifier and a NUL-terminated name, valid for the call.
        let value = unsafe { (self.engine.evaluate_metric)(self.ptr.as_ptr(), name.as_ptr()) };
        self.check_and_clear().map(|()| value)
    }

    /// Evaluates one metric as a double (`verifier_evaluate_approximate_metric`).
    pub fn approximate_metric(&mut self, name: &str) -> Result<f64, Error> {
        let name = CString::new(name).map_err(|_| "metric contains a NUL byte")?;
        // SAFETY: as in `metric`.
        let value = unsafe {
            (self.engine.evaluate_approximate_metric)(self.ptr.as_ptr(), name.as_ptr())
        };
        self.check_and_clear().map(|()| value)
    }

    fn check_and_clear(&mut self) -> Result<(), Error> {
        let result = self.check();
        // SAFETY: live verifier.
        unsafe { (self.engine.error_clear)(self.ptr.as_ptr()) };
        result
    }
}

impl Drop for Verifier {
    fn drop(&mut self) {
        // SAFETY: this handle owns the pointer, so it is destroyed exactly once.
        unsafe { (self.engine.destroy)(self.ptr.as_ptr()) };
    }
}
