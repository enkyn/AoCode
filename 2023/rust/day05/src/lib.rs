include!("../../wasm-glue.rs");

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,

    maps: [Vec<[u32; 3]>; 7],
}

fn parse_section(section_str: &str) -> Vec<[u32; 3]> {
    let mut section:Vec<[u32; 3]> = section_str.lines().skip(1)
        .map(|map_str| {
            let nums: Vec<u32> = map_str.split_whitespace()
                .map(|num_str| u32::from_str_radix(num_str, 10).unwrap())
                .collect();

            // map ranges to [src, dst, len]
            [nums[1], nums[0], nums[2]]
        })
        .collect();

    section.sort_unstable_by_key(|map| map[0]);

    section
}

fn parse_input(input_buf: &[u8]) -> Almanac {
    let input_str = String::from_utf8_lossy(input_buf);
    let mut sections = input_str.trim().split("\n\n");

    // seeds
    let (_, seeds_str) = sections.next().unwrap()
        .lines().next().unwrap()
        .split_once(": ").unwrap();
    let seeds: Vec<u32> = seeds_str.split_whitespace()
        .map(|seed_str| u32::from_str_radix(seed_str, 10).unwrap())
        .collect();

    // parse mapping sections
    let seed_soil = parse_section(sections.next().unwrap());
    let soil_fertilizer = parse_section(sections.next().unwrap());
    let fertilizer_water = parse_section(sections.next().unwrap());
    let water_light = parse_section(sections.next().unwrap());
    let light_temperature = parse_section(sections.next().unwrap());
    let temperature_humidity = parse_section(sections.next().unwrap());
    let humidity_location = parse_section(sections.next().unwrap());

    Almanac {
        seeds,
        maps: [
            seed_soil, soil_fertilizer, fertilizer_water, water_light, light_temperature, temperature_humidity, humidity_location
        ],
    }
}

fn traverse(maps: &[Vec<[u32; 3]>; 7], map: usize, current: u32) -> u32 {
    if map == maps.len() {
        return current;
    }

    for [src, dst, len] in &maps[map] {
        if current >= *src && current <= (src + len) {
            return traverse(maps, map + 1, dst + (current - src));
        }
    }

    traverse(maps, map + 1, current)
}

fn part_one(input_buf: &[u8]) -> Vec<u8> {
    let almanac = parse_input(&input_buf);

    almanac.seeds.iter()
        .map(|seed| traverse(&almanac.maps, 0, *seed))
        .min().unwrap()
        .to_le_bytes().to_vec()
}

fn part_two(input_buf: &[u8]) -> Vec<u8> {
    unimplemented!("part_two")
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
    unimplemented!("part_two_test")
}