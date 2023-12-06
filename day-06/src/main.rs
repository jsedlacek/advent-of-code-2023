use nom::{
    bytes::complete::tag,
    character::complete::{newline, space0, space1, u64},
    combinator::{map_res, recognize},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

struct Game {
    races: Vec<Race>,
}

impl Game {
    fn parse1(input: &str) -> IResult<&str, Self> {
        // Time:      7  15   30
        let (input, (_, _, time_list, _)) =
            tuple((tag("Time:"), space0, Self::parse_list, newline))(input)?;

        // Distance:  9  40  200
        let (input, (_, _, distance_list)) =
            tuple((tag("Distance:"), space0, Self::parse_list))(input)?;

        let races = time_list
            .iter()
            .zip(distance_list.iter())
            .map(|(&time, &distance)| Race::new(time, distance))
            .collect();

        Ok((input, Self { races }))
    }

    fn parse2(input: &str) -> IResult<&str, Self> {
        // Time:      7  15   30
        let (input, (_, _, time, _)) =
            tuple((tag("Time:"), space0, Self::parse_list_as_number, newline))(input)?;

        // Distance:  9  40  200
        let (input, (_, _, distance)) =
            tuple((tag("Distance:"), space0, Self::parse_list_as_number))(input)?;

        let race = Race::new(time, distance);

        Ok((input, Self { races: vec![race] }))
    }

    fn parse_list(input: &str) -> IResult<&str, Vec<u64>> {
        separated_list0(space1, u64)(input)
    }

    fn parse_list_as_number(input: &str) -> IResult<&str, u64> {
        map_res(recognize(Self::parse_list), |s: &str| {
            let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
            s.parse::<u64>()
        })(input)
    }

    fn puzzle(&self) -> u64 {
        self.races.iter().map(|r| r.record_count()).product()
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
