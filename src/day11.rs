use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone)]
struct StarMap {
    tiles: Vec<Vec<Tile>>,
}

impl StarMap {
    fn rows_to_expand(&self) -> Vec<usize> {
        let mut rows = Vec::new();
        for (y, row) in self.tiles.iter().enumerate() {
            if row.iter().all(|tile| *tile == Tile::Empty) {
                rows.push(y);
            }
        }
        rows
    }

    fn cols_to_expand(&self) -> Vec<usize> {
        let mut cols = Vec::new();

        for x in 0..self.tiles[0].len() {
            let mut all_empty = true;
            for y in 0..self.tiles.len() {
                if self.tiles[y][x] != Tile::Empty {
                    all_empty = false;
                    break;
                }
            }

            if all_empty {
                cols.push(x);
            }
        }

        cols
    }

    fn expand(&mut self) {
        let rows = self.rows_to_expand();
        let cols = self.cols_to_expand();

        println!("rows: {:?}", rows);
        println!("cols: {:?}", cols);

        let mut new_tiles = Vec::new();

        for (y, row) in self.tiles.iter().enumerate() {
            let mut new_row = Vec::new();

            for (x, tile) in row.iter().enumerate() {
                if cols.contains(&x) {
                    new_row.push(*tile);
                }

                new_row.push(*tile);
            }

            if rows.contains(&y) {
                new_tiles.push(new_row.clone());
            }

            new_tiles.push(new_row);
        }

        self.tiles = new_tiles;
    }

    fn find_galaxies(&self) -> Vec<(usize, usize)> {
        let mut galaxies = Vec::new();

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Galaxy {
                    galaxies.push((y, x));
                }
            }
        }

        galaxies
    }

    fn shortest_path((y1, x1): (usize, usize), (y2, x2): (usize, usize)) -> usize {
        (y1 as isize - y2 as isize).unsigned_abs() + (x1 as isize - x2 as isize).unsigned_abs()
    }

    fn shortest_paths_between_galaxies(&self) -> usize {
        let galaxies = self.find_galaxies();

        galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let (y1, x1) = pair[0];
                let (y2, x2) = pair[1];

                Self::shortest_path((*y1, *x1), (*y2, *x2))
            })
            .sum()
    }

    fn shortest_paths_with_expansion(&self, multiple: usize) -> usize {
        let rows_to_expand = self.rows_to_expand();
        let cols_to_expand = self.cols_to_expand();

        let galaxies = self.find_galaxies();

        galaxies
            .iter()
            .combinations(2)
            .map(|pair| {
                let (y1, x1) = pair[0];
                let (y2, x2) = pair[1];

                let expanded_rows = rows_to_expand
                    .iter()
                    .filter(|r| **r < *y1 && **r > *y2 || **r < *y2 && **r > *y1)
                    .collect::<HashSet<_>>();
                let expanded_cols = cols_to_expand
                    .iter()
                    .filter(|c| **c < *x1 && **c > *x2 || **c < *x2 && **c > *x1)
                    .collect::<HashSet<_>>();

                let mut shortest_path = Self::shortest_path((*y1, *x1), (*y2, *x2));

                if !expanded_rows.is_empty() {
                    shortest_path += expanded_rows.len() * (multiple - 1);
                }

                if !expanded_cols.is_empty() {
                    shortest_path += expanded_cols.len() * (multiple - 1);
                }

                shortest_path
            })
            .sum()
    }
}

impl FromStr for StarMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Galaxy,
                        _ => panic!("Invalid tile"),
                    })
                    .collect()
            })
            .collect();

        Ok(StarMap { tiles })
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> StarMap {
    input.parse().unwrap()
}

#[aoc(day11, part1)]
fn part1(input: &StarMap) -> usize {
    let mut star_map = input.clone();

    star_map.expand();

    star_map.shortest_paths_between_galaxies()
}

#[aoc(day11, part2)]
fn part2(star_map: &StarMap) -> usize {
    star_map.shortest_paths_with_expansion(1000000)
}
