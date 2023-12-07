use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn supply_stacks() {
    let filename = "src/day5/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut init = false;

    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(Vec::new());
    }
    let mut stacks_p2: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks_p2.push(Vec::new());
    }

    for line in reader.lines() {
        let line = line.unwrap();
        if !init {
            let line: Vec<char> = line.chars().collect();
            if line[1] == '1' {
                stacks = stacks.into_iter().map(|x| x.into_iter().rev().collect()).collect();
                stacks_p2 = stacks.clone();
                init = true;
                continue;
            }

            if !line.is_empty() && line[1] != ' ' { stacks[0].push(line[1]); }
            if line.len() >= 5 && line[5] != ' ' { stacks[1].push(line[5]); }
            if line.len() >= 9 && line[9] != ' ' { stacks[2].push(line[9]); }
            if line.len() >= 13 && line[13] != ' ' { stacks[3].push(line[13]); }
            if line.len() >= 17 && line[17] != ' ' { stacks[4].push(line[17]); }
            if line.len() >= 21 && line[21] != ' ' { stacks[5].push(line[21]); }
            if line.len() >= 25 && line[25] != ' ' { stacks[6].push(line[25]); }
            if line.len() >= 29 && line[29] != ' ' { stacks[7].push(line[29]); }
            if line.len() >= 33 && line[33] != ' ' { stacks[8].push(line[33]); }
        } else {
            let line: Vec<&str> = line.split(' ').collect();
            if line.len() < 6 { continue; }

            let count = line[1].parse::<u32>().unwrap();
            let from = line[3].parse::<usize>().unwrap();
            let to = line[5].parse::<usize>().unwrap();

            let mut tmp_stack = Vec::new();
            for _ in 0..count {
                let tmp = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(tmp);
                tmp_stack.push(stacks_p2[from-1].pop().unwrap());
            }
            for _ in 0..count {
                stacks_p2[to-1].push(tmp_stack.pop().unwrap());
            }
        }
    }

    let mut total = String::new();
    total.push(stacks[0].pop().unwrap());
    total.push(stacks[1].pop().unwrap());
    total.push(stacks[2].pop().unwrap());
    total.push(stacks[3].pop().unwrap());
    total.push(stacks[4].pop().unwrap());
    total.push(stacks[5].pop().unwrap());
    total.push(stacks[6].pop().unwrap());
    total.push(stacks[7].pop().unwrap());
    total.push(stacks[8].pop().unwrap());

    let mut total2 = String::new();
    total2.push(stacks_p2[0].pop().unwrap());
    total2.push(stacks_p2[1].pop().unwrap());
    total2.push(stacks_p2[2].pop().unwrap());
    total2.push(stacks_p2[3].pop().unwrap());
    total2.push(stacks_p2[4].pop().unwrap());
    total2.push(stacks_p2[5].pop().unwrap());
    total2.push(stacks_p2[6].pop().unwrap());
    total2.push(stacks_p2[7].pop().unwrap());
    total2.push(stacks_p2[8].pop().unwrap());

    println!("Day 5 part 1: {}", total);
    println!("Day 5 part 2: {}", total2);
}
