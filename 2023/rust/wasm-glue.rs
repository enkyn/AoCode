use core::mem::ManuallyDrop;
use core::slice;

/// Allocate some space for the input, returning a mutable pointer to it.
#[no_mangle]
pub fn wasm_alloc(len: usize) -> *mut u8 {
    let mut input_buf = ManuallyDrop::new(Vec::with_capacity(len));
    input_buf.as_mut_ptr()
}

/// Takes a pointer to the input (and its length), attempts to solve the puzzle, then returns a ((pointer, len) as u64).
#[no_mangle]
pub fn wasm_part_one(input_ptr: *const u8, input_len: usize) -> u64 {
    let input_buf = unsafe { slice::from_raw_parts(input_ptr, input_len) };
    let result_buf = ManuallyDrop::new(part_one(input_buf));
    
    ((result_buf.as_ptr() as u64) << 32) | result_buf.len() as u64
}

#[no_mangle]
pub fn wasm_part_two(input_ptr: *const u8, input_len: usize) -> u64 {
    let input_buf = unsafe { slice::from_raw_parts(input_ptr, input_len) };
    let result_buf = ManuallyDrop::new(part_two(input_buf));
    
    ((result_buf.as_ptr() as u64) << 32) | result_buf.len() as u64
}