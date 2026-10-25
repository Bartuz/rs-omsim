//! `omsim` on the Rust engine: a port of `main.c` with the same options and output.
use omsim_native::{parse::*, verifier::*};
use std::{
    env,
    ffi::{CStr, CString, OsString, c_char, c_int, c_void},
    io::{self, Write},
    os::unix::ffi::OsStringExt,
    process::ExitCode,
};

const USAGE: &str = "Usage: omsim [--metric <metric name>]* [--disable-limits] \
[--puzzle-file <file> | --puzzle-folder <folder>] solution(s)...\n";

const OPTIONS: &str = "Options:
  -p --puzzle-file <file>     Specify a puzzle file
  -f --puzzle-folder <folder> Specify a folder containing puzzles
  -m --metric <name>          Evaluate the given metric
  -i --output-intervals       Print output intervals
  -s --steady-state           Simulate up to steady-state
  -d --disable-limits         Disable simulation limits
  -h --help                   Show this help message and exit
";

/// (long name, short name, takes an argument), as in main.c's getopt_long table.
const LONG_OPTIONS: [(&str, u8, bool); 7] = [
    ("puzzle-file", b'p', true),
    ("puzzle-folder", b'f', true),
    ("metric", b'm', true),
    ("output-intervals", b'i', false),
    ("steady-state", b's', false),
    ("disable-limits", b'd', false),
    ("help", b'h', false),
];

#[derive(Default)]
struct Options {
    puzzle_file: Option<CString>,
    puzzle_folder: Option<Vec<u8>>,
    metrics: Vec<CString>,
    steady_state: bool,
    disable_limits: bool,
    print_output_intervals: bool,
    solutions: Vec<CString>,
}

enum Parsed {
    Run(Options),
    Exit(u8),
}

fn cstring(bytes: Vec<u8>) -> CString {
    // Arguments come from argv, which cannot contain NUL bytes.
    CString::new(bytes).expect("argument contains a NUL byte")
}

/// GNU getopt_long as main.c uses it: options and operands may be interleaved, long
/// options may be abbreviated or written `--name=value`, and short options may be grouped.
fn parse_args(program: &str, args: Vec<OsString>) -> Parsed {
    let mut o = Options::default();
    let mut operands = Vec::new();
    let mut args = args.into_iter().map(OsString::into_vec);
    let fail = |message: String| {
        eprintln!("{program}: {message}");
        eprint!("{USAGE}{OPTIONS}");
        Parsed::Exit(255)
    };
    while let Some(arg) = args.next() {
        let (short, mut value): (u8, Option<Vec<u8>>);
        if arg == b"--" {
            operands.extend(args.by_ref());
            break;
        } else if let Some(long) = arg.strip_prefix(b"--") {
            let (name, inline) = match long.iter().position(|&b| b == b'=') {
                Some(i) => (&long[..i], Some(long[i + 1..].to_vec())),
                None => (long, None),
            };
            let exact = LONG_OPTIONS.iter().find(|(n, ..)| n.as_bytes() == name);
            let mut prefixed = LONG_OPTIONS.iter().filter(|(n, ..)| n.as_bytes().starts_with(name));
            let shown = String::from_utf8_lossy(name);
            let &(long_name, s, takes_argument) = match (exact, prefixed.next(), prefixed.next()) {
                (Some(option), ..) => option,
                (None, Some(option), None) => option,
                (None, Some(_), Some(_)) => {
                    let all = LONG_OPTIONS.iter().filter(|(n, ..)| n.as_bytes().starts_with(name));
                    let all: Vec<_> = all.map(|(n, ..)| format!("'--{n}'")).collect();
                    let arg = String::from_utf8_lossy(long);
                    return fail(format!("option '--{arg}' is ambiguous; possibilities: {}", all.join(" ")));
                }
                (None, None, _) => return fail(format!("unrecognized option '--{shown}'")),
            };
            if !takes_argument && inline.is_some() {
                return fail(format!("option '--{long_name}' doesn't allow an argument"));
            }
            value = inline;
            if takes_argument && value.is_none() {
                value = args.next();
                if value.is_none() {
                    return fail(format!("option '--{long_name}' requires an argument"));
                }
            }
            short = s;
            if let Some(exit) = apply(&mut o, short, value) {
                return exit;
            }
        } else if arg.len() > 1 && arg[0] == b'-' {
            let mut j = 1;
            while j < arg.len() {
                let c = arg[j];
                let Some(&(_, s, takes_argument)) = LONG_OPTIONS.iter().find(|o| o.1 == c) else {
                    return fail(format!("invalid option -- '{}'", c as char));
                };
                value = None;
                if takes_argument {
                    value = if j + 1 < arg.len() { Some(arg[j + 1..].to_vec()) } else { args.next() };
                    if value.is_none() {
                        return fail(format!("option requires an argument -- '{}'", c as char));
                    }
                    j = arg.len();
                } else {
                    j += 1;
                }
                if let Some(exit) = apply(&mut o, s, value) {
                    return exit;
                }
            }
        } else {
            operands.push(arg);
        }
    }
    if o.puzzle_file.is_some() && o.puzzle_folder.is_some() {
        eprintln!("cannot specify both -p|--puzzle-file and -f|--puzzle-folder");
        return Parsed::Exit(255);
    } else if o.puzzle_file.is_none() && o.puzzle_folder.is_none() {
        eprintln!("must specify either -p|--puzzle-file or -f|--puzzle-folder");
        return Parsed::Exit(255);
    }
    if operands.is_empty() {
        eprint!("{USAGE}");
        return Parsed::Exit(255);
    }
    o.solutions = operands.into_iter().map(cstring).collect();
    Parsed::Run(o)
}

