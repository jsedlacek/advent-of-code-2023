use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64},
    combinator::{map, map_res, recognize},
    multi::separated_list0,
    sequence::{delimited, preceded, tuple},
    IResult,
};

struct Game {
    races: Vec<Race>,
}

impl Game {
    fn parse1(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                // Time:      7  15   30
                delimited(tuple((tag("Time:"), space0)), Self::parse_list, newline),
                // Distance:  9  40  200
                preceded(tuple((tag("Distance:"), space0)), Self::parse_list),
            )),
            |(time_list, distance_list)| {
                let races = time_list
                    .iter()
                    .zip(distance_list.iter())
                    .map(|(&time, &distance)| Race::new(time, distance))
                    .collect();

                Self { races }
            },
        )(input)
    }

    fn parse2(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                // Time:      7  15   30
                delimited(
                    tuple((tag("Time:"), space0)),
                    Self::parse_list_as_number,
                    newline,
                ),
                // Distance:  9  40  200
                preceded(
                    tuple((tag("Distance:"), space0)),
                    Self::parse_list_as_number,
                ),
            )),
            |(time, distance)| {
                let races = vec![Race::new(time, distance)];
                Self { races }
            },
        )(input)
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list0(space1, u64)(input)
    }

    fn parse_list_as_number(input: &str) -> IResult<&str, u64> {
        map_res(recognize(Self::parse_list), |s: &str| {
            s.chars()
                .filter(|c| !c.is_whitespace())
                .collect::<String>()
                .parse::<u64>()
        })(input)
    }

    fn puzzle(&self) -> u64 {
        self.races.iter().map(Race::record_count).product()
    }
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn new(time: u64, distance: u64) -> Self {
        Self { time, distance }
    }

    fn record_count(&self) -> u64 {
        (0..self.time)
            .filter(|time_charging| {
                let time_remaining = self.time - time_charging;
                let speed = time_charging;
                let distance = speed * time_remaining;
                distance > self.distance
            })
            .count() as u64
    }
}

fn main() {
    let (_, game1) = Game::parse1(include_str!("input.txt")).unwrap();
    dbg!(game1.puzzle());

    let (_, game2) = Game::parse2(include_str!("input.txt")).unwrap();
    dbg!(game2.puzzle());
}

#[test]
fn part1() {
    let (_, game) = Game::parse1(include_str!("sample-input.txt")).unwrap();
    assert_eq!(game.puzzle(), 288);
}

#[test]
fn part2() {
    let (_, game) = Game::parse2(include_str!("sample-input.txt")).unwrap();
    assert_eq!(game.puzzle(), 71503);
}
