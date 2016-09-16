Rust bindings to LLVM's C API.

# Usage

```toml
[dependencies]
llvm-sys = "0.4.0"
```

You will need LLVM (>= 3.6) installed on your system to compile these bindings.
The `llvm-config` tool must be on PATH so the build scripts can find it.

See the `examples` directory in this repository for API examples. There also
exist some other projects using these bindings which may be
informative<sup>1</sup>:

 * [Tari's merthc](https://bitbucket.org/tari/merthc)
 * [Wilfred's BF compiler](https://crates.io/crates/bfc)

Most of the interfaces are not documented in these bindings. Refer to the
[LLVM documentation](http://llvm.org/docs/) for more information, particularly
the [generated API documentation](http://llvm.org/doxygen/).

<sup>1</sup> If you have your own project using these bindings that you think
is worth mentioning here, by all means let me know.

# Tips

When developing against LLVM, use of a build with debug assertions enabled is
highly recommended. It is often unclear how ownership moves around the system
when using many different features, and debug assertions can help detect errors
before they cause hard-to-debug crashes.

## API versioning

Note that currently there is no API versioning, so newer versions of LLVM may
not work as expected. These should manifest as link-time errors in most cases,
but [LLVM's API stability guarantees][c-api-stability] are not very strong so
using newer versions of the library may cause more esoteric bugs (none are known
to exist, but be aware).

[c-api-stability]: http://llvm.org/releases/3.8.0/docs/DeveloperPolicy.html#c-api-changes

## Windows

LLVM supports Windows and [others have had success][issue-6] using llvm-sys
on that platforms, but setting it up is not very easy. The binaries distributed
[on llvm.org][llvm-downloads] are not suitable for building against LLVM as
a library, so you'll probably need to compile it yourself. Ensure that the
compiler you use for Rust and LLVM are the same (MSVC or MinGW).

[issue-6]: https://bitbucket.org/tari/llvm-sys.rs/issues/6/build-on-windows
[llvm-downloads]: http://llvm.org/releases/download.html

## Why not use `librustc_llvm`?

In many cases, the interfaces exposed by `librustc_llvm` are sufficient for
code generation or whatever else you need to do with LLVM. When they are
not, however, you must link to LLVM yourself, which if mixed with rustc's
LLVM runs a significant risk of library version mismatches. Users should
take care not to mix uses of the two crates for this reason.

Additionally, `rustc_llvm` is a private API for the Rust compiler and is subject
to change without notice. This crate provides a stable API.
