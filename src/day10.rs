use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Tile {
    Horizontal,
    Vertical,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    None,
    StartPosition,
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum TileType {
    Inside,
    Outside,
    Pipe,
}

impl Map {
    fn get(&self, y: usize, x: usize) -> Option<&Tile> {
        self.tiles.get(y).and_then(|row| row.get(x))
    }

    fn start_position(&self) -> (usize, usize) {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::StartPosition {
                    return (y, x);
                }
            }
        }

        panic!("No start position found");
    }

    fn start_directions(&self) -> (Direction, Direction) {
        let (y, x) = self.start_position();
        let mut directions = Vec::new();

        if y > 0 {
            match self.get(y - 1, x) {
                Some(Tile::Vertical) => {
                    directions.push(Direction::Up);
                }
                Some(Tile::SouthToEast) => {
                    directions.push(Direction::Up);
                }
                Some(Tile::SouthToWest) => {
                    directions.push(Direction::Up);
                }
                _ => {}
            }
        }

        if y < self.tiles.len() - 1 {
            match self.get(y + 1, x) {
                Some(Tile::Vertical) => {
                    directions.push(Direction::Down);
                }
                Some(Tile::NorthToEast) => {
                    directions.push(Direction::Down);
                }
                Some(Tile::NorthToWest) => {
                    directions.push(Direction::Down);
                }
                _ => {}
            }
        }

        if x > 0 {
            match self.get(y, x - 1) {
                Some(Tile::Horizontal) => {
                    directions.push(Direction::Left);
                }
                Some(Tile::NorthToEast) => {
                    directions.push(Direction::Left);
                }
                Some(Tile::SouthToEast) => {
                    directions.push(Direction::Left);
                }
                _ => {}
            }
        }

        if x < self.tiles[y].len() - 1 {
            match self.get(y, x + 1) {
                Some(Tile::Horizontal) => {
                    directions.push(Direction::Right);
                }
                Some(Tile::NorthToWest) => {
                    directions.push(Direction::Right);
                }
                Some(Tile::SouthToWest) => {
                    directions.push(Direction::Right);
                }
                _ => {}
            }
        }

        (directions[0], directions[1])
    }

    fn traversal_from_start<F>(&self, initial_direction: &Direction, mut callback: F) -> usize
    where
        F: FnMut(usize, usize),
    {
        let (mut y, mut x) = self.start_position();

        callback(y, x);

        let mut distance = 0usize;
        let mut direction = *initial_direction;

        loop {
            match direction {
                Direction::Left => {
                    x -= 1;
                }
                Direction::Right => {
                    x += 1;
                }
                Direction::Up => {
                    y -= 1;
                }
                Direction::Down => {
                    y += 1;
                }
            }

            distance += 1;
            callback(y, x);

            match (self.get(y, x), direction) {
                (Some(Tile::Horizontal), _) => {}
                (Some(Tile::Vertical), _) => {}
                (Some(Tile::NorthToEast), Direction::Down) => {
                    direction = Direction::Right;
                }
                (Some(Tile::NorthToEast), Direction::Left) => {
                    direction = Direction::Up;
                }
                (Some(Tile::NorthToWest), Direction::Down) => {
                    direction = Direction::Left;
                }
                (Some(Tile::NorthToWest), Direction::Right) => {
                    direction = Direction::Up;
                }
                (Some(Tile::SouthToWest), Direction::Right) => {
                    direction = Direction::Down;
                }
                (Some(Tile::SouthToWest), Direction::Up) => {
                    direction = Direction::Left;
                }
                (Some(Tile::SouthToEast), Direction::Left) => {
                    direction = Direction::Down;
                }
                (Some(Tile::SouthToEast), Direction::Up) => {
                    direction = Direction::Right;
                }
                (Some(Tile::StartPosition), _) => {
                    break;
                }
                _ => {
                    panic!(
                        "Unexpectedly found at ({}, {}): {:?}, {:?}",
                        y,
                        x,
                        self.get(y, x),
                        direction
                    );
                }
            }
        }

        distance
    }

    fn max_distance_from_start(&self) -> usize {
        let (left, _) = self.start_directions();
        let left_distance = self.traversal_from_start(&left, |_, _| ());

        left_distance / 2
    }

    fn count_enclosed_tiles(&self) -> usize {
        let (left, _) = self.start_directions();
        let mut tilemap = vec![vec![None; self.tiles[0].len()]; self.tiles.len()];

        self.traversal_from_start(&left, |y, x| {
            tilemap[y][x] = Some(TileType::Pipe);
        });

        0
    }
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles = Vec::new();

        for line in s.lines() {
            let mut row = Vec::new();

            for c in line.chars() {
                let tile = match c {
                    '-' => Tile::Horizontal,
                    '|' => Tile::Vertical,
                    'L' => Tile::NorthToEast,
                    'J' => Tile::NorthToWest,
                    '7' => Tile::SouthToWest,
                    'F' => Tile::SouthToEast,
                    '.' => Tile::None,
                    'S' => Tile::StartPosition,
                    _ => Tile::None,
                };

                row.push(tile);
            }

            tiles.push(row);
        }

        Ok(Map { tiles })
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Map {
    input.parse().unwrap()
}

#[aoc(day10, part1)]
fn part1(map: &Map) -> usize {
    map.max_distance_from_start()
}

#[aoc(day10, part2)]
fn part2(map: &Map) -> usize {
    map.count_enclosed_tiles()
}
