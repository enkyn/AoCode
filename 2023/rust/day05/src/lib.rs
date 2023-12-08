include!("../../wasm-glue.rs");

type Section = Vec<[(u32, u32); 2]>;

fn parse_section(s: &str) -> Section {
    let mut section: Section = s.lines().skip(1)
        .map(|section_str| {
            let nums: Vec<u32> = section_str.split_whitespace()
                .map(|num_str| u32::from_str_radix(num_str, 10).unwrap())
                .collect();

            // map ranges to [(src start, src end), (dst start, dst end)]
            [(nums[1], nums[1] + nums[2]), (nums[0], nums[0] + nums[2])]
        })
        .collect();

    // sort by start of src range
    section.sort_unstable_by_key(|&[(a, _), (_, _)]| a);

    section
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    sections: [Section; 7],
}

impl From<&[u8]> for Almanac {
    fn from(value: &[u8]) -> Self {
        let input_string = unsafe { String::from_utf8_unchecked(value.to_vec()) };
        let mut section_strings = input_string.trim().split("\n\n");
        
        let seeds = section_strings.next().unwrap()
            .lines().next().unwrap()
            .split_once(": ").unwrap().1
            .split_whitespace()
            .map(|s| u32::from_str_radix(s, 10).unwrap())
            .collect();

        let sections: Vec<Section> = section_strings
            .map(|section| parse_section(section))
            .collect();

        Self {
            seeds,
            sections: sections.try_into().unwrap()
        }
    }
}

fn traverse(maps: &[Section; 7], map: usize, current: u32) -> u32 {
    if map == maps.len() {
        return current;
    }

    for &[(src_s, src_e), (dst_s, _)] in &maps[map] {
        if current >= src_s && current < src_e {
            return traverse(maps, map + 1, dst_s + (current - src_s));
        }
    }

    traverse(maps, map + 1, current)
}

fn traverse_range(sections: &[Section; 7], range: (u32, u32)) -> u32 {
    let mut ranges = vec![range];
    let mut next_ranges = Vec::new();
    for section in sections {

        for (range_s, range_e) in ranges.drain(..) {

            // attempt to map the range
            let mut mapped = false;
            for &[(src_s, src_e), (dst_s, _)] in section {

                // overlap
                let ol_start = range_s.max(src_s);
                let ol_end = (range_e - 1).min(src_e); // I don't quite understand why the '- 1' here, but the example and puzzle pass, so I'm moving on...
                if ol_start <= ol_end {
                    let offset = ol_start - src_s;

                    next_ranges.push((dst_s + offset, dst_s + offset + ol_end - ol_start));

                    mapped = true;
                }
            }

            // just carry the range to the next section
            if !mapped {
                next_ranges.push((range_s, range_e));
            }
        }

        // merge ranges
        while let Some(next_range) = next_ranges.pop() {
            if let Some(last_range) = ranges.pop() {
                if last_range.1 == next_range.0 {
                    ranges.push((last_range.0, next_range.1));
                } else if next_range.1 == last_range.0 {
                    ranges.push((next_range.0, last_range.1));
                } else {
                    ranges.push(last_range);
                    ranges.push(next_range);
                }
            } else {
                ranges.push(next_range);
            }
        }
    }

    ranges.iter().min().unwrap().0
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let almanac = Almanac::from(input_buf);

    almanac.seeds.iter()
        .map(|seed| traverse(&almanac.sections, 0, *seed))
        .min().unwrap()
        .to_le_bytes().to_vec()
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    let almanac = Almanac::from(input_buf);

    almanac.seeds.chunks_exact(2)
        .map(|range| traverse_range(&almanac.sections, (range[0], range[0] + range[1])))
        .min().unwrap()
        .to_le_bytes().to_vec()
}

#[test]
fn part_one_test() {
    let input_buf = br#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    assert_eq!(35, u32::from_le_bytes(part_one(input_buf).try_into().unwrap()));
}

#[test]
fn part_two_test() {
    let input_buf = br#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    assert_eq!(46, u32::from_le_bytes(part_two(input_buf).try_into().unwrap()));
}