use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1, u32},
    combinator::{all_consuming, map},
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            many0(delimited(multispace0, Card::parse, multispace0)),
            |cards| Self { cards },
        )(input)
    }

    fn part1(&self) -> u32 {
        self.cards.iter().map(|c| c.get_score()).sum()
    }

    fn part2(&self) -> Result<u32> {
        let mut card_counts: Vec<_> = self.cards.iter().map(|_| 1).collect();

        for (index, card) in self.cards.iter().enumerate() {
            let winning_card_count = card.get_matching_count();

            let card_count = *card_counts
                .get(index)
                .ok_or(anyhow!("Card count not found: {}", card.id))?;

            let winning_range = index + 1..=index + winning_card_count as usize;

            let winning_card_counts = card_counts
                .get_mut(winning_range.clone())
                .ok_or(anyhow!(
                    "Winning cards not available: Range {:?}",
                    winning_range
                ))?
                .iter_mut();

            for winning_card_count in winning_card_counts {
                *winning_card_count += card_count;
            }
        }

        Ok(card_counts.iter().sum())
    }
}

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

impl Card {
    fn parse_header(input: &str) -> IResult<&str, u32> {
        delimited(tuple((tag("Card"), space1)), u32, tuple((tag(":"), space0)))(input)
    }

    fn parse_numbers(input: &str) -> IResult<&str, Vec<u32>> {
        separated_list0(space1, u32)(input)
    }

    fn parse(input: &str) -> IResult<&str, Self> {
        // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53

        map(
            tuple((
                Self::parse_header,
                Self::parse_numbers,
                tuple((space0, tag("|"), space0)),
                Self::parse_numbers,
            )),
            |(id, winning_numbers, _, numbers)| Card {
                id,
                winning_numbers: HashSet::from_iter(winning_numbers),
                numbers: HashSet::from_iter(numbers),
            },
        )(input)
    }

    fn get_matching_count(&self) -> u32 {
        self.numbers.intersection(&self.winning_numbers).count() as u32
    }

    fn get_score(&self) -> u32 {
        let matching_count = self.get_matching_count();

        match matching_count {
            0 => 0,
            _ => 2u32.pow(matching_count - 1),
        }
    }
}

fn main() -> Result<()> {
    let (_, game) = all_consuming(Game::parse)(include_str!("input.txt"))?;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2()?);

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt"))?;

    assert_eq!(sample_game.part1(), 13);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt"))?;

    assert_eq!(sample_game.part2()?, 30);

    Ok(())
}
