use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dfs_reach;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Plot {
    Start,
    Garden,
    Rock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct GardenMap {
    start: (isize, isize),
    width: isize,
    height: isize,
    data: HashMap<(isize, isize), Plot>,
}

impl FromStr for GardenMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        (y as isize, x as isize),
                        match c {
                            'S' => Plot::Start,
                            '.' => Plot::Garden,
                            '#' => Plot::Rock,
                            _ => panic!("invalid char"),
                        },
                    )
                })
            })
            .collect::<HashMap<_, _>>();

        let start = data
            .iter()
            .find(|(_, plot)| **plot == Plot::Start)
            .map(|(pos, _)| *pos)
            .unwrap();

        let width = data.iter().map(|((_, x), _)| *x).max().unwrap() + 1;

        let height = data.iter().map(|((y, _), _)| *y).max().unwrap() + 1;

        Ok(Self {
            start,
            height,
            width,
            data,
        })
    }
}

impl GardenMap {
    fn get(&self, pos: &(isize, isize), with_repeats: bool) -> Option<&Plot> {
        let y = if with_repeats {
            pos.0 % self.height
        } else {
            pos.0
        };
        let x = if with_repeats {
            pos.1 % self.width
        } else {
            pos.1
        };

        self.data.get(&(y, x))
    }

    fn reachable_in_k_steps(&self, k: usize, with_repeats: bool) -> usize {
        let start_node = (self.start, 0usize);

        let nodes = dfs_reach(start_node, |((y, x), steps)| {
            let mut neighbors: Vec<((isize, isize), usize)> = Vec::new();

            if steps < &k {
                if let Some(g) = self.get(&(y - 1, *x), with_repeats) {
                    if g != &Plot::Rock {
                        neighbors.push(((y - 1, *x), steps + 1));
                    }
                }
                if let Some(g) = self.get(&(y + 1, *x), with_repeats) {
                    if g != &Plot::Rock {
                        neighbors.push(((y + 1, *x), steps + 1));
                    }
                }
                if let Some(g) = self.get(&(*y, x - 1), with_repeats) {
                    if g != &Plot::Rock {
                        neighbors.push(((*y, x - 1), steps + 1));
                    }
                }
                if let Some(g) = self.get(&(*y, x + 1), with_repeats) {
                    if g != &Plot::Rock {
                        neighbors.push(((*y, x + 1), steps + 1));
                    }
                }
            }

            neighbors
        })
        .filter(|(_, steps)| *steps == k)
        .collect::<HashSet<_>>();

        nodes.len()
    }
}

#[aoc_generator(day21)]
fn parse_input(input: &str) -> GardenMap {
    input.parse().unwrap()
}

#[aoc(day21, part1)]
fn part1(map: &GardenMap) -> usize {
    map.reachable_in_k_steps(64, false)
}

#[aoc(day21, part2)]
fn part2(map: &GardenMap) -> usize {
    map.reachable_in_k_steps(26501365, true)
}
