use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lens_library() {
    let test = false;
    let filename = if test { "src/y2023/day15/test" } else { "src/y2023/day15/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut acc = 0;
    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(',').collect::<Vec<&str>>();
        for s in line {
            let tmp = hash(s);
            acc += tmp;
        }
    }

    println!("Year 2023 day 15 part 1: {}", acc);
}

fn hash(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += (c as u8) as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}