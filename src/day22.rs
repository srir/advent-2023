use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{Point2, Point3};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start_x: usize,
    start_y: usize,
    start_z: usize,
    end_x: usize,
    end_y: usize,
    end_z: usize,
    height: usize,
}

impl Brick {
    fn points(&self, at_height: usize) -> Vec<Point3<usize>> {
        let mut points = Vec::new();

        let height_offset = (at_height as isize) - (self.start_z as isize);

        for x in self.start_x..=self.end_x {
            for y in self.start_y..=self.end_y {
                for z in self.start_z..=self.end_z {
                    points.push(Point3::new(x, y, ((z as isize) + height_offset) as usize));
                }
            }
        }

        points
    }

    fn footprint(&self) -> Vec<Point2<usize>> {
        let mut points = Vec::new();

        for x in self.start_x..=self.end_x {
            for y in self.start_y..=self.end_y {
                points.push(Point2::new(x, y));
            }
        }

        points
    }
}

#[derive(Debug, Clone)]
struct BrickStack {
    bricks: Vec<Rc<Brick>>,
    space: HashMap<Point3<usize>, Rc<Brick>>,
    reverse_space: HashMap<Rc<Brick>, Vec<Point3<usize>>>,
    height_placements: HashMap<Rc<Brick>, usize>,
    heights: HashMap<Point2<usize>, usize>,
}

impl BrickStack {
    fn build_space(&mut self) {
        for brick in &self.bricks {
            let footprint = brick.footprint();

            let min_height = footprint
                .iter()
                .flat_map(|p| self.heights.get(p))
                .max()
                .unwrap_or(&0);

            let mut new_heights = HashMap::<Point2<usize>, usize>::new();

            let points = brick.points(min_height + 1);
            for point in &points {
                self.space.insert(*point, brick.clone());
                new_heights.insert(Point2::new(point.x, point.y), point.z);
            }

            self.reverse_space.insert(brick.clone(), points);
            self.height_placements.insert(brick.clone(), min_height + 1);
            self.heights.extend(new_heights);
        }
    }

    fn count_destructible_bricks(&self) -> usize {
        self.bricks
            .iter()
            .filter(|&brick| {
                let supported_bricks = self
                    .reverse_space
                    .get(brick)
                    .unwrap()
                    .iter()
                    .map(|p| Point3::new(p.x, p.y, p.z + 1))
                    .filter_map(|p| self.space.get(&p))
                    .filter(|b| *b != brick)
                    .collect::<HashSet<_>>();

                supported_bricks.iter().all(|&supported_brick| {
                    let points = self.reverse_space.get(supported_brick).unwrap();

                    let bricks_below = points
                        .iter()
                        .map(|p| Point3::new(p.x, p.y, p.z - 1))
                        .filter_map(|p| self.space.get(&p))
                        .filter(|b| *b != supported_brick && *b != brick)
                        .collect::<HashSet<_>>();

                    !bricks_below.is_empty()
                })
            })
            .count()
    }
}

impl FromStr for BrickStack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bricks = s
            .lines()
            .map(|line| {
                let (start_coords, end_coords) = line.split_once('~').unwrap();

                let mut parts = start_coords.split(',');
                let start_x = parts.next().unwrap().parse().unwrap();
                let start_y = parts.next().unwrap().parse().unwrap();
                let start_z = parts.next().unwrap().parse().unwrap();

                let mut parts = end_coords.split(',');
                let end_x = parts.next().unwrap().parse().unwrap();
                let end_y = parts.next().unwrap().parse().unwrap();
                let end_z = parts.next().unwrap().parse().unwrap();

                Rc::new(Brick {
                    start_x,
                    start_y,
                    start_z,
                    end_x,
                    end_y,
                    end_z,
                    height: end_z - start_z + 1,
                })
            })
            .collect();

        Ok(Self {
            bricks,
            space: HashMap::new(),
            heights: HashMap::new(),
            height_placements: HashMap::new(),
            reverse_space: HashMap::new(),
        })
    }
}

#[aoc_generator(day22)]
fn parse_input(input: &str) -> BrickStack {
    input.parse().unwrap()
}

#[aoc(day22, part1)]
fn part1(stack: &BrickStack) -> usize {
    let mut stack = stack.clone();
    stack.build_space();

    stack.count_destructible_bricks()
}
