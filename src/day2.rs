use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
pub struct Hand {
    red: u32,
    green: u32,
    blue: u32,
}

impl Hand {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

lazy_static! {
    static ref RE_RED: Regex = Regex::new(r"(\d+) red").unwrap();
    static ref RE_GREEN: Regex = Regex::new(r"(\d+) green").unwrap();
    static ref RE_BLUE: Regex = Regex::new(r"(\d+) blue").unwrap();
}

fn re_to_num(re: &Regex, s: &str) -> Option<u32> {
    if let Some(caps) = re.captures(s) {
        caps.get(1).unwrap().as_str().parse::<u32>().ok()
    } else {
        None
    }
}

impl FromStr for Hand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hand {
            red: re_to_num(&RE_RED, s).unwrap_or(0),
            green: re_to_num(&RE_GREEN, s).unwrap_or(0),
            blue: re_to_num(&RE_BLUE, s).unwrap_or(0),
        })
    }
}

#[derive(Debug)]
pub struct Game {
    id: u32,
    hands: Vec<Hand>,
}

impl Game {
    fn is_valid(&self) -> bool {
        self.hands.iter().all(|h| h.is_valid())
    }

    fn minimum_hand(&self) -> Hand {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;

        for hand in &self.hands {
            if hand.red > min_red {
                min_red = hand.red;
            }

            if hand.green > min_green {
                min_green = hand.green;
            }

            if hand.blue > min_blue {
                min_blue = hand.blue;
            }
        }

        Hand {
            red: min_red,
            green: min_green,
            blue: min_blue,
        }
    }
}

impl FromStr for Game {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game_w_id, rest) = s.split_once(":").unwrap();
        let id = game_w_id.split_once(" ").unwrap().1.parse::<u32>().unwrap();

        let hands = rest.split(";");
        Ok(Game {
            id,
            hands: hands.map(|h| h.parse::<_>().unwrap()).collect::<Vec<_>>(),
        })
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|s| s.parse::<Game>().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter_map(|g| g.is_valid().then_some(g.id))
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(games: &[Game]) -> u32 {
    games.iter().map(|g| g.minimum_hand().power()).sum()
}
