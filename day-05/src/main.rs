use std::{collections::HashMap, error::Error};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha0, multispace0, newline, space0, space1, u64},
    combinator::all_consuming,
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn new(start: u64, length: u64) -> Self {
        Self { start, length }
    }

    fn end(self) -> u64 {
        self.start + self.length
    }

    fn intersect(self, other: Range) -> Option<Range> {
        let max_start = std::cmp::max(self.start, other.start);
        let min_end = std::cmp::min(self.end(), other.end());

        if max_start < min_end {
            Some(Range::new(max_start, min_end - max_start))
        } else {
            None
        }
    }

    fn subtract(self, other: Range) -> Vec<Range> {
        // No overlap
        if self.start >= other.end() || self.end() <= other.start {
            return vec![self];
        }

        let mut result = Vec::new();

        // Partial overlap at the start of "self"
        if other.start > self.start {
            result.push(Range::new(self.start, other.start - self.start));
        }

        // Partial overlap at the end of "self"
        if other.end() < self.end() {
            result.push(Range::new(other.end(), self.end() - other.end()));
        }

        result
    }

    fn subtract_ranges(self, ranges_to_subtract: &[Range]) -> Vec<Range> {
        let mut current_ranges = vec![self];

        for &range_to_subtract in ranges_to_subtract {
            current_ranges = current_ranges
                .iter()
                .map(|r| r.subtract(range_to_subtract).into_iter())
                .flatten()
                .collect();
        }

        current_ranges
    }
}

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
                source_range: Range::new(source_range_start, range_length),
                destination_range_start,
            },
        ))
    }

    fn map(&self, range: Range) -> Option<Range> {
        match range.intersect(self.source_range) {
            Some(intersect) => Some(Range::new(
                intersect.start - self.source_range.start + self.destination_range_start,
                intersect.length,
            )),
            None => None,
        }
    }
}

struct Map {
    source_category: String,
    destination_category: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn parse(input: &str) -> IResult<&str, Self> {
        // seed-to-soil map:
        // 50 98 2
        // 52 50 48

        let (input, (source_category, _, destination_category, _, _, _)) =
            tuple((alpha0, tag("-to-"), alpha0, space1, tag("map:"), newline))(input)?;

        let (input, mappings) = separated_list0(newline, Mapping::parse)(input)?;

        Ok((
            input,
            Self {
                source_category: source_category.to_string(),
                destination_category: destination_category.to_string(),
                mappings,
            },
        ))
    }

    fn map(&self, range: Range) -> Vec<Range> {
        let ranges: Vec<_> = self.mappings.iter().map(|m| m.source_range).collect();

        let other_ranges = range.subtract_ranges(&ranges);

        let other_mappings: Vec<_> = other_ranges
            .iter()
            .map(|r| Mapping {
                source_range: r.clone(),
                destination_range_start: r.start,
            })
            .collect();

        self.mappings
            .iter()
            .chain(other_mappings.iter())
            .map(|m| m.map(range))
            .flatten()
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

    fn part1(&self) -> Result<u64, Box<dyn Error>> {
        let ranges: Vec<_> = self.seeds.iter().map(|&s| Range::new(s, 1)).collect();

        let min_value = self
            .find_category_ranges("seed", &ranges, "location")?
            .into_iter()
            .map(|r| r.start)
            .min()
            .ok_or("No minimal value")?;

        Ok(min_value)
    }

    fn part2(&self) -> Result<u64, Box<dyn Error>> {
        let ranges: Vec<_> = self
            .seeds
            .chunks(2)
            .map(|chunk| {
                let start = chunk[0];
                let length = chunk[1];
                Range::new(start, length)
            })
            .collect();

        self.find_minimal_value(&ranges)
    }

    fn find_minimal_value(&self, ranges: &[Range]) -> Result<u64, Box<dyn Error>> {
        let min_value = self
            .find_category_ranges("seed", ranges, "location")?
            .into_iter()
            .map(|r| r.start)
            .min()
            .ok_or("No minimal value")?;

        Ok(min_value)
    }

    fn find_category_ranges(
        &self,
        source_category: &str,
        source_ranges: &[Range],
        destination_category: &str,
    ) -> Result<Vec<Range>, Box<dyn Error>> {
        let mut category = source_category.to_string();
        let mut ranges = Vec::from(source_ranges);

        loop {
            if category == destination_category {
                return Ok(ranges);
            }

            let map = self
                .maps
                .get(&category)
                .ok_or(format!("Category not found: {category}"))?;

            category = map.destination_category.clone();

            ranges = ranges
                .iter()
                .map(|&r| map.map(r).into_iter())
                .flatten()
                .collect();
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt"))?;
    assert_eq!(sample_game.part1()?, 35);
    assert_eq!(sample_game.part2()?, 46);

    let (_, game) = all_consuming(Game::parse)(include_str!("input.txt"))?;
    dbg!(game.part1()?);
    dbg!(game.part2()?);

    Ok(())
}
