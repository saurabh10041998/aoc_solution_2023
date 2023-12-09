use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

static BRANCH: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"(\w\w\w) = \((\w\w\w), (\w\w\w)\)")
        .expect("Unable to compile Regex for branch pattern")
});

struct Branch<'a> {
    left: &'a str,
    right: &'a str,
}

impl<'a> Branch<'a> {
    fn new(left: &'a str, right: &'a str) -> Self {
        Branch { left, right }
    }
}

fn parse_branch(line: &str) -> (&str, Branch) {
    let matches = BRANCH.captures(line).unwrap();
    let src = matches.get(1).unwrap().as_str();
    let left = matches.get(2).unwrap().as_str();
    let right = matches.get(3).unwrap().as_str();
    (src, Branch::new(left, right))
}

fn gcd(a: usize, b: usize) -> usize {
    match a.cmp(&b) {
        std::cmp::Ordering::Equal => a,
        std::cmp::Ordering::Less => {
            if a == 0 {
                b
            } else {
                gcd(a, b % a)
            }
        }
        std::cmp::Ordering::Greater => {
            if b == 0 {
                a
            } else {
                gcd(a % b, b)
            }
        }
    }
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part1(input: &str) -> usize {
    let mut iterations = 0;
    let mut navigator_map = HashMap::new();
    let mut cmd = None;
    for line in input.lines() {
        match cmd {
            None => {
                cmd = Some(line);
                continue;
            }
            Some(_) => {}
        };
        if line.is_empty() {
            continue;
        }
        let (src, branch) = parse_branch(line);
        navigator_map.entry(src).or_insert(branch);
    }
    let mut current = "AAA";
    for direction in cmd.unwrap().chars().cycle() {
        if current == "ZZZ" {
            break;
        }
        match direction {
            'L' => {
                current = navigator_map.get(&current).unwrap().left;
            }
            'R' => {
                current = navigator_map.get(&current).unwrap().right;
            }
            _ => panic!("Unsupported move!!"),
        }
        iterations += 1;
    }
    iterations
}

fn part2(input: &str) -> usize {
    let mut iterations = vec![];
    let mut navigator_map = HashMap::new();
    let mut sources = vec![];
    let mut cmd = None;
    for line in input.lines() {
        match cmd {
            Some(_) => {}
            None => {
                cmd = Some(line);
                continue;
            }
        }
        if line.is_empty() {
            continue;
        }
        let (src, branch) = parse_branch(line);
        if src.ends_with("A") {
            sources.push(src);
        }
        navigator_map.entry(src).or_insert(branch);
    }
    for source in sources {
        let mut iteration = 0usize;
        let mut current = source;
        for direction in cmd.unwrap().chars().cycle() {
            if current.ends_with("Z") {
                break;
            }
            match direction {
                'L' => {
                    current = navigator_map.get(&current).unwrap().left;
                }
                'R' => {
                    current = navigator_map.get(&current).unwrap().right;
                }
                _ => panic!("Unsuppored move!!"),
            }
            iteration += 1;
        }
        iterations.push(iteration);
    }
    let mut lcm_of_lst = iterations[0];

    for i in iterations.into_iter().skip(1) {
        lcm_of_lst = lcm(lcm_of_lst, i);
    }
    lcm_of_lst
}

fn main() {
    let input = include_str!("input.txt");
    println!("[+] Part 1 ans => {}", part1(input));
    println!("[+] Part 2 ans => {}", part2(input));
}
