mod color;
mod game;
mod puzzle;
mod set;

use anyhow::Result;
use nom::combinator::all_consuming;

use crate::puzzle::Puzzle;

fn main() -> Result<()> {
    let (_, puzzle) = all_consuming(Puzzle::parse)(include_str!("input.txt"))?;

    println!("Part 1: {}", puzzle.part1());
    println!("Part 2: {}", puzzle.part2());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, sample_puzzle) = all_consuming(Puzzle::parse)(include_str!("sample-input.txt"))?;
    assert_eq!(sample_puzzle.part1(), 8);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, sample_puzzle) = all_consuming(Puzzle::parse)(include_str!("sample-input.txt"))?;
    assert_eq!(sample_puzzle.part2(), 2286);

    Ok(())
}
