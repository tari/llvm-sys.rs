use std::str::from_utf8_unchecked;
use std::process::Command;

fn is_whitespace(x: &u8) -> bool {
    ['\n', '\t', ' '].contains(&(*x as char))
}

fn main() {
    // Are we using LLVM as a shared object or static library?
    let llvm_libtype = match std::env::var("CARGO_FEATURE_LLVM_DYLIB") {
        Ok(_) => "dylib",
        Err(_) => "static"
    };
    
    // Is llvm-config available?
    let output = match Command::new("llvm-config").arg("--version").output() {
        // TODO kick off the whole process to download and compile LLVM
        Err(e) => panic!("System LLVM not found and compiling is not (yet) supported: {:?}", e),
        Ok(x) => x
    };

    let version = output.stdout;
    println!("Found LLVM version {}", unsafe {
        // We really just want to write bytes to stdout (no guarantees the system
        // is UTF-8), so it's easiest to just do this unchecked conversion rather
        // than use std::io to write raw bytes.
        from_utf8_unchecked(&version)
    });

    // llvm-config --ldflags: extract -L<dir> options
    let output = Command::new("llvm-config").arg("--ldflags").output().unwrap().stdout;
    for arg in output.split(is_whitespace) {
        if arg.starts_with(b"-L") {
            println!("cargo:rustc-link-search=native={}", unsafe {
                from_utf8_unchecked(&arg[2..])
            });
        }
    }

    // llvm-config --libs --system-libs: extract -l<lib> options
    let output = Command::new("llvm-config").args(&["--libs", "--system-libs"]).output().unwrap().stdout;
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
        Command::new("llvm-config").arg("--cxxflags").output().unwrap().stdout
    ).unwrap();
    let libcpp = if output.contains("stdlib=libc++") {
        "c++"
    } else {
        "stdc++"
    };
    println!("cargo:rustc-link-lib={}", libcpp);
}
