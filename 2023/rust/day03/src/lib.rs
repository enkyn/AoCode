include!("../../wasm-glue.rs");

/*
    This is ugly and inefficient, but it works on my input so I'm moving on for now.
    Part two is the worse of the two and completes in around 8-13ms.
*/

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
struct Number(usize, usize);

/// Parses into a flattenned vector of chars (and the row/line length).
fn parse_schematic(input_str: &str) -> (Vec<char>, usize) {
    let input = input_str.trim();
    let chars = input.lines()
        .filter_map(|line| {
            if line.len() == 0 {
                return None;
            }

            Some(line.chars()
                .map(|c| {
                    match c {
                        '.' => '.',
                        '*' => '*',
                        c if c.is_numeric() => c,
                        _ => c,
                    }
                })
                .collect::<Vec<char>>())
        })
        .flatten()
        .collect();

    (chars, input.lines().next().unwrap().len())
}

// Determine if the given number fragment (by index) is near a symbol; return (is near symbol, is symbol a gear).
fn near_symbol(chars: &[char], index: usize, x_len: usize) -> bool {
    let (index, x_len) = (index as isize, x_len as isize);

    let mut is_near = false;
    let spaces = [
        index - x_len - 1,
        index - x_len,
        index - x_len + 1,
        index + 1,
        index + x_len + 1,
        index + x_len,
        index + x_len - 1,
        index - 1,
    ];

    for space in spaces {
        if space >= 0 && space < chars.len() as isize && !chars[space as usize].is_numeric() && chars[space as usize] != '.' {
            is_near |= match chars[space as usize] {
                '*' => true,
                _ => true,
            };
        }
    }

    is_near
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);
    let (chars, x_len) = parse_schematic(&input_str);

    let mut number: Option<Number> = None;
    let mut numbers: Vec<u32> = Vec::new();
    for (index, token) in chars.iter().enumerate() {
        match token {
            num if num.is_numeric() => {
                if let Some(Number(start, length)) = number {
                    number = Some(Number(start, length + 1));
                } else {
                    number = Some(Number(index, 1));
                }
            },

            _ => {
                if let Some(Number(start, length)) = number.take() {
                    let mut number = 0;
                    let mut is_part = false;
                    for i in start..start + length {
                        number = (number * 10) + chars[i].to_digit(10).unwrap();

                        is_part |= near_symbol(&chars, i, x_len);
                    }

                    if is_part {
                        numbers.push(number);
                    }
                }
            }
        }
    }

    // handle leftovers
    if let Some(Number(start, length)) = number.take() {
        let mut number = 0;
        let mut is_part = false;
        for i in start..start + length - 1 {
            number = (number * 10) + chars[i].to_digit(10).unwrap();

            is_part |= near_symbol(&chars, i, x_len);
        }

        if is_part {
            numbers.push(number);
        }
    }

    numbers.iter().sum::<u32>()
        .to_le_bytes().to_vec()
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);
    let (chars, x_len) = parse_schematic(&input_str);

    let mut number: Option<Number> = None;
    let mut numbers: Vec<Vec<usize>> = Vec::new();
    let mut symbols: Vec<(usize, bool)> = Vec::new();
    for (index, token) in chars.iter().enumerate() {
        match token {
            num if num.is_numeric() => {
                if let Some(Number(start, length)) = number {
                    number = Some(Number(start, length + 1));
                } else {
                    number = Some(Number(index, 1));
                }
            },

            _ => {
                if *token != '.' {
                    symbols.push((index, *token == '*'));
                }

                if let Some(num) = number.take() {
                    numbers.push((num.0..num.0 + num.1).collect());
                }
            }
        }
    }

    // handle leftover
    if let Some(num) = number.take() {
        numbers.push((num.0..num.0 + num.1).collect());
    }

    // check if symbol has nearby numbers
    let mut part_numbers = BTreeMap::new();
    for (index, _) in symbols.iter().filter(|(_, star)| *star) {
        let east = index + 1;
        let west = index - 1;

        for points in &numbers {
            if points.contains(&east) || points.contains(&west) {
                part_numbers.entry(index)
                    .or_insert(BTreeSet::new())
                    .insert(points);
            }
        }

        // north
        for i in (index - x_len - 1)..=(index - x_len + 1) {
            for points in &numbers {
                if points.contains(&i) {
                    part_numbers.entry(index)
                        .or_insert(BTreeSet::new())
                        .insert(points);
                }
            }
        }

        // south
        for i in (index + x_len - 1)..=(index + x_len + 1) {
            for points in &numbers {
                if points.contains(&i) {
                    part_numbers.entry(index)
                        .or_insert(BTreeSet::new())
                        .insert(points);
                }
            }
        }
    }

    part_numbers.iter()
        .filter(|(_, parts)| parts.len() == 2)
        .map(|(_, parts)| {
            let one = parts.first().unwrap().iter()
                .map(|point| chars[*point].to_digit(10).unwrap())
                .fold(0, |a, n| a * 10 + n);
            let two = parts.last().unwrap().iter()
                .map(|point| chars[*point].to_digit(10).unwrap())
                .fold(0, |a, n| a * 10 + n);

            one * two
        })
        .sum::<u32>()
        .to_le_bytes().to_vec()
}

#[test]
fn part_one_test() {
    let input_str = br#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    let sum = part_one(input_str);
    assert_eq!(4361, u32::from_le_bytes(sum.try_into().unwrap()));
}

#[test]
fn part_two_test() {
    let input_str = br#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    let sum = part_two(input_str);
    assert_eq!(467835, u32::from_le_bytes(sum.try_into().unwrap()));
}