use std::collections::HashMap;

use anyhow::{anyhow, Result};

fn find_first(haystack: &str, map: &HashMap<String, u32>) -> Result<u32> {
    Ok(*map
        .iter()
        .filter_map(|(key, value)| haystack.find(key).map(|p| (p, value)))
        .min_by_key(|&(pos, _)| pos)
        .ok_or(anyhow!("Key not found"))?
        .1)
}

fn find_last(haystack: &str, map: &HashMap<String, u32>) -> Result<u32> {
    Ok(*map
        .iter()
        .filter_map(|(key, value)| haystack.rfind(key).map(|p| (p, value)))
        .max_by_key(|(pos, _)| *pos)
        .ok_or(anyhow!("Key not found"))?
        .1)
}

fn process(input: &str, map: &HashMap<String, u32>) -> Result<u32> {
    Ok(input
        .lines()
        .map(|line| -> Result<u32> {
            let first = find_first(line, map)?;
            let last = find_last(line, map)?;

            Ok(format!("{first}{last}").parse::<u32>()?)
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

fn process_part_1(input: &str) -> Result<u32> {
    let mut map = HashMap::new();

    for i in 1..=9 {
        map.insert(i.to_string(), i);
    }

    process(input, &map)
}

fn process_part_2(input: &str) -> Result<u32> {
    let words = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]
    .iter()
    .map(|&(s, n)| (s.to_string(), n));

    let numbers = (1..=9).map(|n| (n.to_string(), n));

    let map = words.chain(numbers).collect::<HashMap<_, _>>();

    process(input, &map)
}

fn main() -> Result<()> {
    println!("Part 1: {}", process_part_1(include_str!("input.txt"))?);
    println!("Part 2: {}", process_part_2(include_str!("input.txt"))?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    assert_eq!(process_part_1(include_str!("sample-input.txt"))?, 142);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    assert_eq!(process_part_2(include_str!("sample-input-2.txt"))?, 281);

    Ok(())
}
