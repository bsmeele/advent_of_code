use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn point_of_incidence() {
    let test = false;
    let filename = if test { "src/y2023/day13/test" } else { "src/y2023/day13/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut pattern: Vec<Vec<char>> = Vec::new();
    let mut row: Vec<char> = Vec::new();
    let mut acc1 = 0;
    let mut acc2 = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        if line.is_empty() {
            let r1 = find_reflection(&pattern, false);
            acc1 += r1;
            // draw_pattern(&pattern, Some(r1));

            let r2 = find_reflection(&pattern, true);
            acc2 += r2;
            // draw_pattern(&pattern, Some(r2));

            pattern.clear();
        } else {
            for c in line.chars() {
                row.push(c);
            }
            pattern.push(row.clone());
            row.clear()
        }
    }

    let r1 = find_reflection(&pattern, false);
    acc1 += r1;
    // draw_pattern(&pattern, Some(r1));

    let r2 = find_reflection(&pattern, true);
    acc2 += r2;
    // draw_pattern(&pattern, Some(r2));

    println!("Year 2023 day 13 part 1: {}", acc1);
    println!("Year 2023 day 13 part 2: {}", acc2);
}

fn find_reflection(pattern: &Vec<Vec<char>>, smudge: bool) -> usize {
    let mut smudge_fix = true;
    let mut differs_by: usize;

    'horizontal_check: for i in 1..pattern.len() {
        if smudge { smudge_fix = false; }
        for w in 0..min(i, pattern.len() - i ) {
            if smudge_fix && pattern[i-1-w] != pattern[i+w] { continue 'horizontal_check; }
            else if smudge {
                differs_by = 0;
                for x in 0..pattern[0].len() {
                    if pattern[i-1-w][x] != pattern[i+w][x] { differs_by += 1; }
                }
                if differs_by == 1 { smudge_fix = true; }
                else if differs_by > 1 { continue 'horizontal_check; }
            }
        }
        if smudge_fix { return i * 100; }
    }

    'vertical_check: for i in 1..pattern[0].len() {
        if smudge { smudge_fix = false; }
        for w in 0..min(i, pattern[0].len() - i) {
            differs_by = 0;
            for y in 0..pattern.len() {
                if smudge_fix && pattern[y][i-1-w] != pattern[y][i+w] { continue 'vertical_check; }
                else if pattern[y][i-1-w] != pattern[y][i+w] {
                    differs_by += 1;
                }
            }
            if smudge {
                if differs_by == 1 { smudge_fix = true; }
                else if differs_by > 1 { continue 'vertical_check; }
            }
        }
        if smudge_fix { return i; }
    }

    0
}

#[allow(dead_code)]
fn draw_pattern(pattern: &Vec<Vec<char>>, reflection: Option<usize>) {
    for y in 0..pattern.len() {
        if let Some(r) = reflection {
            if r == 100*y {
                for _ in 0..pattern[0].len() {
                    print!("-");
                }
                println!();
            }
        }
        for x in 0..pattern[0].len() {
            if let Some(r) = reflection {
                if r == x {
                    print!("|");
                }
            }
            print!("{}", pattern[y][x]);
        }
        println!();
    }
    println!();
}