fn apply(o: &mut Options, option: u8, value: Option<Vec<u8>>) -> Option<Parsed> {
    match option {
        b'm' => {
            if o.metrics.len() >= 64 {
                eprintln!("too many metrics");
                return Some(Parsed::Exit(255));
            }
            o.metrics.push(cstring(value.unwrap()));
        }
        b'i' => o.print_output_intervals = true,
        b'd' => o.disable_limits = true,
        b'p' => o.puzzle_file = Some(cstring(value.unwrap())),
        b'f' => o.puzzle_folder = value,
        b's' => o.steady_state = true,
        _ => {
            print!("{USAGE}{OPTIONS}");
            return Some(Parsed::Exit(0));
        }
    }
    None
}

/// C's `printf("%g", x)`, through the C library so every corner case formats the same.
fn g(x: f64) -> String {
    unsafe extern "C" {
        fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    }
    let mut buffer = [0u8; 64];
    // SAFETY: the buffer outlives the call, and its length bounds the write.
    unsafe { snprintf(buffer.as_mut_ptr().cast(), buffer.len(), c"%g".as_ptr(), x) };
    CStr::from_bytes_until_nul(&buffer).unwrap().to_string_lossy().into_owned()
}

/// main.c's print_verifier_error.
unsafe fn print_verifier_error(out: &mut impl Write, v: *mut c_void) -> io::Result<()> {
    unsafe {
        let error = rs_verifier_error(v);
        if !error.is_null() {
            out.write_all(CStr::from_ptr(error).to_bytes())?;
            if rs_verifier_error_cycle(v) != 0 {
                write!(
                    out,
                    " on cycle {} at {} {}",
                    rs_verifier_error_cycle(v),
                    rs_verifier_error_location_u(v),
                    rs_verifier_error_location_v(v)
                )?;
            }
            writeln!(out)?;
        }
    }
    Ok(())
}

/// The folder lookup of main.c: `<folder>/<puzzle name in the solution>.puzzle`.
fn puzzle_in_folder(folder: &[u8], solution_path: &CStr) -> Option<CString> {
    // SAFETY: the parser reads the named file; its result is freed below.
    unsafe {
        let sf = rs_parse_solution_file(solution_path.as_ptr());
        if sf.is_null() {
            return None;
        }
        let name = std::slice::from_raw_parts((*sf).puzzle.bytes, (*sf).puzzle.length);
        // snprintf's "%.*s" stops at a NUL byte.
        let name = name.split(|&b| b == 0).next().unwrap_or_default();
        let path = [folder, b"/", name, b".puzzle"].concat();
        rs_free_solution_file(sf);
        Some(cstring(path))
    }
}

