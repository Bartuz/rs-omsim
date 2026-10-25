//! Exact periodic fast-forward for the completion simulation.
//!
//! The simulation is deterministic, so if the dynamic state at cycle t2 equals the state
//! at an earlier cycle t1 with the same tape phase, the run repeats with period
//! P = t2 - t1 forever (until it stops on a limit). Instead of simulating k more periods,
//! this module advances `cycle` and every monotone counter (outputs, output cycle log,
//! atom grabs, collision checks) by k*P, then lets the normal loop simulate the tail.
//! k stops short of completion, the cycle limit and the collision check limit, so every
//! observable value is the one the plain loop would produce.
//!
//! "Dynamic state" is everything a later cycle can read: physical atoms with all flag
//! bits, stale MOVED markers (arms can still see them via get_atom), overlapped atoms,
//! disjoint bonds, the pending flag-reset list, arm/glyph/conduit/input state. Plain
//! area markers (VALID|REMOVED) are inert apart from the area count, which cannot grow
//! once a period repeats. pivot_parity is excluded: nothing in the simulation reads it.
use crate::sim::{atom_at_position, board, solution};
use std::collections::HashMap;

const VALID: u64 = 1;
const REMOVED: u64 = 1 << 27;
const MOVED: u64 = 1 << 28;
const TRANSIENT: u64 = (1 << 59) | (1 << 60); // CONDUIT_SHAPE | VISITED
const OUTPUT_TYPES: u32 = 3 << 1;

/// The keys are already well-mixed 64-bit hashes.
#[derive(Default)]
struct IdHash(u64);
impl std::hash::Hasher for IdHash {
    fn write(&mut self, _: &[u8]) {
        unreachable!("only u64 keys")
    }
    fn write_u64(&mut self, x: u64) {
        self.0 = x;
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

struct Counters {
    cycle: u64,
    outputs: Vec<u64>,
    output_cycles: Vec<u64>,
    grabs: [u32; 16],
    collision_checks: u64,
}

struct Snapshot {
    state: Vec<u64>,
    counters: Counters,
}

fn clone_counters(c: &Counters) -> Counters {
    Counters {
        cycle: c.cycle,
        outputs: c.outputs.clone(),
        output_cycles: c.output_cycles.clone(),
        grabs: c.grabs,
        collision_checks: c.collision_checks,
    }
}

#[inline]
fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::x86_64::_rdtsc()
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        std::time::Instant::now().elapsed().as_nanos() as u64
    }
}

#[inline]
fn mix(mut x: u64) -> u64 {
    x ^= x >> 33;
    x = x.wrapping_mul(0xff51afd7ed558ccd);
    x ^= x >> 33;
    x = x.wrapping_mul(0xc4ceb9fe1a85ec53);
    x ^ (x >> 33)
}

#[inline]
fn pos_key(p: crate::sim::vector) -> u64 {
    ((p.u as u32 as u64) << 32) | p.v as u32 as u64
}

