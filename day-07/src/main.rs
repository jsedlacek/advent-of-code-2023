use std::collections::HashMap;

use nom::{
    character::complete::{one_of, space1},
    character::{complete::newline, complete::u64},
    combinator::map,
    multi::{many_m_n, separated_list0},
    sequence::tuple,
    IResult,
};

use anyhow::Result;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum GameVersion {
    V1,
    V2,
}

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse_1(input: &str) -> IResult<&str, Self> {
        map(separated_list0(newline, Round::parse_1), |rounds| Self {
            rounds,
        })(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        map(separated_list0(newline, Round::parse_2), |rounds| Self {
            rounds,
        })(input)
    }

    fn puzzle(&self) -> u64 {
        let mut rounds = self.rounds.clone();

        rounds.sort_by_key(|r| r.hand.clone());

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
    fn parse_1(input: &str) -> IResult<&str, Self> {
        // 32T3K 765
        map(tuple((Hand::parse_1, space1, u64)), |(hand, _, bid)| Self {
            hand,
            bid,
        })(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        // 32T3K 765
        map(tuple((Hand::parse_2, space1, u64)), |(hand, _, bid)| Self {
            hand,
            bid,
        })(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    version: GameVersion,
}

impl Hand {
    fn parse_1(input: &str) -> IResult<&str, Hand> {
        // 32T3K
        map(many_m_n(5, 5, Card::parse_1), |cards| Self {
            cards,
            version: GameVersion::V1,
        })(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Hand> {
        // 32T3K
        map(many_m_n(5, 5, Card::parse_2), |cards| Self {
            cards,
            version: GameVersion::V2,
        })(input)
    }

    fn get_type(&self) -> HandType {
        let mut card_map = HashMap::new();

        for &c in &self.cards {
            card_map
                .entry(c.0)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        if self.version == GameVersion::V2 && card_map.get(&'J').is_some() && card_map.len() > 1 {
            let &joker_count = card_map.get(&'J').unwrap();
            card_map.remove(&'J');
            let (&card, &count) = card_map.iter().max_by_key(|(_, &count)| count).unwrap();
            card_map.insert(card, count + joker_count);
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
        let self_value = HAND_TYPE_VALUES
            .iter()
            .position(|t| *t == self.get_type())
            .unwrap();

        let other_value = HAND_TYPE_VALUES
            .iter()
            .position(|t| *t == other.get_type())
            .unwrap();

        match self_value.cmp(&other_value) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

const HAND_TYPE_VALUES: [HandType; 7] = [
    HandType::HighCard,
    HandType::OnePair,
    HandType::TwoPair,
    HandType::ThreeOfAKind,
    HandType::FullHouse,
    HandType::FourOfAKind,
    HandType::FiveOfAKind,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Card(char, GameVersion);

const CARD_VALUES_1: &str = "23456789TJQKA";
const CARD_VALUES_2: &str = "J23456789TQKA";

impl Card {
    fn parse_1(input: &str) -> IResult<&str, Self> {
        // "T""

        map(one_of(CARD_VALUES_1), |c| Self(c, GameVersion::V1))(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        // "T""

        map(one_of(CARD_VALUES_2), |c| Self(c, GameVersion::V1))(input)
    }

    fn value(self) -> Option<usize> {
        match self.1 {
            GameVersion::V1 => CARD_VALUES_1.find(self.0),
            GameVersion::V2 => CARD_VALUES_2.find(self.0),
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<()> {
    let (_, game1) = Game::parse_1(include_str!("input.txt"))?;

    dbg!(game1.puzzle());

    let (_, game2) = Game::parse_2(include_str!("input.txt"))?;
    dbg!(game2.puzzle());

    Ok(())
}

#[test]
fn part1() -> Result<()> {
    let (_, game) = Game::parse_1(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 6440);

    Ok(())
}

#[test]
fn part2() -> Result<()> {
    let (_, game) = Game::parse_2(include_str!("sample-input.txt"))?;

    assert_eq!(game.puzzle(), 5905);

    Ok(())
}
