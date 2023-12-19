use anyhow::Result;
use parser::parse_game;

mod game;
mod parser;

fn main() -> Result<()> {
    let (_, game) = parse_game(include_str!("input.txt"))?;

    dbg!(&game);

    println!("Part 1: {}", game.puzzle());

    Ok(())
}
