use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
impl Direction {
    fn inverse(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }
}

#[derive(Debug)]
struct Tile(HashSet<Direction>);

impl Tile {
    fn parse(c: char) -> Option<Self> {
        match c {
            '|' => Some(Self(HashSet::from_iter([
                Direction::North,
                Direction::South,
            ]))),
            '-' => Some(Self(HashSet::from_iter([Direction::East, Direction::West]))),
            'L' => Some(Self(HashSet::from_iter([
                Direction::North,
                Direction::East,
            ]))),

            'J' => Some(Self(HashSet::from_iter([
                Direction::North,
                Direction::West,
            ]))),

            '7' => Some(Self(HashSet::from_iter([
                Direction::South,
                Direction::West,
            ]))),

            'F' => Some(Self(HashSet::from_iter([
                Direction::South,
                Direction::East,
            ]))),

            _ => None,
        }
    }
}

#[derive(Debug)]
struct Game {
    map: HashMap<Point, Tile>,
    start_pos: Point,
}

impl Game {
    fn parse(input: &str) -> Result<Self> {
        let mut start_pos = None;

        let mut map = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                let y = y as i64;
                if c == 'S' {
                    start_pos = Some(Point { x, y });
                } else if let Some(tile) = Tile::parse(c) {
                    map.insert(Point { x, y }, tile);
                }
            }
        }

        let start_pos = start_pos.ok_or(anyhow!("Start position not found"))?;

        let mut set = HashSet::new();

        for dir in [
            Direction::East,
            Direction::South,
            Direction::West,
            Direction::North,
        ] {
            if let Some(tile) = map.get(&start_pos.move_dir(dir)) {
                if tile.0.contains(&dir.inverse()) {
                    set.insert(dir);
                }
            }
        }

        map.insert(start_pos, Tile(set));

        Ok(Self { map, start_pos })
    }

    fn find_farthest(&self) -> i64 {
        let mut distances: HashMap<Point, i64> = HashMap::new();
        let mut queue: VecDeque<(Point, i64)> = VecDeque::new();

        queue.push_back((self.start_pos, 0));

        while let Some((point, step)) = queue.pop_front() {
            if distances.contains_key(&point) {
                continue;
            }

            distances.insert(point, step);

            if let Some(tile) = self.map.get(&point) {
                for dir in &tile.0 {
                    let next_point = point.move_dir(*dir);
                    queue.push_back((next_point, step + 1));
                }
            }
        }

        distances.into_values().max().unwrap_or(0)
    }

    fn find_inside_tiles(&self) -> Result<i64> {
        let mut wall_tiles: HashSet<Point> = HashSet::new();

        let mut wall_queue: VecDeque<(Point, i64)> = VecDeque::new();

        wall_queue.push_back((self.start_pos, 0));

        while let Some((point, step)) = wall_queue.pop_front() {
            if wall_tiles.contains(&point) {
                continue;
            }

            wall_tiles.insert(point);

            if let Some(tile) = self.map.get(&point) {
                for dir in &tile.0 {
                    let next_point = point.move_dir(*dir);
                    wall_queue.push_back((next_point, step + 1));
                }
            }
        }

        let max_x = self
            .map
            .keys()
            .map(|p| p.x)
            .max()
            .ok_or(anyhow!("No map keys"))?;

        let max_y = self
            .map
            .keys()
            .map(|p| p.y)
            .max()
            .ok_or(anyhow!("No map keys"))?;

        let mut count = 0;
        let mut inside = false;

        for y in 0..=max_y {
            for x in 0..=max_x {
                let point = Point { x, y };
                if wall_tiles.contains(&point) {
                    if let Some(tile) = self.map.get(&point) {
                        if tile.0.contains(&Direction::North) {
                            inside = !inside;
                        }
                    }
                } else if inside {
                    count += 1;
                }
            }
        }

        Ok(count)
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
}

fn main() -> Result<()> {
    let game = Game::parse(include_str!("input.txt"))?;

    dbg!(game.find_farthest());

    dbg!(game.find_inside_tiles()?);

    Ok(())
}
