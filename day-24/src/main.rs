use std::io::Write;
use std::process::{Command, Stdio};

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
    lines: Vec<Line>,
}

impl Game {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                newline,
                map(
                    separated_pair(
                        Point3D::parse,
                        tuple((space0, tag("@"), space0)),
                        Point3D::parse,
                    ),
                    |(p, v)| Line(p, v),
                ),
            ),
            |lines| Self { lines },
        )(input)
    }

    fn part1(&self) -> u64 {
        let range = (200000000000000.0, 400000000000000.0);

        self.lines
            .iter()
            .enumerate()
            .flat_map(|(i1, l1)| {
                self.lines
                    .iter()
                    .enumerate()
                    .filter(move |(i2, _)| i1 < *i2)
                    .filter_map(|(_, l2)| l1.intersect_2d(l2))
                    .filter(|p| {
                        p.0 >= range.0 && p.0 <= range.1 && p.1 >= range.0 && p.1 <= range.1
                    })
            })
            .count() as u64
    }

    fn declare_const(name: &str) -> String {
        format!("(declare-const {name} Int)")
    }

    fn assert(eq: &str) -> String {
        format!("(assert ({eq}))")
    }

    fn get_z3_command(&self) -> String {
        let mut res = Vec::new();

        res.push(Self::declare_const("x"));
        res.push(Self::declare_const("y"));
        res.push(Self::declare_const("z"));

        res.push(Self::declare_const("vx"));
        res.push(Self::declare_const("vy"));
        res.push(Self::declare_const("vz"));

        for (index, _) in self.lines.iter().take(3).enumerate() {
            res.push(Self::declare_const(&format!("t{index}")));
        }

        for (index, line) in self.lines.iter().take(3).enumerate() {
            let Line(Point3D(x, y, z), Point3D(vx, vy, vz)) = line;

            res.push(Self::assert(&format!(
                "= (+ {x} (* t{index} {vx})) (+ x (* t{index} vx))",
            )));

            res.push(Self::assert(&format!(
                "= (+ {y} (* t{index} {vy})) (+ y (* t{index} vy))",
            )));

            res.push(Self::assert(&format!(
                "= (+ {z} (* t{index} {vz})) (+ z (* t{index} vz))",
            )));
        }

        res.push(format!("(check-sat)"));
        res.push(format!("(eval (+ (+ x y) z))"));

        res.join("\n")
    }

    fn part2(&self) -> u64 {
        let mut child = Command::new("z3")
            .args(["-in"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        if let Some(ref mut stdin) = child.stdin {
            stdin.write_all(self.get_z3_command().as_bytes()).unwrap();
        }

        let output = child.wait_with_output().unwrap();
        let output = String::from_utf8_lossy(&output.stdout);

        output.lines().last().unwrap().parse::<u64>().unwrap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Line(Point3D, Point3D);

impl Line {
    fn intersect_2d(&self, other: &Self) -> Option<Point3D> {
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

        Some(Point3D(
            p_a.0 + t_a * v_a.0,
            p_a.1 + t_a * v_a.1,
            0.0, /* z is ignored for 2D case */
        ))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point3D(f64, f64, f64);

impl Point3D {
    fn parse(input: &str) -> IResult<&str, Self> {
        map(
            tuple((i64, tag(","), space0, i64, tag(","), space0, i64)),
            |(x, _, _, y, _, _, z)| Self(x as f64, y as f64, z as f64),
        )(input)
    }
}

fn main() {
    let game = Game::parse(include_str!("input.txt")).unwrap().1;

    println!("Part 1: {}", game.part1());
    println!("Part 2: {}", game.part2());
}
