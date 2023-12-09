include!("../../wasm-glue.rs");

fn parse_line(line_str: &str) -> Vec<u32> {
    line_str.split_once(':')
        .map(|(_, r_str)| {
            r_str.trim().split_whitespace()
                .map(|s| u32::from_str_radix(s, 10).unwrap())
                .collect()
        })
        .unwrap()
}

fn parse_input(input_buf: &[u8]) -> Vec<(u32, u32)> {
    let input_str = unsafe { String::from_utf8_unchecked(input_buf.to_vec()) };
    let lines: Vec<&str> = input_str.trim().lines()
        .collect();
    
    let mut iter = lines.iter();
    parse_line(iter.next().unwrap()).iter()
        .zip(parse_line(iter.next().unwrap()))
        .map(|(a, b)| (*a, b))
        .collect()
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let mut product = 1u32;
    for (race_t, race_d) in parse_input(input_buf) {
        let mut wins = u32::MIN;

        for t in 1..race_t {
            if (race_t - t) * t > race_d {
                wins += 1;
            }
        }

        product *= wins;
    }

    product.to_le_bytes().to_vec()
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    // concatenate times and distances
    let mut race_t = String::new();
    let mut race_d = String::new();
    for (t, d) in parse_input(input_buf) {
        race_t.push_str(&t.to_string());
        race_d.push_str(&d.to_string());
    }

    // just try everything, as in part one
    let race_t = u64::from_str_radix(&race_t, 10).unwrap();
    let race_d = u64::from_str_radix(&race_d, 10).unwrap();
    let mut result = u64::MIN;
    for t in 1..race_t {
        if (race_t - t) * t > race_d {
            if race_t % 2 == 0 {
                result = (((race_t / 2) - t) * 2) + 1;
            } else {
                result = (((race_t / 2) + 1) - t) * 2;
            }

            break;
        }
    }

    (result as u32).to_le_bytes().to_vec()
}

#[test]
fn part_one_example() {
    let input_buf = br#"
Time:      7  15   30
Distance:  9  40  200"#;

    let result = u32::from_le_bytes(part_one(input_buf).try_into().unwrap());
    assert_eq!(288, result);
}

#[test]
fn part_two_example() {
    let input_buf = br#"
Time:      7  15   30
Distance:  9  40  200"#;

    let result = u32::from_le_bytes(part_two(input_buf).try_into().unwrap());
    assert_eq!(71503, result);
}