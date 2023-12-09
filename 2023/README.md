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
### Other
There was intent to also use Zig and Inko, but I've already fallen behind. Perhaps (though likely not) I'll revist later.