use std::collections::HashMap;

const CATEGORIES: [&str; 4] = ["x", "m", "a", "s"];

#[derive(Debug, Clone)]
pub struct Game {
    pub workflows: HashMap<String, Workflow>,
    pub ratings: Vec<Rating>,
}

impl Game {
    pub fn part1(&self) -> u64 {
        self.ratings
            .iter()
            .filter(|r| r.eval(&self.workflows) == Action::Accept)
            .map(|r| r.value())
            .sum::<u64>()
    }

    pub fn part2(&self) -> u64 {
        let workflow = self.workflows.get("in").unwrap();

        self.ops_combination_count(&workflow.ops, &[])
    }

    fn action_combination_count(&self, action: &Action, conds: &[Condition]) -> u64 {
        match action {
            Action::Accept => Condition::combination_count(&conds),
            Action::Reject => 0,
            Action::Workflow(ref w) => {
                let workflow = self.workflows.get(w).unwrap();
                self.ops_combination_count(&workflow.ops, &conds)
            }
        }
    }

    fn ops_combination_count(&self, ops: &[Operation], prev_conds: &[Condition]) -> u64 {
        if let [op, rest_ops @ ..] = ops {
            let mut rest_conds = prev_conds.to_vec();
            let mut conds = rest_conds.to_vec();

            if let Some(ref cond) = op.cond {
                conds.push(cond.clone());
                rest_conds.push(cond.inverse());
            }

            self.action_combination_count(&op.action, &conds)
                + self.ops_combination_count(&rest_ops, &rest_conds)
        } else {
            0
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Workflow {
    pub name: String,
    pub ops: Vec<Operation>,
}
impl Workflow {
    fn eval(&self, rating: &Rating) -> Option<Action> {
        for op in &self.ops {
            if op.eval(rating) {
                return Some(op.action.clone());
            }
        }

        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Operation {
    pub cond: Option<Condition>,
    pub action: Action,
}

impl Operation {
    fn eval(&self, rating: &Rating) -> bool {
        match &self.cond {
            Some(cond) => cond.eval(rating),
            None => true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Condition {
    pub var: String,
    pub sign: Sign,
    pub value: u64,
}

impl Condition {
    fn eval(&self, rating: &Rating) -> bool {
        let var_value = rating.var_value(&self.var);

        match self.sign {
            Sign::Greater => var_value > self.value,
            Sign::Less => var_value < self.value,
            Sign::GreaterEq => var_value >= self.value,
            Sign::LessEq => var_value <= self.value,
        }
    }

    fn inverse(&self) -> Self {
        Self {
            value: self.value,
            var: self.var.clone(),
            sign: match self.sign {
                Sign::Greater => Sign::LessEq,
                Sign::Less => Sign::GreaterEq,
                Sign::GreaterEq => Sign::Less,
                Sign::LessEq => Sign::Greater,
            },
        }
    }

    fn combination_count(ops: &[Self]) -> u64 {
        CATEGORIES
            .iter()
            .map(|k| {
                let mut min = 1;
                let mut max = 4000;

                for op in ops.iter().filter(|o| &o.var == k) {
                    match op.sign {
                        Sign::Greater => min = min.max(op.value + 1),
                        Sign::Less => max = max.min(op.value - 1),
                        Sign::GreaterEq => min = min.max(op.value),
                        Sign::LessEq => max = max.min(op.value),
                    };
                }

                if max >= min {
                    max - min + 1
                } else {
                    0
                }
            })
            .product()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    Greater,
    Less,
    GreaterEq,
    LessEq,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Action {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Rating(pub HashMap<String, u64>);

impl Rating {
    fn eval(&self, workflows: &HashMap<String, Workflow>) -> Action {
        let mut action = Action::Workflow("in".to_string());

        while let Action::Workflow(workflow) = action {
            let workflow = workflows.get(&workflow).unwrap();
            action = workflow.eval(self).unwrap();
        }

        action
    }

    fn var_value(&self, var: &str) -> u64 {
        self.0.get(var).copied().unwrap_or_default()
    }

    fn value(&self) -> u64 {
        self.0.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_game;

    use super::*;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");

    #[test]
    fn test_part1() {
        let game = parse_game(SAMPLE_INPUT).unwrap().1;

        assert_eq!(game.part1(), 19114);
    }

    #[test]
    fn test_part2() {
        let game = parse_game(SAMPLE_INPUT).unwrap().1;

        assert_eq!(game.part2(), 167409079868000);
    }

    #[test]
    fn test_combination_count() {
        assert_eq!(
            Condition::combination_count(&[Condition {
                var: "x".to_string(),
                sign: Sign::Greater,
                value: 1000,
            }]),
            192000000000000
        );

        assert_eq!(
            Condition::combination_count(&[Condition {
                var: "x".to_string(),
                sign: Sign::Greater,
                value: 0,
            }]),
            256000000000000
        );

        assert_eq!(
            Condition::combination_count(
                &CATEGORIES
                    .iter()
                    .map(|k| {
                        Condition {
                            var: k.to_string(),
                            sign: Sign::LessEq,
                            value: 1,
                        }
                    })
                    .collect::<Vec<_>>()
            ),
            1
        );
    }
}
