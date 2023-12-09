use std::num::ParseIntError;

struct Game {
    inputs: Vec<Vec<i64>>,
}

impl Game {
    fn parse(input: &str) -> Result<Self, ParseIntError> {
        let inputs = input
            .lines()
            .map(|l| {
                l.split(" ")
                    .map(|v| {
                        let r = v.parse();
                        r
                    })
                    .collect()
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { inputs })
    }

    fn diff(sequence: &Vec<i64>) -> Vec<i64> {
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

    fn next_prediction(sequence: &Vec<i64>) -> i64 {
        if sequence.iter().all(|&i| i == 0) {
            return 0;
        }

        sequence.last().unwrap() + Self::next_prediction(&Self::diff(sequence))
    }

    fn prev_prediction(sequence: &Vec<i64>) -> i64 {
        if sequence.iter().all(|&i| i == 0) {
            return 0;
        }

        sequence.first().unwrap() - Self::prev_prediction(&Self::diff(sequence))
    }

    fn part1(&self) -> i64 {
        self.inputs.iter().map(|i| Self::next_prediction(i)).sum()
    }

    fn part2(&self) -> i64 {
        self.inputs.iter().map(|i| Self::prev_prediction(i)).sum()
    }
}

fn main() {
    let game = Game::parse(include_str!("input.txt")).unwrap();

    dbg!(game.part1());
    dbg!(game.part2());
}

#[test]
fn part1() {
    let game = Game::parse(include_str!("sample-input.txt")).unwrap();

    assert_eq!(game.part1(), 114);
}

#[test]
fn part2() {
    let game = Game::parse(include_str!("sample-input.txt")).unwrap();

    assert_eq!(game.part2(), 2);
}
