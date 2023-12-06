use crate::util::parse_numbers;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
struct Game {
    time_limit: usize,
    distance_to_beat: usize,
}

impl Game {
    fn is_winner(&self, time_held: usize) -> bool {
        if time_held >= self.time_limit {
            return false;
        }

        let distance = (self.time_limit - time_held) * time_held;

        distance > self.distance_to_beat
    }

    fn count_winning(&self) -> usize {
        let mut count = 0usize;

        for time_held in 0..=self.time_limit {
            if self.is_winner(time_held) {
                count += 1;
            }
        }

        count
    }

    fn merge_games(games: &[Game]) -> Game {
        let time_limits = games.iter().map(|g| g.time_limit).collect::<Vec<_>>();
        let distances_to_beat = games.iter().map(|g| g.distance_to_beat).collect::<Vec<_>>();

        let time_limit = time_limits
            .iter()
            .map(|t| t.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        let distance_to_beat = distances_to_beat
            .iter()
            .map(|t| t.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        Game {
            time_limit,
            distance_to_beat,
        }
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<Game> {
    let (times, dists) = input.split_once("\n").unwrap();
    let time_limits = times.strip_prefix("Time:").unwrap();
    let distances = dists.strip_prefix("Distance:").unwrap();

    let time_limits = parse_numbers(time_limits).unwrap();
    let distances = parse_numbers(distances).unwrap();

    time_limits
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| Game {
            time_limit: *t,
            distance_to_beat: *d,
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(games: &[Game]) -> usize {
    games.iter().map(|g| g.count_winning()).product()
}

#[aoc(day6, part2)]
fn part2(games: &[Game]) -> usize {
    let game = Game::merge_games(games);

    game.count_winning()
}
