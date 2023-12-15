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

struct Game1 {
    steps: Vec<String>,
}

impl Game1 {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag(","),
                map(recognize(many0(none_of(",\n"))), |s: &str| s.to_owned()),
            ),
            |steps| Self { steps },
        )(input)
    }

    fn puzzle(&self) -> u64 {
        self.steps.iter().map(|s| hash(s)).sum::<u64>()
    }
}

struct Game2 {
    operations: Vec<Operation>,
}

impl Game2 {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(tag(","), Operation::parse), |operations| {
            Self { operations }
        })(input)
    }

    fn puzzle(&self) -> u64 {
        let mut map: HashMap<u64, Vec<(String, u64)>> = HashMap::new();

        for op in &self.operations {
            match op {
                Operation::Add(name, focus) => {
                    let h = hash(name);
                    let mut vec = map.remove(&h).unwrap_or_default();

                    if let Some(e) = vec.iter_mut().find(|(n, _)| n == name) {
                        e.1 = *focus;
                    } else {
                        vec.push((name.to_owned(), *focus));
                    }

                    map.insert(h, vec);
                }
                Operation::Remove(name) => {
                    let h = hash(name);
                    let vec = map.remove(&h);

                    if let Some(vec) = vec {
                        let vec = vec
                            .into_iter()
                            .filter(|(n, _)| n != name)
                            .collect::<Vec<_>>();

                        map.insert(h, vec);
                    }
                }
            }
        }

        map.iter()
            .map(|(h, vec)| {
                (h + 1)
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
enum Operation {
    Add(String, u64),
    Remove(String),
}

impl Operation {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            map(
                tuple((alpha1::<&str, _>, tag("="), u64)),
                |(s, _, focus)| Self::Add(s.to_owned(), focus),
            ),
            map(tuple((alpha1::<&str, _>, tag("-"))), |(s, _)| {
                Self::Remove(s.to_owned())
            }),
        ))(input)
    }
}

fn hash(input: &str) -> u64 {
    input
        .bytes()
        .fold(0u64, |acc, b| ((acc + b as u64) * 17) % 256)
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
