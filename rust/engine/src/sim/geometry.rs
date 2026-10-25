//! Hex-grid vector helpers, directions, bond-bit rotation and motion arithmetic.
use super::*;

#[inline]
pub(crate) unsafe extern "C" fn rs_vectors_equal(a: vector, b: vector) -> bool {
    a.u == b.u && a.v == b.v
}
#[inline]
pub(crate) unsafe extern "C" fn rs_vector_hash(p: vector) -> u32 {
    let mut hash: u64 = 0x542ddeaec5c75e0f_u64;
    hash = hash.wrapping_add(0xc0c594dd042705fb_u64.wrapping_mul(p.u as u32 as u64));
    hash = hash.wrapping_add(0x93b45776c46130c1_u64.wrapping_mul(p.v as u32 as u64));
    (hash >> 32_i32) as u32
}
#[inline]
pub(crate) unsafe extern "C" fn rs_vector_hexicab_length(p: vector) -> u32 {
    ((abs(p.u) + abs(p.v) + abs(p.u + p.v)) / 2_i32) as u32
}
#[inline]
pub(crate) unsafe extern "C" fn rs_position_may_be_visible_to_solution(
    solution: *mut solution,
    p: vector,
) -> bool {
    p.u >= (*solution).min_visible_u
        && p.u <= (*solution).max_visible_u
        && p.v >= (*solution).min_visible_v
        && p.v <= (*solution).max_visible_v
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_mechanism_relative_position(
    m: mechanism,
    du: i32,
    dv: i32,
    w: i32,
) -> vector {
    vector {
        u: m.direction_u.u * du + m.direction_v.u * dv + m.position.u * w,
        v: m.direction_u.v * du + m.direction_v.v * dv + m.position.v * w,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_position_within_mechanism(
    m: mechanism,
    u: i32,
    v: i32,
    w: i32,
) -> vector {
    vector {
        u: (u - m.position.u * w) * m.direction_v.v - (v - m.position.v * w) * m.direction_v.u,
        v: (v - m.position.v * w) * m.direction_u.u - (u - m.position.u * w) * m.direction_u.v,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_polymer_position_from_global_position(
    io: *mut input_output,
    p: vector,
) -> vector {
    vector {
        u: (*io).repetition_direction_v.v * (p.u - (*io).repetition_origin.u)
            - (*io).repetition_direction_v.u * (p.v - (*io).repetition_origin.v),
        v: -(*io).repetition_direction_u.v * (p.u - (*io).repetition_origin.u)
            + (*io).repetition_direction_u.u * (p.v - (*io).repetition_origin.v),
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_direction_for_offset(d: vector) -> i32 {
    if d.u == 1_i32 && d.v == 0_i32 {
        return 0_i32;
    } else if d.u == 0_i32 && d.v == 1_i32 {
        return 1_i32;
    } else if d.u == -1_i32 && d.v == 1_i32 {
        return 2_i32;
    } else if d.u == -1_i32 && d.v == 0_i32 {
        return 3_i32;
    } else if d.u == 0_i32 && d.v == -1_i32 {
        return 4_i32;
    } else if d.u == 1_i32 && d.v == -1_i32 {
        return 5_i32;
    }
    -1_i32
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_angular_distance_between_grabbers(mechanism_type: u32) -> i32 {
    match mechanism_type & rs_ANY_ARM {
        16384 | 262144 => 6_i32,
        32768 => 3_i32,
        65536 => 2_i32,
        131072 | 524288 | 1048576 | _ => 1_i32,
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_bond_direction(m: mechanism, du: i32, dv: i32) -> atom {
    let base: atom =
        (RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0) as atom;
    let dir: i32 = rs_direction_for_offset(rs_mechanism_relative_position(m, du, dv, 0_i32));
    if dir < 0_i32 { 0 as atom } else { base << dir }
}
pub(crate) unsafe extern "C" fn rs_normalize_direction(mut direction: i32) -> i32 {
    direction %= 6_i32;
    if direction < 0_i32 {
        direction + 6_i32
    } else {
        direction
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_rotate_bond_bits(mut bond_bits: atom, mut rotation: i32) -> atom {
    rotation = rs_normalize_direction(rotation);
    if rotation == 0_i32 {
        return bond_bits;
    }
    bond_bits = (bond_bits
        & 0x3f_u64.wrapping_mul(
            RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0,
        )) as atom;
    bond_bits <<= rotation;
    let mask: atom = (0x3f_u64 >> 6_i32 - rotation) as atom;
    let overflow: atom = (bond_bits
        & (mask as u64).wrapping_mul(
            RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0,
        ) << 6_i32) as atom;
    bond_bits &= !overflow;
    bond_bits |= overflow >> 6_i32;
    bond_bits
}
pub(crate) unsafe extern "C" fn rs_rotate_bonds(a: *mut atom, rotation: i32) {
    let bond_bits: atom = rs_rotate_bond_bits(
        (*a & 0x3f_u64.wrapping_mul(
            RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0,
        )) as atom,
        rotation,
    );
    *a = (*a
        & !0x3f_u64.wrapping_mul(
            RECENT_BOND0 | NORMAL_BOND0 | TRIPLEX_R_BOND0 | TRIPLEX_Y_BOND0 | TRIPLEX_K_BOND0,
        )) as atom;
    *a |= bond_bits;
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_u_offset_for_direction(direction: i32) -> vector {
    match rs_normalize_direction(direction) {
        0 => vector { u: 1_i32, v: 0_i32 },
        1 => vector { u: 0_i32, v: 1_i32 },
        2 => vector {
            u: -1_i32,
            v: 1_i32,
        },
        3 => vector {
            u: -1_i32,
            v: 0_i32,
        },
        4 => vector {
            u: 0_i32,
            v: -1_i32,
        },
        5 => vector {
            u: 1_i32,
            v: -1_i32,
        },
        _ => {
            abort();
        }
    }
}
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rs_v_offset_for_direction(direction: i32) -> vector {
    rs_u_offset_for_direction(direction + 1_i32)
}
pub(crate) unsafe extern "C" fn rs_adjust_axis_magnitude(p: *mut vector, delta: i32) {
    if (*p).u > 0_i32 {
        (*p).u += delta;
    }
    if (*p).u < 0_i32 {
        (*p).u -= delta;
    }
    if (*p).v > 0_i32 {
        (*p).v += delta;
    }
    if (*p).v < 0_i32 {
        (*p).v -= delta;
    }
}
pub(crate) unsafe extern "C" fn rs_normalize_axis(mut p: vector) -> vector {
    if p.u > 0_i32 {
        p.u = 1_i32;
    }
    if p.u < 0_i32 {
        p.u = -1_i32;
    }
    if p.v > 0_i32 {
        p.v = 1_i32;
    }
    if p.v < 0_i32 {
        p.v = -1_i32;
    }
    p
}
pub(crate) unsafe extern "C" fn rs_apply_movement_to_position(
    base: vector,
    translation: vector,
    u: vector,
    v: vector,
    position: *mut vector,
) {
    let mut delta: vector = *position;
    delta.u -= base.u;
    delta.v -= base.v;
    *position = vector {
        u: u.u * delta.u + v.u * delta.v + base.u + translation.u,
        v: u.v * delta.u + v.v * delta.v + base.v + translation.v,
    };
}
pub(crate) unsafe extern "C" fn rs_range_of_periods_where_motion_is_in_interval(
    mut motion: i32,
    mut interval_min: i32,
    mut interval_max: i32,
    entry_period: *mut i32,
    exit_period: *mut i32,
) -> bool {
    if motion == 0_i32 {
        *entry_period = -2147483647_i32 - 1_i32;
        *exit_period = 2147483647_i32;
        return 0_i32 >= interval_min && 0_i32 <= interval_max;
    } else if motion < 0_i32 {
        motion = -motion;
        let tmp: i32 = interval_max;
        interval_max = -interval_min;
        interval_min = -tmp;
    }
    if interval_max < 0_i32 {
        return false;
    }
    *entry_period = (interval_min + motion - 1_i32) / motion;
    *exit_period = interval_max / motion;
    *entry_period <= *exit_period
}
pub(crate) unsafe extern "C" fn rs_position_may_eventually_be_visible_to_solution(
    solution: *mut solution,
    original: vector,
    current: vector,
) -> bool {
    let mut u_entry: i32 = 0;
    let mut u_exit: i32 = 0;
    let mut v_entry: i32 = 0;
    let mut v_exit: i32 = 0;
    if !rs_range_of_periods_where_motion_is_in_interval(
        current.u - original.u,
        (*solution).min_visible_u - original.u,
        (*solution).max_visible_u - original.u,
        &raw mut u_entry,
        &raw mut u_exit,
    ) {
        return false;
    }
    if !rs_range_of_periods_where_motion_is_in_interval(
        current.v - original.v,
        (*solution).min_visible_v - original.v,
        (*solution).max_visible_v - original.v,
        &raw mut v_entry,
        &raw mut v_exit,
    ) {
        return false;
    }
    v_entry <= u_exit && u_entry <= v_exit
}
pub(crate) unsafe extern "C" fn rs_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0_i32 {
        let t: i32 = b;
        b = a % b;
        a = t;
    }
    a
}
