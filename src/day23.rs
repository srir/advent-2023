use crate::util::Direction;
use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::prelude::yen;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct HikingMap {
    tiles: Vec<Vec<Tile>>,
    height: usize,
    width: usize,
    start: (usize, usize),
    end: (usize, usize),
}

impl FromStr for HikingMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => Ok(Tile::Path),
                        '#' => Ok(Tile::Forest),
                        '>' => Ok(Tile::Slope(Direction::Right)),
                        '<' => Ok(Tile::Slope(Direction::Left)),
                        '^' => Ok(Tile::Slope(Direction::Up)),
                        'v' => Ok(Tile::Slope(Direction::Down)),
                        _ => Err(()),
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;

        let start = tiles
            .first()
            .and_then(|row| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if *tile == Tile::Path {
                        Some((0, x))
                    } else {
                        None
                    }
                })
            })
            .ok_or(())?;

        let end = tiles
            .last()
            .and_then(|row| {
                row.iter().enumerate().find_map(|(x, tile)| {
                    if *tile == Tile::Path {
                        Some((tiles.len() - 1, x))
                    } else {
                        None
                    }
                })
            })
            .ok_or(())?;

        let height = tiles.len();
        let width = tiles[0].len();

        Ok(Self {
            tiles,
            start,
            end,
            height,
            width,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pos {
    y: isize,
    x: isize,
}

impl HikingMap {
    fn successors_with_crampons(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut successors = vec![];

        if pos.0 > 0 && self.tiles[pos.0 - 1][pos.1] != Tile::Forest {
            successors.push((pos.0 - 1, pos.1));
        }

        if pos.0 < self.height - 1 && self.tiles[pos.0 + 1][pos.1] != Tile::Forest {
            successors.push((pos.0 + 1, pos.1));
        }

        if pos.1 > 0 && self.tiles[pos.0][pos.1 - 1] != Tile::Forest {
            successors.push((pos.0, pos.1 - 1));
        }

        if pos.1 < self.width - 1 && self.tiles[pos.0][pos.1 + 1] != Tile::Forest {
            successors.push((pos.0, pos.1 + 1));
        }

        successors
    }

    fn successors(&self, pos: &(usize, usize)) -> Vec<(usize, usize)> {
        let mut successors = vec![];

        if let Tile::Slope(direction) = self.tiles[pos.0][pos.1] {
            match direction {
                Direction::Up => {
                    successors.push((pos.0 - 1, pos.1));
                }
                Direction::Down => {
                    successors.push((pos.0 + 1, pos.1));
                }
                Direction::Left => {
                    successors.push((pos.0, pos.1 - 1));
                }
                Direction::Right => {
                    successors.push((pos.0, pos.1 + 1));
                }
            }

            return successors;
        }

        self.successors_with_crampons(pos)
    }

    fn longest_hike(&self) -> usize {
        let paths = yen(
            &self.start,
            |pos| {
                self.successors(pos)
                    .iter()
                    .map(|pos| (*pos, 1))
                    .collect::<Vec<_>>()
            },
            |pos| *pos == self.end,
            1000,
        );
        let (max_path, _) = paths
            .iter()
            .max_by_key(|(path, _)| path.len())
            .expect("no paths found");

        max_path.len() - 1
    }

    fn longest_hike_with_crampons(&self) -> usize {
        let paths = yen(
            &self.start,
            |pos| {
                self.successors_with_crampons(pos)
                    .iter()
                    .map(|pos| (*pos, 1))
                    .collect::<Vec<_>>()
            },
            |pos| *pos == self.end,
            10000,
        );
        let (max_path, _) = paths
            .iter()
            .max_by_key(|(path, _)| path.len())
            .expect("no paths found");

        max_path.len() - 1
    }
}

#[aoc_generator(day23)]
fn parse_input(input: &str) -> HikingMap {
    input.parse().unwrap()
}

#[aoc(day23, part1)]
fn part1(hiking_map: &HikingMap) -> usize {
    hiking_map.longest_hike()
}

#[aoc(day23, part2)]
fn part2(hiking_map: &HikingMap) -> usize {
    hiking_map.longest_hike_with_crampons()
}
