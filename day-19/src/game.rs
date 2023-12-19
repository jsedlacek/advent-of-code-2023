use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Game {
    pub workflows: HashMap<String, Workflow>,
    pub ratings: Vec<Rating>,
}

impl Game {
    pub fn puzzle(&self) -> u64 {
        self.ratings
            .iter()
            .filter(|r| r.eval(&self.workflows) == Action::Accept)
            .map(|r| r.value())
            .sum::<u64>()
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
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Sign {
    Greater,
    Less,
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
            println!("-> Workflow {workflow}");

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
