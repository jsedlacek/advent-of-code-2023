mod color;
mod game;
mod puzzle;
mod set;

use std::error::Error;

use nom::combinator::all_consuming;

use crate::puzzle::Puzzle;

fn main() -> Result<(), Box<dyn Error>> {
    let (_, sample_puzzle) = all_consuming(Puzzle::parse)(include_str!("sample-input.txt"))?;
    let (_, puzzle) = all_consuming(Puzzle::parse)(include_str!("input.txt"))?;

    assert_eq!(sample_puzzle.part1(), 8);
    assert_eq!(puzzle.part1(), 2256);

    dbg!(sample_puzzle.part2());
    dbg!(puzzle.part2());

    Ok(())
}
