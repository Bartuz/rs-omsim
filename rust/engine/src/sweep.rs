//! Per-atom swept-cell cache for the motion-phase board check.
//!
//! The original marks area and checks the board for each sampled atom position by
//! rounding the position to its hex and testing the seven cells around it. Samples are
//! dense (the step shrinks as the rotation radius grows), so consecutive samples of one
//! atom keep landing in the same hex and re-test the same cells. This cache keeps, per
//! moving atom, its current hex, the seven cell centres around it, and which of the
//! seven already passed the test (and were marked/checked). Exactness:
//! - a sample within `SAME_HEX` of a cell centre rounds to that cell (the disk of radius
//!   ~40.98 lies inside the hex cell; the margin covers float error), so `from_xy` is
//!   only called in hex corners;
//! - every distance test is the original float test (same operands, same operation
//!   order) on the original sample, done for all seven cells at once with SSE;
//! - a cell is marked once per atom: marking is idempotent and the board cannot change
//!   during the motion check.
//! So the set of swept cells is the original one. Cells found to hold an atom stay
//! "hot" and are re-tested on every sample, so collision events (and hence the last
//! reported location) occur at the same points as in the original.

thread_local! {
    /// Buffers reused across motion checks (one per half-cycle).
    pub static SCRATCH: std::cell::RefCell<Sweep> = std::cell::RefCell::new(Sweep::new(0));
}

const NEIGHBORS: [(i32, i32); 7] = [(0, 0), (1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)];
const SAME_HEX2: f32 = 40.4 * 40.4;

/// Per atom: current hex, the seven cell centres around it (lane 7 is padding that
/// never passes), and which of the seven already passed and were processed.
#[derive(Clone, Copy)]
struct AtomState {
    u: i32,
    v: i32,
    cx: [f32; 8],
    cy: [f32; 8],
    mask: u8,
    /// Processed cells that hold a non-removed atom: re-tested on every sample, because
    /// the original raises a collision event at every sample that reaches them.
    hot: u8,
    valid: bool,
}

impl Default for AtomState {
    fn default() -> Self {
        Self {
            u: 0,
            v: 0,
            cx: [f32::INFINITY; 8],
            cy: [f32::INFINITY; 8],
            mask: 0,
            hot: 0,
            valid: false,
        }
    }
}

impl AtomState {
    #[inline(always)]
    fn set_hex(&mut self, u: i32, v: i32, to_xy: &impl Fn(i32, i32) -> (f32, f32)) {
        self.u = u;
        self.v = v;
        for (i, &(du, dv)) in NEIGHBORS.iter().enumerate() {
            let (x, y) = to_xy(u + du, v + dv);
            self.cx[i] = x;
            self.cy[i] = y;
        }
        self.mask = 0;
        self.hot = 0;
        self.valid = true;
    }

    /// Bitmasks over the 7 cells: squared distance (original operation order:
    /// (x-cx)^2 + (y-cy)^2 in f32) below `cutoff`, and below SAME_HEX2.
    #[inline(always)]
    fn masks(&self, x: f32, y: f32, cutoff: f32) -> (u32, u32) {
        #[cfg(target_arch = "x86_64")]
        unsafe {
            use core::arch::x86_64::*;
            let (xv, yv) = (_mm_set1_ps(x), _mm_set1_ps(y));
            let d = |k: usize| {
                let ex = _mm_sub_ps(xv, _mm_loadu_ps(self.cx.as_ptr().add(k)));
                let ey = _mm_sub_ps(yv, _mm_loadu_ps(self.cy.as_ptr().add(k)));
                _mm_add_ps(_mm_mul_ps(ex, ex), _mm_mul_ps(ey, ey))
            };
            let (d0, d1) = (d(0), d(4));
            let (c, h) = (_mm_set1_ps(cutoff), _mm_set1_ps(SAME_HEX2));
            let pass = _mm_movemask_ps(_mm_cmplt_ps(d0, c)) | _mm_movemask_ps(_mm_cmplt_ps(d1, c)) << 4;
            let inside = _mm_movemask_ps(_mm_cmplt_ps(d0, h)) | _mm_movemask_ps(_mm_cmplt_ps(d1, h)) << 4;
            (pass as u32, inside as u32)
        }
        #[cfg(not(target_arch = "x86_64"))]
        {
            let (mut pass, mut inside) = (0u32, 0u32);
            for i in 0..8 {
                let ex = x - self.cx[i];
                let ey = y - self.cy[i];
                let d = ex * ex + ey * ey;
                pass |= ((d < cutoff) as u32) << i;
                inside |= ((d < SAME_HEX2) as u32) << i;
            }
            (pass, inside)
        }
    }
}

