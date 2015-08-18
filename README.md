Rust bindings to LLVM's C API.

# Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
llvm-sys = "0.2.0"
```

and this to your crate root:

```rust
extern crate llvm_sys;
```

See the `examples` directory in this repository for API examples.

Most of the interfaces are not documented in these bindings. Refer to the
[LLVM documentation](http://llvm.org/docs/) for more information, particularly
the [generated API documentation](http://llvm.org/doxygen/).

# Dependencies

You will need LLVM (>= 3.6) installed on your system to compile these bindings.
The `llvm-config` tool must be on PATH so the build scripts can find it.

## Why not use `librustc_llvm`?

In many cases, the interfaces exposed by `librustc_llvm` are sufficient for
code generation or whatever else you need to do with LLVM. When they are
not, however, you must link to LLVM yourself, which if mixed with rustc's
LLVM runs a significant risk of library version mismatches. Users should
take care not to mix uses of the two crates for this reason.

Additionally, `rustc_llvm` is a private API for the Rust compiler and is subject
to change without notice. This crate provides a stable API.
