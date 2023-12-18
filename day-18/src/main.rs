use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, space1, u64},
    combinator::{all_consuming, map, value},
    multi::{many0, separated_list1},
    sequence::{delimited, preceded, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    instructions: Vec<Instruction>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(newline, Instruction::parse),
            |instructions| Self { instructions },
        )(input)
    }

    fn find_range(map: &HashSet<Position>) -> ((i64, i64), (i64, i64)) {
        let x = map.iter().map(|Position(x, _)| x);
        let y = map.iter().map(|Position(_, y)| y);

        (
            (*x.clone().min().unwrap(), *x.max().unwrap()),
            (*y.clone().min().unwrap(), *y.max().unwrap()),
        )
    }

    fn find_wall(&self) -> HashSet<Position> {
        let mut wall = HashSet::new();

        let mut pos = Position(0, 0);

        wall.insert(pos);

        for ins in self.instructions.iter() {
            for _ in 0..ins.steps {
                pos = pos.move_dir(ins.dir);
                wall.insert(pos);
            }
        }

        wall
    }

    fn find_outside(&self, wall: &HashSet<Position>) -> HashSet<Position> {
        let ((min_x, max_x), (min_y, max_y)) = Self::find_range(&wall);

        let starting_point = Position(min_x - 1, min_y - 1);

        let (range_x, range_y) = ((min_x - 1)..=(max_x + 1), (min_y - 1)..=(max_y + 1));

        let mut queue = VecDeque::from([starting_point]);

        let mut outside = HashSet::new();

        while let Some(pos) = queue.pop_front() {
            if outside.contains(&pos) {
                continue;
            }

            outside.insert(pos);

            for dir in [
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Up,
            ] {
                let next_pos = pos.move_dir(dir);

                if range_x.contains(&next_pos.0)
                    && range_y.contains(&next_pos.1)
                    && !outside.contains(&next_pos)
                    && !wall.contains(&next_pos)
                {
                    queue.push_back(next_pos);
                }
            }
        }

        outside
    }

    fn print_map(map: &HashSet<Position>) {
        let ((min_x, max_x), (min_y, max_y)) = Self::find_range(&map);

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if map.contains(&Position(x, y)) {
                        "#"
                    } else {
                        " "
                    }
                );
            }
            println!();
        }
    }

    fn part1(&self) -> u64 {
        let wall = self.find_wall();

        let outside = self.find_outside(&wall);

        let ((min_x, max_x), (min_y, max_y)) = Self::find_range(&wall);

        let mut inside = HashSet::new();

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if !outside.contains(&Position(x, y)) {
                    inside.insert(Position(x, y));
                }
            }
        }

        inside.len() as u64
    }
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, game) = all_consuming(delimited(many0(newline), Game::parse, many0(newline)))(s)
            .map_err(|e| e.to_owned())?;

        Ok(game)
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    dir: Direction,
    steps: u64,
    color: Color,
}

impl Instruction {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "R 6 (#70c710)"

        map(
            tuple((
                Direction::parse,
                space1,
                u64,
                space1,
                tag("("),
                Color::parse,
                tag(")"),
            )),
            |(dir, _, steps, _, _, color, _)| Self { dir, steps, color },
        )(input)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Position {
    fn move_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::Right => Self(self.0 + 1, self.1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Up => Self(self.0, self.1 - 1),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
}

#[derive(Debug, Clone)]
struct Color(String);

impl Color {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "#70c710"
        map(preceded(tag("#"), alphanumeric1), |c: &str| {
            Self(c.to_string())
        })(input)
    }
}

fn main() -> Result<()> {
    let game = Game::from_str(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let game = Game::from_str(include_str!("sample-input.txt"))?;

    assert_eq!(game.part1(), 62);

    Ok(())
}
