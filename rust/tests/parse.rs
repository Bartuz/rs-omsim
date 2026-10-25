mod support;
use rs_omsim::parse;

#[test]
fn parses_every_test_file_as_c_does() {
    for (extension, minimum) in [("puzzle", 175), ("solution", 4533)] {
        let paths = support::files("../test", extension);
        assert!(paths.len() >= minimum, "missing reference corpus");
        for path in &paths {
            let bytes = std::fs::read(path).unwrap();
            let hash = if extension == "puzzle" {
                support::puzzle_hash(
                    &parse::puzzle(&bytes).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
                )
            } else {
                support::solution_hash(
                    &parse::solution(&bytes).unwrap_or_else(|e| panic!("{}: {e}", path.display())),
                )
            };
            #[cfg(feature = "reference")]
            {
                let mut reference_hash = 0;
                // SAFETY: trusted, checked-in upstream fixture. The oracle reads
                // synchronously, hashes fields, and frees its own allocations.
                let ok = unsafe {
                    if extension == "puzzle" {
                        support::oracle_puzzle(bytes.as_ptr(), bytes.len(), &mut reference_hash)
                    } else {
                        support::oracle_solution(bytes.as_ptr(), bytes.len(), &mut reference_hash)
                    }
                };
                assert_eq!(ok, 1, "C rejected {}", path.display());
                assert_eq!(
                    hash,
                    reference_hash,
                    "parsed fields differ: {}",
                    path.display()
                );
            }
            std::hint::black_box(hash);
        }
        eprintln!("checked {} {extension} files", paths.len());
    }
}

#[cfg(feature = "reference")]
#[test]
fn computed_cost_matches_c() {
    use std::collections::HashMap;
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
        let solution = parse::solution(&bytes).unwrap();
        let Some(puzzle) = puzzles.get(solution.puzzle) else {
            continue;
        };
        let Ok(mut verifier) = rs_omsim::Verifier::new(&rs_omsim::verifier::C, puzzle, &bytes) else {
            continue;
        };
        let cost = verifier.metric("cost").unwrap();
        assert_eq!(solution.cost(), cost as u64, "{}", path.display());
        checked += 1;
    }
    eprintln!("checked the cost of {checked} decodable solutions");
    assert!(checked > 4000);
}
