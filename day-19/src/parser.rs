use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline, u64},
    combinator::{map, value},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

use crate::game::{Action, Condition, Game, Operation, Rating, Sign, Workflow};

pub fn parse_game(input: &str) -> IResult<&str, Game> {
    map(
        separated_pair(
            separated_list1(newline, parse_workflow),
            many1(newline),
            separated_list1(
                newline,
                delimited(tag("{"), separated_list1(tag(","), parse_rating), tag("}")),
            ),
        ),
        |(workflows, ratings)| {
            let workflows = workflows.into_iter().map(|w| (w.name.clone(), w)).collect();
            let ratings = ratings
                .into_iter()
                .map(|r| {
                    Rating(
                        r.into_iter()
                            .map(|(name, value)| (name.to_string(), value))
                            .collect(),
                    )
                })
                .collect();
            Game { workflows, ratings }
        },
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

fn parse_rating(input: &str) -> IResult<&str, (&str, u64)> {
    // Example: "s=2876"

    separated_pair(alpha1::<&str, _>, tag("="), u64)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

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
