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

trait CardParser {
    fn parse(input: &str) -> IResult<&str, Card>;
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse<T: CardParser>(input: &str) -> IResult<&str, Self> {
        delimited(
            many0(newline),
            map(separated_list0(newline, Round::parse::<T>), |rounds| Self {
                rounds,
            }),
            many0(newline),
        )(input)
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
    fn parse<T: CardParser>(input: &str) -> IResult<&str, Self> {
        // Example: "32T3K 765"
        map(tuple((Hand::parse::<T>, space1, u64)), |(hand, _, bid)| {
            Self { hand, bid }
        })(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn parse<T: CardParser>(input: &str) -> IResult<&str, Self> {
        // Example: "32T3K"
        map(many_m_n(5, 5, T::parse), |cards| Self { cards })(input)
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        let mut card_map: HashMap<Card, u32> = HashMap::new();

        for &c in &self.cards {
            card_map
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if let Some(joker_count) = card_map.remove(&Card::Joker) {
            if let Some((&card, &count)) = card_map.iter().max_by_key(|(_, &count)| count) {
                card_map.insert(card, count + joker_count);
            }
        }

        let mut count_map = HashMap::new();

        for &c in card_map.values() {
            count_map
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if count_map.get(&5) == Some(&1) {
            HandType::FiveOfAKind
        } else if count_map.get(&4) == Some(&1) {
            HandType::FourOfAKind
        } else if count_map.get(&3) == Some(&1) && count_map.get(&2) == Some(&1) {
            HandType::FullHouse
        } else if count_map.get(&3) == Some(&1) {
            HandType::ThreeOfAKind
        } else if count_map.get(&2) == Some(&2) {
            HandType::TwoPair
        } else if count_map.get(&2) == Some(&1) {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.get_type(), &self.cards).cmp(&(other.get_type(), &other.cards))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
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

struct CardParserV1;

impl CardParser for CardParserV1 {
    fn parse(input: &str) -> IResult<&str, Card> {
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
            value(Card::Jack, tag("J")),
            value(Card::Queen, tag("Q")),
            value(Card::King, tag("K")),
            value(Card::Ace, tag("A")),
        ))(input)
    }
}

struct CardParserV2;

impl CardParser for CardParserV2 {
    fn parse(input: &str) -> IResult<&str, Card> {
        alt((value(Card::Joker, tag("J")), CardParserV1::parse))(input)
    }
}

fn main() -> Result<()> {
    let (_, game1) = all_consuming(Game::parse::<CardParserV1>)(include_str!("input.txt"))
        .context("Error parsing input")?;

    dbg!(game1.puzzle());

    let (_, game2) = all_consuming(Game::parse::<CardParserV2>)(include_str!("input.txt"))
        .context("Error parsing input")?;
    dbg!(game2.puzzle());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, game) = Game::parse::<CardParserV1>(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 6440);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, game) = Game::parse::<CardParserV2>(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 5905);

    Ok(())
}

#[test]
fn parse_v1() -> Result<()> {
    let (_, card) = CardParserV1::parse("J")?;

    assert_eq!(card, Card::Jack);

    Ok(())
}

#[test]
fn parse_v2() -> Result<()> {
    let (_, card) = CardParserV2::parse("J")?;

    assert_eq!(card, Card::Joker);

    Ok(())
}
