extern crate alloc;

use alloc::vec::Vec;
use alloc::slice;
use core::mem::ManuallyDrop;

/// Allocate some space for the input, returning a mutable pointer to it.
#[no_mangle]
pub fn allocate(len: usize) -> *mut u8 {
    let mut input_buf = ManuallyDrop::new(Vec::with_capacity(len));
    input_buf.as_mut_ptr()
}

/// Takes a pointer to the input, (hopefully) solves the puzzle, then returns a pointer to the output.
#[no_mangle]
pub fn part_one(input_ptr: *const u8, input_len: usize) -> u64 {
    let input_buf = unsafe { slice::from_raw_parts(input_ptr, input_len) };
    let input_str = String::from_utf8_lossy(input_buf);

    let sum: u32 = input_str.lines()
        .map(|line| {
            let values: Vec<u32> = line.matches(char::is_numeric)
                .map(|num| u32::from_str_radix(num, 10).unwrap())
                .collect();
            
            (values.first().unwrap() * 10) + values.last().unwrap()
        })
        .sum();

    let result_buf = ManuallyDrop::new(sum.to_le_bytes().to_vec());
    ((result_buf.as_ptr() as u64) << 32) | result_buf.len() as u64
}

#[no_mangle]
pub fn part_two(input_ptr: *const u8, input_len: usize) -> u64 {
    let input_buf = unsafe { slice::from_raw_parts(input_ptr, input_len) };
    let input_str = String::from_utf8_lossy(input_buf);
    let numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let sum: u32 = input_str.lines()
        .map(|line| {
            let values: Vec<(usize, u32)> = line.match_indices(char::is_numeric)
                .map(|(index, number)| (index, u32::from_str_radix(number, 10).unwrap()))
                .collect();
            
            let mut first = *values.first().unwrap_or(&(usize::MAX, u32::MAX));
            let mut last = *values.last().unwrap_or(&(usize::MIN, u32::MIN));

            for (i, number_str) in numbers.iter().enumerate() {
                let mut matches = line.match_indices(number_str)
                    .map(|(index, _)| (index, (i + 1) as u32));
                let mut rmatches = line.rmatch_indices(number_str)
                    .map(|(index, _)| (index, (i + 1) as u32));

                if let Some(potential) = matches.next() {
                    first = first.min(potential);
                    last = last.max(potential);
                }
                if let Some(potential) = rmatches.next() {
                    first = first.min(potential);
                    last = last.max(potential);
                }
            }
            
            (first.1 * 10) + last.1
        })
        .sum();

    let result_buf = ManuallyDrop::new(sum.to_le_bytes().to_vec());
    ((result_buf.as_ptr() as u64) << 32) | result_buf.len() as u64
}

#[test]
fn part_one_test() {
    let input_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    let sum: u64 = input_str.lines()
        .map(|line| {
            let nums: Vec<u64> = line.matches(char::is_numeric)
                .map(|num| u64::from_str_radix(num, 10).unwrap())
                .collect();

            (nums.first().unwrap() * 10) + nums.last().unwrap()
        })
        .sum();
    
    assert_eq!(142, sum);
}

#[test]
fn part_two_test() {
    let input_str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    let numbers = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

    let sum: u32 = input_str.lines()
        .map(|line| {
            let values: Vec<(usize, u32)> = line.match_indices(char::is_numeric)
                .map(|(index, number)| (index, u32::from_str_radix(number, 10).unwrap()))
                .collect();
            
            let mut first = *values.first().unwrap_or(&(usize::MAX, u32::MAX));
            let mut last = *values.last().unwrap_or(&(usize::MIN, u32::MIN));

            for (i, number_str) in numbers.iter().enumerate() {
                let mut matches = line.match_indices(number_str)
                    .map(|(index, _)| (index, (i + 1) as u32));
                let mut rmatches = line.rmatch_indices(number_str)
                    .map(|(index, _)| (index, (i + 1) as u32));

                if let Some(potential) = matches.next() {
                    first = first.min(potential);
                    last = last.max(potential);
                }
                if let Some(potential) = rmatches.next() {
                    first = first.min(potential);
                    last = last.max(potential);
                }
            }

            println!("# {first:?}, {last:?}");
            
            (first.1 * 10) + last.1
        })
        .sum();

    assert_eq!(281, sum);

}