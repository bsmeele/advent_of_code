use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use crate::web_stuff::get_input;

pub fn trebuchet() {
    let test = false;
    let file_path = if test { "src/y2023/day1/test1" } else { "src/y2023/day1/input" };
    if let Ok(metadata) = fs::metadata(file_path) {
        if !metadata.is_file() {
            if get_input(2023, 1).is_err() { return; }
        }
    }
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let mut acc1 = 0;
    let mut acc2 = 0;
    let mut first1 = 0;
    let mut first2 = 0;
    let mut last1 = 0;
    let mut last2 = 0;
    let mut flag;
    let mut l;
    let mut r;

    for line in reader.lines().map(|x| x.unwrap()) {
        l = 0;
        r = 0;
        flag = false;

        for c in line.chars() {
            if let Some(n) = c.to_digit(10) {
                first1 = n;
                if !flag { first2 = n; }
                break;
            } else if !flag {
                r += 1;
                if r - l > 5 { l += 1; }
                if let Some(n) = contains_num(&line[l..r]) {
                    first2 = n;
                    flag = true;
                }
            }
        }

        flag = false;
        l = line.len();
        r = line.len();

        for c in line.chars().rev() {
            if let Some(n) = c.to_digit(10) {
                last1 = n;
                if !flag { last2 = n; }
                break;
            } else if !flag {
                l -= 1;
                if r - l > 5 { r -= 1; }
                if let Some(n) = contains_num(&line[l..r]) {
                    last2 = n;
                    flag = true;
                }
            }
        }

        acc1 += first1 * 10 + last1;
        acc2 += first2 * 10 + last2;
    }

    println!("Year 2023 day 1 part 1: {}", acc1);
    println!("Year 2023 day 1 part 2: {}", acc2);
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
