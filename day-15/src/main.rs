use std::collections::HashMap;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, none_of, u64},
    combinator::{map, recognize},
    multi::{many0, separated_list1},
    sequence::tuple,
    IResult,
};

struct Game1<'a> {
    steps: Vec<&'a str>,
}

impl<'a> Game1<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(
            separated_list1(tag(","), recognize(many0(none_of(",\n")))),
            |steps| Self { steps },
        )(input)
    }

    fn puzzle(&self) -> u64 {
        self.steps.iter().map(|s| hash(s) as u64).sum::<u64>()
    }
}

struct Game2<'a> {
    operations: Vec<Operation<'a>>,
}

impl<'a> Game2<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        map(separated_list1(tag(","), Operation::parse), |operations| {
            Self { operations }
        })(input)
    }

    fn puzzle(&self) -> u64 {
        let mut map: HashMap<u8, Vec<(&str, u64)>> = HashMap::new();

        for op in &self.operations {
            match op {
                Operation::Add(name, focus) => {
                    let vec = map.entry(hash(name)).or_default();

                    match vec.iter_mut().find(|(n, _)| n == name) {
                        Some((_, f)) => *f = *focus,
                        None => vec.push((name, *focus)),
                    }
                }
                Operation::Remove(name) => {
                    map.entry(hash(name))
                        .and_modify(|vec| vec.retain(|(n, _)| n != name));
                }
            }
        }

        map.iter()
            .map(|(h, vec)| {
                (*h as u64 + 1)
                    * vec
                        .iter()
                        .enumerate()
                        .map(|(index, (_, focus))| (index as u64 + 1) * focus)
                        .sum::<u64>()
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
enum Operation<'a> {
    Add(&'a str, u64),
    Remove(&'a str),
}

impl<'a> Operation<'a> {
    fn parse(input: &'a str) -> IResult<&str, Self> {
        alt((
            map(tuple((alpha1, tag("="), u64)), |(s, _, focus)| {
                Self::Add(s, focus)
            }),
            map(tuple((alpha1, tag("-"))), |(s, _)| Self::Remove(s)),
        ))(input)
    }
}

fn hash(input: &str) -> u8 {
    input
        .bytes()
        .fold(0u8, |acc, b| ((acc.wrapping_add(b)).wrapping_mul(17)))
}

fn main() -> Result<()> {
    let (_, game) = Game1::parse(include_str!("input.txt"))?;
    println!("Part 1: {}", game.puzzle());

    let (_, game) = Game2::parse(include_str!("input.txt"))?;
    println!("Part 2: {}", game.puzzle());

    Ok(())
}

#[test]
fn test_hash() {
    assert_eq!(hash("HASH"), 52);
}

#[test]
fn test_part1() -> Result<()> {
    let (_, game) = Game1::parse(include_str!("sample-input.txt"))?;
    assert_eq!(game.puzzle(), 1320);

    Ok(())
}

#[test]
fn test_part2() -> Result<()> {
    let (_, game) = Game2::parse(include_str!("sample-input.txt"))?;
    assert_eq!(game.puzzle(), 145);

    Ok(())
}
