use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use crate::util::parse_numbers;

#[derive(Debug, Clone)]
struct SeedRange {
    dest_range_start: usize,
    src_range_start: usize,
    range_length: usize
}

impl SeedRange {
    fn map(&self, src: usize) -> Option<usize> {
        if src < self.src_range_start || src >= self.src_range_start + self.range_length {
            return None;
        }

        let offset = src - self.src_range_start;
        let dest = self.dest_range_start + offset;

        Some(dest)
    }
}

impl FromStr for SeedRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers = parse_numbers(s)?;
        if numbers.len() != 3 {
            return Err(());
        }

        Ok(SeedRange {
            dest_range_start: numbers[0],
            src_range_start: numbers[1],
            range_length: numbers[2]
        })
    }
}

#[derive(Debug, Clone)]
struct SeedMap {
    source_category: String,
    dest_category: String,
    ranges: Vec<SeedRange>,
}

impl SeedMap {
    fn map(&self, src: usize) -> usize {
        for range in &self.ranges {
            if let Some(dest) = range.map(src) {
                return dest;
            }
        }

        src
    }
}

impl FromStr for SeedMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let mapping_str = lines[0].strip_suffix(" map:").ok_or(())?;
        let (source_category, dest_category) = mapping_str.split_once("-to-").ok_or(())?;

        let ranges = lines[1..].iter().map(|l| l.parse()).collect::<Result<_, _>>()?;

        Ok(SeedMap {
            source_category: source_category.to_string(),
            dest_category: dest_category.to_string(),
            ranges
        })
    }
}

#[derive(Debug, Clone)]
struct SeedMapSet {
    seeds: Vec<usize>,
    maps: HashMap<String, SeedMap>,

    _dest_cache: HashMap<(String, usize), usize>,
    _location_cache: HashMap<usize, usize>
}

impl SeedMapSet {
    fn new(seeds: Vec<usize>, maps: HashMap<String, SeedMap>) -> Self {
        SeedMapSet {
            seeds,
            maps,
            _dest_cache: HashMap::new(),
            _location_cache: HashMap::new()
        }
    }

    fn map(&mut self, src_category: &str, src: usize) -> (String, usize) {
        let map = self.maps.get(src_category).unwrap();

        if let Some(dest) = self._dest_cache.get(&(src_category.to_string(), src)) {
            return (map.dest_category.clone(), *dest);
        }

        let dest = map.map(src);
        self._dest_cache.insert((src_category.to_string(), src), dest);

        (map.dest_category.clone(), dest)
    }

    fn map_seed_to_location(&mut self, src: usize) -> usize {
        let mut dest = src;
        let mut category: String = "seed".to_string();

        if let Some(location) = self._location_cache.get(&src) {
            return *location;
        }

        while category != "location" {
            let (next_category, next_dest) = self.map(&category, dest);
            dest = next_dest;
            category = next_category;
        }

        self._location_cache.insert(src, dest);
        dest
    }

    fn map_seeds_to_locations(&mut self) -> Vec<usize> {
        let seeds = self.seeds.clone();

        seeds.iter().map(|s| self.map_seed_to_location(*s)).collect()
    }

    fn lowest_location(&mut self) -> usize {
        *self.map_seeds_to_locations().iter().min().unwrap()
    }

    fn lowest_location_from_ranges(&mut self) -> usize {
        let ranges = self.seeds.clone();
        let mut min = None;

        for range in ranges.chunks(2) {
            let start = range[0];
            let length = range[1];

            for i in start..(start + length) {
                let location = self.map_seed_to_location(i);
                if min.is_none() || location < min.unwrap() {
                    min = Some(location);
                }
            }
        }

        min.unwrap()
    }
}

impl FromStr for SeedMapSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds_str, maps_str) = s.split_once("\n\n").ok_or(())?;
        let (_, seeds_seq) = seeds_str.split_once(":").ok_or(())?;
        let seeds = parse_numbers(seeds_seq)?;

        let map_sections = maps_str.split("\n\n").collect::<Vec<_>>();

        let mut maps = HashMap::new();
        for map_section in map_sections {
            let map = map_section.parse::<SeedMap>()?;
            maps.insert(map.source_category.clone(), map);
        }

        Ok(SeedMapSet::new(seeds, maps))
    }
}

#[aoc_generator(day5)]
fn parse_maps(input: &str) -> SeedMapSet {
    input.parse().unwrap()
}

#[aoc(day5, part1)]
fn part1(input: &SeedMapSet) -> usize {
    let mut mapset = input.clone();
    mapset.lowest_location()
}

#[aoc(day5, part2)]
fn part2(input: &SeedMapSet) -> usize {
    let mut mapset = input.clone();
    mapset.lowest_location_from_ranges()
}