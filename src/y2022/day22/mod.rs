use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn monkey_map() {
    let test = true;
    let filename = if test { "src/y2022/day22/test" } else { "src/y2022/day22/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut path: Vec<String> = Vec::new();
    let mut map: HashMap<(isize, isize), char> = HashMap::new();
    let mut x = 0;
    let mut y = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let chars = line.chars().collect::<Vec<char>>();
        if chars.is_empty() { continue; }
        if chars[0].is_numeric() || chars[0] == 'L' || chars[0] == 'R' {
            let mut num = String::new();
            for c in &chars {
                if c.is_numeric() { num.push(*c); }
                else {
                    if !num.is_empty() {
                    path.push(num.clone());
                    num.clear();
                        }
                    path.push(String::from(*c));
                }
            }
            if !num.is_empty() {
                path.push(num.clone());
            }
        }
        if map.is_empty() { y = 0; }
        for c in chars {
            match c {
                '.' => _ = map.insert((x, y), '.'),
                '#' => _ = map.insert((x, y), '#'),
                _ => (),
            }
            x += 1;
        }
        x = 0;
        y += 1;
    }

    // draw_map(&map);

    let start;
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
                    Some(c) if c == &'.' => {
                        start = (x, y);
                        break 'find_start;
                    }
                    _ => continue,
                }
            }
        }
    }

    let mut facing = Facing::Right;
    let mut loc = start;

    for p in path {
        if let Ok(n) = p.parse::<usize>() {
            match facing {
                Facing::Right => {
                    'move_right: for _ in 0..n {
                        map.insert(loc, '>');
                        match map.get(&(loc.0 + 1, loc.1)) {
                            Some(c) => {
                                if c == &'#' { break; }
                                else { loc.0 += 1; }
                            }
                            None => {
                                for x in 0..=x_max {
                                    if let Some(c) = map.get(&(x, loc.1)) {
                                        if c == &'#' { break 'move_right; }
                                        else {
                                            loc.0 = x;
                                            break;
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                Facing::Down => {
                    'move_down: for _ in 0..n {
                        map.insert(loc, 'V');
                        match map.get(&(loc.0, loc.1 + 1)) {
                            Some(c) => {
                                if c == &'#' { break; }
                                else { loc.1 += 1; }
                            },
                            None => {
                                for y in 0..y_max {
                                    if let Some(c) = map.get(&(loc.0, y)) {
                                        if c == &'#' { break 'move_down; }
                                        else {
                                            loc.1 = y;
                                            break;
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                Facing::Left => {
                    'move_left: for _ in 0..n {
                        map.insert(loc, '<');
                        match map.get(&(loc.0 - 1, loc.1)) {
                            Some(c) => {
                                if c == &'#' { break; }
                                else { loc.0 -= 1; }
                            },
                            None => {
                                for x in (0..=x_max).rev() {
                                    if let Some(c) = map.get(&(x, loc.1)) {
                                        if c == &'#' { break 'move_left; }
                                        else {
                                            loc.0 = x;
                                            break;
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
                Facing::Up => {
                    'move_up: for _ in 0..n {
                        map.insert(loc, 'A');
                        match map.get(&(loc.0, loc.1 - 1)) {
                            Some(c) => {
                                if c == &'#' { break; }
                                else { loc.1 -= 1; }
                            },
                            None => {
                                for y in (0..=y_max).rev() {
                                    if let Some(c) = map.get(&(loc.0, y)) {
                                        if c == &'#' { break 'move_up; }
                                        else {
                                            loc.1 = y;
                                            break;
                                        }
                                    }
                                }
                            },
                        }
                    }
                },
            }
        } else {
            facing = match p.as_str() {
                "L" => Facing::rotate(facing, false),
                "R" => Facing::rotate(facing, true),
                _ => panic!("Unreachable")
            }
        }
    }

    // draw_map(&map);

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
impl Facing {
    fn rotate(current: Self, clockwise: bool) -> Self {
        if clockwise {
            match current {
                Facing::Right => Facing::Down,
                Facing::Down => Facing::Left,
                Facing::Left => Facing::Up,
                Facing::Up => Facing::Right,
            }
        } else {
            match current {
                Facing::Right => Facing::Up,
                Facing::Down => Facing::Right,
                Facing::Left => Facing::Down,
                Facing::Up => Facing::Left,
            }
        }
    }
}

#[allow(dead_code)]
fn draw_map(map: &HashMap<(isize, isize), char>) {
    let mut x_max = 0;
    let mut y_max = 0;
    for ((x, y), _) in map {
        if *x > x_max { x_max = *x; }
        if *y > y_max { y_max = *y; }
    }
    for y in 0..=y_max {
        for x in 0..=x_max {
            match map.get(&(x, y)) {
                Some(c) => print!("{}", c),
                None => print!(" "),
            }
        }
        println!();
    }
    println!();
}