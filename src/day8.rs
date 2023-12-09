use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
}

struct DesertMap {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl DesertMap {
    fn count_steps_in_traversal(&self) -> usize {
        let mut steps = 0usize;
        let mut current_dir_idx = 0usize;
        let mut current_node = "AAA";

        while current_node != "ZZZ" {
            let (left, right) = &self.nodes[current_node];

            match self.directions[current_dir_idx] {
                Direction::Left => {
                    current_node = left.as_str();
                }
                Direction::Right => {
                    current_node = right.as_str();
                }
            }

            current_dir_idx = (current_dir_idx + 1) % self.directions.len();
            steps += 1;
        }

        steps
    }

    fn count_steps_and_loop_info(&self, start_node: &str) -> (usize, usize, usize) {
        let mut steps = 0usize;
        let mut current_dir_idx = 0usize;
        let mut current_node = start_node;
        let mut path = vec![current_node];
        let mut loop_start = None;
        let mut loop_length = None;
        let mut finish = None;

        while loop_length.is_none() || finish.is_none() {
            let (left, right) = &self.nodes[current_node];

            match self.directions[current_dir_idx] {
                Direction::Left => {
                    current_node = left.as_str();
                }
                Direction::Right => {
                    current_node = right.as_str();
                }
            }

            current_dir_idx = (current_dir_idx + 1) % self.directions.len();
            steps += 1;

            if loop_length.is_none() {
                if path.contains(&current_node) {
                    loop_start = Some(path.iter().position(|n| n == &current_node).unwrap());
                    loop_length = Some(path.len() - loop_start.unwrap());
                } else {
                    path.push(current_node);
                }
            }

            if current_node.ends_with('Z') {
                finish = Some(steps);
            }
        }

        (finish.unwrap(), loop_start.unwrap(), loop_length.unwrap())
    }

    fn count_steps_in_ghost_traversal(&self) -> usize {
        let steps = 0usize;

        let start_nodes = self
            .nodes
            .iter()
            .filter(|(k, _)| k.ends_with('A'))
            .map(|(k, _)| k.as_str())
            .collect::<Vec<_>>();

        for start_node in start_nodes {
            println!("Starting at {}", start_node);
            let (steps_to_finish, loop_start, loop_length) =
                self.count_steps_and_loop_info(start_node);

            println!(
                "Steps to finish: {}, loop start: {}, loop length: {}",
                steps_to_finish, loop_start, loop_length
            );
        }

        steps
    }
}

lazy_static! {
    static ref NAME_REGEX: Regex = Regex::new(r"\w+").unwrap();
}

impl FromStr for DesertMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dirs, nodes) = s.split_once("\n\n").ok_or(())?;
        let directions = dirs
            .chars()
            .map(|c| match c {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            })
            .collect::<Vec<_>>();

        let nodes = nodes
            .lines()
            .map(|l| {
                let (name, rest) = l.split_once(" = ").ok_or(())?;

                let dests = NAME_REGEX
                    .find_iter(rest)
                    .map(|m| m.as_str())
                    .collect::<Vec<_>>();

                Ok((
                    name.to_string(),
                    (dests[0].to_string(), dests[1].to_string()),
                ))
            })
            .collect::<Result<_, _>>()?;

        Ok(DesertMap { directions, nodes })
    }
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> DesertMap {
    input.parse().unwrap()
}

#[aoc(day8, part1)]
fn part1(map: &DesertMap) -> usize {
    map.count_steps_in_traversal()
}

#[aoc(day8, part2)]
fn part2(map: &DesertMap) -> usize {
    map.count_steps_in_ghost_traversal()
}
