use once_cell::sync::Lazy;
use regex::Regex;

static TIME: Lazy<Regex> =
    Lazy::new(|| Regex::new("Time:").expect("Unable compile regex for 'Time:' pattern"));
static DISTANCE: Lazy<Regex> =
    Lazy::new(|| Regex::new("Distance:").expect("Unable to compile regex for 'Distance:' pattern"));

#[derive(Debug)]
struct Race {
    length: u64,
    record: u64,
}

fn compute<T>(time: T, length: T) -> T
where
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + Copy,
{
    let speed = time;
    let rem = length - time;
    speed * rem
}

fn part1(input: &str) -> u64 {
    let mut times = vec![];
    let mut distances = vec![];
    for line in input.lines() {
        if TIME.is_match(line) {
            times = line
                .strip_prefix("Time:")
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|t| t.parse::<u64>().expect("Unable to parse time"))
                .collect();
        }

        if DISTANCE.is_match(line) {
            distances = line
                .strip_prefix("Distance:")
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .map(|d| d.parse::<u64>().expect("Unable to parse distance"))
                .collect();
        }
    }

    let races: Vec<Race> = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(length, record)| Race { length, record })
        .collect();

    races
        .into_iter()
        .map(|Race { length, record }| {
            let wins: Vec<u64> = (0..length)
                .filter_map(|time| {
                    let ans = compute(time, length);
                    match ans > record {
                        true => Some(1),
                        false => None,
                    }
                })
                .collect();

            <usize as TryInto<u64>>::try_into(wins.len()).unwrap()
        })
        .product()
}

fn part2(input: &str) -> u64 {
    let mut times = String::new();
    let mut distances = String::new();
    for line in input.lines() {
        if TIME.is_match(line) {
            times = line
                .strip_prefix("Time:")
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .fold(String::new(), |mut accum, t| {
                    accum.push_str(t);
                    accum
                });
        }
        if DISTANCE.is_match(line) {
            distances = line
                .strip_prefix("Distance:")
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .fold(String::new(), |mut accum, t| {
                    accum.push_str(t);
                    accum
                });
        }
    }
    let records = vec![Race {
        length: times.parse::<u64>().expect("unable to parse time"),
        record: distances.parse::<u64>().expect("Unable to parse record"),
    }];

    records
        .into_iter()
        .map(|Race { length, record }| {
            let wins: u64 = (0..length)
                .filter_map(|time| {
                    let ans = compute(time, length);
                    match ans > record {
                        true => Some(1),
                        false => None,
                    }
                })
                .sum();
            wins
        })
        .next()
        .unwrap()
}

fn main() {
    //let input = include_str!("ex.txt");
    let input = include_str!("input.txt");
    println!("Part1 ans => {}", part1(input));
    println!("Part2 ans => {}", part2(input));
}
