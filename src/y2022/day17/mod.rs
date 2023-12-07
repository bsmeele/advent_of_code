use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

const ROCKS: [[[char; 7]; 4]; 5] = [
    [['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '@', '@', '@', '@', '.']],
    [['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '.', '@', '.', '.', '.'],
     ['.', '.', '@', '@', '@', '.', '.'],
     ['.', '.', '.', '@', '.', '.', '.']],
    [['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '.', '.', '@', '.', '.'],
     ['.', '.', '.', '.', '@', '.', '.'],
     ['.', '.', '@', '@', '@', '.', '.']],
    [['.', '.', '@', '.', '.', '.', '.'],
     ['.', '.', '@', '.', '.', '.', '.'],
     ['.', '.', '@', '.', '.', '.', '.'],
     ['.', '.', '@', '.', '.', '.', '.']],
    [['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '.', '.', '.', '.', '.'],
     ['.', '.', '@', '@', '.', '.', '.'],
     ['.', '.', '@', '@', '.', '.', '.']]
];
const EMPTY_ROW: [char; 7] = ['.', '.', '.', '.', '.', '.', '.'];

pub fn pyroclastic_flow() {
    let test = false;
    let filename = if test { "src/y2022/day17/test" } else { "src/y2022/day17/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut jet_pattern: Vec<char> = Vec::new();
    let mut cave: Vec<[char; 7]> = Vec::new();
    let mut rock_id = 0;
    let mut jet_id = 0;
    let mut num_rocks = 0;
    let part = 1;
    let max_rocks = if part == 1 { 2022 } else { 1000000000000 };

    for line in reader.lines() {
        let line = line.unwrap();
        jet_pattern = line.chars().collect();
    }

    // spawn(&mut cave, 0);
    // draw_cave(&cave);

    while num_rocks < max_rocks {
        spawn(&mut cave, rock_id);
        rock_id += 1;
        if rock_id >= ROCKS.len() { rock_id = 0; }

        if jet_pattern[jet_id] == '<' { push(&mut cave, -1); } else { push(&mut cave, 1); }
        jet_id += 1;
        if jet_id >= jet_pattern.len() { jet_id = 0; }
        while lower(&mut cave) {
            if jet_pattern[jet_id] == '<' { push(&mut cave, -1); } else { push(&mut cave, 1); }
            jet_id += 1;
            if jet_id >= jet_pattern.len() { jet_id = 0; }
        }
        num_rocks += 1;

        if rock_id == 0 && jet_id == 0 { break; }
    }

    let repeat = num_rocks;
    let rep_height = get_height(&cave);
    num_rocks *= (max_rocks as f32/repeat as f32).floor() as usize;
    println!("{} {} {}", repeat, rep_height, num_rocks );

    while num_rocks < max_rocks {
        spawn(&mut cave, rock_id);
        rock_id += 1;
        if rock_id >= ROCKS.len() { rock_id = 0; }

        if jet_pattern[jet_id] == '<' { push(&mut cave, -1); } else { push(&mut cave, 1); }
        jet_id += 1;
        if jet_id >= jet_pattern.len() { jet_id = 0; }
        while lower(&mut cave) {
            if jet_pattern[jet_id] == '<' { push(&mut cave, -1); } else { push(&mut cave, 1); }
            jet_id += 1;
            if jet_id >= jet_pattern.len() { jet_id = 0; }
        }
        num_rocks += 1;
    }

    // draw_cave(&cave);

    println!("Year 2022 day 17 part 1: {}", get_height(&cave) + rep_height * ((max_rocks as f32/repeat as f32).floor() as usize - 1));
}

#[allow(dead_code)]
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[allow(dead_code)]
fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    if a < b {
        swap(&mut a, &mut b);
    }
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

#[allow(dead_code)]
fn draw_cave(cave: &Vec<[char; 7]>) {
    for r in cave.into_iter().rev() {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

fn push(cave: &mut Vec<[char; 7]>, dir: isize) {
    // Check if can be pushed
    let mut can_push = true;
    'row_loop: for r in cave.into_iter().rev() {
        for i in 0..r.len() {
            if r[i] == '@' {
                if dir == -1 && i == 0  {
                    can_push = false;
                    break 'row_loop;
                }
                if dir == 1 && i == r.len()-1 {
                    can_push = false;
                    break 'row_loop;
                }
                if r[(i as isize + dir) as usize] == '#' {
                    can_push = false;
                    break 'row_loop;
                }
            }
        }
    }

    // If it can, push rock
    if can_push {
        for r in cave.into_iter().rev() {
            if dir == 1 {
                for i in (0..r.len()).rev() {
                    if r[i] == '@' {
                        r[i] = '.';
                        r[(i as isize + dir) as usize] = '@';
                    }
                }
            } else {
                for i in 0..r.len() {
                    if r[i] == '@' {
                        r[i] = '.';
                        r[(i as isize + dir) as usize] = '@';
                    }
                }
            }
        }
    }
}

fn lower(cave: &mut Vec<[char; 7]>) -> bool {
    // Check if can be lowered
    let mut can_lower = true;
    'row_loop: for r in (0..cave.len()).rev() {
        for c in 0..cave[0].len() {
            if cave[r][c] == '@' {
                if r == 0 {
                    can_lower = false;
                    break 'row_loop;
                }
                if cave[r - 1][c] == '#' {
                    can_lower = false;
                    break 'row_loop;
                }
            }
        }
    }
    // If it can, lower rock and return true
    if can_lower {
        for r in 0..cave.len() {
            for c in 0..cave[0].len() {
                if cave[r][c] == '@' {
                    cave[r][c] = '.';
                    cave[r-1][c] = '@';
                }
            }
        }
    } else {
        // If it can not, settle the rock and return false
        for r in 0..cave.len() {
            for c in 0..cave[0].len() {
                if cave[r][c] == '@' {
                    cave[r][c] = '#';
                }
            }
        }
    }

    can_lower
}

fn get_height(cave: &Vec<[char; 7]>) -> usize {
    let mut highest = 0;
    // Search from top to bottom
    for i in (0..cave.len()).rev() {
        if cave[i].contains(&'#') {
            highest = i + 1;
            break;
        }
    }

    highest
}

fn spawn(cave: &mut Vec<[char; 7]>, rock_id: usize) {
    // Get highest rock
    let highest = get_height(&cave);

    // Ensure cave has room to spawn
    let top = highest + 7;
    if cave.len() < top {
        for _ in 0..(top - cave.len()) { cave.push(EMPTY_ROW) }
    }

    // Spawn rock
    let mut tmp = highest + 3;
    for line in ROCKS[rock_id].into_iter().rev() {
        cave[tmp] = line;
        tmp += 1;
    }
}