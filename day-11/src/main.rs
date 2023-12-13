use std::collections::HashSet;

#[derive(Debug)]
struct Game {
    galaxies: HashSet<(u64, u64)>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut galaxies = HashSet::new();
        for (y, line) in input.lines().enumerate() {
            let y = y as u64;

            for (x, c) in line.chars().enumerate() {
                let x = x as u64;

                if c == '#' {
                    galaxies.insert((x, y));
                }
            }
        }

        Self { galaxies }
    }

    fn expand(&self, expansion_factor: u64) -> Self {
        let max_x = self.galaxies.iter().map(|&(x, _)| x).max().unwrap_or(0);
        let max_y = self.galaxies.iter().map(|&(x, _)| x).max().unwrap_or(0);

        let empty_cols: Vec<_> = (0..=max_x)
            .filter(|x| self.galaxies.iter().filter(|&(gx, _)| gx == x).count() == 0)
            .collect();

        let empty_rows: Vec<_> = (0..=max_y)
            .filter(|y| self.galaxies.iter().filter(|&(_, gy)| gy == y).count() == 0)
            .collect();

        let galaxies = self
            .galaxies
            .iter()
            .map(|&(x, y)| {
                let move_x = empty_cols.iter().filter(|&&empty_x| empty_x < x).count() as u64;
                let move_y = empty_rows.iter().filter(|&&empty_y| empty_y < y).count() as u64;

                (
                    x + move_x * (expansion_factor - 1),
                    y + move_y * (expansion_factor - 1),
                )
            })
            .collect();

        Self { galaxies }
    }

    fn puzzle(&self) -> u64 {
        self.galaxies
            .iter()
            .enumerate()
            .map(|(a, &pos_a)| {
                self.galaxies
                    .iter()
                    .enumerate()
                    .filter_map(
                        move |(b, &pos_b)| {
                            if a < b {
                                Some((pos_a, pos_b))
                            } else {
                                None
                            }
                        },
                    )
            })
            .flatten()
            .map(|(a, b)| Self::distance(a, b))
            .sum::<u64>()
    }

    fn distance(a: (u64, u64), b: (u64, u64)) -> u64 {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
    }
}

fn main() {
    let game = Game::parse(include_str!("sample-input.txt"));
    dbg!(game.puzzle());

    let game2 = game.expand(2);
    dbg!(game2.puzzle());

    let game10 = game.expand(10);
    dbg!(game10.puzzle());

    let game100 = game.expand(100);
    dbg!(game100.puzzle());

    let game_million = game.expand(1_000_000);
    dbg!(game_million.puzzle());
}
