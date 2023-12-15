use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Command {
    AddLens(Lens),
    RemoveLens(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    chars: Vec<char>,
}

lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new(r"([a-zA-Z]+)").unwrap();
    static ref FOCAL_LENGTH_REGEX: Regex = Regex::new(r"=([0-9]+)").unwrap();
    static ref REMOVAL_REGEX: Regex = Regex::new(r"-").unwrap();
}

fn hash(s: &str) -> usize {
    let mut hash = 0usize;

    for c in s.chars() {
        hash += c as usize;
        hash *= 17;
        hash %= 256;
    }

    hash
}

impl Step {
    fn hash(&self) -> usize {
        hash(&self.chars.iter().collect::<String>())
    }

    fn to_command(&self) -> Command {
        let s = self.chars.iter().collect::<String>();

        let label = LABEL_REGEX
            .captures_iter(&s)
            .next()
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .to_string();

        if REMOVAL_REGEX.is_match(&s) {
            Command::RemoveLens(label)
        } else {
            let focal_length = FOCAL_LENGTH_REGEX
                .captures_iter(&s)
                .next()
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();

            Command::AddLens(Lens {
                label,
                focal_length,
            })
        }
    }
}

struct InitSequence {
    steps: Vec<Step>,
}

impl InitSequence {
    fn hash_sum(&self) -> usize {
        self.steps.iter().map(|step| step.hash()).sum()
    }

    fn focusing_power(&self) -> usize {
        let mut boxes = vec![Vec::<Lens>::new(); 256];
        let commands = self
            .steps
            .iter()
            .map(|step| step.to_command())
            .collect::<Vec<_>>();

        for command in commands {
            match command {
                Command::RemoveLens(label) => {
                    let hash = hash(&label);
                    if let Some((pos, _)) = boxes[hash]
                        .iter()
                        .enumerate()
                        .find(|(_, l)| l.label == label)
                    {
                        boxes[hash].remove(pos);
                    }
                }
                Command::AddLens(lens) => {
                    let hash = hash(&lens.label);
                    if let Some((pos, _)) = boxes[hash]
                        .iter()
                        .enumerate()
                        .find(|(_, l)| l.label == lens.label)
                    {
                        boxes[hash][pos] = lens;
                    } else {
                        boxes[hash].push(lens);
                    }
                }
            }
        }

        boxes
            .iter()
            .enumerate()
            .map(|(box_number, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(slot_number, lens)| {
                        (box_number + 1) * (slot_number + 1) * lens.focal_length
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for InitSequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let steps = s
            .trim()
            .split(',')
            .map(|step| Step {
                chars: step.chars().collect(),
            })
            .collect();

        Ok(Self { steps })
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> InitSequence {
    input.parse().unwrap()
}

#[aoc(day15, part1)]
fn part1(init_sequence: &InitSequence) -> usize {
    init_sequence.hash_sum()
}

#[aoc(day15, part2)]
fn part2(init_sequence: &InitSequence) -> usize {
    init_sequence.focusing_power()
}
