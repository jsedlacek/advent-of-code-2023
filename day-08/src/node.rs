use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric0, space0},
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct Node {
    pub id: String,
    pub left: String,
    pub right: String,
}

impl Node {
    pub fn parse(input: &str) -> IResult<&str, Self> {
        // AAA = (BBB, CCC)
        let (input, (id, _, _, _, _, _, left, _, _, _, right, _, _)) = tuple((
            alphanumeric0,
            space0,
            tag("="),
            space0,
            tag("("),
            space0,
            alphanumeric0,
            space0,
            tag(","),
            space0,
            alphanumeric0,
            space0,
            tag(")"),
        ))(input)?;

        Ok((
            input,
            Self {
                id: id.to_string(),
                left: left.to_string(),
                right: right.to_string(),
            },
        ))
    }
}

#[test]
fn parse() {
    assert_eq!(
        Node::parse("AAA = (BBB, CCC)").unwrap().1,
        Node {
            id: "AAA".to_string(),
            left: "BBB".to_string(),
            right: "CCC".to_string(),
        },
    )
}
