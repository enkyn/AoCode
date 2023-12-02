#!/bin/bash

# Takes an argument for the day to run (e.g. 'day01') and an optional 'share' flag.
# The 'share' flag indicates that the, e.g. 'day01-1.tx', input should be used for both parts.

cargo build --release --target=wasm32-unknown-unknown -p $1
cargo run --release -p runner-native $1 $2