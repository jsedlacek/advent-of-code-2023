use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, space0, space1, u32},
    combinator::all_consuming,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, cards) = many0(delimited(multispace0, Card::parse, multispace0))(input)?;

        Ok((input, Self { cards }))
    }

    fn part1(&self) -> u32 {
        self.cards.iter().map(|c| c.get_score()).sum()
    }

    fn part2(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let mut card_counts: Vec<_> = self.cards.iter().map(|_| 1).collect();

        for (index, card) in self.cards.iter().enumerate() {
            let winning_card_count = card.get_matching_count();

            let card_count = *card_counts
                .get(index)
                .ok_or(format!("Card count not found: {}", card.id))?;

            let winning_range = index + 1..=index + winning_card_count as usize;

            let winning_card_counts = card_counts
                .get_mut(winning_range.clone())
                .ok_or(format!(
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

        let (input, (id, winning_numbers, _, numbers)) = tuple((
            Self::parse_header,
            Self::parse_numbers,
            tuple((space0, tag("|"), space0)),
            Self::parse_numbers,
        ))(input)?;

        Ok((
            input,
            Card {
                id: id,
                winning_numbers: HashSet::from_iter(winning_numbers.into_iter()),
                numbers: HashSet::from_iter(numbers.into_iter()),
            },
        ))
    }

    fn get_matching_count(&self) -> u32 {
        self.numbers.intersection(&self.winning_numbers).count() as u32
    }

    fn get_score(&self) -> u32 {
        let matching_count = self.get_matching_count();

        let score = match matching_count {
            0 => 0,
            _ => 2u32.pow(matching_count - 1),
        };

        score
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (_, sample_game) = all_consuming(Game::parse)(include_str!("sample-input.txt"))?;

    assert_eq!(sample_game.part1(), 13);
    assert_eq!(sample_game.part2()?, 30);

    let (_, game) = all_consuming(Game::parse)(include_str!("input.txt"))?;

    dbg!(game.part1());
    dbg!(game.part2()?);

    Ok(())
}
