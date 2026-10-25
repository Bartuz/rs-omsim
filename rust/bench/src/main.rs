//! Differential test and paired timing of omsim verifier libraries.
//!
//!   omsim-bench --lib NAME=PATH[:PREFIX] ... --puzzles DIR --solutions DIR
//!               [--metrics a,b] [--reps N] [--budget-ms M] [--filter S] [--out FILE]
//!
//! Each library is a shared object with the `verifier_*` API of verifier.h; PREFIX is
//! prepended to the symbol names (`rs_` for the Rust engine). The first library is the
//! oracle: every other one must give the same value, or the same error at the same cycle
//! and location, for every metric of every solution. Libraries are opened with
//! RTLD_LOCAL, so several builds of the same code can be compared.
//!
//! A solution is paired with the puzzle file named after the puzzle in it. Each repetition
//! runs every library once on a fresh verifier (what a search loop does per candidate), in
//! an order that rotates per repetition, so drifts in machine load hit all libraries alike.
//! The time of a solution is the minimum over its repetitions.
use std::{
    collections::HashMap,
    ffi::{CStr, CString, c_char, c_int, c_void},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Duration, Instant},
};

#[link(name = "dl")]
unsafe extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlerror() -> *const c_char;
}
const RTLD_NOW: c_int = 2;

type Verifier = *mut c_void;

struct Library {
    name: String,
    find_puzzle_name: unsafe extern "C" fn(*const c_char, c_int, *mut c_int) -> *const c_char,
    create: unsafe extern "C" fn(*const c_char, c_int, *const c_char, c_int) -> Verifier,
    destroy: unsafe extern "C" fn(Verifier),
    set_cycle_limit: unsafe extern "C" fn(Verifier, c_int),
    evaluate: unsafe extern "C" fn(Verifier, *const c_char) -> c_int,
    error: unsafe extern "C" fn(Verifier) -> *const c_char,
    error_clear: unsafe extern "C" fn(Verifier),
    error_cycle: unsafe extern "C" fn(Verifier) -> c_int,
    error_u: unsafe extern "C" fn(Verifier) -> c_int,
    error_v: unsafe extern "C" fn(Verifier) -> c_int,
}

impl Library {
    /// `NAME=PATH[:PREFIX]`.
    fn open(spec: &str) -> Result<Self, String> {
        let (name, rest) = spec.split_once('=').ok_or("--lib wants NAME=PATH[:PREFIX]")?;
        let (path, prefix) = rest.rsplit_once(':').unwrap_or((rest, ""));
        let c_path = CString::new(path).map_err(|e| e.to_string())?;
        // SAFETY: dlopen with a NUL-terminated path; dlerror's text is valid until the
        // next dl* call, and is copied out at once.
        let handle = unsafe { dlopen(c_path.as_ptr(), RTLD_NOW) };
        if handle.is_null() {
            return Err(unsafe { CStr::from_ptr(dlerror()) }.to_string_lossy().into_owned());
        }
        let symbol = |name: &str| -> Result<*mut c_void, String> {
            let full = CString::new(format!("{prefix}{name}")).unwrap();
            // SAFETY: a live handle and a NUL-terminated name.
            let p = unsafe { dlsym(handle, full.as_ptr()) };
            if p.is_null() { Err(format!("{path}: missing symbol {prefix}{name}")) } else { Ok(p) }
        };
        // SAFETY: each symbol has the signature that verifier.h declares for it.
        unsafe {
            use std::mem::transmute as f;
            Ok(Self {
                name: name.to_string(),
                find_puzzle_name: f(symbol("verifier_find_puzzle_name_in_solution_bytes")?),
                create: f(symbol("verifier_create_from_bytes")?),
                destroy: f(symbol("verifier_destroy")?),
                set_cycle_limit: f(symbol("verifier_set_cycle_limit")?),
                evaluate: f(symbol("verifier_evaluate_metric")?),
                error: f(symbol("verifier_error")?),
                error_clear: f(symbol("verifier_error_clear")?),
                error_cycle: f(symbol("verifier_error_cycle")?),
                error_u: f(symbol("verifier_error_location_u")?),
                error_v: f(symbol("verifier_error_location_v")?),
            })
        }
    }

    /// The puzzle name inside a solution file, as the library parses it.
    fn puzzle_name(&self, solution: &[u8]) -> Option<Vec<u8>> {
        let mut length = 0;
        // SAFETY: the buffer is valid for the call; the result points into it.
        unsafe {
            let p = (self.find_puzzle_name)(solution.as_ptr().cast(), solution.len() as c_int, &mut length);
            (!p.is_null()).then(|| std::slice::from_raw_parts(p.cast::<u8>(), length as usize).to_vec())
        }
    }

