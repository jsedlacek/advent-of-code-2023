use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{map_res, value},
    multi::{many1, separated_list1},
    IResult,
};

#[derive(Debug, Clone, PartialEq)]
struct Game {
    map: HashMap<(u64, u64), Rock>,
    max_x: u64,
    max_y: u64,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map_res(
            separated_list1(newline, many1(Rock::parse)),
            |rows| -> Result<Self> {
                let mut map = HashMap::new();

                for (y, row) in rows.into_iter().enumerate() {
                    let y = y as u64;
                    for (x, rock) in row.into_iter().enumerate() {
                        let x = x as u64;
                        if let Some(rock) = rock {
                            map.insert((x, y), rock);
                        }
                    }
                }

                let max_x = map
                    .keys()
                    .copied()
                    .map(|(x, _)| x)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                let max_y = map
                    .keys()
                    .copied()
                    .map(|(_, y)| y)
                    .max()
                    .ok_or(anyhow!("No keys"))?;

                Ok(Self { map, max_x, max_y })
            },
        )(input)
    }

    fn tilt(&mut self, direction: Direction) {
        if direction == Direction::North {
            for x in 0..=self.max_x {
                let mut free_y = 0;

                for y in 0..=self.max_y {
                    if let Some(&rock) = self.map.get(&(x, y)) {
                        if rock == Rock::Cube {
                            free_y = y + 1;
                        } else {
                            if y != free_y {
                                self.map.insert((x, free_y), rock);
                                self.map.remove(&(x, y));
                            }
                            free_y += 1;
                        }
                    }
                }
            }
        }

        if direction == Direction::South {
            for x in 0..=self.max_x {
                let mut free_y = self.max_y;

                for y in (0..=self.max_y).rev() {
                    if let Some(&rock) = self.map.get(&(x, y)) {
                        if rock == Rock::Cube {
                            free_y = y.saturating_sub(1);
                        } else {
                            if y != free_y {
                                self.map.insert((x, free_y), rock);
                                self.map.remove(&(x, y));
                            }

                            free_y = free_y.saturating_sub(1);
                        }
                    }
                }
            }
        }

        if direction == Direction::West {
            for y in 0..=self.max_y {
                let mut free_x = 0;

                for x in 0..=self.max_x {
                    if let Some(&rock) = self.map.get(&(x, y)) {
                        if rock == Rock::Cube {
                            free_x = x + 1;
                        } else {
                            if x != free_x {
                                self.map.insert((free_x, y), rock);
                                self.map.remove(&(x, y));
                            }
                            free_x += 1;
                        }
                    }
                }
            }
        }

        if direction == Direction::East {
            for y in 0..=self.max_y {
                let mut free_x = self.max_x;

                for x in (0..=self.max_x).rev() {
                    if let Some(&rock) = self.map.get(&(x, y)) {
                        if rock == Rock::Cube {
                            free_x = x.saturating_sub(1);
                        } else {
                            if x != free_x {
                                self.map.insert((free_x, y), rock);
                                self.map.remove(&(x, y));
                            }

                            free_x = free_x.saturating_sub(1);
                        }
                    }
                }
            }
        }
    }

    fn tilt_round(&mut self) {
        for direction in [
            Direction::North,
            Direction::West,
            Direction::South,
            Direction::East,
        ] {
            self.tilt(direction);
        }
    }

    fn tilt_multiple_rounds(&mut self, rounds: usize) {
        let mut history: HashMap<usize, Game> = HashMap::new();

        let mut index = 0;

        while index < rounds {
            self.tilt_round();
            index += 1;

            for (history_index, history_game) in &history {
                if self == history_game {
                    let diff = index - history_index;

                    let remaining_index = rounds - index;

                    index += (remaining_index / diff) * diff;
                }
            }

            history.insert(index, self.clone());
        }
    }

    fn puzzle(&self) -> u64 {
        (0..=self.max_y)
            .map(|y| {
                let row_number = self.max_y - y + 1;

                let rounded_rocks = (0..=self.max_x)
                    .filter(|&x| self.map.get(&(x, y)).copied() == Some(Rock::Rounded))
                    .count() as u64;

                row_number * rounded_rocks
            })
            .sum()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if let Some(&rock) = self.map.get(&(x, y)) {
                    if rock == Rock::Cube {
                        result.push('#');
                    } else {
                        result.push('O');
                    }
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
enum Rock {
    Rounded,
    Cube,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Rock {
    fn parse(input: &str) -> IResult<&str, Option<Self>> {
        alt((
            value(Some(Self::Rounded), tag("O")),
            value(Some(Self::Cube), tag("#")),
            value(None, tag(".")),
        ))(input)
    }
}

fn main() -> Result<()> {
    let (_, game) = Game::parse(include_str!("input.txt"))?;

    let mut game1 = game.clone();

    game1.tilt(Direction::North);

    println!("Part 1 {}", game1.puzzle());

    let mut game2 = game.clone();

    game2.tilt_multiple_rounds(1_000_000_000);

    println!("Part 2 {}", game2.puzzle());

    Ok(())
}
