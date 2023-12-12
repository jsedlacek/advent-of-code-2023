use anyhow::Result;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{newline, space1, u128},
    combinator::map,
    multi::{many0, separated_list0},
    sequence::{delimited, tuple},
    IResult,
};

#[derive(Debug, Clone)]
struct Game {
    rows: Vec<Row>,
}

impl Game {
    fn parse_1(input: &str) -> IResult<&str, Self> {
        delimited(
            many0(newline),
            map(separated_list0(newline, Row::parse_1), |rows| Self { rows }),
            many0(newline),
        )(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        delimited(
            many0(newline),
            map(separated_list0(newline, Row::parse_2), |rows| Self { rows }),
            many0(newline),
        )(input)
    }

    fn puzzle(&self) -> u128 {
        self.rows
            .iter()
            .enumerate()
            .map(|(i, row)| {
                let count = row.option_count();

                dbg!(i, count);

                count
            })
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Row {
    springs: Vec<Spring>,
    damaged_groups: Vec<u128>,
}

impl Row {
    fn parse_1(input: &str) -> IResult<&str, Self> {
        map(
            tuple((
                many0(Spring::parse),
                space1,
                separated_list0(tag(","), u128),
            )),
            |(springs, _, damaged_groups)| Self {
                springs,
                damaged_groups,
            },
        )(input)
    }

    fn parse_2(input: &str) -> IResult<&str, Self> {
        let (
            input,
            Self {
                springs,
                damaged_groups,
            },
        ) = Self::parse_1(input)?;

        let mut new_springs = springs.clone();
        let mut new_damaged_groups = damaged_groups.clone();

        for _ in 1..5 {
            new_springs.push(Spring::Unknown);
            new_springs.append(&mut springs.clone());
            new_damaged_groups.append(&mut damaged_groups.clone());
        }

        Ok((
            input,
            Self {
                springs: new_springs,
                damaged_groups: new_damaged_groups,
            },
        ))
    }

    fn valid_count(springs: &[Spring], damaged_groups: &[u128]) -> u128 {
        if damaged_groups.len() > 0
            && (springs.len() as u128)
                < damaged_groups.iter().sum::<u128>() + (damaged_groups.len() as u128 - 1)
        {
            return 0;
        }

        if (springs
            .iter()
            .filter(|&&s| s == Spring::Damaged || s == Spring::Unknown)
            .count() as u128)
            < damaged_groups.iter().sum::<u128>()
        {
            return 0;
        }

        if damaged_groups.len() == 0
            && springs
                .iter()
                .all(|&s| s == Spring::Operational || s == Spring::Unknown)
        {
            return 1;
        }

        if springs.starts_with(&[Spring::Unknown]) {
            let pos = springs
                .iter()
                .position(|&s| s != Spring::Unknown)
                .unwrap_or(springs.len());

            if springs.get(pos) == Some(&Spring::Operational) {
                let (start, end) = springs.split_at(pos);

                let mut count = 0;

                count += Self::valid_count(end, damaged_groups);

                for i in 1..=damaged_groups.len() {
                    let (start_groups, end_groups) = damaged_groups.split_at(i);

                    let min_group_length =
                        start_groups.iter().sum::<u128>() + (start_groups.len() as u128 - 1);

                    if min_group_length > start.len() as u128 {
                        break;
                    }

                    let c = combinations(
                        start_groups.len() as u128 + 1,
                        start.len() as u128 - min_group_length,
                    );

                    if c > 0 {
                        count += c * Self::valid_count(end, end_groups);
                    }

                    // number of combinations
                    // ??? 1,1
                    // #.#
                    //
                    // ???? 1,1
                    // .#.# 0
                    // #..# 1
                    // #.#. 2
                    //
                    // ????? 1,1
                    // ..#.# 00
                    // .#..# 01
                    // #...# 02
                    // .#.#. 11
                    // .#..# 12
                    // ..#.# 22
                }

                return count;
            } else {
                let mut springs_a = springs.to_vec();
                springs_a[pos - 1] = Spring::Damaged;

                let mut springs_b = springs.to_vec();
                springs_b[pos - 1] = Spring::Operational;

                return Self::valid_count(&springs_a, damaged_groups)
                    + Self::valid_count(&springs_b, damaged_groups);
            }
        }

        let mut count = 0;

        if let Some(group) = damaged_groups.first() {
            // Next groups starts at beginning of springs
            let (start, end) = springs.split_at(*group as usize);

            if start
                .iter()
                .all(|&s| s == Spring::Damaged || s == Spring::Unknown)
            {
                if let Some((&mid, end)) = end.split_first() {
                    if mid == Spring::Operational || mid == Spring::Unknown {
                        count += Self::valid_count(end, &damaged_groups[1..]);
                    }
                } else {
                    count += Self::valid_count(end, &damaged_groups[1..]);
                }
            }

            // Next groups does not start yet
            let (start, end) = springs.split_at(1);
            if start
                .iter()
                .all(|&s| s == Spring::Operational || s == Spring::Unknown)
            {
                count += Self::valid_count(end, damaged_groups);
            }
        }

        count
    }

    fn option_count(&self) -> u128 {
        Self::valid_count(&self.springs, &self.damaged_groups)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn parse(input: &str) -> IResult<&str, Self> {
        // ???.### 1,1,3
        alt((
            map(tag("."), |_| Spring::Operational),
            map(tag("#"), |_| Spring::Damaged),
            map(tag("?"), |_| Spring::Unknown),
        ))(input)
    }
}

fn factorial(n: u128) -> u128 {
    (1..=n).product()
}

fn combinations(n: u128, r: u128) -> u128 {
    (r + 1..=r + n - 1).product::<u128>() / factorial(n - 1)
}

fn main() -> Result<()> {
    let (_, game) = Game::parse_2(include_str!("input.txt"))?;

    dbg!(game.puzzle());

    Ok(())
}
