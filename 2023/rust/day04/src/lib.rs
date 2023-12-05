include!("../../wasm-glue.rs");

use std::collections::{BTreeMap, HashSet};

type CardPile = Vec<(HashSet<u8>, HashSet<u8>)>;

fn parse_cards(input_str: &str) -> CardPile {
    let mut cards = Vec::new();

    for card in input_str.trim().lines() {
        let (_, numbers) = card.split_once(':').unwrap();
        let (goals, given) = numbers.split_once('|').unwrap();
        let goals_set: HashSet<u8> = goals.trim().split_whitespace()
            .map(|n| u8::from_str_radix(n, 10).unwrap())
            .collect();
        let given_set: HashSet<u8> = given.trim().split_whitespace()
            .map(|n| u8::from_str_radix(n, 10).unwrap())
            .collect();

        cards.push((goals_set, given_set));
    }

    cards
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);

    parse_cards(&input_str).iter()
        .filter_map(|(goal, mine)| {
            let wins = goal.intersection(&mine).count() as u32;

            (wins > 0).then(|| 2u32.pow(wins - 1))
        })
        .sum::<u32>()
        .to_le_bytes()
        .to_vec()
}

#[test]
fn part_one_test() {
    let input_str = br#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let points = part_one(input_str);
    assert_eq!(13, u32::from_le_bytes(points.try_into().unwrap()));
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    let input_str = String::from_utf8_lossy(input_buf);
    let cards: BTreeMap<usize, usize> = parse_cards(&input_str).iter().enumerate()
        .map(|(number, (goal, mine))| (number + 1, goal.intersection(&mine).count()))
        .collect();

    let mut sum = 0usize;
    let mut copies: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    for (card, wins) in cards.iter() {
        match copies.remove(card) {
            Some((count, wins)) => {
                for _ in 0..count {
                    for i in 1..=wins {
                        let copy = copies.entry(card + i).or_insert((0, *cards.get(&(card + i)).unwrap()));
                        copy.0 += 1;
                        sum += 1;
                    }
                }
            },
            None => {}
        }

        for i in 1..=*wins {
            let copy = copies.entry(card + i).or_insert((0, *cards.get(&(card + i)).unwrap()));
            copy.0 += 1;
            sum += 1;
        }

        sum += 1;
    }

    (sum as u32).to_le_bytes().to_vec()
}

#[test]
fn part_two_test() {
    let input_str = br#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    let points = part_two(input_str);
    assert_eq!(30, u32::from_le_bytes(points.try_into().unwrap()));
}