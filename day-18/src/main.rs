mod game;
mod parser;

use anyhow::Result;

use crate::parser::{parse_game_v1, parse_game_v2};

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (_, game1) = parse_game_v1(INPUT)?;

    println!("Part 1: {}", game1.puzzle());

    let (_, game2) = parse_game_v2(include_str!("input.txt"))?;

    println!("Part 2: {}", game2.puzzle());

    Ok(())
}
