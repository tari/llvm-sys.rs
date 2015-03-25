use std::process::Command;

fn main() {
    let whitespace = ['\n', '\t', ' '];
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

    let version = String::from_utf8(output.stdout).unwrap();
    println!("Found LLVM version {}", version);

    // llvm-config --ldflags: extract -L<dir> options
    let output = String::from_utf8(
        Command::new("llvm-config").arg("--ldflags").output().unwrap().stdout
    ).unwrap();
    for arg in output.split(&whitespace[..]) {
        let arg = arg.trim();
        if arg.starts_with("-L") {
            println!("cargo:rustc-link-search=native={}", &arg[2..]);
        }
    }

    // llvm-config --libs --system-libs: extract -l<lib> options
    let output = String::from_utf8(
        Command::new("llvm-config").args(&["--libs", "--system-libs"]).output().unwrap().stdout
    ).unwrap();
    for arg in output.split(&whitespace[..]) {
        if arg.starts_with("-l") {
            let arg = &arg[2..];
            let libtype = if arg.contains("LLVM") {
                llvm_libtype
            } else {
                "dylib"
            };
            println!("cargo:rustc-link-lib={}={}", libtype, arg);
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
