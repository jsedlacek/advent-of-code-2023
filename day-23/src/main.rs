use std::collections::{HashMap, HashSet, VecDeque};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map, value},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);
impl Position {
    fn move_dir(&self, dir: Direction) -> Self {
        let Self(x, y) = *self;

        match dir {
            Direction::Right => Self(x + 1, y),
            Direction::Down => Self(x, y + 1),
            Direction::Left => Self(x - 1, y),
            Direction::Up => Self(x, y - 1),
        }
    }
}

#[derive(Debug)]
struct Game {
    map: HashMap<Position, Tile>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(separated_list1(newline, many1(Tile::parse)), |lines| {
            let map = lines
                .iter()
                .enumerate()
                .flat_map(move |(y, tiles)| {
                    tiles
                        .iter()
                        .enumerate()
                        .map(move |(x, tile)| (Position(x as i64, y as i64), *tile))
                })
                .collect();

            Self { map }
        })(input)
    }

    fn find_longest_path(
        &self,
        start_pos: Position,
        end_pos: Position,
        ignore_direction: bool,
    ) -> u64 {
        let mut queue = VecDeque::new();

        let mut max_len = 0;

        queue.push_back(((None, start_pos), HashSet::new()));

        while let Some(((prev_pos, pos), visited)) = queue.pop_back() {
            let new_len = visited.len();

            if pos == end_pos {
                if new_len > max_len {
                    max_len = new_len;
                }
            }

            let (next_pos, positions, next_visited) = {
                let mut current_pos = pos;
                let mut next_visited = HashSet::new();

                next_visited.insert(pos);

                loop {
                    let positions = self
                        .find_pos_options(current_pos, ignore_direction)
                        .into_iter()
                        .filter(|p| Some(*p) != prev_pos)
                        .filter(|p| !next_visited.contains(p))
                        .collect::<Vec<_>>();

                    if let [p] = positions[..] {
                        if p != end_pos {
                            current_pos = p;
                            next_visited.insert(current_pos);
                            continue;
                        }
                    }

                    break (current_pos, positions, next_visited);
                }
            };

            let mut visited = visited.clone();
            visited.extend(next_visited);

            for p in positions {
                if !visited.contains(&p) {
                    queue.push_back(((Some(next_pos), p), visited.clone()));
                }
            }
        }

        max_len as u64
    }

    fn find_start(&self) -> Position {
        *self
            .map
            .iter()
            .filter(|(_, &tile)| tile == Tile::Path)
            .min_by_key(|(Position(_, y), _)| y)
            .unwrap()
            .0
    }

    fn find_end(&self) -> Position {
        *self
            .map
            .iter()
            .filter(|(_, &tile)| tile == Tile::Path)
            .max_by_key(|(Position(_, y), _)| y)
            .unwrap()
            .0
    }

    fn part1(&self) -> u64 {
        let (start_pos, end_pos) = (self.find_start(), self.find_end());

        self.find_longest_path(start_pos, end_pos, false)
    }

    fn part2(&self) -> u64 {
        let (start_pos, end_pos) = (self.find_start(), self.find_end());

        self.find_longest_path(start_pos, end_pos, true)
    }

    fn find_pos_options(&self, pos: Position, ignore_direction: bool) -> Vec<Position> {
        let dirs = match self.map.get(&pos) {
            Some(Tile::Path) => [
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Up,
            ]
            .to_vec(),
            Some(Tile::Slope(dir)) => {
                if ignore_direction {
                    [
                        Direction::Left,
                        Direction::Down,
                        Direction::Right,
                        Direction::Up,
                    ]
                    .to_vec()
                } else {
                    [*dir].to_vec()
                }
            }
            _ => vec![],
        };

        dirs.into_iter()
            .map(|dir| pos.move_dir(dir))
            .filter(|p| match self.map.get(p) {
                Some(tile) => tile.can_visit(),
                _ => false,
            })
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl Tile {
    fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Path, tag(".")),
            value(Self::Forest, tag("#")),
            value(Self::Slope(Direction::Up), tag("^")),
            value(Self::Slope(Direction::Right), tag(">")),
            value(Self::Slope(Direction::Down), tag("v")),
            value(Self::Slope(Direction::Left), tag("<")),
        ))(input)
    }

    fn can_visit(&self) -> bool {
        match self {
            Self::Path => true,
            Self::Slope(_) => true,
            _ => false,
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

fn main() {
    let game = Game::parse(include_str!("input.txt")).unwrap().1;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2());
}
