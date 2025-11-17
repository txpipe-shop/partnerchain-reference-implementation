# Troubleshooting

| Previous                                            | Up                         |
|-----------------------------------------------------|----------------------------|
| [Operating instructions](operating_instructions.md) | [Tutorial root](README.md) |

These are some common errors that can happen when developing on Substrate:

## `std` related issues

Errors like:

- ``Double lang item in crate <crate> (which `std`/ `serde` depends on):...``
- `Attempted to define built-in macro more than once`,

happen commonly when using std crates in a non-std environment, like Substrate's runtime. `std` crates can't be used because we compile to WASM. If you run into an error like this and the crate you are using is no-std, make sure you are setting them up correctly. For example, make sure that the dependency is imported with `default-features = false` or that the `std` feature is set correctly in the respective `Cargo.toml`. If you are writing a new module, make sure that it is premised by `#![cfg_attr(not(feature = "std"), no_std)]`.

## `alloc` feature

When trying to use `alloc` features like `vec`, you might run into the trouble that the compiler can't find the `alloc` crate. This feature can be imported from various dependencies like `serde` and `serde_json`. To use it make sure to add `extern crate alloc;` at the top of your file.

| Previous                                            | Up                         |
|-----------------------------------------------------|----------------------------|
| [Operating instructions](operating_instructions.md) | [Tutorial root](README.md) |


## ``Failed to deserialize `.../minimal_template_runtime.wasm`: UnknownOpcode(192)``

An error like the above might arise while building with the `--release` flag while you are modifying
the node (`debug` builds rarely produce it). This might persist even if you complete all
modifications indicated.

In such situations, best thing to do is start a build from scratch by issuing a `cargo clean --release` command,
delete `Cargo.lock` and release-build again. That should do it.

<!-- Local Variables: -->
<!-- mode: Markdown -->
<!-- ispell-local-dictionary: "american" -->
<!-- fill-column: 100 -->
<!-- End: -->
