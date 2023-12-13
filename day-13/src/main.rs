use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
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
        map(
            all_consuming(delimited(
                many0(newline),
                separated_list0(many1(newline), Pattern::parse),
                many0(newline),
            )),
            |patterns| Self { patterns },
        )(input)
    }

    fn part1(&self) -> u64 {
        self.patterns
            .iter()
            .map(|p| p.find_vertical_symmetry())
            .sum::<u64>()
            + self
                .patterns
                .iter()
                .map(|p| p.find_horizontal_symmetry())
                .sum::<u64>()
                * 100
    }
}

#[derive(Debug)]
struct Pattern {
    map: HashMap<(u64, u64), Tile>,
    max_x: u64,
    max_y: u64,
}

impl Pattern {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, pattern) = separated_list1(newline, many1(Tile::parse))(input)?;

        let mut map = HashMap::new();

        let mut max_x = 0;
        let mut max_y = 0;

        for (y, line) in pattern.iter().enumerate() {
            let y = y as u64;

            if y > max_y {
                max_y = y;
            }

            for (x, &tile) in line.iter().enumerate() {
                let x = x as u64;

                if x > max_x {
                    max_x = x;
                }

                map.insert((x, y), tile);
            }
        }

        Ok((input, Self { map, max_x, max_y }))
    }

    fn are_columns_eq(&self, a: u64, b: u64) -> bool {
        for y in 0..=self.max_y {
            if self.map.get(&(a, y)) != self.map.get(&(b, y)) {
                return false;
            }
        }

        return true;
    }

    fn are_rows_eq(&self, a: u64, b: u64) -> bool {
        for x in 0..=self.max_x {
            if self.map.get(&(x, a)) != self.map.get(&(x, b)) {
                return false;
            }
        }

        return true;
    }

    fn is_vertical_symmetry(&self, x: u64) -> bool {
        for diff in 0..=(x.min(self.max_x - (x + 1))) {
            if !self.are_columns_eq(x - diff, x + 1 + diff) {
                return false;
            }
        }

        return true;
    }

    fn find_vertical_symmetry(&self) -> u64 {
        (0..self.max_x)
            .filter_map(|x| {
                if self.is_vertical_symmetry(x) {
                    Some(x + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn is_horizontal_symmetry(&self, y: u64) -> bool {
        for diff in 0..=(y.min(self.max_y - (y + 1))) {
            if !self.are_rows_eq(y - diff, y + 1 + diff) {
                return false;
            }
        }

        return true;
    }

    fn find_horizontal_symmetry(&self) -> u64 {
        (0..self.max_y)
            .filter_map(|y| {
                if self.is_horizontal_symmetry(y) {
                    Some(y + 1)
                } else {
                    None
                }
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Tile {
    Ash,
    Rock,
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((map(tag("."), |_| Self::Ash), map(tag("#"), |_| Self::Rock)))(input)
    }
}

fn main() -> Result<()> {
    let (_, game) = Game::parse(include_str!("input.txt"))?;

    dbg!(&game);

    dbg!(game.part1());

    Ok(())
}
