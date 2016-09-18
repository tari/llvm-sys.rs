extern crate gcc;
#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate semver;

use regex::Regex;
use semver::Version;
use std::process::Command;

/// Get the output from running `llvm-config` with the given argument.
fn llvm_config(arg: &str) -> String {
    let stdout = Command::new("llvm-config")
        .arg(arg)
        .output()
        .expect("Couldn't execute llvm-config")
        .stdout;

    String::from_utf8(stdout).expect("llvm-config output was not UTF-8.")
}

/// Get the LLVM version using llvm-config.
fn llvm_version() -> Version {
    let version_str = llvm_config("--version");

    let re = Regex::new(r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)")
        .unwrap();
    let (start, end) = re.find(&version_str)
        .expect("Could not determine LLVM version from llvm-config.");

    Version::parse(&version_str[start..end]).unwrap()
}

lazy_static! {
    static ref CRATE_VERSION: Version = Version::parse(env!("CARGO_PKG_VERSION"))
        .expect("Crate version is somehow not valid semver");
}

fn main() {
    let target_ver = Version {
        major: CRATE_VERSION.major / 10,
        minor: CRATE_VERSION.major % 10,
        patch: 0, pre: vec![], build: vec![]
    };
    let llvm_ver = llvm_version();

    // Check discovered LLVM version and warn if it does not match
    // the crate version.
    // We'd prefer to make version mismatches a hard error because we can't
    // reasonably detect incompatiblities here, but since setting up LLVM is
    // non-trivial prefer to warn. The API is largely stable across versions
    // and they don't change the semantics of existing functions (fortunately),
    // so errors should be caught at link-time.
    //
    // In the future we'd like to provide an option to compile a version of LLVM
    // precisely matching what the crate expects so these kinds of problems can
    // be worked around with relative ease.
    if target_ver.major != llvm_ver.major || target_ver.minor != llvm_ver.minor {
        println!("cargo:warning={} version {} expects LLVM {}, but found {}",
                 env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"), target_ver, llvm_ver);
        println!("cargo:warning=  Depending on the API features in use, required functions may not exist.");
        println!("cargo:warning=  If you observe LLVM-related linker errors, this is probably the cause.");
    }

    // Parse library linking flags from llvm-config.
    for arg in llvm_config("--ldflags").split_whitespace() {
        if arg.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &arg[2..]);
        }
    }

    for arg in llvm_config("--libs").split_whitespace() {
        if arg.starts_with("-l") {
            println!("cargo:rustc-link-lib={}", &arg[2..]);
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
