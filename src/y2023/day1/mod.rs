use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn trebuchet() {
    let test = false;
    let filename = if test { "src/y2023/day1/test1" } else { "src/y2023/day1/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut acc = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut list: Vec<u32> = Vec::new();

        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                list.push(n);
            }
        }

        acc += list[0] * 10 + list.last().unwrap();
    }

    println!("Day 1 part 1: {}", acc);

    let filename = if test { "src/y2023/day1/test2" } else { "src/y2023/day1/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    acc = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut first: u32 = 0;
        let mut last: u32 = 0;

        let mut substring = String::new();

        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                first = n;
                break;
            } else {
                substring.push(c);
                if let Some(n) = contains_num(&substring) {
                    first = n;
                    substring.clear();
                    break;
                }
            }
        }

        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                last = n;
                break;
            } else {
                substring.push(c);
                let s = substring.chars().rev().collect::<String>();
                if let Some(n) = contains_num(&s) {
                    last = n;
                    break;
                }
            }
        }

        acc += first * 10 + last;
    }

    println!("Day 1 part 2: {}", acc);
}

fn contains_num(s: &str) -> Option<u32> {
    if s.contains("one") { Some(1) }
    else if s.contains("two") { Some(2) }
    else if s.contains("three") { Some(3) }
    else if s.contains("four") { Some(4) }
    else if s.contains("five") { Some(5) }
    else if s.contains("six") { Some(6) }
    else if s.contains("seven") { Some(7) }
    else if s.contains("eight") { Some(8) }
    else if s.contains("nine") { Some(9) }
    else if s.contains("zero") { Some(0) }
    else { None }
}
