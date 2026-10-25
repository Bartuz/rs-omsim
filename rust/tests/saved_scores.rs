mod support;
use rs_omsim::{Verifier, parse, verifier::RUST};
use std::collections::HashMap;

#[test]
fn reproduces_saved_scores() {
    let puzzles: HashMap<_, _> = support::files("../test/puzzle", "puzzle")
        .into_iter()
        .map(|p| {
            (
                p.file_stem().unwrap().to_string_lossy().as_bytes().to_vec(),
                std::fs::read(p).unwrap(),
            )
        })
        .collect();
    let mut checked = 0;
    for path in support::files("../test/solution", "solution") {
        let bytes = std::fs::read(&path).unwrap();
        let s = parse::solution(&bytes).unwrap();
        let Some(m) = &s.metrics else { continue };
        // A quick check without the C; tests/differential.rs and run-tests cover everything.
        if m.cost == 0 || m.cycles == 0 || m.cycles > 1000 {
            continue;
        }
        let Some(puzzle) = puzzles.get(s.puzzle) else {
            continue;
        };
        if parse::puzzle(puzzle).unwrap().production.is_some() {
            continue;
        }
        let mut v = Verifier::new(&RUST, puzzle, &bytes).unwrap();
        v.set_cycle_limit(2000).unwrap();
        for (metric, expected) in [
            ("cost", m.cost),
            ("instructions", m.instructions),
            ("cycles", m.cycles),
            ("area", m.area),
        ] {
            assert_eq!(
                v.metric(metric).unwrap() as u32,
                expected,
                "{}: {metric}",
                path.display()
            );
        }
        checked += 1;
        if checked == 12 {
            break;
        }
    }
    assert_eq!(checked, 12);
}

#[test]
fn malformed_input_is_rejected_before_the_engine_parses_it() {
    assert!(Verifier::new(&RUST, &[], &[]).is_err());
}
