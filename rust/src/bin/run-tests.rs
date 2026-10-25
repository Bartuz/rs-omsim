//! `run-tests` on the Rust engine: a port of `run-tests.c`. Run it from the repository
//! root. It walks `test/puzzle` and `test/solution` with the same `find` commands, in the
//! same order, and checks each solution's saved cost, instructions, cycles and area.
use omsim_native::{decode::*, parse::*, sim::*};
use std::{
    ffi::{CStr, CString, c_char},
    io::{BufRead, BufReader},
    mem::MaybeUninit,
    process::{Command, Stdio},
};

struct Puzzle {
    filename: Vec<u8>,
    pf: *mut puzzle_file,
}

/// `popen(command)` in run-tests.c, line by line.
fn find(arguments: &[&str]) -> impl Iterator<Item = CString> {
    let child = Command::new("find")
        .args(arguments)
        .stdout(Stdio::piped())
        .spawn()
        .expect("cannot run find");
    BufReader::new(child.stdout.unwrap())
        .split(b'\n')
        .map(|line| CString::new(line.expect("cannot read find's output")).unwrap())
}

fn show(s: &CStr) -> std::borrow::Cow<'_, str> {
    s.to_string_lossy()
}

fn main() {
    let mut puzzles = Vec::new();
    for path in find(&["test/puzzle", "-type", "f"]) {
        // SAFETY: a NUL-terminated path; the result is owned by this list until exit.
        let pf = unsafe { rs_parse_puzzle_file(path.as_ptr()) };
        if pf.is_null() {
            eprintln!("couldn't parse puzzle at '{}'", show(&path));
            continue;
        }
        let bytes = path.as_bytes();
        let last_slash = bytes.iter().rposition(|&b| b == b'/').map_or(0, |i| i + 1);
        let last_dot = bytes[last_slash..].iter().rposition(|&b| b == b'.').map_or(0, |i| last_slash + i);
        if last_dot > last_slash {
            // run-tests.c prepends, so later files shadow earlier ones with the same name.
            puzzles.insert(0, Puzzle { filename: bytes[last_slash..last_dot].to_vec(), pf });
        }
    }
    eprintln!("all puzzles parsed");

    let mut total_solutions = 0;
    let mut validated_solutions = 0;
    for path in find(&["test/solution", "-type", "f", "-name", "*.solution"]) {
        let name = show(&path);
        // SAFETY: every pointer below comes from the engine and is freed once, at the end
        // of this iteration, as in run-tests.c.
        unsafe {
            let sf = rs_parse_solution_file(path.as_ptr());
            if sf.is_null() {
                eprintln!("couldn't parse solution at '{name}'");
                continue;
            }
            let wanted = std::slice::from_raw_parts((*sf).puzzle.bytes, (*sf).puzzle.length);
            let Some(puzzle) = puzzles.iter().find(|p| p.filename == wanted) else {
                eprintln!(
                    "couldn't find puzzle named '{}' for '{name}'",
                    String::from_utf8_lossy(wanted.split(|&b| b == 0).next().unwrap_or_default())
                );
                rs_free_solution_file(sf);
                continue;
            };

            let mut solution: solution = MaybeUninit::zeroed().assume_init();
            let mut board: board = MaybeUninit::zeroed().assume_init();
            let mut error: *const c_char = std::ptr::null();
            if !rs_decode_solution(&mut solution, puzzle.pf, sf, &mut error) {
                eprintln!("error in '{name}': {}", show(CStr::from_ptr(error)));
                rs_free_solution_file(sf);
                continue;
            }
            total_solutions += 1;

            if check(&name, sf, puzzle.pf, &mut solution, &mut board) {
                validated_solutions += 1;
            }
            rs_destroy(&mut solution, &mut board);
            rs_free_solution_file(sf);
        }
    }
    eprintln!("{validated_solutions} / {total_solutions} solutions validated!");
}

/// One solution's checks, in run-tests.c's order. False is its `goto fail`.
unsafe fn check(
    name: &str,
    sf: *mut solution_file,
    pf: *mut puzzle_file,
    solution: &mut solution,
    board: &mut board,
) -> bool {
    unsafe {
        let sf = &mut *sf;
        let expect_collision = sf.cost == 0 && sf.instructions == 0;

        let cost = rs_solution_file_cost(sf);
        if u64::from(sf.cost) != cost && !expect_collision {
            eprintln!("cost mismatch for '{name}'");
            eprintln!("solution file says cost is: {}", sf.cost);
            eprintln!("adding up its parts, the cost is: {cost}");
            return false;
        }
        let instructions = rs_solution_instructions(solution);
        if u64::from(sf.instructions) != instructions && !expect_collision {
            eprintln!("instructions mismatch for '{name}'");
            eprintln!("solution file says instruction count is: {}", sf.instructions);
            eprintln!("counting instructions says instruction count is: {instructions}");
            return false;
        }

        rs_initial_setup(solution, board, sf.area);
        while board.cycle < 200000 && !board.complete {
            rs_run(solution, board);
            if board.rs_collision {
                if expect_collision {
                    return true;
                }
                eprintln!(
                    "collision in '{name}' at {}, {}: {}",
                    board.collision_location.u,
                    board.collision_location.v,
                    show(CStr::from_ptr(board.collision_reason))
                );
                break;
            }
        }
        if u64::from(sf.cycles) != board.cycle {
            eprintln!("cycle mismatch for '{name}'");
            eprintln!("solution file says cycle count is: {}", sf.cycles);
            eprintln!("simulation says cycle count is: {}", board.cycle);
            return false;
        }
        let area = rs_used_area(board);
        if (*pf).production_info.is_null() && sf.area != area {
            eprintln!("area mismatch for '{name}'");
            eprintln!("solution file says area is: {}", sf.area);
            eprintln!("simulation says area is: {area}");
            return false;
        }
        true
    }
}
