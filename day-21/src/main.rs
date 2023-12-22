use std::collections::HashSet;

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Game {
    set: HashSet<Point>,
    start_pos: Point,
    size: Point,
}

impl Game {
    fn parse(input: &str) -> Result<Self> {
        let mut start_pos = None;

        let mut set = HashSet::new();

        let (mut max_x, mut max_y) = (0, 0);

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                let y = y as i64;

                max_x = max_x.max(x);
                max_y = max_y.max(y);

                match c {
                    'S' => {
                        start_pos = Some(Point { x, y });
                        set.insert(Point { x, y });
                    }
                    '.' => {
                        set.insert(Point { x, y });
                    }
                    '#' => {}
                    _ => {
                        panic!("Invalid char: {}", c);
                    }
                }
            }
        }

        let start_pos = start_pos.ok_or(anyhow!("Start position not found"))?;

        Ok(Self {
            set,
            start_pos,
            size: Point {
                x: max_x + 1,
                y: max_y + 1,
            },
        })
    }

    fn part1(&self) -> u64 {
        let mut visited = HashSet::new();
        visited.insert(self.start_pos);

        for _ in 0..64 {
            let mut next_visited = HashSet::new();
            for p in visited {
                for dir in [
                    Direction::East,
                    Direction::South,
                    Direction::West,
                    Direction::North,
                ] {
                    let next_point = p.move_dir(dir);
                    if self.set.contains(&next_point.modulus(&self.size)) {
                        next_visited.insert(next_point);
                    }
                }
            }
            visited = next_visited;
        }

        visited.len() as u64
    }

    fn part2(&self, step_count: u64) -> String {
        let mut visited = HashSet::new();
        visited.insert(self.start_pos);

        let mut results = Vec::new();

        for i in 1..=step_count {
            let mut next_visited = HashSet::new();
            for p in visited {
                for dir in [
                    Direction::East,
                    Direction::South,
                    Direction::West,
                    Direction::North,
                ] {
                    let next_point = p.move_dir(dir);
                    if self.set.contains(&next_point.modulus(&self.size)) {
                        next_visited.insert(next_point);
                    }
                }
            }
            visited = next_visited;

            results.push(format!("\n- {i}: {}", visited.len()));
        }

        results.join("")
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn move_dir(&self, dir: Direction) -> Self {
        match dir {
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
        }
    }

    fn modulus(&self, size: &Self) -> Self {
        let Self { mut x, mut y } = self;

        x = x.checked_rem_euclid(size.x).unwrap();
        y = y.checked_rem_euclid(size.y).unwrap();

        Self { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() -> Result<()> {
    let game = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2(10));

    Ok(())
}