    /// One fresh verifier, every metric, each value or error as text.
    fn run(&self, puzzle: &[u8], solution: &[u8], limit: i32, metrics: &[CString]) -> Vec<String> {
        // SAFETY: the buffers outlive the calls and the verifier copies them; v is live
        // until destroy, and error strings are copied before the next call.
        unsafe {
            let v = (self.create)(puzzle.as_ptr().cast(), puzzle.len() as c_int, solution.as_ptr().cast(), solution.len() as c_int);
            if v.is_null() {
                return vec!["ERR:allocation failed".into(); metrics.len()];
            }
            let text = |e: *const c_char| CStr::from_ptr(e).to_string_lossy().into_owned();
            let e = (self.error)(v);
            if !e.is_null() {
                let error = format!("ERR:{}", text(e));
                (self.destroy)(v);
                return vec![error; metrics.len()];
            }
            if limit > 0 {
                (self.set_cycle_limit)(v, limit);
            }
            let values = metrics
                .iter()
                .map(|m| {
                    (self.error_clear)(v);
                    let value = (self.evaluate)(v, m.as_ptr());
                    let e = (self.error)(v);
                    if e.is_null() {
                        value.to_string()
                    } else {
                        format!("ERR:{} @{} ({},{})", text(e), (self.error_cycle)(v), (self.error_u)(v), (self.error_v)(v))
                    }
                })
                .collect();
            (self.destroy)(v);
            values
        }
    }
}

fn files(dir: &Path, extension: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            files(&path, extension, out);
        } else if path.extension().is_some_and(|e| e == extension) {
            out.push(path);
        }
    }
}

struct Case {
    solution: PathBuf,
    puzzle: PathBuf,
}

/// Every solution under `solutions` whose puzzle is under `puzzles`, in path order.
fn discover(oracle: &Library, puzzles: &Path, solutions: &Path, filter: Option<&str>) -> Vec<Case> {
    let mut puzzle_paths = Vec::new();
    files(puzzles, "puzzle", &mut puzzle_paths);
    puzzle_paths.sort();
    let mut by_name: HashMap<Vec<u8>, PathBuf> = HashMap::new();
    for path in puzzle_paths {
        let stem = path.file_stem().unwrap().as_encoded_bytes().to_vec();
        by_name.entry(stem).or_insert(path);
    }
    let mut solution_paths = Vec::new();
    files(solutions, "solution", &mut solution_paths);
    solution_paths.sort();
    let mut unpaired = 0;
    let cases: Vec<Case> = solution_paths
        .into_iter()
        .filter(|path| filter.is_none_or(|f| path.to_string_lossy().contains(f)))
        .filter_map(|solution| {
            let bytes = fs::read(&solution).ok()?;
            let puzzle = oracle.puzzle_name(&bytes).and_then(|name| by_name.get(&name));
            if puzzle.is_none() {
                unpaired += 1;
            }
            Some(Case { puzzle: puzzle?.clone(), solution })
        })
        .collect();
    if unpaired > 0 {
        eprintln!("{unpaired} solutions skipped: no parsable puzzle name, or no such puzzle file");
    }
    cases
}

struct Options {
    libraries: Vec<Library>,
    puzzles: PathBuf,
    solutions: PathBuf,
    metrics: Vec<CString>,
    reps: usize,
    budget: Duration,
    cycle_limit: i32,
    filter: Option<String>,
    out: Option<PathBuf>,
}

fn options() -> Result<Options, String> {
    let mut o = Options {
        libraries: Vec::new(),
        puzzles: PathBuf::new(),
        solutions: PathBuf::new(),
        metrics: Vec::new(),
        reps: 3,
        budget: Duration::from_secs(2),
        cycle_limit: 0,
        filter: None,
        out: None,
    };
    let mut metrics = "cost,instructions,cycles,area".to_string();
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let value = args.next().ok_or(format!("{flag} needs a value"))?;
        let number = |v: &str| v.parse::<u64>().map_err(|e| format!("{flag}: {e}"));
        match flag.as_str() {
            "--lib" => o.libraries.push(Library::open(&value)?),
            "--puzzles" => o.puzzles = value.into(),
            "--solutions" => o.solutions = value.into(),
            "--metrics" => metrics = value,
            "--reps" => o.reps = number(&value)?.max(1) as usize,
            "--budget-ms" => o.budget = Duration::from_millis(number(&value)?),
            "--cycle-limit" => o.cycle_limit = number(&value)? as i32,
            "--filter" => o.filter = Some(value),
            "--out" => o.out = Some(value.into()),
            _ => return Err(format!("unknown option {flag}")),
        }
    }
    if o.libraries.is_empty() || o.puzzles.as_os_str().is_empty() || o.solutions.as_os_str().is_empty() {
        return Err("usage: omsim-bench --lib NAME=PATH[:PREFIX] ... --puzzles DIR --solutions DIR \
[--metrics a,b] [--reps N] [--budget-ms M] [--cycle-limit N] [--filter S] [--out FILE]"
            .into());
    }
    o.metrics = metrics.split(',').map(|m| CString::new(m).unwrap()).collect();
    Ok(o)
}