unsafe fn outputs(s: *mut solution) -> impl Iterator<Item = *mut crate::sim::input_output> {
    let n = (*s).number_of_inputs_and_outputs;
    let base = (*s).inputs_and_outputs;
    (0..n)
        .map(move |i| base.add(i))
        .filter(|&io| (*io).r#type.0 & OUTPUT_TYPES != 0)
}

unsafe fn counters(s: *mut solution, b: *mut board) -> Counters {
    Counters {
        cycle: (*b).cycle,
        outputs: outputs(s).map(|io| (*io).number_of_outputs).collect(),
        output_cycles: outputs(s).map(|io| (*io).number_of_output_cycles).collect(),
        grabs: (*b).atom_grabs,
        collision_checks: (*b).collision_checks,
    }
}

const GRABBING_LOW_BIT: u32 = 1 << 21;
const ANY_ARM: u32 = (1 << 14) | (1 << 15) | (1 << 16) | (1 << 17) | (1 << 18) | (1 << 19) | (1 << 20);

fn u_offset(d: usize) -> (i32, i32) {
    [(1, 0), (0, 1), (-1, 1), (-1, 0), (0, -1), (1, -1)][d]
}

/// Physical atoms via the occupied-slot index, plus the full cell under every grabbing
/// direction of every arm: those are the only places a stale MOVED marker is read
/// (get_atom for arms, and the held-stationary check) while a half-cycle has fewer
/// than 64 movements (larger movement indices alias bond bits).
/// Creates the occupied-slot index from the written bounding box instead of a scan of
/// all 4096 prefix slots (the lazy constructor in board_index scans everything).
unsafe fn ensure_index(b: *mut board) {
    if !(*b).index.is_null() {
        return;
    }
    let slots = 4096 + (*b).grid.hash_capacity as usize;
    let mut index = Box::new(crate::board_index::BoardIndex::new(slots));
    let grid = std::slice::from_raw_parts((*b).grid.atoms_at_positions, slots);
    let [u0, u1, v0, v1] = (*b).written_bbox;
    let (u0, u1, v0, v1) = (u0.max(-32), u1.min(31), v0.max(-32), v1.min(31));
    let mut mark = |slot: usize| {
        if grid[slot].atom & (VALID | REMOVED) == VALID {
            index.set(slot, true);
        }
    };
    if u0 <= u1 && v0 <= v1 {
        for u in u0..=u1 {
            let row = ((u + 32) * 64) as usize;
            for slot in row + (v0 + 32) as usize..=row + (v1 + 32) as usize {
                mark(slot);
            }
        }
    }
    for slot in 4096..slots {
        mark(slot);
    }
    (*b).index = Box::into_raw(index);
}

unsafe fn capture_cells_fast(
    s: *mut solution,
    b: *mut board,
    cells: &mut Vec<(u64, u64)>,
    grabs: &mut Vec<(u64, u64)>,
) {
    let grid = (*b).grid.atoms_at_positions;
    ensure_index(b);
    for slot in crate::board_index::occupied_slots(b) {
        let a = &*grid.add(slot);
        cells.push((pos_key(a.position), a.atom & !TRANSIENT));
    }
    cells.sort_unstable();
    for i in 0..(*s).number_of_arms {
        let m = &*(*s).arms.add(i);
        if m.r#type & ANY_ARM == 0 {
            continue;
        }
        for d in 0..6 {
            if m.r#type & (GRABBING_LOW_BIT << d) == 0 {
                continue;
            }
            let (du, dv) = u_offset(d);
            let p = crate::sim::vector {
                u: m.direction_u.u * du + m.direction_v.u * dv + m.position.u,
                v: m.direction_u.v * du + m.direction_v.v * dv + m.position.v,
            };
            grabs.push((pos_key(p), peek(b, p) & !TRANSIENT));
        }
    }
}

/// Read-only cell lookup: unlike lookup_atom it never inserts or rehashes, so capturing
/// cannot change the table layout the simulation later sees.
unsafe fn peek(b: *mut board, p: crate::sim::vector) -> u64 {
    let grid = (*b).grid.atoms_at_positions;
    if (-32..32).contains(&p.u) && (-32..32).contains(&p.v) {
        return (*grid.add(((p.u + 32) * 64 + p.v + 32) as usize)).atom;
    }
    let cap = (*b).grid.hash_capacity as usize;
    if cap == 0 {
        return 0;
    }
    let mut h = 0x542ddeaec5c75e0fu64;
    h = h.wrapping_add(0xc0c594dd042705fbu64.wrapping_mul(p.u as u32 as u64));
    h = h.wrapping_add(0x93b45776c46130c1u64.wrapping_mul(p.v as u32 as u64));
    let mut i = (h >> 32) as usize & (cap - 1);
    for _ in 0..cap {
        let a = &*grid.add(4096 + i);
        if a.atom == 0 {
            return 0;
        }
        if a.position.u == p.u && a.position.v == p.v {
            return a.atom;
        }
        i = (i + 1) & (cap - 1);
    }
    0
}

