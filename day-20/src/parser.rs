use anyhow::Result;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, space0},
    combinator::{all_consuming, map, map_res},
    multi::{many0, separated_list0, separated_list1},
    sequence::{delimited, preceded, tuple},
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
        map(preceded(tag("%"), parse_line), |(name, outputs)| {
            Module::new(
                name.to_string(),
                outputs,
                ModuleValue::FlipFlop(FlipFlop::new()),
            )
        }),
        map(preceded(tag("&"), parse_line), |(name, outputs)| {
            Module::new(
                name.to_string(),
                outputs,
                ModuleValue::Conjunction(Conjunction::new()),
            )
        }),
        map(parse_line, |(name, outputs)| {
            Module::new(name.to_string(), outputs, ModuleValue::Broadcaster)
        }),
    ))(input)
}

fn parse_line(input: &str) -> IResult<&str, (&str, Vec<String>)> {
    map(
        tuple((alpha1, space0, tag("->"), space0, parse_outputs)),
        |(name, _, _, _, outputs)| (name, outputs),
    )(input)
}

fn parse_outputs(input: &str) -> IResult<&str, Vec<String>> {
    // Example: "a, b, c"

    separated_list0(tuple((tag(","), space0)), map(alpha1, str::to_string))(input)
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
                Module::new(
                    "a".to_string(),
                    vec!["b".to_string()],
                    ModuleValue::FlipFlop(FlipFlop::new()),
                )
            )
        );

        assert_eq!(
            parse_module("&inv -> a").unwrap(),
            (
                "",
                Module::new(
                    "inv".to_string(),
                    vec!["a".to_string()],
                    ModuleValue::Conjunction(Conjunction::new()),
                )
            )
        );

        assert_eq!(
            parse_module("broadcaster -> a, b, c").unwrap(),
            (
                "",
                Module::new(
                    "broadcaster".to_string(),
                    vec!["a".to_string(), "b".to_string(), "c".to_string()],
                    ModuleValue::Broadcaster,
                )
            )
        );
    }
}