fn run(o: &Options, out: &mut impl Write) -> io::Result<u8> {
    let mut retval = 0;
    for solution_path in &o.solutions {
        let puzzle_path = match (&o.puzzle_file, &o.puzzle_folder) {
            (Some(file), _) => file.clone(),
            (None, Some(folder)) => match puzzle_in_folder(folder, solution_path) {
                Some(path) => path,
                None => {
                    eprintln!(
                        "couldn't find puzzle name in solution file {}",
                        solution_path.to_string_lossy()
                    );
                    retval = 255;
                    continue;
                }
            },
            (None, None) => unreachable!(),
        };
        // SAFETY: v is a live verifier from here to its destroy at the end of the body.
        unsafe {
            let v = rs_verifier_create(puzzle_path.as_ptr(), solution_path.as_ptr());
            if !rs_verifier_error(v).is_null() {
                print_verifier_error(out, v)?;
                rs_verifier_destroy(v);
                retval = 255;
                continue;
            }
            if o.disable_limits {
                rs_verifier_disable_limits(v);
            }
            if o.solutions.len() > 1 {
                out.write_all(solution_path.to_bytes())?;
                writeln!(out, ":")?;
            }
            if !o.metrics.is_empty() || o.print_output_intervals {
                for metric in &o.metrics {
                    let value = rs_verifier_evaluate_approximate_metric(v, metric.as_ptr());
                    write!(out, "{}: ", metric.to_string_lossy())?;
                    if !rs_verifier_error(v).is_null() {
                        print_verifier_error(out, v)?;
                        retval = 255;
                        rs_verifier_error_clear(v);
                    } else {
                        writeln!(out, "{}", g(value))?;
                    }
                }
                if o.print_output_intervals {
                    write!(out, "output intervals: ")?;
                    let n = rs_verifier_number_of_output_intervals(v);
                    let mut r = rs_verifier_output_intervals_repeat_after(v);
                    if r < 0 {
                        r = n;
                    }
                    if !rs_verifier_error(v).is_null() {
                        print_verifier_error(out, v)?;
                        retval = 255;
                        rs_verifier_error_clear(v);
                    } else {
                        let mut printed_bracket = false;
                        for i in 0..n {
                            if i > 0 {
                                write!(out, " ")?;
                            }
                            if i >= r && !printed_bracket {
                                write!(out, "[")?;
                                printed_bracket = true;
                            }
                            write!(out, "{}", rs_verifier_output_interval(v, i))?;
                        }
                        if printed_bracket {
                            write!(out, "]")?;
                        }
                        writeln!(out)?;
                    }
                }
            } else if !summary(o, out, v)? {
                retval = 255;
            }
            rs_verifier_destroy(v);
        }
    }
    out.flush()?;
    Ok(retval)
}

/// The default output, `<cost>g/<instructions>i@0 <cycles>c/<area>a@V [<rate>r@INF]`.
/// Returns false where main.c sets its exit status to -1.
unsafe fn summary(o: &Options, out: &mut impl Write, v: *mut c_void) -> io::Result<bool> {
    unsafe {
        let cost = rs_verifier_evaluate_metric(v, c"cost".as_ptr());
        if !rs_verifier_error(v).is_null() {
            print_verifier_error(out, v)?;
            return Ok(false);
        }
        let instructions = rs_verifier_evaluate_metric(v, c"instructions".as_ptr());
        write!(out, "{cost}g/{instructions}i@0")?;
        let cycles = rs_verifier_evaluate_metric(v, c"cycles".as_ptr());
        if !rs_verifier_error(v).is_null() {
            writeln!(out)?;
            print_verifier_error(out, v)?;
            return Ok(false);
        }
        let area = rs_verifier_evaluate_metric(v, c"area".as_ptr());
        write!(out, " {cycles}c/{area}a@V")?;
        if o.steady_state {
            let outputs = rs_verifier_evaluate_metric(v, c"per repetition outputs".as_ptr());
            if !rs_verifier_error(v).is_null() {
                // main.c prints this error but keeps its exit status.
                writeln!(out)?;
                print_verifier_error(out, v)?;
            } else {
                let cycles = rs_verifier_evaluate_metric(v, c"per repetition cycles".as_ptr());
                writeln!(out, " {}r@INF", g(f64::from(cycles) / f64::from(outputs)))?;
            }
        } else {
            writeln!(out)?;
        }
    }
    Ok(true)
}

fn main() -> ExitCode {
    let mut args = env::args_os();
    let program = args.next().map(|p| p.to_string_lossy().into_owned()).unwrap_or_default();
    let options = match parse_args(&program, args.collect()) {
        Parsed::Run(options) => options,
        Parsed::Exit(code) => return ExitCode::from(code),
    };
    // Line-buffered, like C's stdout on a terminal, so it interleaves with stderr the same.
    match run(&options, &mut io::stdout().lock()) {
        Ok(code) => ExitCode::from(code),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => ExitCode::from(255),
        Err(error) => {
            eprintln!("omsim: {error}");
            ExitCode::from(255)
        }
    }
}
