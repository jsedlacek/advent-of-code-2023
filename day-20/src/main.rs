use anyhow::Result;

use crate::parser::parse_input;

mod game;
mod lcm;
mod parser;

fn main() -> Result<()> {
    let mut game = parse_input(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());

    let mut game2 = parse_input(include_str!("input.txt"))?;

    println!("Part 2: {:?}", game2.part2());

    Ok(())
}
