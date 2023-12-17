use std::{
    collections::{BTreeSet, HashMap},
    fmt::Display,
    str::FromStr,
};

use nom::{
    character::complete::{newline, one_of},
    combinator::{all_consuming, map_res},
    multi::{many0, many1, separated_list1},
    sequence::delimited,
    IResult,
};

#[derive(Debug)]
enum GameError {
    Parse(nom::Err<nom::error::Error<String>>),
    NoKeys,
    EndUnreachable,
}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let description = match self {
            Self::Parse(_) => "parsing error",
            Self::NoKeys => "there are no keys in map",
            Self::EndUnreachable => "end point is unreachable",
        };
        write!(f, "{description}")
    }
}

impl std::error::Error for GameError {}

impl From<nom::Err<nom::error::Error<&str>>> for GameError {
    fn from(value: nom::Err<nom::error::Error<&str>>) -> Self {
        Self::Parse(value.to_owned())
    }
}

type Result<T, E = GameError> = std::result::Result<T, E>;

#[derive(Debug)]
struct Game {
    map: HashMap<Position, u64>,
    max_x: i64,
    max_y: i64,
}

impl FromStr for Game {
    type Err = GameError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (_, game) =
            all_consuming(delimited(many0(newline), Game::parse, many0(newline)))(input)?;

        Ok(game)
    }
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            separated_list1(
                newline,
                many1(map_res(one_of("0123456789"), |c| {
                    c.to_string().parse::<u64>()
                })),
            ),
            |rows| -> Result<Self> {
                let map: HashMap<Position, u64> = rows
                    .into_iter()
                    .enumerate()
                    .flat_map(|(y, row)| {
                        row.into_iter()
                            .enumerate()
                            .map(move |(x, heat)| (Position(x as i64, y as i64), heat))
                    })
                    .collect();

                let max_x = *map
                    .keys()
                    .map(|Position(x, _)| x)
                    .max()
                    .ok_or(GameError::NoKeys)?;

                let max_y = *map
                    .keys()
                    .map(|Position(_, y)| y)
                    .max()
                    .ok_or(GameError::NoKeys)?;

                Ok(Self { map, max_x, max_y })
            },
        )(input)
    }

    fn puzzle(&self, min_steps: u64, max_steps: u64) -> Result<u64> {
        let start_pos = Position(0, 0);

        let mut queue = BTreeSet::from([
            Entry::new(start_pos, Direction::Right, 0, 0),
            Entry::new(start_pos, Direction::Down, 0, 0),
        ]);

        let mut results: HashMap<(Position, Direction, u64), u64> = HashMap::new();

        let end_pos = Position(self.max_x, self.max_y);

        while let Some(Entry {
            pos,
            dir,
            steps,
            heat,
        }) = queue.pop_first()
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
                if pos == end_pos {
                    return Ok(heat);
                }

                results.insert((pos, dir, steps), heat);
            }

            if steps < max_steps {
                if let Some(entry) = self.calculate_next_entry(pos, dir, heat, steps, None) {
                    queue.insert(entry);
                }
            }

            if steps >= min_steps {
                for turn in [Turn::Clockwise, Turn::CounterClockwise] {
                    if let Some(entry) =
                        self.calculate_next_entry(pos, dir, heat, steps, Some(turn))
                    {
                        queue.insert(entry);
                    }
                }
            }
        }

        Err(GameError::EndUnreachable)
    }

    fn part1(&self) -> Result<u64> {
        self.puzzle(0, 3)
    }

    fn part2(&self) -> Result<u64> {
        self.puzzle(4, 10)
    }

    fn calculate_next_entry(
        &self,
        pos: Position,
        dir: Direction,
        heat: u64,
        steps: u64,
        turn: Option<Turn>,
    ) -> Option<Entry> {
        let next_dir = if let Some(turn) = turn {
            dir.turn(turn)
        } else {
            dir
        };

        let next_steps = if turn.is_some() { 1 } else { steps + 1 };

        let next_pos = pos.move_dir(next_dir);

        if let Some(next_tile_heat) = self.map.get(&next_pos) {
            let next_heat = heat + next_tile_heat;
            Some(Entry::new(next_pos, next_dir, next_steps, next_heat))
        } else {
            None
        }
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
        (self.heat, self.steps, self.pos, self.dir)
            .cmp(&((other.heat, other.steps, other.pos, other.dir)))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Direction {
    Left,
    Down,
    Right,
    Up,
}

impl Direction {
    fn turn(&self, turn: Turn) -> Direction {
        match (self, turn) {
            (Direction::Right, Turn::Clockwise) => Direction::Down,
            (Direction::Down, Turn::Clockwise) => Direction::Left,
            (Direction::Left, Turn::Clockwise) => Direction::Up,
            (Direction::Up, Turn::Clockwise) => Direction::Right,

            (Direction::Right, Turn::CounterClockwise) => Direction::Up,
            (Direction::Down, Turn::CounterClockwise) => Direction::Right,
            (Direction::Left, Turn::CounterClockwise) => Direction::Down,
            (Direction::Up, Turn::CounterClockwise) => Direction::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Turn {
    Clockwise,
    CounterClockwise,
}

fn main() -> Result<()> {
    let game = include_str!("input.txt").parse::<Game>()?;

    println!("Part 1: {}", game.part1()?);
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let game = include_str!("sample-input.txt").parse::<Game>()?;

    assert_eq!(game.part1()?, 102);

    Ok(())
}

#[test]
fn part2_1() -> Result<()> {
    let game = include_str!("sample-input.txt").parse::<Game>()?;

    assert_eq!(game.part2()?, 94);

    Ok(())
}

#[test]
fn part2_2() -> Result<()> {
    let game = include_str!("sample-input-2.txt").parse::<Game>()?;

    assert_eq!(game.part2()?, 71);

    Ok(())
}

#[test]
fn entry() {
    let tree = BTreeSet::from([
        Entry::new(Position(0, 0), Direction::Up, 0, 1),
        Entry::new(Position(0, 0), Direction::Up, 0, 2),
        Entry::new(Position(0, 0), Direction::Up, 0, 0),
    ]);

    assert_eq!(
        tree.first(),
        Some(&Entry::new(Position(0, 0), Direction::Up, 0, 0))
    );

    assert_eq!(
        tree.last(),
        Some(&Entry::new(Position(0, 0), Direction::Up, 0, 2))
    );
}