/// Canonical dynamic state as a word vector (order-independent parts are sorted).
unsafe fn capture(s: *mut solution, b: *mut board, out: &mut Vec<u64>) {
    out.clear();
    if (*b).movements.capacity < 64 {
        let mut cells = Vec::new();
        let mut grabs = Vec::new();
        capture_cells_fast(s, b, &mut cells, &mut grabs);
        out.push(0);
        out.push(cells.len() as u64);
        for (p, a) in cells {
            out.push(p);
            out.push(a);
        }
        out.push(grabs.len() as u64);
        for (p, a) in grabs {
            out.push(p);
            out.push(a);
        }
        capture_rest(s, b, out);
        return;
    }
    out.push(1);
    let slots = 4096 + (*b).grid.hash_capacity as usize;
    let grid = std::slice::from_raw_parts((*b).grid.atoms_at_positions, slots);
    let mut cells: Vec<(u64, u64)> = Vec::new();
    let mut take = |a: &atom_at_position| {
        let bits = a.atom;
        if bits & VALID != 0 && (bits & REMOVED == 0 || bits & MOVED != 0) {
            cells.push((pos_key(a.position), bits & !TRANSIENT));
        }
    };
    // Array prefix covers [-32, 32)^2; only the box of cells ever written can be valid.
    let [u0, u1, v0, v1] = (*b).written_bbox;
    let (u0, u1, v0, v1) = (u0.max(-32), u1.min(31), v0.max(-32), v1.min(31));
    if u0 <= u1 && v0 <= v1 {
        for u in u0..=u1 {
            let row = ((u + 32) * 64) as usize;
            for a in &grid[row + (v0 + 32) as usize..=row + (v1 + 32) as usize] {
                take(a);
            }
        }
    }
    for a in &grid[4096..] {
        take(a);
    }
    cells.sort_unstable();
    out.push(cells.len() as u64);
    for (p, a) in cells {
        out.push(p);
        out.push(a);
    }
    capture_rest(s, b, out);
}

