mod lcm;
mod node;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::newline,
    combinator::{all_consuming, map},
    multi::{many0, separated_list0},
    sequence::terminated,
    IResult,
};

use lcm::lcm_of_vec;
use node::Node;

#[derive(Debug)]
struct Graph {
    instructions: Vec<Instruction>,
    nodes: Vec<Node>,
}

impl Graph {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, instructions) = many0(alt((
            map(tag("L"), |_| Instruction::Left),
            map(tag("R"), |_| Instruction::Right),
        )))(input)?;

        let (input, _) = many0(newline)(input)?;

        let (input, nodes) =
            terminated(separated_list0(newline, Node::parse), many0(newline))(input)?;

        Ok((
            input,
            Self {
                instructions,
                nodes,
            },
        ))
    }

    fn part1(&self) -> u64 {
        self.find_steps("AAA")
    }

    fn part2(&self) -> u64 {
        let node_ids: Vec<_> = self
            .nodes
            .iter()
            .filter_map(|n| {
                if n.id.ends_with("A") {
                    Some(n.id.clone())
                } else {
                    None
                }
            })
            .collect();

        let steps: Vec<_> = node_ids.iter().map(|id| self.find_steps(id)).collect();

        lcm_of_vec(&steps)
    }

    fn find_steps(&self, starting_node_id: &str) -> u64 {
        let mut node_id = starting_node_id.to_string();

        for (step, i) in self.instructions.iter().cycle().enumerate() {
            if node_id.ends_with("Z") {
                return step as u64;
            }

            let node = self
                .nodes
                .iter()
                .find(|n| n.id == node_id)
                .expect(&format!("Node not found: {node_id}"));

            node_id = {
                match i {
                    Instruction::Left => node.left.clone(),
                    Instruction::Right => node.right.clone(),
                }
            };
        }

        panic!("This should not happen");
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

fn main() {
    let (_, graph) = all_consuming(Graph::parse)(include_str!("input.txt")).unwrap();

    dbg!(graph.part1());

    dbg!(graph.part2());
}

#[test]
fn part1_sample1() {
    let (_, graph) = all_consuming(Graph::parse)(include_str!("sample-input.txt")).unwrap();

    assert_eq!(graph.part1(), 2);
}

#[test]
fn part1_sample2() {
    let (_, graph) = all_consuming(Graph::parse)(include_str!("sample-input-2.txt")).unwrap();

    assert_eq!(graph.part1(), 6);
}

#[test]
fn part2() {
    let (_, graph) = all_consuming(Graph::parse)(include_str!("sample-input-3.txt")).unwrap();

    assert_eq!(graph.part2(), 6);
}
