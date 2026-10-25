//! The Rust engine against the C at the root of the repository, on every test solution:
//! each metric must give the same value, or the same error at the same cycle and place.
//!
//! `OMSIM_EXTRA_CORPUS=DIR` adds the solutions under `DIR/solutions` (puzzles from
//! `DIR/puzzles`), for example a leaderboard snapshot.
#![cfg(feature = "reference")]
mod support;
use rs_omsim::{
    Verifier, parse,
    verifier::{C, Engine, RUST},
};
use std::{collections::HashMap, path::Path, time::Instant};

const METRICS: &[&str] = &[
    "parsed cost",
    "cost",
    "instructions",
    "cycles",
    "area",
    "height",
    "width*2",
    "minimum hexagon",
    "executed instructions",
    "overlap",
    "number of arms",
    "number of track segments",
    "maximum absolute part coordinate",
    "visual loop start cycle",
    "visual loop end cycle",
    "throughput cycles",
    "throughput outputs",
    "throughput waste",
    "per repetition cycles",
    "per repetition outputs",
    "per repetition area",
];
const APPROXIMATE_METRICS: &[&str] = &[
    "per repetition^2 area",
    "per repetition height",
    "per repetition width",
    "per repetition minimum hexagon",
];

fn results(engine: &'static Engine, puzzle: &[u8], solution: &[u8]) -> Vec<String> {
    let mut verifier = match Verifier::new(engine, puzzle, solution) {
        Ok(verifier) => verifier,
        Err(error) => return vec![error],
    };
    let mut values: Vec<String> = METRICS
        .iter()
        .map(|m| format!("{m}: {:?}", verifier.metric(m)))
        .collect();
    values.extend(APPROXIMATE_METRICS.iter().map(|m| {
        format!("{m}: {:?}", verifier.approximate_metric(m).map(f64::to_bits))
    }));
    values
}

fn compare(puzzles: &Path, solutions: &Path) -> usize {
    let puzzles: HashMap<Vec<u8>, Vec<u8>> = support::files(puzzles, "puzzle")
        .into_iter()
        .map(|p| (p.file_stem().unwrap().as_encoded_bytes().to_vec(), std::fs::read(p).unwrap()))
        .collect();
    let paths = support::files(solutions, "solution");
    let (mut checked, mut mismatches, started) = (0, Vec::new(), Instant::now());
    for (i, path) in paths.iter().enumerate() {
        let bytes = std::fs::read(path).unwrap();
        let Some(puzzle) = parse::solution(&bytes).ok().and_then(|s| puzzles.get(s.puzzle)) else {
            continue;
        };
        let (c, rust) = (results(&C, puzzle, &bytes), results(&RUST, puzzle, &bytes));
        if c != rust {
            let diff: Vec<_> = c.iter().zip(&rust).filter(|(a, b)| a != b).collect();
            mismatches.push(format!("{}: {diff:?}", path.display()));
        }
        checked += 1;
        if (i + 1) % 500 == 0 {
            eprintln!("[{}/{}] {:.0?}", i + 1, paths.len(), started.elapsed());
        }
    }
    assert!(mismatches.is_empty(), "{} mismatches:\n{}", mismatches.len(), mismatches.join("\n"));
    eprintln!("{checked} solutions x {} metrics: Rust == C", METRICS.len() + APPROXIMATE_METRICS.len());
    checked
}

#[test]
fn every_metric_matches_c_on_the_test_solutions() {
    assert!(compare(Path::new("../test/puzzle"), Path::new("../test/solution")) > 4400);
}

#[test]
fn every_metric_matches_c_on_an_extra_corpus() {
    let Some(dir) = std::env::var_os("OMSIM_EXTRA_CORPUS") else {
        eprintln!("OMSIM_EXTRA_CORPUS not set; skipped");
        return;
    };
    let dir = Path::new(&dir);
    assert!(compare(&dir.join("puzzles"), &dir.join("solutions")) > 0);
}
