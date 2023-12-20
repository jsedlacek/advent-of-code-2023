use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct Game {
    map: HashMap<String, Module>,
}

impl Game {
    pub fn new(mut modules: Vec<Module>) -> Self {
        let inputs = Self::find_inputs(&modules);
        for module in modules.iter_mut() {
            if let ModuleValue::Conjunction(c) = &mut module.value {
                let module_names = inputs.get(&module.name).unwrap();
                c.init(module_names);
            }
        }

        let map = modules.into_iter().map(|m| (m.name.clone(), m)).collect();

        Self { map }
    }

    pub fn part1(&mut self) -> u64 {
        let mut pulse_count = (0u64, 0u64);

        for _ in 0..1000 {
            self.send_signal("button", "broadcaster", Signal::Low, &mut pulse_count);
        }

        pulse_count.0 * pulse_count.1
    }

    fn find_inputs(modules: &[Module]) -> HashMap<String, Vec<String>> {
        let mut res: HashMap<String, Vec<String>> = HashMap::new();

        for module in modules {
            for output in &module.outputs {
                res.entry(output.clone())
                    .or_default()
                    .push(module.name.clone());
            }
        }

        res
    }

    fn send_signal(
        &mut self,
        from: &str,
        module_name: &str,
        signal: Signal,
        pulse_count: &mut (u64, u64),
    ) {
        let mut queue: VecDeque<(String, String, Signal)> = VecDeque::new();

        queue.push_back((from.to_string(), module_name.to_string(), signal));

        while let Some((from, module_name, signal)) = queue.pop_front() {
            if signal == Signal::Low {
                pulse_count.0 += 1;
            } else {
                pulse_count.1 += 1;
            }

            if let Some(module) = self.map.get_mut(&module_name) {
                queue.append(&mut module.process_signal(&from, signal));
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    pub name: String,
    pub outputs: Vec<String>,
    pub value: ModuleValue,
}

impl Module {
    fn process_signal(&mut self, from: &str, signal: Signal) -> VecDeque<(String, String, Signal)> {
        println!("{from} -{signal:?} -> {}", self.name);

        let mut queue = VecDeque::new();

        match &mut self.value {
            ModuleValue::FlipFlop(flip_flop) => {
                if signal == Signal::Low {
                    flip_flop.state = flip_flop.state.flip();
                    let next_signal = match flip_flop.state {
                        State::On => Signal::High,
                        State::Off => Signal::Low,
                    };
                    for m in &self.outputs {
                        queue.push_back((self.name.clone(), m.clone(), next_signal));
                    }
                }
            }
            ModuleValue::Conjunction(conjunction) => {
                let signals = conjunction.incoming_signals.as_mut().unwrap();
                let entry = signals.get_mut(from).unwrap();
                *entry = signal;

                let next_signal = if signals.values().all(|s| *s == Signal::High) {
                    Signal::Low
                } else {
                    Signal::High
                };

                for m in &self.outputs {
                    queue.push_back((self.name.clone(), m.clone(), next_signal));
                }
            }
            ModuleValue::Broadcaster => {
                for m in &self.outputs {
                    queue.push_back((self.name.clone(), m.clone(), signal));
                }
            }
        }

        queue
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleValue {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster,
}
impl ModuleValue {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlipFlop {
    pub state: State,
}

impl FlipFlop {
    pub fn new() -> Self {
        Self { state: State::Off }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjunction {
    pub incoming_signals: Option<HashMap<String, Signal>>,
}

impl Conjunction {
    pub fn new() -> Self {
        Self {
            incoming_signals: None,
        }
    }

    fn init(&mut self, module_names: &[String]) {
        self.incoming_signals = Some(
            module_names
                .into_iter()
                .map(|name| (name.clone(), Signal::Low))
                .collect(),
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Signal {
    Low,
    High,
}

impl State {
    pub fn flip(&self) -> Self {
        match self {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::parser::parse_game;

    const SAMPLE_INPUT: &str = include_str!("sample-input.txt");
    const SAMPLE_INPUT_2: &str = include_str!("sample-input-2.txt");

    #[test]
    fn test_part1() {
        let mut game = parse_game(SAMPLE_INPUT).unwrap().1;

        assert_eq!(game.part1(), 32000000);

        let mut game2 = parse_game(SAMPLE_INPUT_2).unwrap().1;

        assert_eq!(game2.part1(), 11687500);
    }
}
