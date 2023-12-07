use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

pub fn rope_bridge() {
    let filename = "src/y2022/day9/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut rope1: Vec<(i32, i32)> = vec![(0, 0), (0, 0)];
    let mut rope2: Vec<(i32, i32)> = Vec::new();
    for _ in 0..10 { rope2.push((0, 0)); }

    let mut visited1: HashSet<(i32, i32)> = HashSet::new();
    let mut visited2: HashSet<(i32, i32)> = HashSet::new();
    visited1.insert((0, 0));
    visited2.insert((0, 0));

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(' ').collect();
        let dir = match line[0].parse::<char>().unwrap() {
            'D' => (0, -1),
            'U' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => (0, 0),
        };
        let count = line[1].parse::<u8>().unwrap();

        // println!("{} {}", line[0], line[1]);

        for _ in 0..count {
            // ----- Part 1 -----
            rope1[0].0 += dir.0;
            rope1[0].1 += dir.1;

            rope1[1] = move_rope(rope1[0], rope1[1]);

            visited1.insert(rope1[1]);

            // ----- Part 2 -----
            rope2[0].0 += dir.0;
            rope2[0].1 += dir.1;

            for i in 1..rope2.len() {
                rope2[i] = move_rope(rope2[i-1], rope2[i]);
            }

            // for i in &rope2 { print!("{:?} ", i); }
            // println!();

            visited2.insert(rope2[rope2.len()-1]);
        }

        // println!();
    }

    println!("Day 9 part 1: {}", visited1.len());
    println!("Day 9 part 2: {}", visited2.len());
}

fn move_rope(p: (i32, i32), c: (i32, i32)) -> (i32, i32) {
    let mut c = c;
    let mut dx = p.0 - c.0;
    let mut dy = p.1 - c.1;

    if dx*dx + dy*dy <= 2 { return c; }

    if dx*dx == 4 { dx /= 2; }
    if dy*dy == 4 { dy /= 2; }

    c.0 += dx;
    c.1 += dy;

    c
}