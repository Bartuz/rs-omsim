//! Named bits of omsim's `atom` (u64) and `mechanism.type` (u32) words, from sim.h.
//! C2Rust had expanded every use into `(1 as u64) << N as i32`; each constant has
//! exactly the type and value of the expression it replaces.
use core::ffi::c_ulong;

// atom bits
/// exists (else the slot is empty)
pub const VALID: u64 = 1 << 0;
pub const SALT: u64 = 1 << 1;
pub const AIR: u64 = 1 << 2;
pub const EARTH: u64 = 1 << 3;
pub const FIRE: u64 = 1 << 4;
pub const WATER: u64 = 1 << 5;
pub const QUICKSILVER: u64 = 1 << 6;
pub const GOLD: u64 = 1 << 7;
pub const SILVER: u64 = 1 << 8;
pub const COPPER: u64 = 1 << 9;
pub const IRON: u64 = 1 << 10;
pub const TIN: u64 = 1 << 11;
pub const LEAD: u64 = 1 << 12;
pub const VITAE: u64 = 1 << 13;
pub const MORS: u64 = 1 << 14;
pub const REPEATING_OUTPUT_PLACEHOLDER: u64 = 1 << 15;
pub const QUINTESSENCE: u64 = 1 << 16;
/// another atom sits above this one (same bit as VARIABLE_OUTPUT, which only output atoms carry)
pub const OVERLAPS_ATOMS: u64 = 1 << 17;
/// output accepting any molecule of the right shape (computation puzzles)
pub const VARIABLE_OUTPUT: u64 = 1 << 17;
/// keeps its own bonds when unbonded together with an atom below
pub const DISJOINT_SAFE: u64 = 1 << 18;
/// just dropped; conduits only transport these
pub const BEING_DROPPED: u64 = 1 << 19;
/// part of a Van Berlo's or Ravari's wheel
pub const WHEEL_ATOM: u64 = 1 << 20;
/// motion tracked in the chain-atom side table
pub const IS_CHAIN_ATOM: u64 = 1 << 21;
/// grab count unit; bits 22..27 count simultaneous grabs
pub const GRABBED_ONCE: u64 = 1 << 22;
/// slot is a removed atom or a used-area marker
pub const REMOVED: u64 = 1 << 27;
/// removed because it moves this half-cycle; bits 29.. hold the movement index
pub const MOVED: u64 = 1 << 28;
/// recent-bond bits (29..35): bonds removed this half-cycle; direction d is RECENT_BOND0 << d
pub const RECENT_BOND0: u64 = 1 << 29;
/// normal bond in direction d: NORMAL_BOND0 << d
pub const NORMAL_BOND0: u64 = 1 << 35;
pub const TRIPLEX_R_BOND0: u64 = 1 << 41;
pub const TRIPLEX_Y_BOND0: u64 = 1 << 47;
pub const TRIPLEX_K_BOND0: u64 = 1 << 53;
/// marks atoms inside a conduit's region
pub const CONDUIT_SHAPE: u64 = 1 << 59;
/// molecule flood-fill mark
pub const VISITED: u64 = 1 << 60;
/// has a disjoint bond to look up in the table
pub const HAS_DISJOINT_BOND: u64 = 1 << 61;

// mechanism type bits (`1UL << N` in C, hence c_ulong)
pub const CALCIFICATION: c_ulong = 1 << 0;
pub const ANIMISMUS: c_ulong = 1 << 1;
pub const PROJECTION: c_ulong = 1 << 2;
pub const DISPERSION: c_ulong = 1 << 3;
pub const PURIFICATION: c_ulong = 1 << 4;
pub const DUPLICATION: c_ulong = 1 << 5;
pub const UNIFICATION: c_ulong = 1 << 6;
pub const BONDING: c_ulong = 1 << 7;
pub const UNBONDING: c_ulong = 1 << 8;
pub const TRIPLEX_BONDING: c_ulong = 1 << 9;
pub const MULTI_BONDING: c_ulong = 1 << 10;
pub const DISPOSAL: c_ulong = 1 << 11;
pub const EQUILIBRIUM: c_ulong = 1 << 12;
pub const CONDUIT: c_ulong = 1 << 13;
pub const ARM: c_ulong = 1 << 14;
pub const TWO_ARM: c_ulong = 1 << 15;
pub const THREE_ARM: c_ulong = 1 << 16;
pub const SIX_ARM: c_ulong = 1 << 17;
pub const PISTON: c_ulong = 1 << 18;
pub const VAN_BERLO: c_ulong = 1 << 19;
pub const RAVARI: c_ulong = 1 << 20;
pub const GRABBING_LOW_BIT: c_ulong = 1 << 21;
pub const GRABBING: c_ulong = 1 << 27;
pub const MOVED_GRABBED_ATOMS: c_ulong = 1 << 28;
pub const PROLIFERATION: c_ulong = 1 << 29;
pub const DIVISION: c_ulong = 1 << 30;
pub const REJECTION: c_ulong = 1 << 31;
