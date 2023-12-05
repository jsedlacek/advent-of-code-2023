use nom::{
    bytes::complete::tag,
    character::complete::{space0, space1, u32},
    multi::separated_list0,
    sequence::{separated_pair, tuple},
    IResult,
};

use crate::color::Color;

#[derive(Debug, Clone)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Set {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        // "3 blue, 4 red"
        let (input, colors) = separated_list0(tuple((tag(","), space0)), Self::parse_cubes)(input)?;

        let mut set = Self {
            red: 0,
            green: 0,
            blue: 0,
        };

        for (color, count) in colors {
            match color {
                Color::Red => set.red += count,
                Color::Green => set.green += count,
                Color::Blue => set.blue += count,
            }
        }

        Ok((input, set))
    }

    fn parse_cubes(input: &str) -> IResult<&str, (Color, u32)> {
        // "3 blue"
        let (input, (count, color)) = separated_pair(u32, space1, Color::parse)(input)?;

        Ok((input, (color, count)))
    }
}
