include!("../../wasm-glue.rs");

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
                        c if c.is_numeric() => c,
                        _ => '#',
                    }
                })
                .collect::<Vec<char>>())
        })
        .flatten()
        .collect();

    (chars, input.lines().next().unwrap().len())
}

fn near_symbol(chars: &[char], index: usize, x_len: usize) -> bool {
    let (index, x_len) = (index as isize, x_len as isize);

    let nw: isize = index - x_len - 1;
    let nn: isize = index - x_len;
    let ne: isize = index - x_len + 1;
    let ee: usize = (index + 1) as usize;
    let se: usize = (index + x_len + 1) as usize;
    let ss: usize = (index + x_len) as usize;
    let sw: usize = (index + x_len - 1) as usize;
    let ww: isize = index - 1;

    (nw >= 0 && chars[nw as usize] == '#')
        | (nn >= 0 && chars[nn as usize] == '#')
        | (ne >= 0 && chars[ne as usize] == '#')
        | (ee < chars.len() && chars[ee] == '#')
        | (se < chars.len() && chars[se as usize] == '#')
        | (ss < chars.len() && chars[ss as usize] == '#')
        | (sw < chars.len() && chars[sw as usize] == '#')
        | (ww >= 0 && chars[ww as usize] == '#')
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
                    // println!("{start}, {length}");
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
            // println!("ch: {}", chars[i]);
            number = (number * 10) + chars[i].to_digit(10).unwrap();

            is_part |= near_symbol(&chars, i, x_len);
        }

        if is_part {
            numbers.push(number);
        }
    }

    // println!("{numbers:?}");

    numbers.iter().sum::<u32>()
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

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    unimplemented!("part_two")
}

#[test]
fn part_two_test() {
    //
}