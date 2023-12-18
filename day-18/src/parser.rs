use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, newline, space1, u64},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

use crate::game::{Direction, Game, Instruction};

pub fn parse_game_v1(input: &str) -> IResult<&str, Game> {
    map(separated_list1(newline, parse_instruction_v1), Game::new)(input)
}

pub fn parse_game_v2(input: &str) -> IResult<&str, Game> {
    map(separated_list1(newline, parse_instruction_v2), Game::new)(input)
}

pub fn parse_instruction_v1(input: &str) -> IResult<&str, Instruction> {
    // Example: "R 6 (#70c710)"
    // Ignore the hex color part

    map(
        terminated(
            separated_pair(parse_direction_v1, space1, u64),
            tuple((space1, tag("(#"), alphanumeric1, tag(")"))),
        ),
        |(dir, steps)| Instruction::new(dir, steps),
    )(input)
}

pub fn parse_instruction_v2(input: &str) -> IResult<&str, Instruction> {
    // Example: "R 6 (#70c710)"
    // Take instructions from the #70c710 code
    // - "70c71" is converted as hex into steps
    // - "0" indicates direction

    delimited(
        tuple((parse_direction_v1, space1, u64, space1, tag("(#"))),
        map(
            tuple((
                map_res(take(5u8), |s| u64::from_str_radix(s, 16)),
                parse_direction_v2,
            )),
            |(steps, dir)| Instruction::new(dir, steps),
        ),
        tag(")"),
    )(input)
}

fn parse_direction_v1(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, tag("R")),
        value(Direction::Down, tag("D")),
        value(Direction::Left, tag("L")),
        value(Direction::Up, tag("U")),
    ))(input)
}

fn parse_direction_v2(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, tag("0")),
        value(Direction::Down, tag("1")),
        value(Direction::Left, tag("2")),
        value(Direction::Up, tag("3")),
    ))(input)
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::game::{Direction, Instruction};

    use super::{
        parse_direction_v1, parse_direction_v2, parse_game_v1, parse_game_v2, parse_instruction_v1,
        parse_instruction_v2,
    };

    #[test]
    fn test_parse_game_v1() -> Result<()> {
        parse_game_v1("R 6 (#70c710)")?;

        Ok(())
    }

    #[test]
    fn test_parse_game_v2() -> Result<()> {
        parse_game_v2("R 6 (#70c710)")?;

        Ok(())
    }

    #[test]
    fn test_parse_instruction_v1() -> Result<()> {
        assert_eq!(
            parse_instruction_v1("R 6 (#70c710)")?.1,
            Instruction::new(Direction::Right, 6)
        );

        Ok(())
    }

    #[test]
    fn test_parse_instruction_v2() -> Result<()> {
        assert_eq!(
            parse_instruction_v2("R 6 (#70c710)")?.1,
            Instruction::new(Direction::Right, 461937,)
        );

        Ok(())
    }

    #[test]
    fn test_parse_direction_v1() -> Result<()> {
        assert_eq!(parse_direction_v1("R")?.1, Direction::Right);

        Ok(())
    }

    #[test]
    fn test_parse_direction_v2() -> Result<()> {
        assert_eq!(parse_direction_v2("0")?.1, Direction::Right);

        assert!(parse_direction_v2("5").is_err());

        Ok(())
    }
}
