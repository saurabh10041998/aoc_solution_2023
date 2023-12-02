#[derive(Debug)]
struct Game {
    id: u32,
    round: Vec<Round>,
}

#[derive(Debug)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    //let lines = include_str!("./ex.txt");
    let lines = include_str!("./input.txt");
    let mut games = Vec::new();
    for line in lines.lines() {
        let (id, game) = line.split_once(":").unwrap();
        let id = id
            .strip_prefix("Game ")
            .unwrap()
            .parse::<u32>()
            .expect("Unable to extract the id");
        let mut rounds = Vec::new();
        for round in game.split_terminator(";") {
            let mut r = Round {
                green: 0,
                red: 0,
                blue: 0,
            };
            for part in round.split_terminator(",").map(|part| part.trim()) {
                if let Some(blue) = part.strip_suffix(" blue") {
                    r.blue = blue.parse().expect("Unable to parse blue count");
                    continue;
                }
                if let Some(red) = part.strip_suffix(" red") {
                    r.red = red.parse().expect("Unable to parse red count");
                    continue;
                }
                if let Some(green) = part.strip_suffix(" green") {
                    r.green = green.parse().expect("Unable to parse green count");
                    continue;
                }
                panic!("Strange pattern occured: {}", part)
            }
            rounds.push(r);
        }
        games.push(Game { id, round: rounds });
    }
    // Part 1
    let p1: u32 = games
        .iter()
        .filter(|game| {
            game.round
                .iter()
                .all(|r| r.red <= 12 && r.green <= 13 && r.blue <= 14)
        })
        .map(|game| game.id)
        .sum();
    // Part 2
    let p2: u32 = games
        .iter()
        .map(|game| {
            let min_red = game.round.iter().map(|r| r.red).max().unwrap();
            let min_green = game.round.iter().map(|r| r.green).max().unwrap();
            let min_blue = game.round.iter().map(|r| r.blue).max().unwrap();

            min_red * min_green * min_blue
        })
        .sum();
    println!("[*] Part 1 ans => {}", p1);
    println!("[*] Part 2 ans => {}", p2);
}
