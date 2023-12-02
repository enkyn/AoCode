use std::path::PathBuf;
use std::time::Instant;

use wasmtime::{Engine, Instance, Module, Store};

type Ptr = u32;
type Len = u32;
type PtrLen = u64;

fn main() -> wasmtime::Result<()> {
    // Determine the path for the given day's Wasm file.
    let mut wasm_path = PathBuf::from(std::env::current_dir().unwrap());
    let day = match std::env::args().skip(1).next() {
        Some(day) => {
            let target = PathBuf::from("target/wasm32-unknown-unknown/release");
            wasm_path.push(target);
            wasm_path.push(format!("{day}.wasm"));

            if !wasm_path.exists() {
                eprintln!("day invalid");
                std::process::exit(1);
            }

            day
        },
        None => {
            eprintln!("day must be provided");
            std::process::exit(1);
        }
    };

    // Load the Wasm bytes.
    let wasm = match std::fs::read(wasm_path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("{e}");
            std::process::exit(2);
        }
    };

    // Setup Wasmtime.
    let engine = Engine::default();
    let module = Module::new(&engine, wasm)?;
    let mut store = Store::new(&engine, ());
    let instance = Instance::new(&mut store, &module, &[])?;

    // Instance expected functions.
    let allocate = instance.get_typed_func::<Len, Ptr>(&mut store, "allocate")?;
    let part_one = instance.get_typed_func::<(Ptr, Len), PtrLen>(&mut store, "part_one")?;
    let part_two = instance.get_typed_func::<(Ptr, Len), PtrLen>(&mut store, "part_two")?;

    // Setup the day's input file path.
    let mut input_path = PathBuf::from(std::env::current_dir().unwrap());
    input_path.pop();
    input_path.push("common/input");

    // Part one
    let input_path_one = input_path.join(format!("{day}-1.txt"));
    if let Ok(input) = std::fs::read(input_path_one) {
        let memory = instance.get_memory(&mut store, "memory").unwrap();

        // Store the input in the Wasm's memory.
        let input_ptr = allocate.call(&mut store, input.len() as u32)?;
        memory.write(&mut store, input_ptr as usize, &input)?;

        // Run the function for part one.
        let start_time = Instant::now();
        let result_ptrlen = part_one.call(&mut store, (input_ptr, input.len() as u32))?;
        let (result_ptr, result_len) = ((result_ptrlen >> 32) as usize, (result_ptrlen & 0xffffffff) as usize);
        let mut result_buf = vec![0; result_len];
        memory.read(&mut store, result_ptr, &mut result_buf)?;

        println!("- one:\n  result: {}\n  elapsed: {}ms",
            u32::from_le_bytes(result_buf[..result_len].try_into().unwrap()),
            start_time.elapsed().as_millis());
    }

    // Part two
    let input_path_two = input_path.join(format!("{day}-2.txt"));
    if let Ok(input) = std::fs::read(input_path_two) {
        let memory = instance.get_memory(&mut store, "memory").unwrap();

        // Store the input in the Wasm's memory.
        let input_ptr = allocate.call(&mut store, input.len() as u32)?;
        memory.write(&mut store, input_ptr as usize, &input)?;

        // Run the function for part two.
        let start_time = Instant::now();
        let result_ptrlen = part_two.call(&mut store, (input_ptr, input.len() as u32))?;
        let (result_ptr, result_len) = ((result_ptrlen >> 32) as usize, (result_ptrlen & 0xffffffff) as usize);
        let mut result_buf = vec![0; result_len];
        memory.read(&mut store, result_ptr, &mut result_buf)?;

        println!("- two:\n  result: {}\n  elapsed: {}ms",
            u32::from_le_bytes(result_buf[..result_len].try_into().unwrap()),
            start_time.elapsed().as_millis());
    }

    Ok(())
}
