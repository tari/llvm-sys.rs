extern crate gcc;
extern crate semver;

use semver::{Version, VersionReq};
use semver::ParseError::IncorrectParse;
use std::process::Command;

/// Get the output from running `llvm-config` with the given argument.
fn llvm_config(arg: &str) -> String {
    let stdout = Command::new("llvm-config")
        .arg(arg)
        .output()
        .unwrap_or_else(|e| panic!("Couldn't execute llvm-config. Error: {}", e))
        .stdout;

    String::from_utf8(stdout).ok().expect("llvm-config output was not UTF-8.")
}

/// Get the LLVM version using llvm-config.
fn llvm_version() -> Version {
    match Version::parse(&llvm_config("--version")) {
        // Ignore partial error; particularly constructs like '3.8.0svn' should be accepted,
        // despite being invalid semver.
        Err(IncorrectParse(v, _)) | Ok(v) => v,
        _ => panic!("Could not determine LLVM version from llvm-config."),
    }
}

fn main() {
    // Check for LLVM 3.6 or greater.
    let minimum_llvm_version = VersionReq::parse(">=3.6").unwrap();
    let version = llvm_version();
    if minimum_llvm_version.matches(&version) {
        println!("Found LLVM version {}", version);
    } else {
        panic!("LLVM version 3.6 or higher is required. (Found {})", version);
    };

    // Parse library linking flags from llvm-config.
    for arg in llvm_config("--ldflags").split_whitespace() {
        if arg.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &arg[2..]);
        }
    }

    for arg in llvm_config("--libs").split_whitespace() {
        if arg.starts_with("-l") {
            println!("cargo:rustc-link-lib=static={}", &arg[2..]);
        }
    }

    for arg in llvm_config("--system-libs").split_whitespace() {
        if arg.starts_with("-l") {
            println!("cargo:rustc-link-lib=dylib={}", &arg[2..]);
        }
    }

    // This breaks the link step on Windows with MSVC.
    if !cfg!(windows) {
        // Determine which C++ standard library to use: LLVM's or GCC's.
        let cxxflags = llvm_config("--cxxflags");
        let libcpp = if cxxflags.contains("stdlib=libc++") { "c++" } else { "stdc++" };
        println!("cargo:rustc-link-lib={}", libcpp);
    }

    // Build the extra wrapper functions.
    std::env::set_var("CFLAGS", llvm_config("--cflags").trim());
    gcc::compile_library("libtargetwrappers.a", &["wrappers/target.c"]);
}
