extern crate semver;

use semver::{Version, VersionReq};
use semver::ParseError::IncorrectParse;
use std::borrow::Cow;
use std::path::PathBuf;
use std::str::from_utf8_unchecked;
use std::process::Command;

fn is_whitespace(x: &u8) -> bool {
    ['\n', '\t', ' '].contains(&(*x as char))
}

fn build_wrappers() -> (String, &'static str) {
    // Get Cargo directories
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut src_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    src_dir.push("wrappers");

    // Run cmake generator
    match Command::new("cmake").arg(&src_dir).current_dir(&out_dir).status() {
        Err(e) => panic!("Failed to invoke CMake: {}", e),
        Ok(s) => {
            if !s.success() {
                panic!("CMake configuration of wrappers failed with status {}", s)
            }
        }
    }

    // Do the actual build
    match Command::new("cmake").arg("--build").arg(&out_dir).status() {
        Err(e) => panic!("Failed to invoke CMake: {}", e),
        Ok(s) => {
            if !s.success() {
                panic!("CMake build of wrappers failed with status {}", s)
            }
        }
    }

    (out_dir.into_os_string().into_string().unwrap(),
     "targetwrappers")
}

fn main() {
    let (wrappers_out_dir, wrappers_lib_name) = build_wrappers();
    println!("cargo:rustc-link-search=native={}", wrappers_out_dir);
    println!("cargo:rustc-link-lib=static={}", wrappers_lib_name);

    let minimum_llvm_version = VersionReq::parse(">=3.6").unwrap();
    let (llvm_config, version) = get_llvm_config();
    if minimum_llvm_version.matches(&version) {
        println!("Found LLVM version {}", version);
    } else {
        panic!("LLVM version 3.6 or higher is required. (Found {})", version);
    };

    // Are we using LLVM as a shared object or static library?
    let llvm_libtype = match std::env::var("CARGO_FEATURE_LLVM_DYLIB") {
        Ok(_) => "dylib",
        Err(_) => "static"
    };


    // llvm-config --ldflags: extract -L<dir> options
    let output = Command::new(&*llvm_config).arg("--ldflags")
        .output().ok().expect("Failed to get LDFLAGS from llvm-config").stdout;
    for arg in output.split(is_whitespace) {
        if arg.starts_with(b"-L") {
            println!("cargo:rustc-link-search=native={}", unsafe {
                from_utf8_unchecked(&arg[2..])
            });
        }
    }

    // llvm-config --libs --system-libs: extract -l<lib> options
    let output = Command::new(&*llvm_config).args(&["--libs", "--system-libs"])
        .output().ok().expect("Failed to get libs from llvm-config").stdout;
    for arg in output.split(is_whitespace) {
        if arg.starts_with(b"-l") {
            let arg = &arg[2..];
            let libtype = if arg.starts_with(b"LLVM") {
                llvm_libtype
            } else {
                "dylib"
            };
            println!("cargo:rustc-link-lib={}={}", libtype, unsafe {
                from_utf8_unchecked(arg)
            });
        }
    }

    // llvm-config --cxxflags: determine which libc++ to use: LLVM's or GCC's
    let output = String::from_utf8(
        Command::new(&*llvm_config).arg("--cxxflags")
            .output().ok().expect("Failed to get CXXFLAGS from llvm-config").stdout
    ).ok().expect("llvm-config --cxxflags gave invalid UTF-8");
    let libcpp = if output.contains("stdlib=libc++") {
        "c++"
    } else {
        "stdc++"
    };
    println!("cargo:rustc-link-lib={}", libcpp);
}

fn get_llvm_config() -> (Cow<'static, str>, Version) {
    match Command::new("llvm-config").arg("--version").output() {
        Ok(x) => {
            // llvm-config was on our PATH. Easy.
            let ver = match Version::parse(&String::from_utf8_lossy(&*x.stdout)) {
                // Ignore partial error; particularly constructs like '3.8.0svn'
                // should be accepted, despite being invalid semver.
                Err(IncorrectParse(v, _)) | Ok(v) => v,
                _ => panic!("Could not determine LLVM version")
            };
            (Cow::Borrowed("llvm-config"), ver)
        }
        Err(_) => {
            panic!("llvm-config not found. Install LLVM before attempting to build.");
        }
    }
}
