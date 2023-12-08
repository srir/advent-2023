use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::str::FromStr;

trait HandTrait {
    fn get_cards(&self) -> &[usize];
    fn kind(&self) -> HandKind;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Hand {
    cards: Vec<usize>,
    bid: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn hand_kind(cards: &[usize]) -> HandKind {
    let mut counts = [0usize; 15];

    for card in cards {
        if *card <= 14 {
            counts[*card] += 1;
        }
    }

    let mut counts = counts.to_vec();
    counts.sort();

    let mut counts = counts.into_iter().rev();

    let first = counts.next().unwrap();
    let second = counts.next().unwrap();

    if first == 5 {
        HandKind::FiveOfAKind
    } else if first == 4 {
        HandKind::FourOfAKind
    } else if first == 3 && second == 2 {
        HandKind::FullHouse
    } else if first == 3 {
        HandKind::ThreeOfAKind
    } else if first == 2 && second == 2 {
        HandKind::TwoPair
    } else if first == 2 {
        HandKind::OnePair
    } else {
        HandKind::HighCard
    }
}

impl HandTrait for Hand {
    fn get_cards(&self) -> &[usize] {
        &self.cards
    }

    fn kind(&self) -> HandKind {
        hand_kind(&self.cards)
    }
}

fn parse_card(c: char, with_jokers: bool) -> Result<usize, ()> {
    match c {
        'A' => Ok(14),
        'K' => Ok(13),
        'Q' => Ok(12),
        'J' if !with_jokers => Ok(11),
        'J' if with_jokers => Ok(1),
        'T' => Ok(10),
        _ => c.to_string().parse().map_err(|_| ()),
    }
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(())?;

        let cards = cards
            .chars()
            .map(|c| parse_card(c, false))
            .collect::<Result<_, _>>()
            .map_err(|_| ())?;

        let bid = bid.parse().map_err(|_| ())?;

        Ok(Hand { cards, bid })
    }
}

fn compare_hands<T: HandTrait>(a: &T, b: &T) -> Ordering {
    match a.kind().cmp(&b.kind()) {
        Ordering::Equal => {
            for (a, b) in a.get_cards().iter().zip(b.get_cards().iter()) {
                match a.cmp(b) {
                    Ordering::Equal => continue,
                    o => return o,
                }
            }

            Ordering::Equal
        }
        o => o,
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_hands(self, other)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day7, part1)]
fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(|l| l.parse::<Hand>().unwrap()).collect()
}

#[aoc(day7, part1)]
fn part1(input: &[Hand]) -> usize {
    let mut hands = input.to_vec();

    hands.sort();

    let mut score = 0usize;
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i + 1);
    }

    score
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct HandWithJokers {
    cards: Vec<usize>,
    bid: usize,
    kind: HandKind,
}

impl HandWithJokers {
    fn new(cards: Vec<usize>, bid: usize) -> Self {
        let kind = hand_kind_with_jokers(&cards);

        HandWithJokers { cards, bid, kind }
    }
}

fn hand_kind_with_jokers(hand_cards: &[usize]) -> HandKind {
    let joker_locs = hand_cards
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 1)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut cards = hand_cards.to_vec();
    let mut best_hand = hand_kind(hand_cards);

    // for each joker location, try all possible cards
    // and find the best hand
    for joker_loc in &joker_locs {
        for i in 2..=14 {
            let old_value = cards[*joker_loc];
            cards[*joker_loc] = i;

            let hand = hand_kind_with_jokers(&cards);
            if hand > best_hand {
                best_hand = hand;
            }

            cards[*joker_loc] = old_value;
        }
    }

    best_hand
}

impl HandTrait for HandWithJokers {
    fn get_cards(&self) -> &[usize] {
        &self.cards
    }

    fn kind(&self) -> HandKind {
        self.kind.clone()
    }
}

impl Ord for HandWithJokers {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_hands(self, other)
    }
}

impl PartialOrd for HandWithJokers {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for HandWithJokers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or(())?;

        let cards = cards
            .chars()
            .map(|c| parse_card(c, true))
            .collect::<Result<_, _>>()
            .map_err(|_| ())?;

        let bid = bid.parse().map_err(|_| ())?;

        Ok(HandWithJokers::new(cards, bid))
    }
}

#[aoc_generator(day7, part2)]
fn parse_input_with_jokers(input: &str) -> Vec<HandWithJokers> {
    input
        .lines()
        .map(|l| l.parse::<HandWithJokers>().unwrap())
        .collect()
}

#[aoc(day7, part2)]
fn part2(input: &[HandWithJokers]) -> usize {
    let mut hands = input.to_vec();

    hands.sort();

    let mut score = 0usize;
    for (i, hand) in hands.iter().enumerate() {
        score += hand.bid * (i + 1);
    }

    score
}
