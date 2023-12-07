use std::collections::HashMap;

use nom::{
    character::complete::{one_of, space1},
    character::{complete::newline, complete::u64},
    combinator::map,
    multi::{many_m_n, separated_list0},
    sequence::tuple,
    IResult,
};

struct Game {
    rounds: Vec<Round>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, rounds) = separated_list0(newline, Round::parse)(input)?;

        Ok((input, Self { rounds }))
    }

    fn part1(&self) -> u64 {
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
    fn parse(input: &str) -> IResult<&str, Self> {
        // 32T3K 765
        map(tuple((Hand::parse, space1, u64)), |(hand, _, bid)| Self {
            hand,
            bid,
        })(input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    fn parse(input: &str) -> IResult<&str, Hand> {
        // 32T3K
        map(many_m_n(5, 5, Card::parse), |cards| Self { cards })(input)
    }

    fn get_type(&self) -> HandType {
        let mut card_map = HashMap::new();

        for &c in &self.cards {
            card_map
                .entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
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
struct Card(char);

const CARD_VALUES: &str = "23456789TJQKA";

impl Card {
    fn parse(input: &str) -> IResult<&str, Self> {
        // T

        map(one_of(CARD_VALUES), |c| Self(c))(input)
    }

    fn value(self) -> Option<usize> {
        CARD_VALUES.find(self.0)
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().unwrap().cmp(&other.value().unwrap())
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let (_, game) = Game::parse(include_str!("input.txt")).unwrap();

    dbg!(game.part1());
}

#[test]
fn part1() {
    let (_, game) = Game::parse(include_str!("sample-input.txt")).unwrap();

    assert_eq!(game.part1(), 6440);
}
