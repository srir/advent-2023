use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;
use crate::util::Direction;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    VSplit,
    HSplit,
    TLBRMirror,
    BLTRMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Beam {
    direction: Direction,
    position: (isize, isize),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grid {
    tiles: Vec<Vec<Tile>>,
    energized_tiles: Vec<Vec<bool>>,
}

impl Grid {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let energized_tiles = vec![vec![false; tiles[0].len()]; tiles.len()];

        Grid {
            tiles,
            energized_tiles,
        }
    }

    fn step(&mut self, beams: &[Beam]) -> Vec<Beam> {
        let mut new_beams = Vec::<Beam>::new();

        for beam in beams {
            let (y, x) = beam.position;

            match beam.direction {
                Direction::Up => {
                    let position = ((y - 1) as isize, x as isize);
                    if y > 0 {
                        self.energized_tiles[y as usize - 1][x as usize] = true;
                        match self.tiles[y as usize - 1][x as usize] {
                            Tile::Empty | Tile::VSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Up,
                                    position,
                                });
                            }
                            Tile::HSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Left,
                                    position,
                                });
                                new_beams.push(Beam {
                                    direction: Direction::Right,
                                    position,
                                });
                            }
                            Tile::TLBRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Left,
                                    position,
                                });
                            }
                            Tile::BLTRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Right,
                                    position,
                                });
                            }
                        }
                    }
                }
                Direction::Down => {
                    let position = (y + 1, x);
                    if y < self.tiles.len() as isize - 1 {
                        self.energized_tiles[y as usize + 1][x as usize] = true;
                        match self.tiles[y as usize + 1][x as usize] {
                            Tile::Empty | Tile::VSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Down,
                                    position,
                                });
                            }
                            Tile::HSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Left,
                                    position,
                                });
                                new_beams.push(Beam {
                                    direction: Direction::Right,
                                    position,
                                });
                            }
                            Tile::TLBRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Right,
                                    position,
                                });
                            }
                            Tile::BLTRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Left,
                                    position,
                                });
                            }
                        }
                    }
                }
                Direction::Left => {
                    let position = (y, x - 1);
                    if x > 0 {
                        self.energized_tiles[y as usize][x as usize - 1] = true;
                        match self.tiles[y as usize][x as usize - 1] {
                            Tile::Empty | Tile::HSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Left,
                                    position,
                                });
                            }
                            Tile::VSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Up,
                                    position,
                                });
                                new_beams.push(Beam {
                                    direction: Direction::Down,
                                    position,
                                });
                            }
                            Tile::TLBRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Up,
                                    position,
                                });
                            }
                            Tile::BLTRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Down,
                                    position,
                                });
                            }
                        }
                    }
                }
                Direction::Right => {
                    let position = (y, x + 1);
                    if x < self.tiles[0].len() as isize - 1 {
                        self.energized_tiles[y as usize][x as usize + 1] = true;
                        match self.tiles[y as usize][x as usize + 1] {
                            Tile::Empty | Tile::HSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Right,
                                    position,
                                });
                            }
                            Tile::VSplit => {
                                new_beams.push(Beam {
                                    direction: Direction::Up,
                                    position,
                                });
                                new_beams.push(Beam {
                                    direction: Direction::Down,
                                    position,
                                });
                            }
                            Tile::TLBRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Down,
                                    position,
                                });
                            }
                            Tile::BLTRMirror => {
                                new_beams.push(Beam {
                                    direction: Direction::Up,
                                    position,
                                });
                            }
                        }
                    }
                }
            }
        }

        new_beams
    }

    fn energize_with_initial_beam(&mut self, initial_beam: &Beam) {
        let mut beams = vec![initial_beam.clone()];
        let mut last_energized_count = 0;
        let mut steps_since_last_change = 0;
        let total_tiles = self.tiles.len() * self.tiles[0].len();

        while !beams.is_empty() && steps_since_last_change < total_tiles && beams.len() < 1000000 {
            beams = self.step(&beams);

            if self.num_energized_tiles() == last_energized_count {
                steps_since_last_change += 1;
            } else {
                last_energized_count = self.num_energized_tiles();
                steps_since_last_change = 0;
            }
        }
    }

    fn energize(&mut self) {
        let initial_beam = Beam {
            direction: Direction::Right,
            position: (0, -1),
        };

        self.energize_with_initial_beam(&initial_beam);
    }

    fn num_energized_tiles(&self) -> usize {
        self.energized_tiles
            .iter()
            .map(|row| row.iter().filter(|tile| **tile).count())
            .sum()
    }

    fn max_energy_configuration(&self) -> usize {
        let top_coords = (0..self.tiles.len() as isize)
            .map(|y| (y, -1))
            .collect::<Vec<(isize, isize)>>();

        let bot_coords = (0..self.tiles.len() as isize)
            .map(|y| (y, self.tiles[0].len() as isize))
            .collect::<Vec<(isize, isize)>>();

        let left_coords = (0..self.tiles[0].len() as isize)
            .map(|x| (-1, x))
            .collect::<Vec<(isize, isize)>>();

        let right_coords = (0..self.tiles[0].len() as isize)
            .map(|x| (self.tiles.len() as isize, x))
            .collect::<Vec<(isize, isize)>>();

        let all_start_coords = top_coords
            .iter()
            .chain(bot_coords.iter())
            .chain(left_coords.iter())
            .chain(right_coords.iter())
            .collect::<Vec<_>>();

        let all_initial_beams = all_start_coords
            .iter()
            .map(|(y, x)| {
                if y == &-1 {
                    Beam {
                        direction: Direction::Down,
                        position: (*y, *x),
                    }
                } else if y == &(self.tiles.len() as isize) {
                    Beam {
                        direction: Direction::Up,
                        position: (*y, *x),
                    }
                } else if x == &-1 {
                    Beam {
                        direction: Direction::Right,
                        position: (*y, *x),
                    }
                } else {
                    Beam {
                        direction: Direction::Left,
                        position: (*y, *x),
                    }
                }
            })
            .collect::<Vec<_>>();

        all_initial_beams
            .iter()
            .map(|beam| {
                let mut grid = self.clone();
                grid.energize_with_initial_beam(beam);
                grid.num_energized_tiles()
            })
            .max()
            .unwrap()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();

            for c in line.chars() {
                match c {
                    '.' => row.push(Tile::Empty),
                    '|' => row.push(Tile::VSplit),
                    '-' => row.push(Tile::HSplit),
                    '\\' => row.push(Tile::TLBRMirror),
                    '/' => row.push(Tile::BLTRMirror),
                    _ => panic!("Unknown tile: {}", c),
                }
            }

            tiles.push(row);
        }

        Ok(Grid::new(tiles))
    }
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Grid {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
fn part1(grid: &Grid) -> usize {
    let mut grid = grid.clone();

    grid.energize();

    grid.num_energized_tiles()
}

#[aoc(day16, part2)]
fn part2(grid: &Grid) -> usize {
    grid.max_energy_configuration()
}
