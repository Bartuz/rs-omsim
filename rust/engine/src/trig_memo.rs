//! Memo for the trig in `collision::movement_transform`. Its angles come from a small set
//! (60° rotations, fixed substep progress values), and glibc `cos`/`sin` are pure
//! functions of their input (the rounding mode is never changed), so a table keyed by the
//! f32 angle's bits returns the same values as the calls it replaces.

use core::cell::UnsafeCell;

unsafe extern "C" {
    unsafe fn cos(__x: f64) -> f64;
    unsafe fn sin(__x: f64) -> f64;
}


const BITS: u32 = 9;

#[derive(Clone, Copy)]
struct Entry {
    /// The angle's bits with bit 32 set, so a zeroed entry matches no angle.
    key: u64,
    cos: f32,
    sin: f32,
}

thread_local! {
    static TABLE: UnsafeCell<[Entry; 1 << BITS]> =
        const { UnsafeCell::new([Entry { key: 0, cos: 0.0, sin: 0.0 }; 1 << BITS]) };
}

/// `(cos(angle as f64) as f32, sin(angle as f64) as f32)`, as movement_transform computes it.
#[inline(always)]
pub unsafe fn cos_sin(angle: f32) -> (f32, f32) {
    memo(angle)
}

#[inline(always)]
unsafe fn direct(angle: f32) -> (f32, f32) {
    (cos(angle as f64) as f32, sin(angle as f64) as f32)
}

#[inline(always)]
fn slot(angle: f32) -> usize {
    (angle.to_bits().wrapping_mul(0x9E37_79B1) >> (32 - BITS)) as usize
}

#[inline(always)]
unsafe fn memo(angle: f32) -> (f32, f32) {
    let key = angle.to_bits() as u64 | 1 << 32;
    TABLE.with(|t| {
        let e = &mut (*t.get())[slot(angle)];
        if e.key != key {
            let (cos, sin) = direct(angle);
            *e = Entry { key, cos, sin };
        }
        (e.cos, e.sin)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memo_returns_the_bits_of_direct_libm_calls() {
        unsafe {
            let sixty = core::f32::consts::PI / 3.0;
            let mut angles = vec![0.0f32, -0.0, f32::MIN_POSITIVE / 4.0, 1e30, -1e30, f32::NAN,
                f32::INFINITY, f32::NEG_INFINITY];
            // Movement angles: base rotation − arm rotation · (1 − progress), at every substep.
            for steps in [8u32, 16, 128] {
                let increment = 1.0f32 / steps as f32;
                let mut progress = increment;
                while progress < 1.0 {
                    for base in -6..=6 {
                        for arm in -2..=2 {
                            let (b, a) = (base as f32 * sixty, arm as f32 * sixty);
                            angles.push(b - a * (1.0 - progress));
                            angles.push((0.0 - a) * (1.0 - progress));
                        }
                    }
                    progress += increment;
                }
            }
            // Two angles that share a table entry, alternating, must not leak into each other.
            let k = slot(angles[9]);
            let other = (1..u32::MAX)
                .map(|i| f32::from_bits(angles[9].to_bits() ^ i))
                .find(|&x| slot(x) == k)
                .unwrap();
            angles.extend([other, angles[9], other]);
            for _ in 0..2 {
                for &x in &angles {
                    let (c, s) = memo(x);
                    let (dc, ds) = direct(x);
                    assert_eq!((c.to_bits(), s.to_bits()), (dc.to_bits(), ds.to_bits()), "{x:e}");
                }
            }
        }
    }
}
