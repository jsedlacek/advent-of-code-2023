use std::fmt::Display;

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, newline, space1, u64},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
enum GameVersion {
    V1,
    V2,
}

#[derive(Debug)]
struct Game {
    instructions: Vec<Instruction>,
}

impl Game {
    fn parse(version: GameVersion) -> impl FnOnce(&str) -> IResult<&str, Self> {
        move |input: &str| {
            map(
                separated_list1(
                    newline,
                    if version == GameVersion::V1 {
                        Instruction::parse_v1
                    } else {
                        Instruction::parse_v2
                    },
                ),
                |instructions| Self { instructions },
            )(input)
        }
    }

    fn puzzle(&self) -> u64 {
        let mut pos = Position(0, 0);

        let mut space = 0;

        let mut last_y = 0;

        for ins in self.instructions.iter() {
            pos = pos.move_dir(ins.dir, ins.steps as i64);

            space += pos.0 * (pos.1 - last_y);

            last_y = pos.1;
        }

        let total_steps = self.instructions.iter().map(|i| i.steps).sum::<u64>();

        space as u64 + (total_steps / 2) + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Instruction {
    dir: Direction,
    steps: u64,
}

impl Instruction {
    fn parse_v1(input: &str) -> IResult<&str, Self> {
        // Example: "R 6 (#70c710)"

        map(
            tuple((
                Direction::parse,
                space1,
                u64,
                space1,
                tag("(#"),
                alphanumeric1,
                tag(")"),
            )),
            |(dir, _, steps, _, _, _, _)| Self { dir, steps },
        )(input)
    }

    fn parse_v2(input: &str) -> IResult<&str, Self> {
        // Example: "R 6 (#70c710)"

        map(
            tuple((
                Direction::parse,
                space1,
                u64,
                space1,
                tag("(#"),
                map(
                    tuple((
                        map_res(take(5u8), |s| u64::from_str_radix(s, 16)),
                        Direction::parse_v2,
                    )),
                    |(steps, dir)| Self { dir, steps },
                ),
                tag(")"),
            )),
            |(_, _, _, _, _, ins, _)| ins,
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Position {
    fn move_dir(&self, dir: Direction, count: i64) -> Self {
        match dir {
            Direction::Right => Self(self.0 + count, self.1),
            Direction::Down => Self(self.0, self.1 + count),
            Direction::Left => Self(self.0 - count, self.1),
            Direction::Up => Self(self.0, self.1 - count),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Right, tag("R")),
            value(Self::Down, tag("D")),
            value(Self::Left, tag("L")),
            value(Self::Up, tag("U")),
        ))(input)
    }

    fn parse_v2(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Right, tag("0")),
            value(Self::Down, tag("1")),
            value(Self::Left, tag("2")),
            value(Self::Up, tag("3")),
        ))(input)
    }
}

fn main() -> Result<()> {
    let (_, game1) = Game::parse(GameVersion::V1)(include_str!("input.txt"))?;

    println!("Part 1: {}", game1.puzzle());

    let (_, game2) = Game::parse(GameVersion::V2)(include_str!("input.txt"))?;

    println!("Part 2: {}", game2.puzzle());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, game) = Game::parse(GameVersion::V1)(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 62);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, game) = Game::parse(GameVersion::V2)(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 952408144115);

    Ok(())
}

#[test]
fn parse1() -> Result<()> {
    let (_, instruction) = Instruction::parse_v1("R 6 (#70c710)")?;

    assert_eq!(
        instruction,
        Instruction {
            dir: Direction::Right,
            steps: 6,
        }
    );

    Ok(())
}

#[test]
fn parse2() -> Result<()> {
    let (_, instruction) = Instruction::parse_v2("R 6 (#70c710)")?;

    assert_eq!(
        instruction,
        Instruction {
            dir: Direction::Right,
            steps: 461937,
        }
    );

    Ok(())
}
