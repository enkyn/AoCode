# Advent of Code
Solutions are sorted into directories by year, then programming language and an additional `common` directory.

## Programming Languages
### Rust
The solutions are written intending to target `wasm32-unknown-unknown`. Two runners are intended to be included: one utilizing Wasmtime and the other being a simple HTML page with JavaScript bindings to run the compiled Wasm.
#### Building and Running
The following assumes the current working directory to be `[year]/rust`.

A script (`run.sh`) will be included that takes a `[day]` as input and runs:  
`cargo build --release --target=wasm32-unknown-unknown -p [day]`  
`cargo run --release -p runner-native [day]`
### Inko
### Zig