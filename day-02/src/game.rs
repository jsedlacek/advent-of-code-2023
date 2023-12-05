use nom::{
    bytes::complete::tag,
    character::complete::{space0, u32},
    multi::separated_list0,
    sequence::{delimited, tuple},
    IResult,
};

use crate::set::Set;

#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    sets: Vec<Set>,
}

impl Game {
    pub fn parse(input: &str) -> IResult<&str, Game> {
        // "Game 1: "
        let (input, id) =
            delimited(tuple((tag("Game"), space0)), u32, tuple((tag(":"), space0)))(input)?;

        let (input, sets) = separated_list0(tuple((tag(";"), space0)), Set::parse)(input)?;

        Ok((input, Game { id, sets }))
    }

    pub fn is_possible(&self) -> bool {
        // limit: 12 red cubes, 13 green cubes, and 14 blue cubes

        let set = self.max_set();

        if set.red > 12 {
            return false;
        }

        if set.green > 13 {
            return false;
        }

        if set.blue > 14 {
            return false;
        }

        true
    }

    pub fn power(&self) -> u32 {
        let set = self.max_set();

        set.red * set.green * set.blue
    }

    fn max_set(&self) -> Set {
        Set {
            red: self.sets.iter().map(|c| c.red).max().unwrap_or(0),
            green: self.sets.iter().map(|c| c.green).max().unwrap_or(0),
            blue: self.sets.iter().map(|c| c.blue).max().unwrap_or(0),
        }
    }
}
