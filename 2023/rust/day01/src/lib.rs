include!("../../wasm-glue.rs");

const NUMBERS: [&'static str; 9] = [ "one", "two", "three", "four", "five", "six", "seven", "eight", "nine" ];

/// Parse out calibration values and calculate their sum.
fn sum(input_str: &str, words: bool) -> u32 {
    input_str.lines()
        .map(|line| {
            let values: Vec<(usize, u32)> = line.match_indices(char::is_numeric)
                .map(|(index, number)| (index, u32::from_str_radix(number, 10).unwrap()))
                .collect();
            
            let mut first = *values.first().unwrap_or(&(usize::MAX, u32::MAX));
            let mut last = *values.last().unwrap_or(&(usize::MIN, u32::MIN));

            if words {
                for (i, number_str) in NUMBERS.iter().enumerate() {
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
            }
            
            (first.1 * 10) + last.1
        })
        .sum()
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);

    sum(&input_str, false).to_le_bytes().to_vec()
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);

    sum(&input_str, true).to_le_bytes().to_vec()
}

#[test]
fn part_one_test() {
    let input_str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
    let sum = sum(input_str, false);
    
    assert_eq!(142, sum);
}

#[test]
fn part_two_test() {
    let input_str = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
    let sum = sum(input_str, true);

    assert_eq!(281, sum);

}