fn main() -> ExitCode {
    let o = match options() {
        Ok(o) => o,
        Err(e) => {
            eprintln!("{e}");
            return ExitCode::from(2);
        }
    };
    let cases = discover(&o.libraries[0], &o.puzzles, &o.solutions, o.filter.as_deref());
    let mut out = o.out.as_ref().map(|path| {
        let mut f = std::io::BufWriter::new(fs::File::create(path).expect("cannot create --out"));
        let metrics: Vec<_> = o.metrics.iter().map(|m| m.to_string_lossy()).collect();
        writeln!(f, "solution\tpuzzle\tengine\t{}\tmin_ns\tmedian_ns\treps", metrics.join("\t")).unwrap();
        f
    });
    let n = o.libraries.len();
    let (mut total, mut mismatches) = (vec![Duration::ZERO; n], vec![0usize; n]);
    let (started, mut reported, mut printed) = (Instant::now(), Instant::now(), 0);
    for (c, case) in cases.iter().enumerate() {
        let (Ok(puzzle), Ok(solution)) = (fs::read(&case.puzzle), fs::read(&case.solution)) else {
            eprintln!("cannot read {} or {}", case.solution.display(), case.puzzle.display());
            continue;
        };
        let mut values = vec![Vec::new(); n];
        let mut times = vec![Vec::new(); n];
        let case_started = Instant::now();
        for rep in 0..o.reps {
            for i in 0..n {
                let k = (i + rep) % n;
                let t = Instant::now();
                let v = o.libraries[k].run(&puzzle, &solution, o.cycle_limit, &o.metrics);
                times[k].push(t.elapsed());
                if rep == 0 {
                    values[k] = v;
                }
            }
            if rep >= 1 && case_started.elapsed() > o.budget {
                break;
            }
        }
        for k in 0..n {
            times[k].sort();
            total[k] += times[k][0];
            if values[k] != values[0] {
                mismatches[k] += 1;
                if printed < 30 {
                    printed += 1;
                    eprintln!("MISMATCH {} {}: {:?} vs {:?}", case.solution.display(), o.libraries[k].name, values[0], values[k]);
                }
            }
            if let Some(f) = &mut out {
                let reps = times[k].len();
                writeln!(
                    f,
                    "{}\t{}\t{}\t{}\t{}\t{}\t{reps}",
                    case.solution.display(),
                    case.puzzle.file_stem().unwrap().to_string_lossy(),
                    o.libraries[k].name,
                    values[k].join("\t"),
                    times[k][0].as_nanos(),
                    times[k][reps / 2].as_nanos()
                )
                .unwrap();
            }
        }
        if reported.elapsed() > Duration::from_secs(10) || c + 1 == cases.len() {
            reported = Instant::now();
            let elapsed = started.elapsed().as_secs_f64();
            eprintln!(
                "[{}/{}] {elapsed:.0}s elapsed, ETA {:.0}s, mismatches {}",
                c + 1,
                cases.len(),
                elapsed / (c + 1) as f64 * (cases.len() - c - 1) as f64,
                mismatches.iter().sum::<usize>()
            );
        }
    }
    if let Some(mut f) = out {
        f.flush().unwrap();
    }
    eprintln!("done: {} solutions, {:.1}s", cases.len(), started.elapsed().as_secs_f64());
    for k in 0..n {
        eprintln!(
            "  {:<12} total(min) {:9.3}s  speed-up vs {} {:.3}x  mismatches {}",
            o.libraries[k].name,
            total[k].as_secs_f64(),
            o.libraries[0].name,
            total[0].as_secs_f64() / total[k].as_secs_f64(),
            mismatches[k]
        );
    }
    if mismatches.iter().sum::<usize>() > 0 { ExitCode::FAILURE } else { ExitCode::SUCCESS }
}
