use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Game {
    map: HashMap<Position, u64>,
    max_x: i64,
    max_y: i64,
}

impl Game {
    fn parse(input: &str) -> Result<Self> {
        let mut map = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let value = c.to_digit(10).ok_or_else(|| anyhow!("Error parsing {c}"))? as u64;
                map.insert(Position(x as i64, y as i64), value);
            }
        }

        let max_x = *map
            .keys()
            .map(|Position(x, _)| x)
            .max()
            .ok_or(anyhow!("No keys"))?;
        let max_y = *map
            .keys()
            .map(|Position(_, y)| y)
            .max()
            .ok_or(anyhow!("No keys"))?;

        Ok(Self { map, max_x, max_y })
    }

    fn puzzle(&self, min_steps: u64, max_steps: u64) -> Result<u64> {
        let start_pos = Position(0, 0);

        let mut queue: VecDeque<Entry> = VecDeque::from([
            Entry::new(start_pos, Direction::Right, 0, 0),
            Entry::new(start_pos, Direction::Down, 0, 0),
        ]);

        let mut results: HashMap<(Position, Direction, u64), u64> = HashMap::new();

        while let Some(Entry {
            pos,
            dir,
            steps,
            heat,
        }) = queue.pop_front()
        {
            if !self.map.contains_key(&pos) {
                continue;
            }

            if let Some(&existing_heat) = results.get(&(pos, dir, steps)) {
                if existing_heat <= heat {
                    continue;
                }
            }

            if steps >= min_steps {
                results.insert((pos, dir, steps), heat);
            }

            if steps < max_steps {
                let next_pos = pos.move_dir(dir);
                if let Some(next_tile_heat) = self.map.get(&next_pos) {
                    queue.push_back(Entry::new(next_pos, dir, steps + 1, heat + next_tile_heat))
                }
            }

            if steps >= min_steps {
                for turn in [Turn::Left, Turn::Right] {
                    let next_dir = dir.turn(turn);

                    let next_pos = pos.move_dir(next_dir);
                    if let Some(next_tile_heat) = self.map.get(&next_pos) {
                        queue.push_back(Entry::new(next_pos, next_dir, 1, heat + next_tile_heat))
                    }
                }
            }
        }

        dbg!(results.len());

        let end_pos = Position(self.max_x, self.max_y);

        let total_heat = results
            .iter()
            .filter_map(|((pos, _, _), heat)| if *pos == end_pos { Some(*heat) } else { None })
            .min()
            .ok_or(anyhow!("End unreachable"))?;

        Ok(total_heat)
    }

    fn part1(&self) -> Result<u64> {
        self.puzzle(0, 3)
    }

    fn part2(&self) -> Result<u64> {
        self.puzzle(4, 10)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Entry {
    pos: Position,
    dir: Direction,
    steps: u64,
    heat: u64,
}

impl Entry {
    fn new(pos: Position, dir: Direction, steps: u64, heat: u64) -> Self {
        Self {
            pos,
            dir,
            steps,
            heat,
        }
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.heat.cmp(&(other.heat))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Position {
    fn move_dir(self, dir: Direction) -> Self {
        match dir {
            Direction::Right => Self(self.0 + 1, self.1),
            Direction::Down => Self(self.0, self.1 + 1),
            Direction::Left => Self(self.0 - 1, self.1),
            Direction::Up => Self(self.0, self.1 - 1),
        }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Up, Turn::Left) => Direction::Left,

            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Up, Turn::Right) => Direction::Right,
        }
    }
}

enum Turn {
    Left,
    Right,
}

fn main() -> Result<()> {
    let game = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1()?);
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let game = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part1()?, 102);

    Ok(())
}

#[test]
fn part2_1() -> Result<()> {
    let game = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part2()?, 94);

    Ok(())
}

#[test]
fn part2_2() -> Result<()> {
    let game = Game::parse(include_str!("sample-input-2.txt"))?;

    assert_eq!(game.part2()?, 71);

    Ok(())
}
