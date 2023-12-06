use rayon::prelude::*;
use std::collections::HashMap;

const NUM_THREADS: usize = 10;

#[derive(Debug)]
struct Converter {
    src: String,
    dst: String,
    ranges: Vec<(Range, Range)>,
}

impl From<&str> for Converter {
    fn from(line: &str) -> Self {
        let line = line.strip_suffix(" map:").unwrap();
        let mut spliter = line.split(|c| c == '-');
        let src = spliter.next().unwrap();
        let _ = spliter.next().unwrap();
        let dst = spliter.next().unwrap();
        Converter {
            src: String::from(src),
            dst: String::from(dst),
            ranges: vec![],
        }
    }
}

impl Converter {
    fn add_ranges(&mut self, dst_start: usize, src_start: usize, len: usize) {
        let dest = Range {
            start: dst_start,
            end: dst_start + len - 1,
        };
        let src = Range {
            start: src_start,
            end: src_start + len - 1,
        };
        self.ranges.push((src, dest))
    }
    fn get_destination_val(&self, value: usize) -> usize {
        for range in self.ranges.iter() {
            if value >= range.0.start && value <= range.0.end {
                let span = value - range.0.start;
                return range.1.start + span;
            }
        }
        value
    }
}

#[derive(Debug)]
struct Range {
    start: usize,
    end: usize,
}

fn part1(seeds: &Vec<usize>, converters: &HashMap<String, Converter>) -> usize {
    let mut min_val = usize::MAX;
    for seed in seeds {
        let mut val = *seed;
        let mut dst = "seed";
        while dst != "location" {
            match converters.get(&String::from(dst)) {
                Some(conv) => {
                    dst = &conv.dst;
                    val = conv.get_destination_val(val);
                }
                None => unreachable!(),
            }
        }
        min_val = usize::min(min_val, val);
    }
    min_val
}

fn main() {
    //let input = include_str!("./ex.txt");
    let input = include_str!("./input.txt");
    let mut seeds = Vec::new();
    let mut current_converter: Option<Converter> = None;
    let mut converters = HashMap::new();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        } else if line.starts_with("seeds:") {
            seeds = line
                .strip_prefix("seeds: ")
                .map(|line| line.trim())
                .unwrap()
                .split_ascii_whitespace()
                .map(|seed| seed.parse::<usize>().expect("Unable to parse seeds"))
                .collect();
        } else if line.ends_with("map:") {
            // Map conversion
            match current_converter.take() {
                Some(converter) => {
                    converters
                        .entry(String::from(&converter.src))
                        .or_insert(converter);
                }
                None => {}
            };
            let c: Converter = line.into();
            current_converter = Some(c);
        } else {
            let mut spliter = line.split_ascii_whitespace();
            let dst_start = spliter
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("[!!] Unable to parse");
            let src_start = spliter
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("[!!] Unable to parse");
            let len = spliter
                .next()
                .unwrap()
                .parse::<usize>()
                .expect("[!!] Unable to parse");
            match current_converter.as_mut() {
                Some(converter) => converter.add_ranges(dst_start, src_start, len),
                None => {
                    panic!("[!!] Cannot add ranges as map itself is not created")
                }
            };
        }
    }
    match current_converter.take() {
        Some(converter) => {
            converters
                .entry(String::from(&converter.src))
                .or_insert(converter);
        }
        None => {}
    }
    let p1 = part1(&seeds, &converters);
    println!("Part-1 ans: => {}", p1);

    let mut sets = vec![];
    for i in (0..seeds.len()).step_by(2) {
        let set = (seeds[i]..seeds[i] + seeds[i + 1]).collect::<Vec<usize>>();
        sets.push(set);
    }

    let ans = sets
        .par_iter()
        .map(|s| part1(&s, &converters))
        .min()
        .unwrap();

    println!("Part-2 ans => {}", ans);
}
