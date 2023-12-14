use std::{collections::HashMap, ops::RangeInclusive};

use anyhow::Result;

#[derive(Debug)]
struct Game {
    map: HashMap<(usize, usize), Cell>,
    ranges: (RangeInclusive<usize>, RangeInclusive<usize>),
}

impl Game {
    fn parse(input: &str) -> Result<Game> {
        let mut map = HashMap::new();

        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                let cell = Cell::parse(cell)?;
                if let Some(cell) = cell {
                    map.insert((x, y), cell);
                }

                if x > width {
                    width = x;
                }

                if y > height {
                    height = y;
                }
            }
        }

        let x_range = 0..=width;
        let y_range = 0..=height;

        Ok(Game {
            map,
            ranges: (x_range, y_range),
        })
    }

    fn part1(&self) -> Result<u32> {
        Ok(self
            .find_numbers()
            .into_iter()
            .filter(|n| self.is_part_number(n))
            .map(|n| n.value())
            .collect::<Result<Vec<_>>>()?
            .iter()
            .sum())
    }

    fn part2(&self) -> Result<u32> {
        let gears = self.find_gears();
        let numbers = self.find_numbers();

        gears
            .iter()
            .filter_map(|g| {
                let adjacent_numbers: Vec<_> = numbers
                    .iter()
                    .filter(|n| {
                        let (x, y) = n.surrounding_bounds();
                        x.contains(&g.0) && y.contains(&g.1)
                    })
                    .collect();

                if let [a, b] = adjacent_numbers[..] {
                    Some([a, b])
                } else {
                    None
                }
            })
            .map(|numbers| -> Result<u32> {
                Ok(numbers
                    .into_iter()
                    .map(|n| n.value())
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .product::<u32>())
            })
            .sum()
    }

    fn find_numbers(&self) -> Vec<Number> {
        let mut numbers = vec![];
        let mut current_number: Option<Number> = None;

        for y in self.ranges.1.clone() {
            for x in self.ranges.0.clone() {
                let cell = self.map.get(&(x, y));

                if let Some(Cell::Number(n)) = cell {
                    match current_number {
                        Some(ref mut number) => number.add_part(x, *n),
                        None => current_number = Some(Number::new(y, x, *n)),
                    }
                } else if let Some(number) = current_number {
                    numbers.push(number);
                    current_number = None;
                }
            }
        }

        numbers
    }

    fn find_gears(&self) -> Vec<(usize, usize)> {
        let mut gears = vec![];

        for (position, cell) in &self.map {
            if let Cell::Symbol('*') = cell {
                gears.push(*position);
            }
        }

        gears
    }

    fn is_part_number(&self, number: &Number) -> bool {
        let (x_range, y_range) = number.surrounding_bounds();

        for x in x_range {
            for y in y_range.clone() {
                if let Some(Cell::Symbol(_)) = self.map.get(&(x, y)) {
                    return true;
                }
            }
        }

        false
    }
}

#[derive(Debug)]
enum Cell {
    Number(u32),
    Symbol(char),
}

impl Cell {
    fn parse(input: char) -> Result<Option<Self>> {
        match input {
            '0'..='9' => Ok(Some(Cell::Number(input.to_string().parse()?))),
            '.' => Ok(None),
            _ => Ok(Some(Self::Symbol(input))),
        }
    }
}

#[derive(Debug)]
struct Number {
    number: String,
    y: usize,
    x_start: usize,
    x_end: usize,
}

impl Number {
    fn new(y: usize, x: usize, value: u32) -> Self {
        Number {
            number: value.to_string(),
            y,
            x_start: x,
            x_end: x,
        }
    }

    fn add_part(&mut self, x: usize, value: u32) {
        self.x_end = x;
        self.number += &value.to_string();
    }

    fn surrounding_bounds(&self) -> (RangeInclusive<usize>, RangeInclusive<usize>) {
        let x_start = if self.x_start == 0 {
            0
        } else {
            self.x_start - 1
        };
        let x_end = self.x_end + 1;

        let y_start = if self.y == 0 { 0 } else { self.y - 1 };
        let y_end = self.y + 1;

        (x_start..=x_end, y_start..=y_end)
    }

    fn value(&self) -> Result<u32> {
        Ok(self.number.parse::<u32>()?)
    }
}

fn main() -> Result<()> {
    let game = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1()?);
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let sample_game = Game::parse(include_str!("sample-input.txt"))?;
    assert_eq!(sample_game.part1()?, 4361);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let sample_game = Game::parse(include_str!("sample-input.txt"))?;
    assert_eq!(sample_game.part2()?, 467835);

    Ok(())
}
