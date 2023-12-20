use anyhow::Result;

use crate::parser::parse_input;

mod game;
mod parser;

fn main() -> Result<()> {
    let mut game = parse_input(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());

    Ok(())
}
