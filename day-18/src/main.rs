mod game;
mod parser;

use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let (_, game1) = parser::v1::parse_game(INPUT)?;

    println!("Part 1: {}", game1.puzzle());

    let (_, game2) = parser::v2::parse_game(INPUT)?;

    println!("Part 2: {}", game2.puzzle());

    Ok(())
}
