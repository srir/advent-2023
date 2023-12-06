use crate::util::parse_numbers;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct SeedRange {
    dest_range_start: usize,
    src_range_start: usize,
    range_length: usize,
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

    fn map_rev(&self, dest: usize) -> Option<usize> {
        if dest < self.dest_range_start || dest >= self.dest_range_start + self.range_length {
            return None;
        }

        let offset = dest - self.dest_range_start;
        let src = self.src_range_start + offset;

        Some(src)
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
            range_length: numbers[2],
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

    fn map_rev(&self, dest: usize) -> usize {
        for range in &self.ranges {
            if let Some(src) = range.map_rev(dest) {
                return src;
            }
        }

        dest
    }
}

impl FromStr for SeedMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<_>>();

        let mapping_str = lines[0].strip_suffix(" map:").ok_or(())?;
        let (source_category, dest_category) = mapping_str.split_once("-to-").ok_or(())?;

        let ranges = lines[1..]
            .iter()
            .map(|l| l.parse())
            .collect::<Result<_, _>>()?;

        Ok(SeedMap {
            source_category: source_category.to_string(),
            dest_category: dest_category.to_string(),
            ranges,
        })
    }
}

#[derive(Debug, Clone)]
struct SeedMapSet {
    seeds: Vec<usize>,
    maps_by_src: HashMap<String, SeedMap>,
    maps_by_dest: HashMap<String, SeedMap>,

    _dest_cache: HashMap<(String, usize), usize>,
    _location_cache: HashMap<usize, usize>,
}

impl SeedMapSet {
    fn new(seeds: Vec<usize>, maps: HashMap<String, SeedMap>) -> Self {
        let maps_by_dest = maps
            .clone()
            .into_iter()
            .map(|(_k, v)| (v.dest_category.clone(), v))
            .collect::<HashMap<_, _>>();

        SeedMapSet {
            seeds,
            maps_by_src: maps,
            maps_by_dest,
            _dest_cache: HashMap::new(),
            _location_cache: HashMap::new(),
        }
    }

    fn map(&self, src_category: &str, src: usize) -> (String, usize) {
        let map = self.maps_by_src.get(src_category).unwrap();

        let dest = map.map(src);

        (map.dest_category.clone(), dest)
    }

    fn map_rev(&self, dest_category: &str, dest: usize) -> (String, usize) {
        let map = self.maps_by_dest.get(dest_category).unwrap();
        let src = map.map_rev(dest);

        (map.source_category.clone(), src)
    }

    fn map_seed_to_location(&self, src: usize) -> usize {
        let mut dest = src;
        let mut category: String = "seed".to_string();

        while category != "location" {
            let (next_category, next_dest) = self.map(&category, dest);
            dest = next_dest;
            category = next_category;
        }

        dest
    }

    fn map_seeds_to_locations(&self) -> Vec<usize> {
        self.seeds
            .iter()
            .map(|s| self.map_seed_to_location(*s))
            .collect()
    }

    fn lowest_location(&self) -> usize {
        self.map_seeds_to_locations().iter().min().unwrap().clone()
    }

    fn map_location_to_seed(&self, dest: usize) -> usize {
        let mut src = dest;
        let mut category: String = "location".to_string();

        while category != "seed" {
            let (next_category, next_src) = self.map_rev(&category, src);
            src = next_src;
            category = next_category;
        }

        src
    }

    fn lowest_location_from_ranges(&self) -> usize {
        let mut i = 0usize;
        let mut ranges: Vec<(usize, usize)> = Vec::new();

        for range in self.seeds.chunks(2) {
            let start = range[0];
            let length = range[1];

            ranges.push((start, length));
        }

        while !ranges
            .iter()
            .any(|r| range_contains(r, self.map_location_to_seed(i)))
        {
            i += 1;
        }

        i
    }
}

fn range_contains(range: &(usize, usize), n: usize) -> bool {
    let (start, length) = range;
    let end = start + length;

    n >= *start && n < end
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
fn part1(mapset: &SeedMapSet) -> usize {
    mapset.lowest_location()
}

#[aoc(day5, part2)]
fn part2(mapset: &SeedMapSet) -> usize {
    mapset.lowest_location_from_ranges()
}
