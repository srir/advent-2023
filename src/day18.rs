use crate::util::Direction;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::{component_index, connected_components};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DigOp {
    direction: Direction,
    distance: isize,
    color: String,
}

impl FromStr for DigOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let toks = s.split_whitespace().collect::<Vec<_>>();

        let direction = match toks[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => return Err(()),
        };
        let distance = toks[1].parse().map_err(|_| ())?;
        let color = toks[2]
            .strip_prefix("(#")
            .ok_or(())?
            .strip_suffix(")")
            .ok_or(())?
            .to_string();
        Ok(Self {
            direction,
            distance,
            color,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Hole {
    color: String,
    depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DigMap {
    ops: Vec<DigOp>,
    map: HashMap<(isize, isize), Hole>,
}

impl FromStr for DigMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let map = HashMap::new();
        Ok(Self { ops, map })
    }
}

impl DigMap {
    fn dig_trench(&mut self) {
        let mut pos = (0, 0);

        for op in &self.ops {
            for _ in 0..op.distance {
                match op.direction {
                    Direction::Up => pos.1 += 1,
                    Direction::Down => pos.1 -= 1,
                    Direction::Left => pos.0 -= 1,
                    Direction::Right => pos.0 += 1,
                }
                self.map.insert(
                    pos,
                    Hole {
                        color: op.color.clone(),
                        depth: 1,
                    },
                );
            }
        }
    }

    fn dig_size(&self) -> usize {
        let ((min_x, max_x), (min_y, max_y)) = self.grid_bounds();
        let starts = ((min_x - 1)..=(max_x + 1))
            .flat_map(|x| ((min_y - 1)..=(max_y + 1)).map(move |y| (x, y)))
            .collect::<Vec<_>>();

        let comps = connected_components(&starts, |&(x, y)| {
            let is_in_trench = self.map.contains_key(&(x, y));
            let mut neighbors = vec![];
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;

                match (is_in_trench, self.map.get(&(nx, ny))) {
                    (true, Some(_)) => {
                        neighbors.push((nx, ny));
                    }
                    (false, None) => {
                        neighbors.push((nx, ny));
                    }
                    _ => (),
                }
            }
            neighbors
        });

        let comp_index = component_index(&comps);

        let outer_comp_idx = comp_index.get(&(min_x - 1, min_y - 1)).unwrap();
        let outer_comp = &comps[*outer_comp_idx];

        let total_cell_count = comps.iter().map(|c| c.len()).sum::<usize>();

        total_cell_count - outer_comp.len()
    }

    fn grid_bounds(&self) -> ((isize, isize), (isize, isize)) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;

        for (x, y) in self.map.keys() {
            if *x < min_x {
                min_x = *x;
            }
            if *x > max_x {
                max_x = *x;
            }
            if *y < min_y {
                min_y = *y;
            }
            if *y > max_y {
                max_y = *y;
            }
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> DigMap {
    input.parse().unwrap()
}

#[aoc(day18, part1)]
fn part1(dig_map: &DigMap) -> usize {
    let mut dig_map = dig_map.clone();
    dig_map.dig_trench();

    dig_map.dig_size()
}
