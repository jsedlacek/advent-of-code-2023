use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, multispace0, newline, space0, space1, u64},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Range {
    start: u64, // inclusive
    end: u64,   // exclusive
}

impl Range {
    fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    fn intersect(self, other: Range) -> Option<Range> {
        let max_start = std::cmp::max(self.start, other.start);
        let min_end = std::cmp::min(self.end, other.end);

        if max_start < min_end {
            Some(Range::new(max_start, min_end))
        } else {
            None
        }
    }

    fn subtract(self, other: Range) -> HashSet<Range> {
        // No overlap
        if self.start >= other.end || self.end <= other.start {
            return [self].into();
        }

        let mut result = HashSet::new();

        // Partial overlap at the start of "self"
        if other.start > self.start {
            result.insert(Range::new(self.start, other.start));
        }

        // Partial overlap at the end of "self"
        if other.end < self.end {
            result.insert(Range::new(other.end, self.end));
        }

        result
    }

    fn subtract_ranges(self, ranges_to_subtract: &[Range]) -> HashSet<Range> {
        let mut current_ranges: HashSet<_> = [self].into();

        for &range_to_subtract in ranges_to_subtract {
            current_ranges = current_ranges
                .iter()
                .flat_map(|r| r.subtract(range_to_subtract).into_iter())
                .collect();
        }

        current_ranges
    }
}

#[derive(Hash, PartialEq, Eq)]
struct Mapping {
    source_range: Range,
    destination_range_start: u64,
}

impl Mapping {
    fn parse(input: &str) -> IResult<&str, Self> {
        // 50 98 2
        let (input, (destination_range_start, _, source_range_start, _, range_length)) =
            tuple((u64, space1, u64, space1, u64))(input)?;

        Ok((
            input,
            Self {
                source_range: Range::new(source_range_start, source_range_start + range_length),
                destination_range_start,
            },
        ))
    }

    fn map(&self, range: Range) -> Option<Range> {
        range.intersect(self.source_range).map(|intersect| {
            Range::new(
                intersect.start - self.source_range.start + self.destination_range_start,
                intersect.end - self.source_range.start + self.destination_range_start,
            )
        })
    }
}

struct Map {
    source_category: String,
    destination_category: String,
    mappings: HashSet<Mapping>,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        // seed-to-soil map:

        let (input, (source_category, _, destination_category, _, _, _)) =
            tuple((alpha0, tag("-to-"), alpha0, space1, tag("map:"), newline))(input)?;

        let (input, mappings) = separated_list0(newline, Mapping::parse)(input)?;

        Ok((
            input,
            Self {
                source_category: source_category.to_string(),
                destination_category: destination_category.to_string(),
                mappings: HashSet::from_iter(mappings),
            },
        ))
    }

    fn map(&self, range: Range) -> Vec<Range> {
        let ranges: Vec<_> = self.mappings.iter().map(|m| m.source_range).collect();

        let other_ranges = range.subtract_ranges(&ranges);

        let other_mappings: Vec<_> = other_ranges
            .iter()
            .map(|&r| Mapping {
                source_range: r,
                destination_range_start: r.start,
            })
            .collect();

        self.mappings
            .iter()
            .chain(other_mappings.iter())
            .flat_map(|m| m.map(range))
            .collect()
    }
}

struct Game {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        // seeds: 79 14 55 13
        let (input, seeds) = nom::sequence::preceded(
            tuple((tag("seeds:"), space0)),
            separated_list0(space1, u64),
        )(input)?;

        let (input, maps_vec) = delimited(
            multispace0,
            separated_list0(multispace0, Map::parse),
            multispace0,
        )(input)?;

        let mut maps = HashMap::new();

        for map in maps_vec {
            maps.insert(map.source_category.clone(), map);
        }

        Ok((input, Game { seeds, maps }))
    }

    fn part1(&self) -> Result<u64> {
        let ranges: Vec<_> = self.seeds.iter().map(|&s| Range::new(s, s + 1)).collect();

        let min_value = self
            .find_category_ranges("seed", &ranges, "location")?
            .into_iter()
            .map(|r| r.start)
            .min()
            .ok_or(anyhow!("No minimal value"))?;

        Ok(min_value)
    }

    fn part2(&self) -> Result<u64> {
        let ranges: Vec<_> = self
            .seeds
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let length = chunk[1];
                Range::new(start, start + length)
            })
            .collect();

        self.find_minimal_value(&ranges)
    }

    fn find_minimal_value(&self, ranges: &[Range]) -> Result<u64> {
        let min_value = self
            .find_category_ranges("seed", ranges, "location")?
            .into_iter()
            .map(|r| r.start)
            .min()
            .ok_or(anyhow!("No minimal value"))?;

        Ok(min_value)
    }

    fn find_category_ranges(
        &self,
        source_category: &str,
        source_ranges: &[Range],
        destination_category: &str,
    ) -> Result<Vec<Range>> {
        let mut category = source_category.to_string();
        let mut ranges = Vec::from(source_ranges);

        loop {
            if category == destination_category {
                return Ok(ranges);
            }

            let map = self
                .maps
                .get(&category)
                .ok_or(anyhow!("Category not found: {category}"))?;

            category = map.destination_category.clone();

            ranges = ranges
                .iter()
                .flat_map(|&r| map.map(r).into_iter())
                .collect();
        }
    }
}

fn main() -> Result<()> {
    let (_, game) = all_consuming(Game::parse)(include_str!("input.txt"))?;
    dbg!(game.part1()?);
    dbg!(game.part2()?);

    Ok(())
}

#[test]
fn test_part1() -> Result<()> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt")).unwrap();
    assert_eq!(sample_game.part1()?, 35);

    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt")).unwrap();
    assert_eq!(sample_game.part2()?, 46);

    Ok(())
}
