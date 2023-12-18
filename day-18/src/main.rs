mod game;
mod parser;

use anyhow::Result;

const INPUT: &str = include_str!("input.txt");

fn main() -> Result<()> {
    let game1 = parser::v1::parse_game(INPUT)?.1;
    println!("Part 1: {}", game1.puzzle());

    let game2 = parser::v2::parse_game(INPUT)?.1;
    println!("Part 2: {}", game2.puzzle());

    Ok(())
}
