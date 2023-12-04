# Advent of Code 2023
Solutions are sorted by programming language with an additional `common` directory of containing inputs.

## Programming Languages
### Rust
The solutions are written intending to target `wasm32-unknown-unknown`. Two runners are intended to be included: one utilizing Wasmtime and the other being a simple HTML page with JavaScript bindings to run the compiled Wasm.
#### Building and Running
The following assumes the current working directory to be `[year]/rust`.

A script (`run.sh`) will be included that takes a `[day]` (and optionally, `share`) as input and runs:  
`cargo build --release --target=wasm32-unknown-unknown -p [day]`  
`cargo run --release -p runner-native [day] <share>`
### Inko
An attempt to setup an environment for this programming language was made, but I encountered dependency issues. Specifically, Inko's version manager, `ivm`, would install fine through the `cargo install ivm` command, but then fail to install Inko itself. `ivm` fails to build Inko version `0.13.1` because of an unknown feature `proc_macro_span_shrink` in `proc-macro2` version `1.0.58`. Attempting to build Inko from source results in `llvm-sys` version `150.1.2` failing to build (seemingly) because I have a more recent version of `llvm` installed and don't feel like downgrading at the moment.

Unfortunately, simply getting the compiler and/or runtime to actually work (on my system: Fedora 38, Rust 1.76.0, llvm 16.0.6) seems like it would be more involved than I had anticipated. I'm left with a bit of a bad impression, but maybe I'll put forth the effort to get it working at some point in the future. Even better, perhaps it'll just work a year from now.