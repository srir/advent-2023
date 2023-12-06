use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Part {
    y: isize,
    x_start: isize,
    x_end: usize,
    number: usize,
}

impl Part {
    fn is_contacting(&self, y: usize, x: usize) -> bool {
        (self.y - 1 <= y as isize && y as isize <= self.y + 1)
            && (self.x_start - 1 <= x as isize && x <= self.x_end + 1)
    }
}

#[derive(Debug)]
struct Loc {
    y: usize,
    x: usize,
    symbol: char,
}

#[derive(Debug)]
struct Engine {
    parts: Vec<Part>,
    engine_locations: Vec<Loc>,
}

impl Engine {
    fn parts_contacting(&self) -> HashSet<&Part> {
        let mut parts_contacting = HashSet::new();

        for loc in &self.engine_locations {
            for part in &self.parts {
                if part.is_contacting(loc.y, loc.x) {
                    parts_contacting.insert(part);
                }
            }
        }

        parts_contacting
    }

    fn sum_parts(&self) -> usize {
        self.parts_contacting().iter().map(|p| p.number).sum()
    }

    fn sum_gear_ratios(&self) -> usize {
        let mut gear_ratio = 0usize;

        for loc in &self.engine_locations {
            if loc.symbol != '*' {
                continue;
            }

            let parts_contacting = self
                .parts
                .iter()
                .filter(|p| p.is_contacting(loc.y, loc.x))
                .collect::<Vec<_>>();

            if parts_contacting.len() == 2 {
                gear_ratio += parts_contacting[0].number * parts_contacting[1].number;
            }
        }

        gear_ratio
    }
}

lazy_static! {
    static ref RE_PART: Regex = Regex::new(r"(\d+)").unwrap();
    static ref RE_ENGINE: Regex = Regex::new(r"[^\d.]").unwrap();
}

impl FromStr for Engine {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = Vec::new();
        let mut engine_locations = Vec::new();

        for (y, line) in s.lines().enumerate() {
            for part_match in RE_PART.find_iter(line) {
                let number = part_match.as_str().parse::<usize>().unwrap();

                parts.push(Part {
                    y: y as isize,
                    x_start: part_match.start() as isize,
                    x_end: part_match.end() - 1,
                    number,
                });
            }

            for engine_match in RE_ENGINE.find_iter(line) {
                let x_start = engine_match.start();

                engine_locations.push(Loc {
                    y,
                    x: x_start,
                    symbol: engine_match.as_str().chars().next().unwrap(),
                });
            }
        }

        Ok(Engine {
            parts,
            engine_locations,
        })
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Engine {
    input.parse::<Engine>().unwrap()
}

#[aoc(day3, part1)]
fn part1(engine: &Engine) -> usize {
    engine.sum_parts()
}

#[aoc(day3, part2)]
fn part2(engine: &Engine) -> usize {
    engine.sum_gear_ratios()
}
