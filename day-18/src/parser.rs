use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{alphanumeric1, newline, space1, u64},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::tuple,
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

    map(
        tuple((
            parse_direction_v1,
            space1,
            u64,
            space1,
            tag("(#"),
            alphanumeric1,
            tag(")"),
        )),
        |(dir, _, steps, _, _, _, _)| Instruction::new(dir, steps),
    )(input)
}

pub fn parse_instruction_v2(input: &str) -> IResult<&str, Instruction> {
    // Example: "R 6 (#70c710)"

    map(
        tuple((
            parse_direction_v1,
            space1,
            u64,
            space1,
            tag("(#"),
            map(
                tuple((
                    map_res(take(5u8), |s| u64::from_str_radix(s, 16)),
                    parse_direction_v2,
                )),
                |(steps, dir)| Instruction::new(dir, steps),
            ),
            tag(")"),
        )),
        |(_, _, _, _, _, ins, _)| ins,
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
    use crate::{
        game::{Direction, Instruction},
        parser::{parse_game_v1, parse_instruction_v1, parse_instruction_v2},
    };

    #[test]
    fn test_parse_game_v1() -> anyhow::Result<()> {
        parse_game_v1("R 6 (#70c710)")?;

        Ok(())
    }

    #[test]
    fn test_parse_game_v2() -> anyhow::Result<()> {
        parse_game_v1("R 6 (#70c710)")?;

        Ok(())
    }

    #[test]
    fn test_parse_instruction_v1() -> anyhow::Result<()> {
        let (_, instruction) = parse_instruction_v1("R 6 (#70c710)")?;

        assert_eq!(instruction, Instruction::new(Direction::Right, 6));

        Ok(())
    }

    #[test]
    fn test_parse_instruction_v2() -> anyhow::Result<()> {
        let (_, instruction) = parse_instruction_v2("R 6 (#70c710)")?;

        assert_eq!(instruction, Instruction::new(Direction::Right, 461937,));

        Ok(())
    }
}
