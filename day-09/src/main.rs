use anyhow::{anyhow, Result};

struct Game {
    inputs: Vec<Vec<i64>>,
}

impl Game {
    fn parse(input: &str) -> Result<Self> {
        let inputs = input
            .lines()
            .map(|l| l.split(' ').map(|v| v.parse()).collect())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { inputs })
    }

    fn diff(sequence: &[i64]) -> Vec<i64> {
        sequence
            .windows(2)
            .map(|window| {
                if let [a, b] = window {
                    b - a
                } else {
                    panic!("Invalid window")
                }
            })
            .collect()
    }

    fn next_prediction(sequence: &[i64]) -> Result<i64> {
        if sequence.iter().all(|&i| i == 0) {
            return Ok(0);
        }

        Ok(sequence.last().ok_or(anyhow!("Empty sequence"))?
            + Self::next_prediction(&Self::diff(sequence))?)
    }

    fn prev_prediction(sequence: &[i64]) -> Result<i64> {
        if sequence.iter().all(|&i| i == 0) {
            return Ok(0);
        }

        Ok(sequence.first().ok_or(anyhow!("Empty sequence"))?
            - Self::prev_prediction(&Self::diff(sequence))?)
    }

    fn part1(&self) -> Result<i64> {
        self.inputs.iter().map(|i| Self::next_prediction(i)).sum()
    }

    fn part2(&self) -> Result<i64> {
        self.inputs.iter().map(|i| Self::prev_prediction(i)).sum()
    }
}

fn main() -> Result<()> {
    let game = Game::parse(include_str!("input.txt"))?;

    dbg!(game.part1()?);
    dbg!(game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let game = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part1()?, 114);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let game = Game::parse(include_str!("sample-input.txt"))?;

    assert_eq!(game.part2()?, 2);

    Ok(())
}
