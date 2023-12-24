use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Behavior {
    FlipFlop { state: FlipFlopState },
    Conjunction { memory: HashMap<String, Pulse> },
    Broadcaster,
    Sink,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Module {
    name: String,
    behavior: Behavior,
    inputs: Vec<String>,
    outputs: Vec<String>,
}

impl Module {
    fn flip_flop(name: String, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Self {
            name,
            inputs,
            outputs,
            behavior: Behavior::FlipFlop {
                state: FlipFlopState::Off,
            },
        }
    }

    fn conjunction(name: String, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        let memory = inputs.iter().map(|s| (s.clone(), Pulse::Low)).collect();

        Self {
            name,
            inputs,
            outputs,
            behavior: Behavior::Conjunction { memory },
        }
    }

    fn broadcaster(name: String, inputs: Vec<String>, outputs: Vec<String>) -> Self {
        Self {
            name,
            inputs,
            outputs,
            behavior: Behavior::Broadcaster,
        }
    }

    fn sink(name: String, inputs: Vec<String>) -> Self {
        Self {
            name,
            inputs,
            outputs: vec![],
            behavior: Behavior::Sink,
        }
    }

    fn handle_pulse(&mut self, input: &str, pulse: &Pulse) -> Option<Pulse> {
        match (&mut self.behavior, pulse) {
            (Behavior::FlipFlop { .. }, Pulse::High) => None,
            (Behavior::FlipFlop { state }, Pulse::Low) => match state {
                FlipFlopState::Off => {
                    *state = FlipFlopState::On;
                    Some(Pulse::High)
                }
                FlipFlopState::On => {
                    *state = FlipFlopState::Off;
                    Some(Pulse::Low)
                }
            },
            (Behavior::Conjunction { ref mut memory }, pulse) => {
                memory.insert(input.to_string(), *pulse);
                if memory.values().all(|p| p == &Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            (Behavior::Broadcaster, pulse) => Some(*pulse),
            (Behavior::Sink, _) => None,
        }
    }

    fn handle_pulses(&mut self, input: &str, pulse: &Pulse) -> Vec<(String, Pulse)> {
        let output = self.handle_pulse(input, pulse);

        match output {
            Some(output) => self.outputs.iter().map(|o| (o.clone(), output)).collect(),
            None => vec![],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ModuleConfig {
    modules: RefCell<HashMap<String, Rc<RefCell<Module>>>>,
}

impl ModuleConfig {
    fn push_button(&mut self) -> (usize, usize) {
        let mut low_pulse_count = 0usize;
        let mut high_pulse_count = 0usize;

        let mut pulses = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];
        let modules = self.modules.borrow();

        while !pulses.is_empty() {
            low_pulse_count += pulses.iter().filter(|(_, _, p)| p == &Pulse::Low).count();
            high_pulse_count += pulses.iter().filter(|(_, _, p)| p == &Pulse::High).count();

            let mut next_pulses = vec![];

            for (input, dest, pulse) in pulses {
                if let Some(module) = modules.get(dest.as_str()) {
                    let mut module = module.borrow_mut();
                    let dest_pulses = module.handle_pulses(input.as_str(), &pulse);
                    next_pulses.extend(
                        dest_pulses
                            .iter()
                            .map(|(d, p)| (module.name.clone(), d.clone(), *p)),
                    );
                }
            }

            pulses = next_pulses;
        }

        (low_pulse_count, high_pulse_count)
    }

    fn pulse_score(&self, n: usize) -> usize {
        let mut module_config = self.clone();
        let mut low_pulse_count = 0usize;
        let mut high_pulse_count = 0usize;

        for _ in 0..n {
            let (low, high) = module_config.push_button();

            low_pulse_count += low;
            high_pulse_count += high;
        }

        low_pulse_count * high_pulse_count
    }

    fn button_presses_until(&self, dest_module: String, dest_pulse: Pulse) -> usize {
        let module_config = self.clone();
        let mut num_presses = 0usize;
        let mut cond_reached = false;

        let modules = module_config.modules.borrow();

        while !cond_reached {
            num_presses += 1;
            println!("num_presses: {}", num_presses);
            let mut pulses = vec![("button".to_string(), "broadcaster".to_string(), Pulse::Low)];

            while !cond_reached && !pulses.is_empty() {
                println!("pulse");
                let mut next_pulses = vec![];

                for (input, dest, pulse) in pulses {
                    if let Some(module) = modules.get(dest.as_str()) {
                        let mut module = module.borrow_mut();
                        let dest_pulses = module.handle_pulses(input.as_str(), &pulse);

                        if dest_pulses
                            .iter()
                            .any(|(d, p)| p == &dest_pulse && d == &dest_module)
                        {
                            cond_reached = true;
                            break;
                        }

                        next_pulses.extend(
                            dest_pulses
                                .iter()
                                .map(|(d, p)| (module.name.clone(), d.clone(), *p)),
                        );
                    }
                }

                pulses = next_pulses;
            }
        }

        num_presses
    }
}

lazy_static! {
    static ref RE_MODULE: Regex = Regex::new(r"([%|&]?)(\w+|broadcaster) -> (.*)").unwrap();
}

impl FromStr for ModuleConfig {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let module_lines = s.lines().collect::<Vec<_>>();

        let mut module_names = HashSet::<String>::new();
        let mut inputs = HashMap::<String, Vec<String>>::new();
        let mut outputs = HashMap::<String, Vec<String>>::new();
        let mut behaviors = HashMap::<String, String>::new();

        for line in module_lines {
            let caps = RE_MODULE.captures(line).ok_or(())?;
            let behavior = caps.get(1).unwrap().as_str().to_string();
            let name = caps.get(2).unwrap().as_str().to_string();
            let module_outputs = caps
                .get(3)
                .unwrap()
                .as_str()
                .split(", ")
                .map(|s| s.to_string())
                .collect::<Vec<_>>();

            for output in &module_outputs {
                inputs.entry(output.clone()).or_default().push(name.clone());
                module_names.insert(output.clone());
            }

            outputs.insert(name.clone(), module_outputs);
            behaviors.insert(name.clone(), behavior);

            module_names.insert(name.clone());
        }

        let modules = module_names
            .into_iter()
            .map(|name| {
                let behavior = behaviors.get(&name).map(|s| s.as_str());
                let mod_inputs = inputs.remove(&name).unwrap_or_default();
                let mod_outputs = outputs.remove(&name).unwrap_or_default();

                (
                    name.clone(),
                    Rc::new(RefCell::new(match behavior {
                        Some("%") => Module::flip_flop(name, mod_inputs, mod_outputs),
                        Some("&") => Module::conjunction(name, mod_inputs, mod_outputs),
                        Some(_) => Module::broadcaster(name, mod_inputs, mod_outputs),
                        None => Module::sink(name, mod_inputs),
                    })),
                )
            })
            .collect::<HashMap<_, _>>()
            .into();

        Ok(Self { modules })
    }
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> ModuleConfig {
    input.parse().unwrap()
}

#[aoc(day20, part1)]
fn part1(config: &ModuleConfig) -> usize {
    config.pulse_score(1000)
}

#[aoc(day20, part2)]
fn part2(config: &ModuleConfig) -> usize {
    config.button_presses_until("rx".to_string(), Pulse::Low)
}
