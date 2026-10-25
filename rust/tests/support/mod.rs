#![allow(dead_code)]
use rs_omsim::parse::{Molecule, Puzzle, Solution};
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn files(root: impl AsRef<Path>, extension: &str) -> Vec<PathBuf> {
    fn walk(path: &Path, extension: &str, result: &mut Vec<PathBuf>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, extension, result);
            } else if path.extension().and_then(|s| s.to_str()) == Some(extension) {
                result.push(path);
            }
        }
    }
    let mut result = Vec::new();
    walk(root.as_ref(), extension, &mut result);
    result.sort();
    result
}

struct Hash(u64);
impl Hash {
    fn new() -> Self {
        Self(14695981039346656037)
    }
    fn bytes(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.0 = (self.0 ^ u64::from(*byte)).wrapping_mul(1099511628211);
        }
    }
    fn n(&mut self, n: impl Into<i128>) {
        self.bytes(&(n.into() as u64).to_le_bytes());
    }
    fn len(&mut self, len: usize) {
        self.n(len as u64);
    }
    fn s(&mut self, bytes: &[u8]) {
        self.len(bytes.len());
        self.bytes(bytes);
    }
    fn pos<T: Into<i128> + Copy>(&mut self, p: [T; 2]) {
        self.n(p[0]);
        self.n(p[1]);
    }
    fn molecule(&mut self, m: &Molecule) {
        self.len(m.atoms.len());
        for a in &m.atoms {
            self.n(a.kind);
            self.pos(a.offset);
        }
        self.len(m.bonds.len());
        for b in &m.bonds {
            self.n(b.kind);
            self.pos(b.from);
            self.pos(b.to);
        }
    }
}
pub fn puzzle_hash(p: &Puzzle<'_>) -> u64 {
    let mut h = Hash::new();
    h.s(p.name);
    // Upstream read_uint64 mistakenly returns int32_t: reproduce that ONLY in
    // the test oracle fingerprint, never in the native public model.
    h.n(p.creator as i32);
    h.n(p.parts_available as i32);
    h.len(p.inputs.len());
    for m in &p.inputs {
        h.molecule(m);
    }
    h.len(p.outputs.len());
    for m in &p.outputs {
        h.molecule(m);
    }
    h.n(p.output_scale);
    h.n(p.production.is_some() as u8);
    if let Some(v) = &p.production {
        h.n(v.shrink_left as u8);
        h.n(v.shrink_right as u8);
        h.n(v.isolate_inputs_from_outputs as u8);
        h.len(v.cabinets.len());
        for c in &v.cabinets {
            h.pos(c.position);
            h.s(c.kind);
        }
        h.len(v.conduits.len());
        for c in &v.conduits {
            h.pos(c.starting_position_a);
            h.pos(c.starting_position_b);
            h.len(c.hexes.len());
            for p in &c.hexes {
                h.pos(*p);
            }
        }
        h.len(v.vials.len());
        for v in &v.vials {
            h.pos(v.position);
            h.n(v.style);
            h.n(v.count);
        }
    }
    h.0
}
pub fn solution_hash(s: &Solution<'_>) -> u64 {
    let mut h = Hash::new();
    h.s(s.puzzle);
    h.s(s.name);
    h.n(s.metrics.is_some() as u8);
    if let Some(m) = &s.metrics {
        h.n(m.cycles);
        h.n(m.cost);
        h.n(m.area);
        h.n(m.instructions);
    }
    h.len(s.parts.len());
    for p in &s.parts {
        h.s(p.name);
        h.pos(p.position);
        h.n(p.size);
        h.n(p.rotation);
        h.n(p.which_input_or_output);
        h.len(p.instructions.len());
        for i in &p.instructions {
            h.n(i.index);
            h.n(i.instruction);
        }
        h.len(p.track_hexes.len());
        for pos in &p.track_hexes {
            h.pos(*pos);
        }
        h.n(p.arm_number);
        h.n(p.conduit_id);
        h.len(p.conduit_hexes.len());
        for pos in &p.conduit_hexes {
            h.pos(*pos);
        }
    }
    h.0
}
#[cfg(feature = "reference")]
unsafe extern "C" {
    pub fn oracle_puzzle(bytes: *const u8, len: usize, hash: *mut u64) -> i32;
    pub fn oracle_solution(bytes: *const u8, len: usize, hash: *mut u64) -> i32;
}
