use std::collections::VecDeque;

use anyhow::Result;
use nom::{
    bytes::complete::tag,
    character::{complete::i64, streaming::newline},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    bricks: Vec<Brick>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "1,0,1"
        map(separated_list1(newline, Brick::parse), |bricks| Self {
            bricks,
        })(input)
    }

    fn part1(&mut self) -> u64 {
        self.settle();

        self.bricks
            .iter()
            .filter(|&removed_brick| {
                let mut game = self.clone();
                game.bricks.retain(|b| b != removed_brick);

                game.bricks.iter().all(|brick| !game.can_brick_fall(brick))
            })
            .count() as u64
    }

    fn part2(&mut self) -> u64 {
        self.settle();

        self.bricks
            .iter()
            .map(|removed_brick| {
                let mut game = self.clone();
                game.bricks.retain(|b| b != removed_brick);

                game.settle()
            })
            .sum::<u64>() as u64
    }

    fn settle(&mut self) -> u64 {
        self.bricks.sort_by_key(|b| b.start.2.min(b.end.2));
        let mut bricks = VecDeque::from(self.bricks.clone());
        let mut res_bricks = Vec::new();
        let mut count_fallen = 0;

        while let Some(mut brick) = bricks.pop_front() {
            let other_bricks = res_bricks
                .iter()
                .filter(|&b| b != &brick)
                .collect::<Vec<_>>();

            let mut has_fallen = false;

            while brick.can_fall(&other_bricks) {
                brick.fall();
                has_fallen = true;
            }

            if has_fallen {
                count_fallen += 1;
            }

            res_bricks.push(brick);
        }

        self.bricks = res_bricks;

        count_fallen
    }

    fn can_brick_fall(self: &Self, brick: &Brick) -> bool {
        let other_bricks = self
            .bricks
            .iter()
            .filter(|&b| b != brick)
            .collect::<Vec<_>>();

        brick.can_fall(&other_bricks)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    start: Point3D,
    end: Point3D,
}

impl Brick {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "1,0,1~1,2,1"

        map(
            separated_pair(Point3D::parse, tag("~"), Point3D::parse),
            |(start, end)| Self { start, end },
        )(input)
    }

    fn fall(&mut self) {
        // println!("Brick falling {:?}", &self);
        self.start.2 -= 1;
        self.end.2 -= 1;
    }

    fn insersets_with(&self, next_brick: &Brick) -> bool {
        // Check if the current brick intersects with the next brick in any dimension
        let x_overlap = (self.start.0 <= next_brick.end.0 && self.end.0 >= next_brick.start.0)
            || (next_brick.start.0 <= self.end.0 && next_brick.end.0 >= self.start.0);
        let y_overlap = (self.start.1 <= next_brick.end.1 && self.end.1 >= next_brick.start.1)
            || (next_brick.start.1 <= self.end.1 && next_brick.end.1 >= self.start.1);
        let z_overlap = self.end.2 == next_brick.start.2;

        x_overlap && y_overlap && z_overlap
    }

    fn can_fall(self: &Self, other_bricks: &[&Brick]) -> bool {
        if self.start.2 == 1 || self.end.2 == 1 {
            return false;
        }

        let mut next_brick = self.clone();
        next_brick.fall();

        other_bricks.iter().all(|b| !b.insersets_with(&next_brick))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point3D(i64, i64, i64);

impl Point3D {
    fn parse(input: &str) -> IResult<&str, Self> {
        // Example: "1,0,1"
        map(
            tuple((i64, tag(","), i64, tag(","), i64)),
            |(x, _, y, _, z)| Self(x, y, z),
        )(input)
    }
}

fn main() -> Result<()> {
    let (_, mut game) = Game::parse(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    fn test_part1() {
        let (_, mut game) = Game::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(game.part1(), 5);
    }

    #[test]
    fn test_part2() {
        let (_, mut game) = Game::parse(SAMPLE_INPUT).unwrap();

        assert_eq!(game.part2(), 7);
    }
}
