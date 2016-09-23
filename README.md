Rust bindings to LLVM's C API.

# Usage

```toml
[dependencies]
llvm-sys = "39"
```

There must be a compatible version of LLVM available. By default `llvm-sys`
will look for `llvm-config` on `PATH` to find a system-wide copy of LLVM and
use that if it is a compatible version. Custom options for finding LLVM
on your system can be specified via environment variables. See
[LLVM compatibility](#llvm-compatibility) for more information.

## Documentation

See the `examples` directory in this repository for API examples. There also
exist some other projects using these bindings which may be
informative:

 * [Tari's merthc](https://bitbucket.org/tari/merthc)
 * [Wilfred's BF compiler](https://crates.io/crates/bfc)

Most of the interfaces are not documented in these bindings. Refer to the
[LLVM documentation](http://llvm.org/docs/) for more information, particularly
the [generated API documentation](http://llvm.org/doxygen/).

If you have your own project using these bindings that you think is worth
mentioning here, by all means let me know.

## LLVM compatibility

Because the LLVM C [API stability guarantees][c-api-stability] are so weak, this
crate enforces that the LLVM release in use exactly match the one it was made
for. The crate version is constructed by treating the LLVM version as a real
number and multiplying by 10, ignoring any fractional part. Thus `llvm-sys`
version 37 is compatible with LLVM 3.7.x, and `llvm-sys` 41 would be compatible
with LLVM 4.1.x.

[c-api-stability]: http://llvm.org/releases/3.8.0/docs/DeveloperPolicy.html#c-api-changes

Due to this strictness with versioning, it may be difficult or even impossible
to provide a compatible LLVM version system-wide for a given project (consider
a program using two libraries that internally use different versions of LLVM!)
so environment variables can be set to help the build scripts find your copy
of the libraries.

`LLVM_SYS_<version>_PREFIX` specifies the install prefix for a compiled and
installed copy of the libraries, where `<version>` is the major version of
`llvm-sys` (for example, `LLVM_SYS_37_PREFIX`). For information on compiling
a copy of LLVM yourself, see [Compiling LLVM](#compiling-llvm).

In the future this library may offer the ability to download and compile LLVM
automatically, but such a feature should only be considered for building
one-off releases because its high cost is ill-suited to repeated builds.

## Compiling LLVM

While the [getting started guide](http://llvm.org/docs/GettingStarted.html) is
the official guide to compiling LLVM, this section will attempt to provide
minimum guidance in creating usable libraries. If you encounter problems, refer
to the official documentation.

### Download sources

Download and unpack a copy of the source for the required version.

```sh
wget https://llvm.org/releases/3.9.0/llvm-3.9.0.src.tar.xz
tar xJf llvm-3.9.0.src.tar.xz
```

Note that you do not need to compile Clang or the test suite.

### Configure

Configure the build using [CMake][cmake] (you will need a copy of CMake
installed).

[cmake]: https://cmake.org/

```sh
mkdir -p llvm-3.9.0.src/build
cd llvm-3.9.0.src/build
cmake .. -DCMAKE_INSTALL_PREFIX=$HOME/llvm-3.9.0
```

Some of the useful options that can be specified at configuration-time
(via `-D` passed to CMake):

 * `CMAKE_INSTALL_PREFIX` sets the location to install everything in the install
   step (later). In the above example this is under your home directory.
 * `CMAKE_BUILD_TYPE` specifies the build mode, one of Debug, Release,
   MinSizeRel or RelWithDebInfo. Unless you plan to debug LLVM itself,
   Release or MinSizeRel is probably a good choice.
 * `LLVM_ENABLE_ASSERTIONS` enables internal sanity checks, highly recommended
   when writing and debugging your own program that uses LLVM because it can
   detect many usage errors that could otherwise result in hard-to-debug
   crashes.

Passing `-G <generator>` to CMake will make it use a different build system, but
by default it will select one appropriate to your system. If you have
[ninja][ninja] available, it is recommended due to its speed (`-G Ninja`).

[ninja]: https://ninja-build.org/

### Compile and install

```sh
cmake --build . --target install
```

This will automatically invoke the build system and copy binaries into the
prefix specified at configuration-time when done. Then you can compile llvm-sys
against it.

```sh
cd your/crate/path
LLVM_SYS_39_PREFIX=$HOME/llvm-3.9.0 cargo build
```

## Windows

You must use a version of Rust that uses the same compiler as you build LLVM
with, either MSVC or MinGW. Fortunately, a mismatch like this will cause errors
at compile-time when llvm-config provides options which are supported by only
one of them, so if you're using the other it will cause the build to fail.

## Cross-compilation

Will theoretically work, but hasn't been tested. Let us know if you try.
