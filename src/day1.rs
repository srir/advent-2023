use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|s| s.to_string()).collect()
}

#[derive(Debug)]
struct CalibrationValue {
    first: u32,
    last: u32,
}

fn word_to_number(word: &str) -> Option<u32> {
    match word {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

impl CalibrationValue {
    fn from_string(s: String) -> Self {
        let mut first = None;
        let mut last = None;

        for c in s.chars() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c.to_digit(10).unwrap());
                }

                last = Some(c.to_digit(10).unwrap());
            }
        }

        Self {
            first: first.unwrap(),
            last: last.unwrap(),
        }
    }

    fn from_string_with_words(s: String) -> Self {
        let re_start = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();
        let re_end = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)$").unwrap();

        let mut first_numeric = None;
        let mut first_numeric_loc = None;
        let mut last_numeric = None;
        let mut last_numeric_loc = None;

        let mut last_word_match = None;

        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                if first_numeric.is_none() {
                    first_numeric = Some(c.to_digit(10).unwrap());
                    first_numeric_loc = Some(i);
                }

                last_numeric = Some(c.to_digit(10).unwrap());
                last_numeric_loc = Some(i);
            }

            if c.is_alphabetic() {
                let current_word = &s[..i + 1];

                if let Some(end_match) = re_end.find(&current_word) {
                    last_word_match = Some(end_match);
                }
            }
        }

        if let Some(last_word_match) = last_word_match.clone() {
            if last_numeric_loc.is_none() || last_word_match.end() > last_numeric_loc.unwrap() {
                last_numeric = word_to_number(last_word_match.as_str());
            }
        }

        if let Some(first_word_match) = re_start.find(&s) {
            if first_numeric_loc.is_none() || first_word_match.start() < first_numeric_loc.unwrap()
            {
                first_numeric = word_to_number(first_word_match.as_str());
            }
        }

        Self {
            first: first_numeric.unwrap(),
            last: last_numeric.unwrap(),
        }
    }

    fn get(&self) -> u32 {
        (self.first * 10) + self.last
    }
}

#[aoc(day1, part1)]
pub fn part1(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| {
            let value = CalibrationValue::from_string(s.to_string());
            value.get()
        })
        .sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &[String]) -> u32 {
    input
        .iter()
        .map(|s| {
            let value = CalibrationValue::from_string_with_words(s.to_string());
            value.get()
        })
        .sum()
}
