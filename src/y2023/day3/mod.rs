use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn gear_ratios() {
    let test = false;
    let filename = if test { "src/y2023/day3/test" } else { "src/y2023/day3/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<String> = Vec::new();

        let mut num: String = String::new();
        for c in line.chars() {
            if c.is_numeric() { num.push(c); }
            else {
                if !num.is_empty() {
                    for _ in 0..num.len() { row.push(num.clone()); }
                    num.clear();
                }
                row.push(String::from(c));
            }
        }
        if !num.is_empty() {
            for _ in 0..num.len() { row.push(num.clone()); }
        }
        map.push(row);
    }

    let mut acc = 0;
    let mut added: Vec<usize> = Vec::new();
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if is_symbol(&map[r][c]) {
                if r > 0 && c > 0 {
                    let num = check(&map[r-1][c-1]);
                    added.push(num);
                    acc += num;
                }
                if r > 0 {
                    let num = check(&map[r-1][c]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if r > 0 && c < map[0].len()-1 {
                    let num = check(&map[r-1][c+1]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if c > 0 {
                    let num = check(&map[r][c-1]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if c < map[0].len()-1 {
                    let num = check(&map[r][c+1]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if r < map.len()-1 && c > 0 {
                    let num = check(&map[r+1][c-1]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if r < map.len()-1 {
                    let num = check(&map[r+1][c]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                if r < map.len()-1 && c < map[0].len()-1 {
                    let num = check(&map[r+1][c+1]);
                    if !added.contains(&num) {
                        added.push(num);
                        acc += num;
                    }
                }
                added.clear();
            }
        }
    }

    println!("Year 2023 day 3 part 1: {}", acc);

    acc = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == "*" {
                if r > 0 && c > 0 {
                    let num = check(&map[r-1][c-1]);
                    if num != 0 { added.push(num); }
                }
                if r > 0 {
                    let num = check(&map[r-1][c]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if r > 0 && c < map[0].len()-1 {
                    let num = check(&map[r-1][c+1]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if c > 0 {
                    let num = check(&map[r][c-1]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if c < map[0].len()-1 {
                    let num = check(&map[r][c+1]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if r < map.len()-1 && c > 0 {
                    let num = check(&map[r+1][c-1]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if r < map.len()-1 {
                    let num = check(&map[r+1][c]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if r < map.len()-1 && c < map[0].len()-1 {
                    let num = check(&map[r+1][c+1]);
                    if !added.contains(&num) && num != 0 { added.push(num); }
                }
                if added.len() == 2 { acc += added[0] * added[1]; }
                added.clear();
            }
        }
    }

    println!("Year 2023 day 3 part 2: {}", acc);
}

fn check(s: &str) -> usize {
    if let Ok(n) = s.parse::<usize>() { n }
    else { 0 }
}

fn is_symbol(s: &str) -> bool {
    if let Ok(_) = s.parse::<usize>() { false }
    else if s == "." { false }
    else { true }
}

