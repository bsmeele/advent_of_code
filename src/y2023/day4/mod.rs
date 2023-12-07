use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn scratchcards() {
    let test = false;
    let filename = if test { "src/y2023/day4/test" } else { "src/y2023/day4/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }

    let mut winning: Vec<usize> = Vec::new();
    let mut points = 0;
    let mut acc = 0;
    let mut num_winning = 0;
    let mut card_id = 0;
    let mut cards = vec![1; input.len()];
    let mut acc2 = 0;

    for line in input {
        let line: Vec<&str> = line.split(' ').collect();

        let mut i = 2;
        loop {
            let s = line[i];
            if s == "|" { break; }
            if let Ok(n) = s.parse::<usize>() {
                winning.push(n);
            }
            i += 1;
        }
        i += 1;
        loop {
            if i == line.len() { break; }
            if let Ok(n) = line[i].parse::<usize>() {
                if winning.contains(&n) {
                    if points == 0 { points = 1; }
                    else { points *= 2; }
                    num_winning += 1;
                }
            }

            i += 1;
        }

        for card in (card_id+1)..=(card_id+num_winning) { cards[card] += cards[card_id]; }

        acc += points;
        points = 0;
        num_winning = 0;
        card_id += 1;
        winning.clear();
    }

    for n in cards { acc2 += n; }

    println!("Year 2023 day 4 part 1: {}", acc);
    println!("Year 2023 day 4 part 2: {}", acc2);
}