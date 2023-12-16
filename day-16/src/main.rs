use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map, value},
    multi::{many0, many1, separated_list1},
    sequence::delimited,
    IResult,
};

type BoundsInclusive = ((i64, i64), (i64, i64));

#[derive(Debug, Clone)]
struct Game {
    map: HashMap<Position, Tile>,
    bounds: BoundsInclusive,
}

impl std::str::FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let (_, game) = Self::parse(input)
            .map_err(|e| anyhow!(e.to_owned()).context("Failed to parse game input"))?;

        Ok(game)
    }
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            all_consuming(delimited(
                many0(newline),
                separated_list1(newline, many1(Tile::parse)),
                many0(newline),
            )),
            |rows| {
                let map = Self::create_map(rows);
                let bounds = Self::calculate_bounds(&map);

                Self { map, bounds }
            },
        )(input)
    }

    fn create_map(rows: Vec<Vec<Option<Tile>>>) -> HashMap<Position, Tile> {
        rows.iter()
            .enumerate()
            .flat_map(|(y, row)| {
                let y = y as i64;
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    tile.map(|tile| {
                        let x = x as i64;
                        (Position(x, y), tile)
                    })
                })
            })
            .collect()
    }

    fn calculate_bounds(map: &HashMap<Position, Tile>) -> BoundsInclusive {
        let max_x = map.keys().map(|pos| pos.0).max().unwrap_or(-1);

        let max_y = map.keys().map(|pos| pos.1).max().unwrap_or(-1);

        ((0, max_x), (0, max_y))
    }

    fn part1(&self) -> u64 {
        self.calculate_energy(Position(0, 0), Direction::Right)
    }

    fn part2(&self) -> Result<u64> {
        let ((min_x, max_x), (min_y, max_y)) = self.bounds;

        let horizontal = [(min_x, Direction::Right), (max_x, Direction::Left)]
            .into_iter()
            .flat_map(|(start_x, dir)| (min_y..=max_y).map(move |y| (Position(start_x, y), dir)));

        let vertical = [(min_y, Direction::Down), (max_y, Direction::Up)]
            .into_iter()
            .flat_map(|(start_y, dir)| (min_x..=max_x).map(move |x| (Position(x, start_y), dir)));

        horizontal
            .chain(vertical)
            .map(|(pos, dir)| self.calculate_energy(pos, dir))
            .max()
            .ok_or(anyhow!("No bounds"))
    }

    fn calculate_energy(&self, start_pos: Position, start_dir: Direction) -> u64 {
        let mut beams = VecDeque::new();
        let mut visited = HashSet::new();
        let mut energized = HashSet::new();

        beams.push_back((start_pos, start_dir));

        while let Some((pos, dir)) = beams.pop_front() {
            if !self.is_pos_valid(pos) {
                continue;
            }

            if !visited.insert((pos, dir)) {
                continue;
            }

            energized.insert(pos);

            let next_dirs = if let Some(&next_tile) = self.map.get(&pos) {
                Beam::encounter(dir, next_tile)
            } else {
                vec![dir]
            };

            for d in next_dirs {
                beams.push_back((pos.move_dir(d), d));
            }
        }

        energized.len() as u64
    }

    fn is_pos_valid(&self, pos: Position) -> bool {
        let ((min_x, max_x), (min_y, max_y)) = self.bounds;

        (min_x..=max_x).contains(&pos.0) && (min_y..=max_y).contains(&pos.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Position {
    fn move_dir(self, dir: Direction) -> Self {
        let Self(x, y) = self;

        match dir {
            Direction::Up => Self(x, y - 1),
            Direction::Down => Self(x, y + 1),
            Direction::Left => Self(x - 1, y),
            Direction::Right => Self(x + 1, y),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn inverse(self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Left => Self::Right,
            Self::Down => Self::Up,
            Self::Right => Self::Left,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    MirrorUL,  // upper-left to downer-right
    MirrorUR,  // upper-right to downer-left
    SplitterU, // vertical
    SplitterL, // horizontal
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Option<Self>> {
        alt((
            value(None, tag(".")),
            value(Some(Self::MirrorUL), tag("\\")),
            value(Some(Self::MirrorUR), tag("/")),
            value(Some(Self::SplitterU), tag("|")),
            value(Some(Self::SplitterL), tag("-")),
        ))(input)
    }
}

struct Beam;

impl Beam {
    fn encounter(dir: Direction, tile: Tile) -> Vec<Direction> {
        match (dir, tile) {
            (Direction::Right, Tile::MirrorUL) => vec![Direction::Down],
            (Direction::Right, Tile::MirrorUR) => vec![Direction::Up],
            (Direction::Right, Tile::SplitterU) => vec![Direction::Up, Direction::Down],
            (Direction::Right, Tile::SplitterL) => vec![Direction::Right],

            (Direction::Left, _) => Self::encounter(Direction::Right, tile)
                .iter()
                .map(|d| d.inverse())
                .collect(),

            (Direction::Up, Tile::MirrorUL) => vec![Direction::Left],
            (Direction::Up, Tile::MirrorUR) => vec![Direction::Right],
            (Direction::Up, Tile::SplitterU) => vec![Direction::Up],
            (Direction::Up, Tile::SplitterL) => vec![Direction::Left, Direction::Right],

            (Direction::Down, _) => Self::encounter(Direction::Up, tile)
                .iter()
                .map(|d| d.inverse())
                .collect(),
        }
    }
}

fn main() -> Result<()> {
    let game = include_str!("input.txt").parse::<Game>()?;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let game = include_str!("sample-input.txt").parse::<Game>()?;

    assert_eq!(game.part1(), 46);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let game = include_str!("sample-input.txt").parse::<Game>()?;

    assert_eq!(game.part2()?, 51);

    Ok(())
}
