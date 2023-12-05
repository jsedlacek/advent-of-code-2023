use std::{collections::HashMap, convert::identity};

const SAMPLE_INPUT: &str = include_str!("sample-input.txt");
const SAMPLE_INPUT_2: &str = include_str!("sample-input-2.txt");
const INPUT: &str = include_str!("input.txt");

fn find_first(haystack: &str, map: &HashMap<String, u32>) -> u32 {
    *map.iter()
        .map(|(key, value)| {
            let pos = haystack.find(key);

            return pos.map(|p| (p, value));
        })
        .filter_map(identity)
        .min_by_key(|(pos, _)| *pos)
        .unwrap()
        .1
}

fn find_last(haystack: &str, map: &HashMap<String, u32>) -> u32 {
    *map.iter()
        .map(|(key, value)| {
            let pos = haystack.rfind(key);

            return pos.map(|p| (p, value));
        })
        .filter_map(identity)
        .max_by_key(|(pos, _)| *pos)
        .unwrap()
        .1
}

fn process(input: &str, map: &HashMap<String, u32>) -> u32 {
    input
        .lines()
        .map(|line| {
            let first = find_first(line, map);
            let last = find_last(line, map);

            format!("{first}{last}").parse::<u32>().unwrap()
        })
        .sum()
}

fn process_part_1(input: &str) -> u32 {
    let mut map = HashMap::new();

    for i in 1..=9 {
        map.insert(i.to_string(), i);
    }

    process(&input, &map)
}

fn process_part_2(input: &str) -> u32 {
    let mut map = HashMap::new();

    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    map.insert("three".to_string(), 3);
    map.insert("four".to_string(), 4);
    map.insert("five".to_string(), 5);
    map.insert("six".to_string(), 6);
    map.insert("seven".to_string(), 7);
    map.insert("eight".to_string(), 8);
    map.insert("nine".to_string(), 9);

    for i in 1..=9 {
        map.insert(i.to_string(), i);
    }

    process(&input, &map)
}

fn main() {
    assert_eq!(process_part_1(SAMPLE_INPUT), 142);
    assert_eq!(process_part_1(INPUT), 54239);

    dbg!(process_part_2(SAMPLE_INPUT_2));
    dbg!(process_part_2(INPUT));
}
