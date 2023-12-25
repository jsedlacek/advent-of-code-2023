use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline, space0},
    combinator::map,
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

#[derive(Debug)]
struct Game {
    list: Vec<Line>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                newline,
                map(
                    separated_pair(
                        map(Point3D::parse, Point3D::to_2d),
                        tuple((space0, tag("@"), space0)),
                        map(Point3D::parse, Point3D::to_2d),
                    ),
                    |(p, v)| Line(p, v),
                ),
            ),
            |list| Self { list },
        )(input)
    }

    fn part1(&self) -> u64 {
        let range = (200000000000000.0, 400000000000000.0);

        self.list
            .iter()
            .enumerate()
            .flat_map(|(i1, l1)| {
                self.list
                    .iter()
                    .enumerate()
                    .filter(move |(i2, _)| i1 < *i2)
                    .filter_map(|(i1, l2)| l1.intersect(l2))
                    .filter(|p| {
                        p.0 >= range.0 && p.0 <= range.1 && p.1 >= range.0 && p.1 <= range.1
                    })
            })
            .count() as u64
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Line(Point2D, Point2D);

impl Line {
    fn intersect(&self, other: &Self) -> Option<Point2D> {
        let Self(p_a, v_a) = *self;
        let Self(p_b, v_b) = *other;

        let det = v_a.0 * v_b.1 - v_a.1 * v_b.0;

        if det.abs() < f64::EPSILON {
            return None;
        }

        let t_a = ((p_b.0 - p_a.0) * v_b.1 - (p_b.1 - p_a.1) * v_b.0) / det;
        let t_b = ((p_a.0 - p_b.0) * v_a.1 - (p_a.1 - p_b.1) * v_a.0) / det;

        if t_a < 0.0 || t_b > 0.0 {
            return None;
        }

        Some(Point2D(p_a.0 + t_a * v_a.0, p_a.1 + t_a * v_a.1))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2D(f64, f64);

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3D(f64, f64, f64);

impl Point3D {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((i64, tag(","), space0, i64, tag(","), space0, i64)),
            |(x, _, _, y, _, _, z)| Self(x as f64, y as f64, z as f64),
        )(input)
    }

    fn to_2d(self) -> Point2D {
        let Self(x, y, _) = self;

        Point2D(x, y)
    }
}

fn main() {
    let game = Game::parse(include_str!("input.txt")).unwrap().1;

    dbg!(game.part1());
}
