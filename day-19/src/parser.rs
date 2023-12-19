use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
    combinator::{map, value},
    multi::{many0, many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::game::{Action, Condition, Game, Operation, Rating, Sign, Workflow};

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    delimited(
        many0(newline),
        map(
            separated_pair(
                map(separated_list1(newline, parse_workflow), |workflows| {
                    workflows.into_iter().map(|w| (w.name.clone(), w)).collect()
                }),
                many1(newline),
                separated_list1(newline, parse_rating),
            ),
            |(workflows, ratings)| Game { workflows, ratings },
        ),
        many0(newline),
    )(input)
}

pub fn parse_workflow(input: &str) -> IResult<&str, Workflow> {
    // Example: "px{a<2006:qkq,m>2090:A,rfg}"

    map(
        tuple((
            alpha1,
            tag("{"),
            separated_list1(tag(","), parse_operation),
            tag("}"),
        )),
        |(name, _, ops, _)| Workflow {
            name: name.to_string(),
            ops,
        },
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    // Examples:
    // - "a<2006:qkq"
    // - "rfg"
    alt((
        map(
            tuple((parse_condition, tag(":"), parse_action)),
            |(cond, _, action)| Operation {
                cond: Some(cond),
                action,
            },
        ),
        map(parse_action, |action| Operation { cond: None, action }),
    ))(input)
}

fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((
        value(Action::Accept, tag("A")),
        value(Action::Reject, tag("R")),
        map(alpha1, |s: &str| Action::Workflow(s.to_string())),
    ))(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    // - "a<2006"

    map(
        tuple((
            alpha1::<&str, _>,
            alt((value(Sign::Less, tag("<")), value(Sign::Greater, tag(">")))),
            u64,
        )),
        |(var, sign, value)| Condition {
            var: var.to_string(),
            sign,
            value,
        },
    )(input)
}

fn parse_rating(input: &str) -> IResult<&str, Rating> {
    // Example: "{x=787,m=2655,a=1222,s=2876}"

    delimited(
        tag("{"),
        map(
            separated_list1(tag(","), separated_pair(alpha1::<&str, _>, tag("="), u64)),
            |r| {
                Rating(
                    r.into_iter()
                        .map(|(name, value)| (name.to_string(), value))
                        .collect(),
                )
            },
        ),
        tag("}"),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    fn test_parse_game() {
        let (remainder, game) = parse_game(SAMPLE_INPUT).unwrap();

        assert!(remainder.is_empty());
        assert_eq!(game.workflows.len(), 11);
        assert_eq!(game.ratings.len(), 5);
    }

    #[test]
    fn test_parse_workflow() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}";

        let expected = Workflow {
            name: "px".to_string(),
            ops: vec![
                Operation {
                    cond: Some(Condition {
                        var: "a".to_string(),
                        sign: Sign::Less,
                        value: 2006,
                    }),
                    action: Action::Workflow("qkq".to_string()),
                },
                Operation {
                    cond: Some(Condition {
                        var: "m".to_string(),
                        sign: Sign::Greater,
                        value: 2090,
                    }),
                    action: Action::Accept,
                },
                Operation {
                    cond: None,
                    action: Action::Workflow("rfg".to_string()),
                },
            ],
        };

        let (_, workflow) = parse_workflow(input).unwrap();
        assert_eq!(workflow, expected);
    }
}
