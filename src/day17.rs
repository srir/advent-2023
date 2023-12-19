use crate::util::Direction;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::dijkstra;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    width: usize,
    height: usize,
    data: Vec<Vec<usize>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    y: isize,
    x: isize,
    direction: Direction,
    steps_taken_in_direction: usize,
}

impl Pos {
    fn new(y: isize, x: isize, direction: Direction) -> Self {
        Self {
            y,
            x,
            direction,
            steps_taken_in_direction: 0,
        }
    }

    fn step_in_direction(&self, direction: &Direction, grid: &Grid) -> Option<Pos> {
        match direction {
            Direction::Up => {
                if self.y > 0 {
                    Some(Pos {
                        y: self.y - 1,
                        x: self.x,
                        direction: Direction::Up,
                        steps_taken_in_direction: if self.direction == Direction::Up {
                            self.steps_taken_in_direction + 1
                        } else {
                            1
                        },
                    })
                } else {
                    None
                }
            }
            Direction::Down => {
                if self.y < (grid.height as isize) - 1 {
                    Some(Pos {
                        y: self.y + 1,
                        x: self.x,
                        direction: Direction::Down,
                        steps_taken_in_direction: if self.direction == Direction::Down {
                            self.steps_taken_in_direction + 1
                        } else {
                            1
                        },
                    })
                } else {
                    None
                }
            }
            Direction::Left => {
                if self.x > 0 {
                    Some(Pos {
                        y: self.y,
                        x: self.x - 1,
                        direction: Direction::Left,
                        steps_taken_in_direction: if self.direction == Direction::Left {
                            self.steps_taken_in_direction + 1
                        } else {
                            1
                        },
                    })
                } else {
                    None
                }
            }
            Direction::Right => {
                if self.x < (grid.width as isize) - 1 {
                    Some(Pos {
                        y: self.y,
                        x: self.x + 1,
                        direction: Direction::Right,
                        steps_taken_in_direction: if self.direction == Direction::Right {
                            self.steps_taken_in_direction + 1
                        } else {
                            1
                        },
                    })
                } else {
                    None
                }
            }
        }
    }

    fn successors(&self, grid: &Grid) -> Vec<Pos> {
        let mut successors = Vec::new();

        if self.steps_taken_in_direction <= 2 {
            if let Some(p) = self.step_in_direction(&self.direction, grid) {
                successors.push(p);
            }
        }

        vec![
            self.step_in_direction(&self.direction.left(), grid),
            self.step_in_direction(&self.direction.right(), grid),
        ]
        .into_iter()
        .flatten()
        .for_each(|p| successors.push(p));

        successors
    }

    fn ultra_successors(&self, grid: &Grid) -> Vec<Pos> {
        let mut successors = Vec::new();

        if self.steps_taken_in_direction < 10 {
            if let Some(p) = self.step_in_direction(&self.direction, grid) {
                successors.push(p);
            }
        }

        if self.steps_taken_in_direction >= 4 {
            vec![
                self.step_in_direction(&self.direction.left(), grid),
                self.step_in_direction(&self.direction.right(), grid),
            ]
            .into_iter()
            .flatten()
            .for_each(|p| successors.push(p));
        }

        successors
    }
}

impl Grid {
    fn minimal_heat_loss(&self) -> usize {
        let initial_pos = Pos::new(0, 0, Direction::Right);

        let (_, cost) = dijkstra(
            &initial_pos,
            |pos| {
                pos.successors(self)
                    .into_iter()
                    .map(|p| (p, self.data[p.y as usize][p.x as usize]))
            },
            |pos| pos.x == (self.width as isize) - 1 && pos.y == (self.height as isize) - 1,
        )
        .unwrap();

        cost
    }

    fn ultra_crucible_minimal_heat_loss(&self) -> usize {
        let initial_pos = Pos {
            y: 0,
            x: 0,
            direction: Direction::Right,
            steps_taken_in_direction: 1,
        };

        let (_, cost) = dijkstra(
            &initial_pos,
            |pos| {
                pos.ultra_successors(self)
                    .into_iter()
                    .map(|p| (p, self.data[p.y as usize][p.x as usize]))
            },
            |pos| {
                pos.x == (self.width as isize) - 1
                    && pos.y == (self.height as isize) - 1
                    && pos.steps_taken_in_direction >= 4
            },
        )
        .unwrap();

        cost
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(c.to_digit(10).unwrap() as usize);
            }
            data.push(row);
        }

        let width = data[0].len();
        let height = data.len();

        Ok(Grid {
            width,
            height,
            data,
        })
    }
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day17, part1)]
fn part1(grid: &Grid) -> usize {
    grid.minimal_heat_loss()
}

#[aoc(day17, part2)]
fn part2(grid: &Grid) -> usize {
    grid.ultra_crucible_minimal_heat_loss()
}
