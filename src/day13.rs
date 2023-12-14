use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::min;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Pattern {
    pattern: Vec<Vec<char>>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}

impl Pattern {
    fn find_reflection(&self) -> Reflection {
        if let Some(reflection) = self.find_vertical_reflection() {
            reflection
        } else {
            self.find_horizontal_reflection().unwrap()
        }
    }

    fn find_vertical_reflection(&self) -> Option<Reflection> {
        let num_cols = self.pattern[0].len();

        for x in 0..num_cols - 1 {
            if self.is_vertical_reflection(&Reflection::Vertical(x)) {
                return Some(Reflection::Vertical(x));
            }
        }

        None
    }

    fn is_vertical_reflection(&self, reflection: &Reflection) -> bool {
        match reflection {
            Reflection::Vertical(x_line) => {
                for y in 0..self.pattern.len() {
                    let reflection_end = min(self.pattern[y].len() - x_line - 2, *x_line);
                    for x_offset in 0..=reflection_end {
                        if self.pattern[y][*x_line + x_offset + 1]
                            != self.pattern[y][*x_line - x_offset]
                        {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    fn find_horizontal_reflection(&self) -> Option<Reflection> {
        let num_rows = self.pattern.len();

        for y in 0..num_rows - 1 {
            if self.is_horizontal_reflection(&Reflection::Horizontal(y)) {
                return Some(Reflection::Horizontal(y));
            }
        }

        None
    }

    fn is_horizontal_reflection(&self, reflection: &Reflection) -> bool {
        match reflection {
            Reflection::Horizontal(y_line) => {
                let reflection_end = min(self.pattern.len() - y_line - 2, *y_line);
                for y_offset in 0..=reflection_end {
                    for x in 0..self.pattern[y_offset].len() {
                        if self.pattern[*y_line + y_offset + 1][x]
                            != self.pattern[*y_line - y_offset][x]
                        {
                            return false;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}

impl FromStr for Pattern {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern = s.lines().map(|l| l.chars().collect()).collect::<Vec<_>>();

        Ok(Pattern { pattern })
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Pattern> {
    input
        .split("\n\n")
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>()
}

#[aoc(day13, part1)]
fn part1(input: &[Pattern]) -> usize {
    let reflections = input
        .iter()
        .map(|p| p.find_reflection())
        .collect::<Vec<_>>();

    reflections
        .iter()
        .map(|r| match r {
            Reflection::Vertical(v) => *v + 1,
            Reflection::Horizontal(h) => 100 * (h + 1),
        })
        .sum()
}
