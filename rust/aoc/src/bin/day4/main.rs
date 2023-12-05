use std::collections::{HashMap, HashSet};

fn part1(lines: &Vec<&str>) -> u64 {
    let mut total = 0;
    for line in lines {
        let (_x, cards) = line.split_once(":").unwrap();
        let (wincards, cardset) = cards.split_once("|").unwrap();
        let mut winset = HashSet::new();
        for w in wincards.split_ascii_whitespace().map(|part| part.trim()) {
            match w.parse::<u64>() {
                Ok(dw) => winset.insert(dw),
                Err(_) => panic!("Unable to parse winner card numbers"),
            };
        }
        let mut count = 0;
        for c in cardset.split_ascii_whitespace().map(|part| part.trim()) {
            match c.parse::<u64>() {
                Ok(dc) => {
                    if winset.contains(&dc) {
                        count += 1;
                    }
                }
                Err(_) => panic!("Unable to parse card set number"),
            }
        }
        total += if count != 0 {
            u64::pow(2, count - 1)
        } else {
            0
        }
    }
    total
}

fn part2(lines: &Vec<&str>) -> u64 {
    let mut hs = HashMap::new();
    let n = lines.len();
    for i in 0..n {
        hs.entry(i).or_insert(1);
    }
    for (i, line) in lines.iter().enumerate() {
        let (_x, cards) = line.split_once(":").unwrap();
        let (wincards, cardset) = cards.split_once("|").unwrap();
        let mut winset = HashSet::new();
        for w in wincards.split_ascii_whitespace().map(|part| part.trim()) {
            match w.parse::<u64>() {
                Ok(dw) => winset.insert(dw),
                Err(_) => panic!("Unable to parse winner card numbers"),
            };
        }
        let mut count = 0;
        for c in cardset.split_ascii_whitespace().map(|part| part.trim()) {
            match c.parse::<u64>() {
                Ok(dc) => {
                    if winset.contains(&dc) {
                        count += 1;
                    }
                }
                Err(_) => panic!("Unable to parse card set number"),
            }
        }

        let cnt = *hs.get(&i).unwrap();

        for n in i + 1..i + 1 + count {
            match hs.get_mut(&n) {
                Some(val) => *val += cnt,
                None => unreachable!(), // based on the question
            }
        }
    }
    hs.values().into_iter().sum()
}

fn main() {
    let input = include_str!("./input.txt");
    let lines = input.lines().collect::<Vec<&str>>();

    println!("[+] Part1 ans => {}", part1(&lines));
    println!("[+] Part2 ans => {}", part2(&lines));
}
