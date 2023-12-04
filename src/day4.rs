use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug, Clone)]
struct Card {
    winning_numbers: Vec<usize>,
    card_numbers: Vec<usize>,
}

impl Card {
    fn score(&self) -> usize {
        let count_winning = self.count_winning();

        match count_winning {
            0 => 0,
            _ => 2usize.pow((count_winning - 1) as u32)
        }
    }

    fn count_winning(&self) -> usize {
        return self.winning_numbers.iter().filter(|n| self.card_numbers.contains(n)).count();
    }
}

lazy_static! {
    static ref NUM_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

impl FromStr for Card {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, rest) = s.split_once(":").ok_or(())?;
        let (winning, card) = rest.split_once("|").ok_or(())?;

        let winning_numbers = NUM_REGEX.find_iter(winning).map(|n| n.as_str().parse().unwrap()).collect();
        let card_numbers = NUM_REGEX.find_iter(card).map(|n| n.as_str().parse().unwrap()).collect();

        Ok(Card { winning_numbers, card_numbers })
    }
}

struct Game {
    cards: Vec<Card>,
}

impl Game {
    fn score(&self) -> usize {
        self.cards.iter().map(|c| c.score()).sum()
    }

    fn cascading_card_count(&self) -> usize {
        let mut instance_counts: HashMap<usize, usize> = HashMap::new();

        for i in 0..self.cards.len() {
            instance_counts.insert(i, 1);
        }

        for (i, card) in self.cards.iter().enumerate() {
            let card_count = *instance_counts.get(&i).unwrap();
            let count_winning = card.count_winning();

            for j in 0..count_winning {
                let index = i + j + 1;
                instance_counts.get_mut(&index).map(|c| *c += card_count);
            }
        }

        instance_counts.values().sum()
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards =
            s.lines().map(|l| l.parse()).collect::<Result<_, _>>()?;

        Ok(Game { cards })
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Game {
    input.parse().unwrap()
}

#[aoc(day4, part1)]
fn part1(game: &Game) -> usize {
    game.score()
}

#[aoc(day4, part2)]
fn part2(game: &Game) -> usize {
    game.cascading_card_count()
}