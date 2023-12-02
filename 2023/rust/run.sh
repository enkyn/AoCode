#!/bin/bash
cargo build --release --target=wasm32-unknown-unknown -p $1
cargo run --release -p runner-native $1