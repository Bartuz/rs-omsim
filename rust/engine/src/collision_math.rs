//! Handwritten, safe helpers for the optimized collision engine.
//! Boundary behavior is part of the compatibility contract with the C oracle.

// Handwritten optimization: the input is f32, so widening before adding a
// signed half is exact throughout the i32 range. The cast truncates toward zero
// and saturates, matching round(f64::from(value)) as i32, including NaN/infinity.
// Adding the half in f32 instead would incorrectly round values just below .5.
#[inline]
pub(crate) fn round_hex_component(value: f32) -> i32 {
    let wide = f64::from(value);
    (wide + 0.5f64.copysign(wide)) as i32
}

#[cfg(test)]
mod optimization_tests {
    use super::{external_collider_indices, round_hex_component};

    #[test]
    fn external_colliders_keep_original_diagnostic_order() {
        for length in 0..32 {
            for start in 0..40 {
                for end in 0..40 {
                    let original: Vec<_> =
                        (0..length).filter(|&i| !(i >= start && i < end)).collect();
                    assert_eq!(
                        external_collider_indices(length, start, end).collect::<Vec<_>>(),
                        original
                    );
                }
            }
        }
        // This case must produce two checks, independent of polymer size.
        assert_eq!(
            external_collider_indices(1_000_002, 1, 1_000_001).collect::<Vec<_>>(),
            [0, 1_000_001]
        );
    }

    #[test]
    fn hex_rounding_matches_double_round_then_saturating_cast() {
        let check = |value: f32| {
            assert_eq!(
                round_hex_component(value),
                f64::from(value).round() as i32,
                "value={value:?}, bits={:08x}",
                value.to_bits()
            );
        };
        // Exact ties, adjacent representable values, sign, subnormals,
        // saturation boundaries, and every exponent (including NaN/infinity).
        for center in [
            0.0f32,
            0.5,
            1.5,
            31.5,
            127.5,
            65535.5,
            8388607.5,
            i32::MAX as f32,
            i32::MIN as f32,
            f32::MAX,
            f32::INFINITY,
        ] {
            for offset in -16i64..=16 {
                let bits = i64::from(center.to_bits()) + offset;
                if let Ok(bits) = u32::try_from(bits) {
                    check(f32::from_bits(bits));
                    check(-f32::from_bits(bits));
                }
            }
        }
        let mut bits = 0x542d_dead_u32;
        for _ in 0..1_000_000 {
            bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
            check(f32::from_bits(bits));
        }
    }

    #[test]
    fn squared_thresholds_match_rounded_sqrt_at_collision_boundaries() {
        use super::squared_distance_is_less_than;
        for radius in [30.0f32, 35.0, 40.0, 44.0, 49.0, 58.0] {
            let check = |squared: f32| {
                assert_eq!(
                    squared_distance_is_less_than(squared, radius),
                    (f64::from(squared).sqrt() as f32) < radius,
                    "squared={squared:?}, radius={radius}"
                );
            };
            // Both monotone predicates must change at the same representable
            // squared distance, including the one-ULP difference from r*r.
            for offset in -256i64..=256 {
                check(f32::from_bits(
                    ((radius * radius).to_bits() as i64 + offset) as u32,
                ));
            }
            let mut bits = 0x542d_dead_u32;
            for _ in 0..1_000_000 {
                bits = bits.wrapping_mul(1664525).wrapping_add(1013904223);
                check(f32::from_bits(bits & 0x7fff_ffff));
            }
            check(-0.0);
        }
        // Naive d*d < r*r changes the original contact boundary for r=44.
        let just_below = f32::from_bits((44.0f32 * 44.0).to_bits() - 1);
        assert!(just_below < 44.0 * 44.0);
        assert!(!((f64::from(just_below).sqrt() as f32) < 44.0));
        assert!(!squared_distance_is_less_than(just_below, 44.0));
    }
}

// All callers supply a sum of squares (nonnegative or NaN). The six contact
// radii come from the engine's three collider radii: 15, 20, 29. Compare against
// the first f32 squared distance whose rounded sqrt reaches that radius.
// For 35, 40, and 44 this is ONE ULP BELOW r*r: blindly squaring the radius
// changes collision semantics. This helper's domain is the engine's six fixed
// contact radii; debug builds catch extensions that need a new boundary proof.
// Keep the expression branchless so collision-pair loops can still vectorize.
#[inline(always)]
pub(crate) fn contact_distance_cutoff(radius: f32) -> f32 {
    debug_assert!([30.0, 35.0, 40.0, 44.0, 49.0, 58.0].contains(&radius));
    let adjust = (radius == 35.0) | (radius == 40.0) | (radius == 44.0);
    f32::from_bits((radius * radius).to_bits() - u32::from(adjust))
}

#[inline(always)]
pub(crate) fn squared_distance_is_less_than(squared: f32, radius: f32) -> bool {
    squared < contact_distance_cutoff(radius)
}

#[inline]
pub(crate) fn external_collider_indices(
    length: usize,
    movement_start: usize,
    movement_end: usize,
) -> impl Iterator<Item = usize> {
    let start = movement_start.min(length);
    let end = movement_end.min(length).max(start);
    (0..start).chain(end..length)
}
