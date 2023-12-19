use std::collections::HashMap;

use anyhow::{anyhow, Result};

const CATEGORIES: [&str; 4] = ["x", "m", "a", "s"];

#[derive(Debug, Clone)]
pub struct Game {
    pub workflows: HashMap<String, Workflow>,
    pub ratings: Vec<Rating>,
}

impl Game {
    pub fn part1(&self) -> Result<u64> {
        self.ratings
            .iter()
            .try_fold(0, |sum, rating| match rating.eval(&self.workflows) {
                Ok(Action::Accept) => Ok(sum + rating.value()),
                Ok(_) => Ok(sum),
                Err(e) => Err(e),
            })
    }

    pub fn part2(&self) -> Result<u64> {
        let workflow = self
            .workflows
            .get("in")
            .ok_or(anyhow!("Workflow not found: in"))?;

        self.ops_combination_count(&workflow.ops, &[])
    }

    fn action_combination_count(&self, action: &Action, conds: &[Condition]) -> Result<u64> {
        Ok(match action {
            Action::Accept => Condition::combination_count(conds),
            Action::Reject => 0,
            Action::Workflow(ref w) => {
                let workflow = self
                    .workflows
                    .get(w)
                    .ok_or(anyhow!("Workflow not found: {w}"))?;

                self.ops_combination_count(&workflow.ops, conds)?
            }
        })
    }

    fn ops_combination_count(&self, ops: &[Operation], prev_conds: &[Condition]) -> Result<u64> {
        ops.split_first().map_or(Ok(0), |(op, rest_ops)| {
            let mut rest_conds = prev_conds.to_vec();
            let mut conds = rest_conds.to_vec();

            if let Some(ref cond) = op.cond {
                conds.push(cond.clone());
                rest_conds.push(cond.inverse());
            }

            Ok(self.action_combination_count(&op.action, &conds)?
                + self.ops_combination_count(rest_ops, &rest_conds)?)
        })
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

    fn combination_count(conds: &[Self]) -> u64 {
        CATEGORIES
            .iter()
            .map(|&category| {
                let (mut min, mut max) = (1, 4000);
                for cond in conds.iter().filter(|cond| cond.var == category) {
                    match cond.sign {
                        Sign::Greater => min = min.max(cond.value + 1),
                        Sign::Less => max = max.min(cond.value - 1),
                        Sign::GreaterEq => min = min.max(cond.value),
                        Sign::LessEq => max = max.min(cond.value),
                    }
                }
                if min <= max {
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
    fn eval(&self, workflows: &HashMap<String, Workflow>) -> Result<Action> {
        let mut action = Action::Workflow("in".to_string());

        while let Action::Workflow(workflow) = action {
            let workflow = workflows
                .get(&workflow)
                .ok_or(anyhow!("Workflow not found: {workflow}"))?;

            action = workflow
                .eval(self)
                .ok_or(anyhow!("Eval did not find any result"))?;
        }

        Ok(action)
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

        assert_eq!(game.part1().unwrap(), 19114);
    }

    #[test]
    fn test_part2() {
        let game = parse_game(SAMPLE_INPUT).unwrap().1;

        assert_eq!(game.part2().unwrap(), 167409079868000);
    }

    #[test]
    fn test_combination_count() {
        assert_eq!(Condition::combination_count(&[]), 256000000000000);

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