unsafe fn capture_rest(s: *mut solution, b: *mut board, out: &mut Vec<u64>) {
    // Overlapped atoms: order is significant (raise/lookup walk the list in order).
    let n = (*b).number_of_overlapped_atoms as usize;
    out.push(n as u64);
    for i in 0..n {
        let a: atom_at_position = *(*b).overlapped_atoms.add(i);
        out.push(pos_key(a.position));
        out.push(a.atom & !TRANSIENT);
    }
    // Disjoint bonds: a set keyed by position; slot layout is not observable.
    let t = &(*b).disjoint_bond_table;
    let mut bonds: Vec<(u64, u64)> = Vec::new();
    for i in 0..t.size as usize {
        let d = *t.bonds.add(i);
        let f = pos_key(d.from_position);
        let to = pos_key(d.to_position);
        let empty = f == 0 && to == 0;
        let removed = f == 1 && to == 1; // {(0,1),(0,1)}
        if !empty && !removed {
            bonds.push((f, to));
        }
    }
    bonds.sort_unstable();
    out.push(bonds.len() as u64);
    for (f, to) in bonds {
        out.push(f);
        out.push(to);
    }
    // Pending temporary-flag resets, as grid slot offsets (a set).
    let base = (*b).grid.atoms_at_positions as usize;
    let mut resets: Vec<u64> = (0..(*b).flag_reset_length as usize)
        .map(|i| (*(*b).flag_reset.add(i) as usize).wrapping_sub(base) as u64)
        .collect();
    resets.sort_unstable();
    resets.dedup();
    out.push(resets.len() as u64);
    out.extend(resets);
    for i in 0..(*s).number_of_arms {
        let m = &*(*s).arms.add(i);
        out.push(m.r#type as u64);
        out.push(pos_key(m.position));
        out.push(pos_key(m.direction_u));
        out.push(pos_key(m.direction_v));
        out.push(m.arm_rotation as u32 as u64);
        out.push(pos_key(m.movement));
        out.push(m.conversion);
    }
    for i in 0..(*s).number_of_glyphs {
        out.push((*(*s).glyphs.add(i)).conversion);
    }
    for i in 0..(*s).number_of_conduits {
        let c = &*(*s).conduits.add(i);
        out.push(c.number_of_molecules as u64);
        let mut total = 0usize;
        for j in 0..c.number_of_molecules as usize {
            let l = *c.molecule_lengths.add(j) as usize;
            out.push(l as u64);
            total += l;
        }
        for j in 0..total {
            let a = *c.atoms.add(j);
            out.push(pos_key(a.position));
            out.push(a.atom);
        }
        out.push(c.disjoint_bond_table.count as u64);
    }
    for i in 0..(*s).number_of_inputs_and_outputs {
        out.push((*(*s).inputs_and_outputs.add(i)).r#type.0 as u64);
    }
}

fn hash_words(w: &[u64]) -> u64 {
    let mut h = 0x9e3779b97f4a7c15u64;
    for &x in w {
        h = mix(h ^ x).wrapping_add(0x9e3779b97f4a7c15);
    }
    h
}

/// Replacement for `while (no limit || cycle < limit) && !complete && !collision { cycle() }`.
/// `limit` None means limits are disabled.
pub unsafe fn run(s: *mut solution, b: *mut board, limit: Option<u64>) {
    let cont = |b: *mut board| limit.map_or(true, |l| (*b).cycle < l) && !(*b).complete && !(*b).rs_collision;
    // Tape phase is periodic from t0 on, with this unit.
    let unit = if (*s).tape_period == 0 { 1 } else { (*s).tape_period };
    let mut t0 = 0u64;
    for i in 0..(*s).number_of_arms {
        let start = (*(*s).arm_tape_start_cycle.add(i)).max(0) as u64;
        let halt = *(*s).arm_tape_halt_index.add(i);
        let ready = if halt != usize::MAX {
            start.saturating_add(halt as u64)
        } else if (*s).tape_period == 0 {
            start.saturating_add(*(*s).arm_tape_length.add(i) as u64)
        } else {
            start
        };
        t0 = t0.max(ready);
    }
    // Runs this short cannot gain from skipping; don't pay for snapshots.
    const MIN_CYCLE: u64 = 16;
    if t0 < MIN_CYCLE {
        t0 += (MIN_CYCLE - t0).div_ceil(unit) * unit;
    }
    // The output cycle log is only extended for runs that record every output.
    let mut active = outputs(s).all(|io| (*io).output_cycles_end == u64::MAX);
    // Hash of every captured boundary; full states only for the most recent ones. A hit
    // on an older boundary is confirmed by replay: capture again one period later.
    const RING: usize = 64;
    let mut hashes: HashMap<u64, u64, std::hash::BuildHasherDefault<IdHash>> = HashMap::default();
    let mut ring: std::collections::VecDeque<(u64, Snapshot)> = std::collections::VecDeque::new();
    let mut anchor: Option<(u64, Snapshot)> = None;
    // Capture every `stride`-th boundary; doubles while a capture costs more than 1/16 of
    // the simulation since the previous one. Timing only decides when skipping happens.
    let mut stride = 1u64;
    let mut tsc_last = rdtsc();
    let mut words = Vec::new();
    while cont(b) {
        crate::sim::rs_run(s, b);
        let t = (*b).cycle;
        if !active || t < t0 || (t - t0) % unit != 0 || !cont(b) {
            continue;
        }
        let boundary = (t - t0) / unit;
        let anchored = anchor.as_ref().is_some_and(|(at, _)| *at == t);
        if boundary % stride != 0 && !anchored {
            continue;
        }
        let tsc_sim = rdtsc();
        capture(s, b, &mut words);
        let h = hash_words(&words);
        let mut matched: Option<Counters> = None;
        if anchored {
            let (_, snap) = anchor.take().unwrap();
            if snap.state == words {
                matched = Some(snap.counters);
            }
        }
        if matched.is_none() {
            if let Some(&t1) = hashes.get(&h) {
                if let Some((_, snap)) = ring.iter().find(|(c, sn)| *c == t1 && sn.state == words) {
                    matched = Some(Counters { ..clone_counters(&snap.counters) });
                } else if !ring.iter().any(|(c, _)| *c == t1) && anchor.is_none() {
                    anchor = Some((t + (t - t1), Snapshot { state: words.clone(), counters: counters(s, b) }));
                }
            }
        }
        let Some(then) = matched else {
            hashes.insert(h, t);
            ring.push_back((t, Snapshot { state: words.clone(), counters: counters(s, b) }));
            if ring.len() > RING {
                ring.pop_front();
            }
            let tsc_end = rdtsc();
            if (tsc_end - tsc_sim) * 16 > tsc_sim - tsc_last && stride < 1 << 12 {
                stride *= 2;
            }
            tsc_last = tsc_end;
            continue;
        };
        let then = &then;
        let now = counters(s, b);
        let period = t - then.cycle;
        // Largest k such that the run is still incomplete at t + k*period.
        let target = (*s).target_number_of_outputs;
        let mut k = u64::MAX;
        if target != u64::MAX {
            let mut k_comp = 0u64;
            let mut any = false;
            for (o, &n) in now.outputs.iter().enumerate() {
                let d = n - then.outputs[o];
                if n >= target {
                    continue;
                }
                any = true;
                let kk = if d == 0 { u64::MAX } else { (target - n - 1) / d };
                k_comp = k_comp.max(kk);
            }
            if any {
                k = k.min(k_comp);
            } else {
                k = 0;
            }
        }
        match limit {
            Some(l) => k = k.min((l - t) / period),
            None if k == u64::MAX => {
                active = false; // the plain loop would never end; keep its behaviour
                continue;
            }
            None => {}
        }
        let dcc = now.collision_checks - then.collision_checks;
        let cc_limit = (*b).collision_check_limit;
        if cc_limit > 0 && dcc > 0 {
            k = k.min(cc_limit.saturating_sub(now.collision_checks) / dcc);
        }
        // Stay within the integer ranges the measurements use.
        k = k.min((i32::MAX as u64).saturating_sub(t) / period);
        active = false;
        if k == 0 {
            continue;
        }
        let shift = k * period;
        (*b).cycle = t + shift;
        (*b).collision_checks = now.collision_checks + k * dcc;
        for i in 0..16 {
            let d = now.grabs[i] - then.grabs[i];
            (*b).atom_grabs[i] = now.grabs[i] + (k as u32).wrapping_mul(d);
        }
        for (o, io) in outputs(s).enumerate() {
            let d = now.outputs[o] - then.outputs[o];
            (*io).number_of_outputs = now.outputs[o] + k * d;
            let (c1, c2) = (then.output_cycles[o] as usize, now.output_cycles[o] as usize);
            let log: Vec<u64> = std::slice::from_raw_parts((*io).output_cycles, c2)[c1..c2].to_vec();
            let need = c2 + k as usize * log.len();
            if need as u64 > (*io).output_cycles_capacity {
                let cap = need.max(6);
                (*io).output_cycles =
                    libc_realloc((*io).output_cycles.cast(), cap * 8).cast();
                (*io).output_cycles_capacity = cap as u64;
            }
            let mut w = c2;
            for m in 1..=k {
                for &e in &log {
                    *(*io).output_cycles.add(w) = e + m * period;
                    w += 1;
                }
            }
            (*io).number_of_output_cycles = w as u64;
        }
    }
}

unsafe extern "C" {
    #[link_name = "realloc"]
    fn libc_realloc(p: *mut core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
}
