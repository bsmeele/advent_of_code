use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn rucksack_reorganization() {
    let test = false;
    let filename = if test { "src/y2022/day3/test" } else { "src/y2022/day3/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut first_half: Vec<char> = line.chars().collect();
        let half = first_half.len()/2;
        let second_half = first_half.split_off(half);

        for c in first_half {
            if find_in_vec(c, &second_half) {
                total += get_num_from_char(c);
                break;
            }
        }
    }
    println!("Day 3 part 1: {}", total);
    part_two();
}

fn part_two() {
    let filename = "src/day3/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u32 = 0;
    let lines: Vec<Result<String, _>> = reader.lines().collect();
    let groups = lines.len()/3;
    for g in 0..groups {
        let elf1: Vec<char> = lines[3*g].as_ref().unwrap().chars().collect();
        let elf2: Vec<char> = lines[3*g + 1].as_ref().unwrap().chars().collect();
        let elf3: Vec<char> = lines[3*g + 2].as_ref().unwrap().chars().collect();
        for c in elf1 {
            if find_in_vec(c, &elf2) && find_in_vec(c, &elf3) {
                total += get_num_from_char(c);
                break;
            }
        }
    }
    println!("Day 3 part 2: {}", total);
}

fn find_in_vec(c: char, vec: &Vec<char>) -> bool {
    for e in vec {
        if *e == c { return true; }
    }
    false
}

fn get_num_from_char(c: char) -> u32 {
    match c {
        'a' => 1,
        'b' => 2,
        'c' => 3,
        'd' => 4,
        'e' => 5,
        'f' => 6,
        'g' => 7,
        'h' => 8,
        'i' => 9,
        'j' => 10,
        'k' => 11,
        'l' => 12,
        'm' => 13,
        'n' => 14,
        'o' => 15,
        'p' => 16,
        'q' => 17,
        'r' => 18,
        's' => 19,
        't' => 20,
        'u' => 21,
        'v' => 22,
        'w' => 23,
        'x' => 24,
        'y' => 25,
        'z' => 26,
        'A' => 27,
        'B' => 28,
        'C' => 29,
        'D' => 30,
        'E' => 31,
        'F' => 32,
        'G' => 33,
        'H' => 34,
        'I' => 35,
        'J' => 36,
        'K' => 37,
        'L' => 38,
        'M' => 39,
        'N' => 40,
        'O' => 41,
        'P' => 42,
        'Q' => 43,
        'R' => 44,
        'S' => 45,
        'T' => 46,
        'U' => 47,
        'V' => 48,
        'W' => 49,
        'X' => 50,
        'Y' => 51,
        'Z' => 52,
        _ => 0,
    }
}
