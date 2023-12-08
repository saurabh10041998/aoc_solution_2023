use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
    Joker,
}

impl From<char> for Card {
    fn from(ch: char) -> Self {
        match ch {
            'A' => Self::Ace,
            'K' => Self::King,
            'Q' => Self::Queen,
            'J' => Self::Jack,
            'W' => Self::Joker,
            'T' => Self::Ten,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            '1' => Self::One,
            _ => panic!("Invalid card {}!!", ch),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Hand {
    FiveOfKind(Vec<Card>),
    FourOfKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeOfKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let most_freq_card: Option<char> = line
            .chars()
            .fold(HashMap::new(), |mut accum, ch| {
                if ch != 'W' {
                    accum
                        .entry(ch)
                        .and_modify(|f: &mut u32| *f += 1)
                        .or_insert(1u32);
                }
                accum
            })
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(key, _)| key);

        let most_freq_card = match most_freq_card {
            Some(c) => c,
            None => 'W',
        };

        let counters: Vec<u32> = line
            .chars()
            .fold(HashMap::new(), |mut accum, ch| {
                if ch == 'W' {
                    accum
                        .entry(most_freq_card)
                        .and_modify(|f: &mut u32| *f += 1)
                        .or_insert(1u32);
                } else {
                    accum
                        .entry(ch)
                        .and_modify(|f: &mut u32| *f += 1)
                        .or_insert(1u32);
                }
                accum
            })
            .into_iter()
            .map(|(_, count)| count)
            .collect();

        let cards: Vec<Card> = line.chars().map(|ch| ch.into()).collect();
        match counters.len() {
            1 => Self::FiveOfKind(cards),
            2 if counters.contains(&4) => Self::FourOfKind(cards),
            2 => Self::FullHouse(cards),
            3 if counters.contains(&3) => Self::ThreeOfKind(cards),
            3 => Self::TwoPair(cards),
            4 => Self::OnePair(cards),
            5 => Self::HighCard(cards),
            _ => panic!("Invalid hand!!"),
        }
    }
}

struct Hands {
    hands: Vec<(Hand, usize)>,
}

impl Hands {
    fn new() -> Self {
        Hands { hands: vec![] }
    }

    fn part1(&mut self, input: &str) -> u64 {
        let mut part1 = 0;
        for line in input.lines() {
            let mut spliter = line.split_ascii_whitespace();
            let p1: &str = spliter.next().unwrap();
            let bid: usize = spliter
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("[!!] Unable to parse bid");
            let hand: Hand = p1.into();
            self.hands.push((hand, bid))
        }
        let mut pq = self.hands.drain(..).collect::<BinaryHeap<(Hand, usize)>>();
        let mut rank = 1;
        while let Some((_, bid)) = pq.pop() {
            part1 += bid as u64 * rank;
            rank += 1;
        }
        part1
    }
    fn part2(&mut self, input: &str) -> u64 {
        let mut part2 = 0;
        static JOKER: Lazy<Regex> =
            Lazy::new(|| Regex::new("J").expect("[!!] Unable to compile regex for 'J' pattern"));

        for line in input.lines() {
            let mut spliter = line.split_ascii_whitespace();
            let p1: &str = spliter.next().unwrap();
            let p1 = String::from(JOKER.replace_all(p1, "W"));
            let hand: Hand = p1.as_str().into();
            let bid: usize = spliter
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("[!!] Unable to parse bid");
            self.hands.push((hand, bid));
        }
        let mut pq = self.hands.drain(..).collect::<BinaryHeap<(Hand, usize)>>();
        let mut rank = 1;
        while let Some((_, bid)) = pq.pop() {
            part2 += bid as u64 * rank;
            rank += 1;
        }
        part2
    }
}

fn main() {
    //let input = include_str!("./ex.txt");
    let input = include_str!("./input.txt");
    let mut hands = Hands::new();
    println!("Part1 ans: {:?}", hands.part1(input));
    hands = Hands::new();
    println!("Part2 ans: {:?}", hands.part2(input));
}
