use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect()
        })
        .collect::<Vec<_>>()
}

fn next_number_in_sequence(input: &[isize]) -> isize {
    let diffs = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    if diffs.iter().all(|d| *d == 0) {
        input[input.len() - 1]
    } else {
        input[input.len() - 1] + next_number_in_sequence(&diffs)
    }
}

fn prev_number_in_sequence(input: &[isize]) -> isize {
    let diffs = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    if diffs.iter().all(|d| *d == 0) {
        input[0]
    } else {
        input[0] - prev_number_in_sequence(&diffs)
    }
}

#[aoc(day9, part1)]
fn part1(sequences: &[Vec<isize>]) -> isize {
    let next_nums = sequences
        .iter()
        .map(|s| next_number_in_sequence(s))
        .collect::<Vec<_>>();

    next_nums.iter().sum()
}

#[aoc(day9, part2)]
fn part2(sequences: &[Vec<isize>]) -> isize {
    let prev_nums = sequences
        .iter()
        .map(|s| prev_number_in_sequence(s))
        .collect::<Vec<_>>();

    prev_nums.iter().sum()
}
