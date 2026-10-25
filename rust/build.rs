// With the `reference` feature, the C at the root of this repository is linked into the
// tests as their oracle. Its symbols are unprefixed and the Rust engine's carry `rs_`, so
// both engines live in one test binary.
fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    if std::env::var_os("CARGO_FEATURE_REFERENCE").is_none() {
        return;
    }
    let mut build = cc::Build::new();
    build.include("..").flag_if_supported("-std=c11");
    for module in [
        "collision",
        "decode",
        "parse",
        "sim",
        "steady-state",
        "verifier",
    ] {
        println!("cargo:rerun-if-changed=../{module}.c");
        println!("cargo:rerun-if-changed=../{module}.h");
        build.file(format!("../{module}.c"));
    }
    println!("cargo:rerun-if-changed=tests/support/oracle.c");
    build.file("tests/support/oracle.c");
    build.warnings(false).compile("omsim_reference");
    println!("cargo:rustc-link-lib=m");
}
