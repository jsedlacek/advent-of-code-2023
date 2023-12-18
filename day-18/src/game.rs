use std::fmt::Display;

#[derive(Debug)]
pub struct Game {
    instructions: Vec<Instruction>,
}

impl Game {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    pub fn puzzle(&self) -> u64 {
        let mut pos = Position(0, 0);

        let mut space = 0;

        let mut last_y = 0;

        for ins in self.instructions.iter() {
            pos = pos.move_dir(ins.dir, ins.steps as i64);

            space += pos.0 * (pos.1 - last_y);

            last_y = pos.1;
        }

        let total_steps = self.instructions.iter().map(|i| i.steps).sum::<u64>();

        space as u64 + (total_steps / 2) + 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instruction {
    dir: Direction,
    steps: u64,
}

impl Instruction {
    pub fn new(dir: Direction, steps: u64) -> Self {
        Self { dir, steps }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(i64, i64);

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl Position {
    fn move_dir(&self, dir: Direction, count: i64) -> Self {
        match dir {
            Direction::Right => Self(self.0 + count, self.1),
            Direction::Down => Self(self.0, self.1 + count),
            Direction::Left => Self(self.0 - count, self.1),
            Direction::Up => Self(self.0, self.1 - count),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse_game_v1, parse_game_v2};

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    fn part1() -> anyhow::Result<()> {
        let (_, game) = parse_game_v1(SAMPLE_INPUT)?;

        assert_eq!(game.puzzle(), 62);

        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let (_, game) = parse_game_v2(SAMPLE_INPUT)?;

        assert_eq!(game.puzzle(), 952408144115);

        Ok(())
    }
}
