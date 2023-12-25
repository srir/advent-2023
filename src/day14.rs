use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Rounded,
    Cube,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Platform {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Option<Rock>>>,
}

impl Platform {
    fn total_load(&self) -> usize {
        let mut total = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Rock::Rounded) = self.tiles[y][x] {
                    total += self.height - y;
                }
            }
        }

        total
    }

    fn tilt_north(&mut self) -> bool {
        let mut moved = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Rock::Rounded) = self.tiles[y][x] {
                    if y > 0 && self.tiles[y - 1][x].is_none() {
                        moved = true;

                        self.tiles[y][x] = None;
                        self.tiles[y - 1][x] = Some(Rock::Rounded);
                    }
                }
            }
        }

        moved && self.tilt_north()
    }

    fn tilt_east(&mut self) -> bool {
        let mut moved = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Rock::Rounded) = self.tiles[y][x] {
                    if x < self.width - 1 && self.tiles[y][x + 1].is_none() {
                        moved = true;

                        self.tiles[y][x] = None;
                        self.tiles[y][x + 1] = Some(Rock::Rounded);
                    }
                }
            }
        }

        moved && self.tilt_east()
    }

    fn tilt_south(&mut self) -> bool {
        let mut moved = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Rock::Rounded) = self.tiles[y][x] {
                    if y < self.height - 1 && self.tiles[y + 1][x].is_none() {
                        moved = true;

                        self.tiles[y][x] = None;
                        self.tiles[y + 1][x] = Some(Rock::Rounded);
                    }
                }
            }
        }

        moved && self.tilt_south()
    }

    fn tilt_west(&mut self) -> bool {
        let mut moved = false;

        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(Rock::Rounded) = self.tiles[y][x] {
                    if x > 0 && self.tiles[y][x - 1].is_none() {
                        moved = true;

                        self.tiles[y][x] = None;
                        self.tiles[y][x - 1] = Some(Rock::Rounded);
                    }
                }
            }
        }

        moved && self.tilt_west()
    }

    fn n_cycles(&mut self, n: usize) {
        for i in 0..n {
            if i % 1000 == 0 {
                println!("Cycle {}", i);
                self.print();
            }

            self.tilt_north();
            self.tilt_west();
            self.tilt_south();
            self.tilt_east();

            println!("Total load: {}, cycle {}", self.total_load(), i);
        }
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles[y][x] {
                    Some(Rock::Rounded) => print!("O"),
                    Some(Rock::Cube) => print!("#"),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rows = Vec::new();

        for line in s.lines() {
            let mut tiles = Vec::new();
            for c in line.chars() {
                match c {
                    '.' => tiles.push(None),
                    'O' => tiles.push(Some(Rock::Rounded)),
                    '#' => tiles.push(Some(Rock::Cube)),
                    _ => panic!("Invalid character: {}", c),
                }
            }

            rows.push(tiles);
        }

        Ok(Platform {
            width: rows[0].len(),
            height: rows.len(),
            tiles: rows,
        })
    }
}

#[aoc_generator(day14)]
fn parse_input(input: &str) -> Platform {
    input.parse().unwrap()
}

#[aoc(day14, part1)]
fn part1(platform: &Platform) -> usize {
    let mut platform = platform.clone();

    platform.tilt_north();

    platform.total_load()
}

#[aoc(day14, part2)]
fn part2(platform: &Platform) -> usize {
    let mut platform = platform.clone();

    platform.n_cycles(1000000000);

    platform.print();

    platform.total_load()
}
