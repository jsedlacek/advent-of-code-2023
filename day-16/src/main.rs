use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::RangeInclusive,
};

use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    map: HashMap<Position, Tile>,
    bounds: (RangeInclusive<i64>, RangeInclusive<i64>),
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            separated_list1(newline, many1(Tile::parse)),
            |rows| -> Result<Self> {
                let map: HashMap<Position, Tile> = rows
                    .into_iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        let y = y as i64;
                        row.into_iter().enumerate().filter_map(move |(x, tile)| {
                            tile.map(|tile| {
                                let x = x as i64;
                                (Position(x, y), tile)
                            })
                        })
                    })
                    .collect();

                let max_x = map
                    .keys()
                    .map(|pos| pos.0)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                let max_y = map
                    .keys()
                    .map(|pos| pos.1)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                let bounds = ((0..=max_x), (0..=max_y));

                Ok(Self { map, bounds })
            },
        )(input)
    }

    fn part1(&self) -> u64 {
        self.calculate_energy(Position(0, 0), Direction::Right)
    }

    fn part2(&self) -> Result<u64> {
        let from_top = self
            .bounds
            .0
            .clone()
            .map(|x| (Position(x, *self.bounds.1.start()), Direction::Down));

        let from_bottom = self
            .bounds
            .0
            .clone()
            .map(|x| (Position(x, *self.bounds.1.end()), Direction::Up));

        let from_left = self
            .bounds
            .1
            .clone()
            .map(|y| (Position(*self.bounds.0.start(), y), Direction::Right));

        let from_right = self
            .bounds
            .1
            .clone()
            .map(|y| (Position(*self.bounds.0.end(), y), Direction::Left));

        from_top
            .chain(from_bottom)
            .chain(from_left)
            .chain(from_right)
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
                HashSet::from([dir])
            };

            let mut next_beams = next_dirs
                .into_iter()
                .map(|d| (pos.move_dir(d), d))
                .collect();

            beams.append(&mut next_beams);
        }

        energized.len() as u64
    }

    fn is_pos_valid(&self, pos: Position) -> bool {
        self.bounds.0.contains(&pos.0) && self.bounds.1.contains(&pos.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Position {
    fn move_dir(self, dir: Direction) -> Self {
        let Self(mut x, mut y) = self;
        match dir {
            Direction::Up => y = self.1 - 1,
            Direction::Down => y = self.1 + 1,
            Direction::Left => x = self.0 - 1,
            Direction::Right => x = self.0 + 1,
        }

        Self(x, y)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    MirrorUL,
    MirrorUR,
    SplitterU,
    SplitterL,
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
    fn encounter(dir: Direction, tile: Tile) -> HashSet<Direction> {
        match (dir, tile) {
            (Direction::Right, Tile::MirrorUL) => HashSet::from([Direction::Down]),
            (Direction::Right, Tile::MirrorUR) => HashSet::from([Direction::Up]),
            (Direction::Right, Tile::SplitterU) => HashSet::from([Direction::Up, Direction::Down]),
            (Direction::Right, Tile::SplitterL) => HashSet::from([Direction::Right]),

            (Direction::Left, Tile::MirrorUL) => HashSet::from([Direction::Up]),
            (Direction::Left, Tile::MirrorUR) => HashSet::from([Direction::Down]),
            (Direction::Left, Tile::SplitterU) => HashSet::from([Direction::Up, Direction::Down]),
            (Direction::Left, Tile::SplitterL) => HashSet::from([Direction::Left]),

            (Direction::Up, Tile::MirrorUL) => HashSet::from([Direction::Left]),
            (Direction::Up, Tile::MirrorUR) => HashSet::from([Direction::Right]),
            (Direction::Up, Tile::SplitterU) => HashSet::from([Direction::Up]),
            (Direction::Up, Tile::SplitterL) => HashSet::from([Direction::Left, Direction::Right]),

            (Direction::Down, Tile::MirrorUL) => HashSet::from([Direction::Right]),
            (Direction::Down, Tile::MirrorUR) => HashSet::from([Direction::Left]),
            (Direction::Down, Tile::SplitterU) => HashSet::from([Direction::Down]),
            (Direction::Down, Tile::SplitterL) => {
                HashSet::from([Direction::Left, Direction::Right])
            }
        }
    }
}

fn main() -> Result<()> {
    let (_, game) = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, game) = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part1(), 46);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, game) = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part2()?, 51);

    Ok(())
}
