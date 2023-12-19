use anyhow::Result;

use parser::parse_game;

mod game;
mod parser;

fn main() -> Result<()> {
    let (_, game) = parse_game(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1()?);
    println!("Part 2: {}", game.part2()?);

    Ok(())
}
