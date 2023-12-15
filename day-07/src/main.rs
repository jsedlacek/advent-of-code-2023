use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::{
        complete::newline,
        complete::{space1, u64},
    },
    combinator::{all_consuming, map, value},
    multi::{many0, many_m_n, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

use anyhow::{Context, Result};

struct Game {
    rounds: Vec<Round>,
}

#[derive(Clone, Copy, Default)]
enum JParse {
    #[default]
    Jack,
    Joker,
}

#[derive(Clone, Copy, Default)]
struct ParserOptions {
    j_parse: JParse,
}

impl Game {
    fn parse(options: ParserOptions) -> impl Fn(&str) -> IResult<&str, Self> {
        move |input: &str| {
            delimited(
                many0(newline),
                map(separated_list0(newline, Round::parse(options)), |rounds| {
                    Self { rounds }
                }),
                many0(newline),
            )(input)
        }
    }

    fn puzzle(&self) -> u64 {
        let mut rounds: Vec<_> = self.rounds.iter().collect();

        rounds.sort_by_key(|a| &a.hand);

        rounds
            .iter()
            .enumerate()
            .map(|(index, round)| {
                let rank = index + 1;
                (rank as u64) * round.bid
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Round {
    hand: Hand,
    bid: u64,
}

impl Round {
    fn parse(options: ParserOptions) -> impl Fn(&str) -> IResult<&str, Self> {
        // Example: "32T3K 765"
        move |input: &str| {
            map(
                tuple((Hand::parse(options), space1, u64)),
                |(hand, _, bid)| Self { hand, bid },
            )(input)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn parse(options: ParserOptions) -> impl Fn(&str) -> IResult<&str, Self> {
        // Example: "32T3K"
        move |input: &str| map(many_m_n(5, 5, Card::parse(options)), |cards| Self { cards })(input)
    }
}

impl Hand {
    fn get_set_counts(&self) -> Vec<u64> {
        let mut card_map: HashMap<Card, u64> = HashMap::new();

        for &c in &self.cards {
            *card_map.entry(c).or_insert(0) += 1;
        }

        if let Some(joker_count) = card_map.remove(&Card::Joker) {
            if let Some((&card, &count)) = card_map.iter().max_by_key(|(_, &count)| count) {
                card_map.insert(card, count + joker_count);
            }
        }

        (1..=5)
            .rev()
            .map(|count| card_map.values().filter(|&&c| c == count).count() as u64)
            .collect()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.get_set_counts(), &self.cards).cmp(&(other.get_set_counts(), &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn parse(options: ParserOptions) -> impl Fn(&str) -> IResult<&str, Self> {
        move |input: &str| {
            alt((
                value(Card::Two, tag("2")),
                value(Card::Three, tag("3")),
                value(Card::Four, tag("4")),
                value(Card::Five, tag("5")),
                value(Card::Six, tag("6")),
                value(Card::Seven, tag("7")),
                value(Card::Eight, tag("8")),
                value(Card::Nine, tag("9")),
                value(Card::Ten, tag("T")),
                value(
                    match options.j_parse {
                        JParse::Jack => Card::Jack,
                        JParse::Joker => Card::Joker,
                    },
                    tag("J"),
                ),
                value(Card::Queen, tag("Q")),
                value(Card::King, tag("K")),
                value(Card::Ace, tag("A")),
            ))(input)
        }
    }
}

fn main() -> Result<()> {
    let (_, game1) =
        all_consuming(Game::parse(ParserOptions::default()))(include_str!("input.txt"))
            .context("Error parsing input")?;

    dbg!(game1.puzzle());

    let (_, game2) = all_consuming(Game::parse(ParserOptions {
        j_parse: JParse::Joker,
    }))(include_str!("input.txt"))
    .context("Error parsing input")?;
    dbg!(game2.puzzle());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, game) = Game::parse(ParserOptions::default())(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 6440);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, game) = Game::parse(ParserOptions {
        j_parse: JParse::Joker,
    })(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 5905);

    Ok(())
}

#[test]
fn parse_v1() -> Result<()> {
    let (_, card) = Card::parse(ParserOptions::default())("J")?;

    assert_eq!(card, Card::Jack);

    Ok(())
}

#[test]
fn parse_v2() -> Result<()> {
    let (_, card) = Card::parse(ParserOptions {
        j_parse: JParse::Joker,
    })("J")?;

    assert_eq!(card, Card::Joker);

    Ok(())
}
