use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn monkey_map() {
    let test = true;
    let filename = if test { "src/y2022/day22/test" } else { "src/y2022/day22/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut path: Vec<char> = Vec::new();
    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        x = 0;
        let line = line.chars().collect::<Vec<char>>();
        if line.is_empty() { continue; }
        if line[0].is_numeric() || line[0] == 'L' || line[0] == 'R' {
            path = line.clone();
        }
        if map.is_empty() { y = 0; }
        for c in line {
            match c {
                '.' => _ = map.insert((x, y), false),
                '#' => _ = map.insert((x, y), true),
                _ => (),
            }
            x += 1;
        }
        y += 1;
    }

    let mut start = (0,0);
    let mut x_max = 0;
    let mut y_max = 0;
    for ((x, y), _) in &map {
        if *x > x_max { x_max = *x; }
        if *y > y_max { y_max = *y; }
    }
    'find_start: loop {
        for y in 0..=y_max {
            for x in 0..=x_max {
                match map.get(&(x, y)) {
                    Some(b) if !*b => {
                        start = (x, y);
                        break 'find_start;
                    }
                    _ => continue,
                }
            }
        }
    }

    // draw_map(&map);

    let mut facing = Facing::Right;
    let mut loc = start;

    todo!()

    let result = 1000 * (loc.1 + 1) + 4 * (loc.0 + 1) + match facing {
        Facing::Right => 0,
        Facing::Down => 1,
        Facing::Left => 2,
        Facing::Up => 3,
    };
    println!("year 2022 day 22 part 1: {}", result);
}

enum Facing {
    Right,
    Down,
    Left,
    Up,
}

#[allow(dead_code)]
fn draw_map(map: &HashMap<(usize, usize), bool>) {
    let mut x_max = 0;
    let mut y_max = 0;
    for ((x, y), _) in map {
        if *x > x_max { x_max = *x; }
        if *y > y_max { y_max = *y; }
    }
    for y in 0..=y_max {
        for x in 0..=x_max {
            match map.get(&(x, y)) {
                Some(b) => if *b { print!("#") } else { print!(".") },
                None => print!(" "),
            }
        }
        println!();
    }
    println!();
}