use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

#[derive(Debug, Clone, Copy)]
pub enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        alt((
            value(Self::Blue, tag("blue")),
            value(Self::Red, tag("red")),
            value(Self::Green, tag("green")),
        ))(input)
    }
}
