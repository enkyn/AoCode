include!("../../wasm-glue.rs");

#[derive(Debug)]
struct Game {
    grabs: Vec<[u8; 3]>,
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        // { _: (Game 1), record: ( 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green) }
        let (_, record) = s.split_once(": ").unwrap();
        // [ (3 blue, 4 red), (1 red, 2 green, 6 blue), (2 green) ]
        let grabs: Vec<[u8; 3]> = record.split("; ")
            .map(|grab| {
                // [ (3 blue), (4 red) ]
                let mut counts = [0u8; 3];
                for color_count in grab.split(", ") {
                    let (count, color) = color_count.split_once(' ').unwrap();
                    let count = u8::from_str_radix(count, 10).unwrap();
                    match color {
                        "red" => counts[0] = count,
                        "green" => counts[1] = count,
                        "blue" => counts[2] = count,
                        _ => {},
                    }
                }

                counts
            })
            .collect();

        Game { grabs }
    }
}

impl Game {
    fn is_valid(&self, colors: [u8; 3]) -> bool {
        for grab in &self.grabs {
            if grab[0] > colors[0] || grab[1] > colors[1] || grab[2] > colors[2] {
                return false;
            }
        }

        true
    }

    fn min_cubes(&self) -> [u8; 3] {
        let mut cubes = [0u8; 3];
        for grab in &self.grabs {
            cubes[0] = cubes[0].max(grab[0]);
            cubes[1] = cubes[1].max(grab[1]);
            cubes[2] = cubes[2].max(grab[2]);
        }
        cubes
    }
}

fn valid_sum(input_str: &str, colors: [u8; 3]) -> u32 {
    input_str.trim().lines().enumerate()
        .filter_map(|(id, line)| {
            let game = Game::from(line);
            if game.is_valid(colors) {
                Some(id as u32 + 1)
            } else {
                None
            }
        })
        .sum()
}

fn min_cubes(input_str: &str) -> u32 {
    input_str.trim().lines()
        .map(|line| {
            let game = Game::from(line);
            let min_cubes = game.min_cubes();
            min_cubes[0] as u32 * min_cubes[1] as u32 * min_cubes[2] as u32
        })
        .sum()
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);

    valid_sum(&input_str, [12, 13, 14])
        .to_le_bytes().to_vec()
}

#[test]
fn part_one_test() {
    let input_str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;
    
    assert_eq!(8, valid_sum(&input_str, [12, 13, 14]));
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);

    min_cubes(&input_str)
        .to_le_bytes().to_vec()
}

#[test]
fn part_two_test() {
    let input_str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    assert_eq!(2286, min_cubes(&input_str));
}