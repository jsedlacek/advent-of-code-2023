use nom::{
    bytes::complete::tag,
    character::complete::{space0, u32},
    combinator::map,
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
        map(
            tuple((
                // "Game 1: "
                delimited(tuple((tag("Game"), space0)), u32, tuple((tag(":"), space0))),
                // "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
                separated_list0(tuple((tag(";"), space0)), Set::parse),
            )),
            |(id, sets)| Game { id, sets },
        )(input)
    }

    pub fn is_possible(&self) -> bool {
        // "limit: 12 red cubes, 13 green cubes, and 14 blue cubes"

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
