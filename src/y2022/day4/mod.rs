use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn camp_cleanup() {
    let test = false;
    let filename = if test { "src/y2022/day4/input"} else { "src/y2022/day4/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;
    let mut total2: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(',').collect();
        let pair1: Vec<&str> = line[0].split('-').collect();
        let pair2: Vec<&str> = line[1].split('-').collect();
        let a: u32 = pair1[0].parse().unwrap();
        let b: u32 = pair1[1].parse().unwrap();
        let c: u32 = pair2[0].parse().unwrap();
        let d: u32 = pair2[1].parse().unwrap();

        if (a >= c && b <= d) || (c >= a && d <= b) { total += 1; }

        if a >= c {
            if b <= d || a <= d { total2 += 1; }
        } else if d <= b || c <= b { total2 += 1; }
    }
    println!("Day 4 part 1: {}", total);
    println!("Day 4 part 2: {}", total2);
}