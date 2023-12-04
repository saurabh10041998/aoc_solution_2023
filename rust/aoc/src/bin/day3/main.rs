struct Number {
    x: usize,
    y: usize,
    value: u64,
    is_symbol: bool,
}

fn is_symbol(c: u8) -> bool {
    !(c == b'.' || (c as char).is_digit(10))
}

fn lookup(input: &Vec<Vec<Option<usize>>>, x: isize, y: isize) -> Option<usize> {
    if y < 0 || y >= input.len() as isize {
        return None;
    }
    let y = y as usize;
    if x < 0 || x >= input[0].len() as isize {
        return None;
    }
    let x = x as usize;
    input[y][x]
}

fn main() {
    let input = std::fs::read("input.txt").unwrap();
    let input: Vec<Vec<u8>> = input
        .split(|c| *c == b'\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.to_vec())
        .collect();
    let mut numbers = Vec::new();
    let height = input.len();
    let width = input[0].len();
    let mut coords_to_numbers = vec![vec![None; width]; height];

    for y in 0..height {
        let mut curr_number = None;
        for x in 0..width {
            if (input[y][x] as char).is_digit(10) {
                if curr_number.is_none() {
                    curr_number = Some(numbers.len());
                    numbers.push(Number {
                        x,
                        y,
                        value: 0,
                        is_symbol: false,
                    });
                }
            } else {
                curr_number = None
            }
            coords_to_numbers[y][x] = curr_number;
        }
    }

    for number in &mut numbers {
        let mut value = 0;
        for i in number.x..width {
            match (input[number.y][i] as char).to_digit(10) {
                Some(digit) => value = (value * 10) + (digit as u64),
                None => break,
            }
        }
        number.value = value;
    }

    // mark the symbol
    for y in 0..height as isize {
        for x in 0..width as isize {
            if is_symbol(input[y as usize][x as usize]) {
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some(num) = lookup(&coords_to_numbers, x + dx, y + dy) {
                            numbers[num].is_symbol = true;
                        }
                    }
                }
            }
        }
    }

    let mut ratios = Vec::new();
    for y in 0..height as isize {
        for x in 0..width as isize {
            if is_symbol(input[y as usize][x as usize]) {
                let mut adjacent_numbers = Vec::new();
                for dx in [-1, 0, 1] {
                    for dy in [-1, 0, 1] {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if let Some(num) = lookup(&coords_to_numbers, x + dx, y + dy) {
                            adjacent_numbers.push(num);
                        }
                    }
                }
                adjacent_numbers.sort_unstable();
                adjacent_numbers.dedup();

                if let &[n1, n2] = adjacent_numbers.as_slice() {
                    ratios.push(numbers[n1].value * numbers[n2].value);
                }
            }
        }
    }

    // part 1
    let part1: u64 = numbers
        .iter()
        .filter(|num| num.is_symbol)
        .map(|num| num.value)
        .sum();
    let part2: u64 = ratios.into_iter().sum();

    println!("[+] Part 1 ans => {}", part1);
    println!("[+] Part 2 ans => {}", part2);
}