/// Pairwise prefilter state for one motion check (filled by collision.rs). Each moving
/// atom's samples lie in its bounding box; a collider whose disk cannot reach that box
/// (radius sum plus a 1 px margin, far above float error) fails every distance test.
#[derive(Default)]
pub struct Pairs {
    /// (v.x, v.y, rx, ry) per substep and movement, as computed by the substep loop.
    pub transforms: Vec<(f32, f32, f32, f32)>,
    pub movements: usize,
    /// Fixed colliders occupy [0, fixed_end) of the collider list.
    pub fixed_end: usize,
    /// Per atom: fixed colliders within reach, as a range into `candidate_list`.
    pub candidates: Vec<(u32, u32)>,
    pub candidate_list: Vec<u32>,
    /// may_touch[j * movements + i]: movements j and i can come within contact range.
    pub may_touch: Vec<bool>,
    /// Start of each movement's colliders in the current substep.
    pub starts: Vec<usize>,
    /// Scratch: sample bounding boxes per atom and per movement.
    pub atom_boxes: Vec<[f32; 4]>,
    pub movement_boxes: Vec<[f32; 4]>,
}

pub struct Sweep {
    atoms: Vec<AtomState>,
    pub board_collision: bool,
    pub pairs: Pairs,
}

impl Sweep {
    pub fn new(atoms: usize) -> Self {
        Self { atoms: vec![AtomState::default(); atoms], board_collision: false, pairs: Pairs::default() }
    }

    /// Reuses the buffers for a new motion check with `atoms` moving atoms.
    pub fn reset(&mut self, atoms: usize) {
        self.atoms.clear();
        self.atoms.resize(atoms, AtomState::default());
        self.board_collision = false;
    }

    #[inline(always)]
    pub fn transform(&self, step: usize, movement: usize) -> (f32, f32, f32, f32) {
        self.pairs.transforms[step * self.pairs.movements + movement]
    }

    #[inline(always)]
    pub fn begin_movement(&mut self, movement: usize, start: usize) {
        self.pairs.starts[movement] = start;
    }

    #[inline(always)]
    pub fn fixed_candidates(&self, atom: usize) -> &[u32] {
        let (a, b) = self.pairs.candidates[atom];
        &self.pairs.candidate_list[a as usize..b as usize]
    }

    #[inline(always)]
    pub fn fixed_end(&self) -> usize {
        self.pairs.fixed_end
    }

    #[inline(always)]
    pub fn movement_start(&self, movement: usize) -> usize {
        self.pairs.starts[movement]
    }

    /// Colliders of movement `j` (< the current one) in the current substep.
    #[inline(always)]
    pub fn movement_range(&self, j: usize) -> std::ops::Range<usize> {
        self.pairs.starts[j]..self.pairs.starts[j + 1]
    }

    #[inline(always)]
    pub fn movements_may_touch(&self, j: usize, i: usize) -> bool {
        self.pairs.may_touch[j * self.pairs.movements + i]
    }

