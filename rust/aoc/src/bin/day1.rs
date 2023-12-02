use std::borrow::Cow;
use {once_cell::sync::Lazy, regex::Regex};

fn part1(lines: &Vec<&str>) -> i64 {
    static NONDIGIT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\D").expect("Unable to compile regex"));
    let mut total = 0;

    for line in lines {
        let digits: Cow<str> = NONDIGIT.replace_all(line, "");
        let digits = digits.chars().collect::<Vec<char>>();
        match (
            digits.first().unwrap().to_string().parse::<i64>(),
            digits.last().unwrap().to_string().parse::<i64>(),
        ) {
            (Ok(first), Ok(last)) => total += first * 10 + last,
            (_, _) => panic!(
                "[!!] Unable to parse the digits, {:?} and {:?}",
                digits.first().unwrap(),
                digits.last().unwrap()
            ),
        }
    }

    total
}

fn part2(lines: &Vec<&str>) -> i64 {
    let mut total = 0;
    static NONDIGIT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"\D").expect("Unable to compile regex for non-digit pattern"));
    static ONE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"one").expect("Unable to compile regex for 'one' pattern"));
    static TWO: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"two").expect("Unable to compile regex for 'two' pattern"));
    static THREE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"three").expect("Unable to compile regex for 'three' pattern"));
    static FOUR: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"four").expect("Unable to compile regex for 'four' pattern"));
    static FIVE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"five").expect("Unable to compile regex for 'five' pattern"));
    static SIX: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"six").expect("Unable to compile regex for 'six' pattern"));
    static SEVEN: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"seven").expect("Unable to compile regex for 'seven' pattern"));
    static EIGHT: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"eight").expect("Unable to compile regex for 'eight' pattern"));
    static NINE: Lazy<Regex> =
        Lazy::new(|| Regex::new(r"nine").expect("Unable to compile regex for 'nine' pattern"));

    let mut mappings: Vec<(&Lazy<Regex>, &str)> = vec![];

    mappings.push((&ONE, "one1one"));
    mappings.push((&TWO, "two2two"));
    mappings.push((&THREE, "three3three"));
    mappings.push((&FOUR, "four4four"));
    mappings.push((&FIVE, "five5five"));
    mappings.push((&SIX, "six6six"));
    mappings.push((&SEVEN, "seven7seven"));
    mappings.push((&EIGHT, "eight8eight"));
    mappings.push((&NINE, "nine9nine"));

    for line in lines {
        let mut replace_line = String::from(*line);
        for (pattern, replacer) in mappings.iter() {
            replace_line = String::from(pattern.replace_all(&replace_line, *replacer));
        }
        let digits: Cow<str> = NONDIGIT.replace_all(&replace_line, "");
        let digits: Vec<char> = digits.chars().collect();
        match (
            digits.first().unwrap().to_string().parse::<i64>(),
            digits.last().unwrap().to_string().parse::<i64>(),
        ) {
            (Ok(first), Ok(last)) => total += first * 10 + last,
            (_, _) => panic!(
                "[!!] Unable to parse the digits, {:?} and {:?}",
                digits.first().unwrap(),
                digits.last().unwrap()
            ),
        };
    }
    total
}

fn main() {
    let lines = include_str!("./input.txt");
    let lines = lines.lines().collect::<Vec<&str>>();

    println!("Part 1 answer => {:?}", part1(&lines));
    println!("Part 2 answer => {:?}", part2(&lines));
}
