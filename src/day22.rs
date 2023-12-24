use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

struct Brick {
    start_x: usize,
    start_y: usize,
    start_z: usize,
    end_x: usize,
    end_y: usize,
    end_z: usize,
}

struct BrickStack {
    bricks: Vec<Brick>,
}

impl FromStr for BrickStack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks = s
            .lines()
            .map(|line| {
                let (start_coords, end_coords) = line.split_once("~").unwrap();

                let mut parts = start_coords.split(",");
                let start_x = parts.next().unwrap().parse().unwrap();
                let start_y = parts.next().unwrap().parse().unwrap();
                let start_z = parts.next().unwrap().parse().unwrap();

                let mut parts = end_coords.split(",");
                let end_x = parts.next().unwrap().parse().unwrap();
                let end_y = parts.next().unwrap().parse().unwrap();
                let end_z = parts.next().unwrap().parse().unwrap();

                Brick {
                    start_x,
                    start_y,
                    start_z,
                    end_x,
                    end_y,
                    end_z,
                }
            })
            .collect();

        Ok(Self { bricks })
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> BrickStack {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
fn part1(_input: &BrickStack) -> usize {
    0
}