    /// One sample of atom `atom` at (x, y). `process(u, v)` marks the cell and returns
    /// whether a non-removed atom is there; `event(u, v)` is called, in the original's
    /// neighbour order, for every cell this sample reaches that holds an atom (exactly
    /// where the original reports a collision). `to_xy`/`from_xy` are the engine's.
    #[inline(always)]
    #[allow(clippy::too_many_arguments)]
    pub fn sample(
        &mut self,
        atom: usize,
        x: f32,
        y: f32,
        cutoff: f32,
        process: &mut impl FnMut(i32, i32) -> bool,
        event: &mut impl FnMut(i32, i32),
        to_xy: &impl Fn(i32, i32) -> (f32, f32),
        from_xy: &impl Fn(f32, f32) -> (i32, i32),
    ) {
        let st = &mut self.atoms[atom];
        if !st.valid {
            let (u, v) = from_xy(x, y);
            st.set_hex(u, v, to_xy);
        }
        let (mut pass, inside) = st.masks(x, y, cutoff);
        if inside & 1 == 0 {
            // Left the inscribed disk: usually into a neighbour's disk, which determines
            // from_xy without rounding; otherwise (hex corners) round as the original.
            let n = inside & 0x7e;
            let (u, v) = if n != 0 {
                let i = n.trailing_zeros() as usize;
                (st.u + NEIGHBORS[i].0, st.v + NEIGHBORS[i].1)
            } else {
                from_xy(x, y)
            };
            if u != st.u || v != st.v {
                st.set_hex(u, v, to_xy);
                pass = st.masks(x, y, cutoff).0;
            }
        }
        let pass = (pass & 0x7f) as u8;
        let mut new = pass & !st.mask;
        if new == 0 && pass & st.hot == 0 {
            return;
        }
        st.mask |= new;
        let (u, v) = (st.u, st.v);
        while new != 0 {
            let i = new.trailing_zeros() as usize;
            new &= new - 1;
            if process(u + NEIGHBORS[i].0, v + NEIGHBORS[i].1) {
                st.hot |= 1 << i;
                self.board_collision = true;
            }
        }
        let mut events = pass & st.hot;
        while events != 0 {
            let i = events.trailing_zeros() as usize;
            events &= events - 1;
            event(u + NEIGHBORS[i].0, v + NEIGHBORS[i].1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::collision::{from_xy_for_tests as from_xy, to_xy_for_tests as to_xy};

    #[test]
    fn same_hex_disk_rounds_to_its_hex() {
        // Checks the SAME_HEX claim against the engine's from_xy on hexes near and far
        // from the origin.
        let mut seed = 12345u64;
        let mut rnd = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 33) as f32 / (1u64 << 31) as f32
        };
        for &(u, v) in &[(0, 0), (5, -3), (-31, 31), (200, -150), (-1000, 900), (3000, 2000)] {
            let (cx, cy) = to_xy(u, v);
            for _ in 0..200_000 {
                let r = SAME_HEX2.sqrt() * rnd().sqrt();
                let a = rnd() * std::f32::consts::TAU;
                let (x, y) = (cx + r * a.cos(), cy + r * a.sin());
                let (dx, dy) = (x - cx, y - cy);
                if dx * dx + dy * dy < SAME_HEX2 {
                    assert_eq!(from_xy(x, y), (u, v), "point ({x},{y}) near hex ({u},{v})");
                }
            }
        }
    }

    #[test]
    fn simd_masks_match_scalar_tests() {
        let mut st = AtomState::default();
        st.set_hex(7, -4, &to_xy);
        let mut seed = 99u64;
        let mut rnd = || {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            (seed >> 40) as f32 / (1u64 << 24) as f32
        };
        let cutoff = crate::collision_math::contact_distance_cutoff(58.0);
        for _ in 0..100_000 {
            let (x, y) = (st.cx[0] + (rnd() - 0.5) * 200.0, st.cy[0] + (rnd() - 0.5) * 200.0);
            let (pass, inside) = st.masks(x, y, cutoff);
            for i in 0..7 {
                let ex = x - st.cx[i];
                let ey = y - st.cy[i];
                let d = ex * ex + ey * ey;
                assert_eq!(pass >> i & 1 == 1, d < cutoff);
                assert_eq!(inside >> i & 1 == 1, d < SAME_HEX2);
            }
            assert_eq!(pass >> 7 & 1, 0);
        }
    }
}
