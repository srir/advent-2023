use aoc_runner_derive::{aoc, aoc_generator};
use nalgebra::{Point2, Vector2, Vector3};
use std::str::FromStr;

struct TestArea {
    x_min: f64,
    y_min: f64,
    x_max: f64,
    y_max: f64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    position: Vector3<f64>,
    velocity: Vector3<f64>,
}

impl Hailstone {
    fn will_intersect_2d(&self, other: &Hailstone, test_area: &TestArea) -> bool {
        // Calculate time of intersection
        let p1 = Point2::new(self.position.x, self.position.y);
        let p2 = Point2::new(other.position.x, other.position.y);
        let v1 = Vector2::new(self.velocity.x, self.velocity.y);
        let v2 = Vector2::new(other.velocity.x, other.velocity.y);

        let dx = p2.x - p1.x;
        let dy = p2.y - p1.y;
        let det = v2.x * v1.y - v2.y * v1.x;

        if det == 0.0 {
            return false;
        }

        let u = (dy * v2.x - dx * v2.y) / det;
        let v = (dy * v1.x - dx * v1.y) / det;

        // Check if the positions at time t are within the test area
        if u >= 0.0 && v >= 0.0 {
            let intersection_point = p1 + (v1 * u);
            if test_area.x_min <= intersection_point.x
                && intersection_point.x <= test_area.x_max
                && test_area.y_min <= intersection_point.y
                && intersection_point.y <= test_area.y_max
            {
                return true;
            }
        }

        false
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Hailstorm {
    hailstones: Vec<Hailstone>,
}

impl FromStr for Hailstorm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hailstones = s
            .lines()
            .map(|line| {
                let (pos, vel) = line.split_once("@").unwrap();
                let mut parts = pos.trim().split(", ");
                let x = parts.next().unwrap().trim().parse().unwrap();
                let y = parts.next().unwrap().trim().parse().unwrap();
                let z = parts.next().unwrap().trim().parse().unwrap();
                let position = Vector3::new(x, y, z);
                let mut parts = vel.trim().split(", ");
                let x = parts.next().unwrap().trim().parse().unwrap();
                let y = parts.next().unwrap().trim().parse().unwrap();
                let z = parts.next().unwrap().trim().parse().unwrap();
                let velocity = Vector3::new(x, y, z);
                Hailstone { position, velocity }
            })
            .collect();

        Ok(Self { hailstones })
    }
}

impl Hailstorm {
    fn count_intersections(&self, test_area: &TestArea) -> usize {
        let mut count = 0;

        for i in 0..self.hailstones.len() {
            for j in i + 1..self.hailstones.len() {
                if self.hailstones[i].will_intersect_2d(&self.hailstones[j], test_area) {
                    count += 1;
                }
            }
        }

        count
    }
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Hailstorm {
    input.parse().unwrap()
}

#[aoc(day24, part1)]
fn part1(storm: &Hailstorm) -> usize {
    storm.count_intersections(&TestArea {
        x_min: 200000000000000.0,
        y_min: 200000000000000.0,
        x_max: 400000000000000.0,
        y_max: 400000000000000.0,
    })
}
