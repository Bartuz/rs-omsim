# Optimizations in the Rust engine

The Rust engine started as a function-by-function translation of the C at the root of
this repository. It is faster because of the changes below. None of them is specific
to Rust: each could go into the C.

Every one of them is **exact**. Every metric, and every error with its cycle and
location, stays the same, and so does anything that can influence them later, such as
the order in which cells enter the grid's hash table. `cargo test --release --features
reference` checks this against the C (see BENCHMARKS.md).

| optimization | Rust | in C |
|---|---|---|
| [Swept-cell cache](#swept-cell-cache) | `engine/src/sweep.rs` | ported: branch `c-fast-paths`, PR #1 |
| [Pairwise prefilter](#pairwise-prefilter) | `engine/src/collision.rs` | ported: branch `c-fast-paths`, PR #1 |
| [Periodic fast-forward](#periodic-fast-forward) | `engine/src/ffwd.rs` | ported: branch `c-fast-paths`, PR #1 |
| [Glyph read plans](#glyph-read-plans) | `engine/src/glyph_plan.rs` | not yet |
| [Movement trig memo](#movement-trig-memo) | `engine/src/trig_memo.rs` | not yet |
| [Occupied-slot index](#occupied-slot-index) | `engine/src/board_index.rs` | not yet |
| [Smaller fixes in the motion check](#smaller-fixes-in-the-motion-check) | `engine/src/collision_math.rs`, `collision.rs` | partly |

With the first three ported, C is 1.55–1.57× faster on the test and leaderboard
corpora (PR #1). The Rust engine with all of them is 2.24–2.49× faster (BENCHMARKS.md).

## Where the time goes

Most of the time is spent in the motion check. Each half-cycle, every moving atom is
sampled along its path. The step shrinks as the rotation radius grows, down to 1/4096
of a half-cycle for long arms. At every sample, omsim rounds the atom to a hex, tests
the 7 cells around it against the board (marking them as used area), and tests it
against every other collider. On typical solutions this takes 65–80% of the time. The
heaviest 1% of solutions take 62–72% of all simulation time, and nearly all of their
cost is half-cycles with more than a hundred samples.

## Swept-cell cache

**Idea.** Consecutive samples of one atom almost always land in the same hex, so the C
re-tests the same 7 cells again and again. The cache keeps, per moving atom, its current
hex, the 7 cell centres around it, and which of them have already been tested and
marked. A cell is marked once per atom instead of once per sample. Rounding to a hex
(`from_xy`) only runs when a sample is near a hex corner.

**Why it's exact.** Each distance test is still the original float test on the original
sample. Marking is idempotent, and the board can't change during a motion check, so the
first mark of each cell happens at the same moment and the hash table sees the same
insertions in the same order. Cells that hold an atom stay "hot" and are re-tested at
every sample, so collisions are reported at the same samples and locations as in the
C. The 7 tests run as one SSE pass.

## Pairwise prefilter

**Idea.** Before the sample loop, compute each movement's transform once per substep,
and the bounding box of each atom's samples. Fixed colliders that can't reach an atom's
box, and pairs of movements whose boxes never come within contact distance, are skipped
without testing every sample.

**Why it's exact.** Only colliders at least 1 px beyond contact distance from every
sample are dropped. The fast path runs only while coordinates stay below 2^18 px,
where float error is far below that margin; beyond that, the original loop runs.

## Periodic fast-forward

**Idea.** The simulation is deterministic. If the board state at the start of a tape
period equals the state at an earlier one, every later period repeats it. The run then
jumps ahead by whole periods: it advances the cycle and every counter that grows with
time (outputs, the output-cycle log, atom grabs, collision checks). It stops short of
completion and of the cycle and collision-check limits, and simulates the remaining
cycles normally. Solutions that loop until the cycle limit become thousands of times
faster.

**Why it's exact.** The compared state is everything a later cycle can read: atoms with
all their flags, overlapped atoms in order, disjoint bonds, pending flag resets, arms,
and glyph, conduit and input/output contents. Area markers are left out, because a
repeating period can't use new area. How often a state is captured only decides *when*
skipping starts, never the result.

## Glyph read plans

**Idea.** Every cycle, each glyph reads a few positions in its 3×3 neighbourhood. In the
C, each read computes the rotated position and looks it up in the grid (a hash-table
probe outside the central 64×64 array). A plan built once per board keeps each glyph's
nine positions and the grid slot each one resolved to, so most reads are a single
indexed load.

**Why it's exact.** Only slot numbers are cached, never atom values or pointers. A
rehash moves every slot, so it invalidates all plans and the next read resolves them
again. Cells in the fixed array never move, so they stay cached even while empty.

**For C.** Add a per-glyph array of 9 cached slots, invalidated by a generation counter
that `rehash` increments. It pays most on glyph-heavy machines: 1.6× on puzzle P294,
less on typical solutions.

## Movement trig memo

**Idea.** The motion check takes `cos` and `sin` of each movement's angle at every
substep. Those angles come from a small set (multiples of 60° times fixed substep
fractions), so a small hash table keyed by the angle's bit pattern replaces most libm
calls.

**Why it's exact.** `cos` and `sin` are pure functions of their input (the rounding mode
is never changed), so the table returns the values the calls would return.

**For C.** A static table of a few hundred entries is enough. The gain is small, about
3%.

## Occupied-slot index

**Idea.** The grid holds both physical atoms and markers left behind as used area, and
on long runs the markers can far outnumber the atoms. A few operations need only the
physical atoms, but the C finds them by scanning every slot of the grid. Examples are
the polymer collision check (`EXTEND_CHAIN`) and the fast-forward's state capture. A
bitset with one bit per grid slot is updated on insert, remove and rehash, and gives
the occupied slots directly, in grid order.

**Why it's exact.** It is a pure index: the grid stays the single source of truth, and
iteration runs in grid order, as the scan does. It is built lazily, so solutions that
never need it don't pay for it.

## Smaller fixes in the motion check

Each of these is small alone, but they sit in the innermost loops:

- **Distance tests without `sqrt`.** The C compares `(float)sqrt(d²) < r`. The Rust
  compares `d²` directly against the exact float threshold that gives the same answer:
  `r²`, except for radii 35, 40 and 44, where it is one ULP below `r²`. Squaring the
  radius naively changes which contacts count as collisions. PR #1 already does this
  for the board test (radius 58).
- **Rounding without `round()`.** `(int)round(x)` in `from_xy` becomes an inline
  "widen to double, add ±0.5, truncate". It is exact over the whole range, including
  NaN and infinity. A C port has to clamp out-of-range values by hand, because there the
  cast is undefined behaviour.
- **Skipping a polymer's own atoms.** For chain colliders the C loops over every
  collider and skips the ones in the atom's own movement. The Rust loops only over the
  two ranges outside it, so a long polymer no longer costs a loop iteration per atom per
  substep.
- **Cached sort keys.** Before combining chain colliders, the C sorts them with a
  comparator that recomputes each element's period and equivalence class (a division
  and a remainder) in every comparison. The Rust computes each key once and uses a
  stable sort, so colliders with equal keys keep their order.

## The compiler's share

The C built with clang (`-O2`) is 1.17× faster than with gcc. Clang uses the same LLVM
back end as rustc, so that part of the gap is compiler, not algorithm. Of the Rust
build options measured, `-C target-cpu=native` gains 5% and profile-guided optimization
6%. C compilers offer the same two (`-march=native`, `-fprofile-use`).
