macro_rules! sequence_difference {
    ($vec: expr) => {{
        let mut differences = Vec::new();
        if $vec.len() > 1 {
            for i in 1..$vec.len() {
                differences.push($vec[i] - $vec[i - 1]);
            }
        }
        differences
    }};
}

fn next_item(seq: Vec<i32>) -> i32 {
    let mut v = seq;
    let mut result = vec![];
    while v.windows(2).any(|w| w[1] - w[0] != 0) {
        v = sequence_difference!(v);
        let ele = *v.last().unwrap();
        result.push(ele);
    }
    result.into_iter().sum()
}

fn main() {
    //let input = include_str!("ex.txt");
    let input = include_str!("input.txt");
    let part1 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("[!!] Unable to parse number"))
                .collect::<Vec<i32>>()
        })
        .into_iter()
        .map(|seq: Vec<i32>| *seq.last().unwrap() + next_item(seq))
        .sum::<i32>();
    println!("Part 1 ans => {}", part1);
    let part2 = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|num| num.parse::<i32>().expect("[!!] Unable to parse number"))
                .collect::<Vec<i32>>()
        })
        .into_iter()
        .map(|seq: Vec<i32>| seq[0] + next_item(seq.into_iter().rev().collect()))
        .sum::<i32>();
    println!("Part 2 ans => {}", part2);
}
