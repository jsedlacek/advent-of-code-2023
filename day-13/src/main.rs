use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map, map_res, value},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
struct Game {
    patterns: Vec<Pattern>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        all_consuming(delimited(
            many0(newline),
            map(
                separated_list0(many1(newline), Pattern::parse),
                |patterns| Self { patterns },
            ),
            many0(newline),
        ))(input)
    }

    fn part1(&self) -> u64 {
        self.patterns.iter().map(|p| p.value()).sum::<u64>()
    }

    fn part2(&self) -> Result<u64> {
        Ok(self
            .patterns
            .iter()
            .map(|p| p.value2())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .sum::<u64>())
    }
}

#[derive(Debug, Clone)]
struct Pattern {
    map: HashMap<(u64, u64), Tile>,
    max_x: u64,
    max_y: u64,
}

impl Pattern {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            separated_list1(newline, many1(Tile::parse)),
            |pattern| -> Result<Self> {
                let mut map = HashMap::new();

                for (y, line) in pattern.iter().enumerate() {
                    let y = y as u64;

                    for (x, &tile) in line.iter().enumerate() {
                        let x = x as u64;

                        map.insert((x, y), tile);
                    }
                }

                let max_x = map
                    .keys()
                    .copied()
                    .map(|(x, _)| x)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                let max_y = map
                    .keys()
                    .copied()
                    .map(|(_, y)| y)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                Ok(Self { map, max_x, max_y })
            },
        )(input)
    }

    fn are_columns_eq(&self, a: u64, b: u64) -> bool {
        (0..=self.max_y).all(|y| self.map.get(&(a, y)) == self.map.get(&(b, y)))
    }

    fn are_rows_eq(&self, a: u64, b: u64) -> bool {
        (0..=self.max_x).all(|x| self.map.get(&(x, a)) == self.map.get(&(x, b)))
    }

    fn is_vertical_symmetry(&self, x: u64) -> bool {
        (0..=(x.min(self.max_x - (x + 1)))).all(|diff| self.are_columns_eq(x - diff, x + 1 + diff))
    }

    fn find_vertical_symmetry(&self) -> HashSet<u64> {
        (0..self.max_x)
            .filter_map(|x| {
                if self.is_vertical_symmetry(x) {
                    Some(x + 1)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_horizontal_symmetry(&self, y: u64) -> bool {
        (0..=(y.min(self.max_y - (y + 1)))).all(|diff| self.are_rows_eq(y - diff, y + 1 + diff))
    }

    fn find_horizontal_symmetry(&self) -> HashSet<u64> {
        (0..self.max_y)
            .filter_map(|y| {
                if self.is_horizontal_symmetry(y) {
                    Some((y + 1) * 100)
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_symmetry(&self) -> HashSet<u64> {
        self.find_vertical_symmetry()
            .union(&self.find_horizontal_symmetry())
            .into_iter()
            .copied()
            .collect()
    }

    fn value(&self) -> u64 {
        self.find_symmetry().iter().sum()
    }

    fn value2(&self) -> Result<u64> {
        let mut clone = self.clone();
        let original_value = self.find_symmetry();

        for (&key, &value) in self.map.iter() {
            let new_value = value.inverse();

            clone.map.insert(key, new_value);
            let result = clone.find_symmetry();

            let result: HashSet<_> = result.difference(&original_value).copied().collect();

            if result.len() == 1 {
                return Ok(result.iter().sum::<u64>());
            }

            clone.map.insert(key, value);
        }

        Err(anyhow!("No value"))
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((value(Self::Ash, tag(".")), value(Self::Rock, tag("#"))))(input)
    }

    fn inverse(&self) -> Self {
        match self {
            Self::Ash => Self::Rock,
            Self::Rock => Self::Ash,
        }
    }
}

fn main() -> Result<()> {
    let (_, game) = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());

    println!("Part 2: {}", game.part2()?);

    Ok(())
}
