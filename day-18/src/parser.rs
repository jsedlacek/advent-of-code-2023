pub mod v1 {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, newline, space1, u64},
        combinator::{map, value},
        multi::separated_list1,
        sequence::{separated_pair, terminated, tuple},
        IResult,
    };

    use crate::game::{Direction, Game, Instruction};

    pub fn parse_game(input: &str) -> IResult<&str, Game> {
        map(separated_list1(newline, parse_instruction), Game::new)(input)
    }

    pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        // Example: "R 6 (#70c710)"
        // Ignore the hex color part

        map(
            terminated(
                separated_pair(parse_direction, space1, u64),
                tuple((space1, tag("(#"), alphanumeric1, tag(")"))),
            ),
            |(dir, steps)| Instruction::new(dir, steps),
        )(input)
    }

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Right, tag("R")),
            value(Direction::Down, tag("D")),
            value(Direction::Left, tag("L")),
            value(Direction::Up, tag("U")),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use crate::game::{Direction, Instruction};

        use super::{parse_direction, parse_game, parse_instruction};

        #[test]
        fn test_parse_game() {
            assert!(parse_game("R 6 (#70c710)").is_ok());
        }

        #[test]
        fn test_parse_instruction() {
            assert_eq!(
                parse_instruction("R 6 (#70c710)").unwrap().1,
                Instruction::new(Direction::Right, 6)
            );
        }

        #[test]
        fn test_parse_direction() {
            assert_eq!(parse_direction("R").unwrap().1, Direction::Right);
        }
    }
}

pub mod v2 {
    use nom::{
        branch::alt,
        bytes::complete::{tag, take},
        character::complete::{newline, one_of, space1, u64},
        combinator::{map, map_res, value},
        multi::separated_list1,
        sequence::{delimited, tuple},
        IResult,
    };

    use crate::game::{Direction, Game, Instruction};

    pub fn parse_game(input: &str) -> IResult<&str, Game> {
        map(separated_list1(newline, parse_instruction), Game::new)(input)
    }

    pub fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
        // Example: "R 6 (#70c710)"
        // Take instructions from the #70c710 code
        // - "70c71" is converted as hex into "steps" value
        // - "0" indicates direction

        delimited(
            tuple((one_of("LDRU"), space1, u64, space1, tag("(#"))),
            map(
                tuple((
                    map_res(take(5u8), |s| u64::from_str_radix(s, 16)),
                    parse_direction,
                )),
                |(steps, dir)| Instruction::new(dir, steps),
            ),
            tag(")"),
        )(input)
    }

    fn parse_direction(input: &str) -> IResult<&str, Direction> {
        alt((
            value(Direction::Right, tag("0")),
            value(Direction::Down, tag("1")),
            value(Direction::Left, tag("2")),
            value(Direction::Up, tag("3")),
        ))(input)
    }

    #[cfg(test)]
    mod tests {
        use crate::game::{Direction, Instruction};

        use super::{parse_direction, parse_game, parse_instruction};

        #[test]
        fn test_parse_game_v2() {
            assert!(parse_game("R 6 (#70c710)").is_ok());
        }

        #[test]
        fn test_parse_instruction_v2() {
            assert_eq!(
                parse_instruction("R 6 (#70c710)").unwrap().1,
                Instruction::new(Direction::Right, 461937,)
            );
        }

        #[test]
        fn test_parse_direction_v2() {
            assert_eq!(parse_direction("0").unwrap().1, Direction::Right);
            assert_eq!(parse_direction("1").unwrap().1, Direction::Down);

            assert!(parse_direction("5").is_err());
        }
    }
}
