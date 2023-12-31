use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::{anyhow, Result};

use crate::lcm::lcm_of_slice;

#[derive(Debug)]
pub struct Game {
    map: HashMap<String, Module>,
}

impl Game {
    pub fn new(mut modules: Vec<Module>) -> Result<Self> {
        let inputs = Self::find_inputs(&modules);

        for module in modules.iter_mut() {
            if let ModuleBehavior::Conjunction(c) = &mut module.behavior {
                let input_names = inputs.get(&module.name).map_or_else(Vec::new, |names| {
                    names.iter().map(String::as_str).collect::<Vec<_>>()
                });
                c.init(&input_names);
            }
        }

        let map = modules.into_iter().map(|m| (m.name.clone(), m)).collect();

        Ok(Self { map })
    }

    pub fn part1(&mut self) -> Result<u64> {
        let mut pulse_count = (0u64, 0u64);

        for _ in 0..1000 {
            self.send_signal(
                "button",
                "broadcaster",
                Signal::Low,
                Some(&mut pulse_count),
                &[],
            )?;
        }

        Ok(pulse_count.0 * pulse_count.1)
    }

    pub fn part2(&mut self) -> Result<u64> {
        let inputs = Self::find_inputs(&self.map.values().cloned().collect::<Vec<_>>());

        let modules = inputs
            .get("rx")
            .ok_or(anyhow!("Input not found for module: rx",))?;

        let modules_inputs = modules
            .iter()
            .filter_map(|m| inputs.get(m))
            .flatten()
            .collect::<HashSet<_>>() // Unique
            .into_iter()
            .map(String::as_str)
            .collect::<Vec<_>>(); // Collection into vec

        let mut target_results: HashMap<String, u64> = HashMap::new();

        for i in 1.. {
            let targets =
                self.send_signal("button", "broadcaster", Signal::Low, None, &modules_inputs)?;

            for t in &targets {
                if !target_results.contains_key(t) {
                    target_results.insert(t.clone(), i);
                }
            }

            if modules_inputs
                .iter()
                .all(|t| target_results.contains_key(&t.to_string()))
            {
                return Ok(lcm_of_slice(
                    &target_results.values().copied().collect::<Vec<_>>(),
                ));
            }
        }

        panic!("Unreachable");
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
        mut pulse_count: Option<&mut (u64, u64)>,
        targets: &[&str],
    ) -> Result<Vec<String>> {
        let mut res = Vec::new();

        let mut queue: VecDeque<(String, String, Signal)> = VecDeque::new();

        queue.push_back((from.to_string(), module_name.to_string(), signal));

        while let Some((from, module_name, signal)) = queue.pop_front() {
            if let Some(pulse_count) = pulse_count.as_mut() {
                if signal == Signal::Low {
                    pulse_count.0 += 1;
                } else {
                    pulse_count.1 += 1;
                }
            }

            if targets.contains(&module_name.as_str()) && signal == Signal::Low {
                res.push(module_name.clone());
            }

            if let Some(module) = self.map.get_mut(&module_name) {
                queue.append(&mut module.process_signal(&from, signal)?);
            }
        }

        Ok(res)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module {
    name: String,
    outputs: Vec<String>,
    behavior: ModuleBehavior,
    signal_count: (u64, u64),
}

impl Module {
    pub fn new(name: String, outputs: Vec<String>, behavior: ModuleBehavior) -> Self {
        Self {
            name,
            outputs,
            signal_count: (0, 0),
            behavior,
        }
    }

    fn process_signal(
        &mut self,
        from: &str,
        signal: Signal,
    ) -> Result<VecDeque<(String, String, Signal)>> {
        if signal == Signal::Low {
            self.signal_count.0 += 1;
        } else {
            self.signal_count.1 += 1;
        }

        let mut queue = VecDeque::new();

        match &mut self.behavior {
            ModuleBehavior::FlipFlop(flip_flop) => {
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
            ModuleBehavior::Conjunction(conjunction) => {
                let signals = conjunction
                    .incoming_signals
                    .as_mut()
                    .ok_or(anyhow!("Conjunction not inited"))?;

                let entry = signals
                    .get_mut(from)
                    .ok_or(anyhow!("Incoming signal not exists: {}", from))?;

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
            ModuleBehavior::Broadcaster => {
                for m in &self.outputs {
                    queue.push_back((self.name.clone(), m.clone(), signal));
                }
            }
        }

        Ok(queue)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleBehavior {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlipFlop {
    state: State,
}

impl FlipFlop {
    pub fn new() -> Self {
        Self { state: State::Off }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjunction {
    incoming_signals: Option<HashMap<String, Signal>>,
}

impl Conjunction {
    pub fn new() -> Self {
        Self {
            incoming_signals: None,
        }
    }

    fn init(&mut self, input_names: &[&str]) {
        self.incoming_signals = Some(
            input_names
                .iter()
                .map(|name| (name.to_string(), Signal::Low))
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

        assert_eq!(game.part1().unwrap(), 32000000);

        let mut game2 = parse_game(SAMPLE_INPUT_2).unwrap().1;

        assert_eq!(game2.part1().unwrap(), 11687500);
    }
}
