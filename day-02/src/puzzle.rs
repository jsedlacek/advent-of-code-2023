use nom::{character::complete::multispace0, multi::many0, sequence::delimited, IResult, Parser};

use crate::game::Game;

pub struct Puzzle {
    games: Vec<Game>,
}

impl Puzzle {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        let (input, games) =
            many0(delimited(multispace0, Game::parse, multispace0)).parse(input)?;

        Ok((input, Self { games }))
    }

    pub fn part1(&self) -> u32 {
        self.games
            .iter()
            .filter(|g| g.is_possible())
            .map(|g| g.id)
            .sum()
    }

    pub fn part2(&self) -> u32 {
        self.games.iter().map(|g| g.power()).sum()
    }
}
