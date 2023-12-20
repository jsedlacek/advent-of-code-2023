use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, space0},
    combinator::{all_consuming, map, map_res},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, tuple},
    IResult,
};

use crate::game::{Conjunction, FlipFlop, Game, Module, ModuleValue};

pub fn parse_input(input: &str) -> Result<Game> {
    let (_, game) = all_consuming(delimited(many0(newline), parse_game, many0(newline)))(input)
        .map_err(|e| e.to_owned())?;

    Ok(game)
}

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    map_res(separated_list1(newline, parse_module), Game::new)(input)
}

fn parse_module(input: &str) -> IResult<&str, Module> {
    // Example: "broadcaster -> a, b, c"

    alt((
        map(
            tuple((tag("%"), alpha1, space0, tag("->"), space0, parse_outputs)),
            |(_, name, _, _, _, outputs)| Module {
                name: name.to_string(),
                outputs,
                received_signals: (0, 0),
                value: ModuleValue::FlipFlop(FlipFlop::new()),
            },
        ),
        map(
            tuple((tag("&"), alpha1, space0, tag("->"), space0, parse_outputs)),
            |(_, name, _, _, _, outputs)| Module {
                name: name.to_string(),
                outputs,
                received_signals: (0, 0),
                value: ModuleValue::Conjunction(Conjunction::new()),
            },
        ),
        map(
            tuple((alpha1, space0, tag("->"), space0, parse_outputs)),
            |(name, _, _, _, outputs)| Module {
                name: name.to_string(),
                outputs,
                received_signals: (0, 0),
                value: ModuleValue::Broadcaster,
            },
        ),
    ))(input)

    // alt((map(
    //     tuple((
    //         tag("%"),
    //         // alpha1, space0, tag("->"), space0, parse_outputs
    //     )),
    //     |(
    //         _,
    //         // name, _, _, _, outputs
    //     )| Module {
    //         name: "".to_string(),
    //         outputs: vec![],
    //         value: ModuleValue::Broadcaster,
    //     },
    // )))(input)
}

fn parse_outputs(input: &str) -> IResult<&str, Vec<String>> {
    // Example: "a, b, c"

    separated_list0(tuple((tag(","), space0)), map(alpha1, str::to_string))(input)
    // todo!()
}

#[cfg(test)]
mod test {
    use crate::game::Conjunction;

    use super::*;

    #[test]
    fn test_parse_module() {
        assert_eq!(
            parse_module("%a -> b").unwrap(),
            (
                "",
                Module {
                    name: "a".to_string(),
                    outputs: vec!["b".to_string()],
                    received_signals: (0, 0),
                    value: ModuleValue::FlipFlop(FlipFlop::new()),
                }
            )
        );

        assert_eq!(
            parse_module("&inv -> a").unwrap(),
            (
                "",
                Module {
                    name: "inv".to_string(),
                    outputs: vec!["a".to_string()],
                    received_signals: (0, 0),
                    value: ModuleValue::Conjunction(Conjunction::new()),
                }
            )
        );

        assert_eq!(
            parse_module("broadcaster -> a, b, c").unwrap(),
            (
                "",
                Module {
                    name: "broadcaster".to_string(),
                    outputs: vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    received_signals: (0, 0),
                    value: ModuleValue::Broadcaster,
                }
            )
        );
    }
}
