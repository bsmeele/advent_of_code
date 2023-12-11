use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};


pub fn pipe_maze() {
    let test = false;
    let filename = if test { "src/y2023/day10/test8" } else { "src/y2023/day10/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();
    let mut start = (0, 0);
    let mut y = 0;
    let mut x = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                'S' => {
                    start = (x, y);
                    row.push('S');
                },
                '-' => row.push('─'),
                '|' => row.push('│'),
                'F' => row.push('┌'),
                '7' => row.push('┐'),
                'J' => row.push('┘'),
                'L' => row.push('└'),
                _ => row.push(c),
            }
            x += 1;
        }
        map.push(row);
        y += 1;
        x = 0;
    }

    // println!("start: ({}, {})", start.0, start.1);
    // print_map(&map, &Vec::new());

    let mut biggest_loop: Vec<(usize, usize)> = Vec::new();

    if start.0 + 1 != map[0].len() {
        match map[start.1][start.0 + 1] {
            '─' | '┐' | '┘' => {
                let (is_loop, res) = get_loop(&map, (start.0 + 1, start.1));
                // print_map(&map, &res);
                if is_loop && res.len() > biggest_loop.len() { biggest_loop = res; }
            },
            _ => (),
        }
    }
    if start.0 != 0 {
        match map[start.1][start.0 - 1] {
            '─' | '┌' | '└' => {
                let (is_loop, res) = get_loop(&map, (start.0 - 1, start.1));
                // print_map(&map, &res);
                if is_loop && res.len() > biggest_loop.len() { biggest_loop = res; }
            },
            _ => (),
        }
    }
    if start.1 + 1 != map.len() {
        match map[start.1 + 1][start.0] {
            '│' | '└' | '┘' => {
                let (is_loop, res) = get_loop(&map, (start.0, start.1 + 1));
                // print_map(&map, &res);
                if is_loop && res.len() > biggest_loop.len() { biggest_loop = res; }
            },
            _ => (),
        }
    }
    if start.1 != 0 {
        match map[start.1 - 1][start.0] {
            '│' | '┌' | '┐' => {
                let (is_loop, res) = get_loop(&map, (start.0, start.1 - 1));
                // print_map(&map, &res);
                if is_loop && res.len() > biggest_loop.len() { biggest_loop = res; }
            },
            _ => (),
        }
    }
    biggest_loop.push(start);

    // println!("{:?}", biggest_loop);
    // print_map(&map, &biggest_loop);

    println!("Year 2023 day 10 part 1: {}", ((biggest_loop.len() as f32)/ 2.).ceil());

    let loop_set = biggest_loop.clone().into_iter().collect::<HashSet<_>>();
    let max = (map[0].len(), map.len());
    let mut left = 0;
    let mut right = 0;
    let mut orientation =
        if start.0 + 1 < map[0].len() && biggest_loop[0].0 == start.0 + 1 && biggest_loop[0].1 == start.1 { Orientation::Right }
        else if start.0 > 0 && biggest_loop[0].0 == start.0 - 1 && biggest_loop[0].1 == start.1 { Orientation::Left }
        else if start.1 + 1 < map.len() && biggest_loop[0].0 == start.0 && biggest_loop[0].1 == start.1 + 1 { Orientation::Up }
        else { Orientation::Down };
    for (x, y) in &biggest_loop {
        match (map[*y][*x], orientation) {
            ('─', _)  | ('│', _) => {
                if let Ok((lx, ly)) = Orientation::get_left(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(lx, ly)) && map[ly][lx] != 'l' {
                        map[ly][lx] = 'l';
                        left += 1;
                    }
                }
                if let Ok((rx, ry)) = Orientation::get_right(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(rx, ry)) && map[ry][rx] != 'r' {
                        map[ry][rx] = 'r';
                        right += 1;
                    }
                }
            },
            ('┌', Orientation::Up) => {
                if let Ok((lx, ly)) = Orientation::get_left(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(lx, ly)) && map[ly][lx] != 'l' {
                        map[ly][lx] = 'l';
                        left += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'l' {
                        map[uy][ux] = 'l';
                        left += 1;
                    }
                }
            },
            ('┌', Orientation::Left) => {
                if let Ok((rx, ry)) = Orientation::get_right(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(rx, ry)) && map[ry][rx] != 'r' {
                        map[ry][rx] = 'r';
                        right += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'r' {
                        map[uy][ux] = 'r';
                        right += 1;
                    }
                }
            },
            ('┐', Orientation::Up) => {
                if let Ok((rx, ry)) = Orientation::get_right(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(rx, ry)) && map[ry][rx] != 'r' {
                        map[ry][rx] = 'r';
                        right += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'r' {
                        map[uy][ux] = 'r';
                        right += 1;
                    }
                }
            },
            ('┐', Orientation::Right) => {
                if let Ok((lx, ly)) = Orientation::get_left(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(lx, ly)) && map[ly][lx] != 'l' {
                        map[ly][lx] = 'l';
                        left += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'l' {
                        map[uy][ux] = 'l';
                        left += 1;
                    }
                }
            },
            ('┘', Orientation::Down) => {
                if let Ok((lx, ly)) = Orientation::get_left(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(lx, ly)) && map[ly][lx] != 'l' {
                        map[ly][lx] = 'l';
                        left += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'l' {
                        map[uy][ux] = 'l';
                        left += 1;
                    }
                }
            },
            ('┘', Orientation::Right) => {
                if let Ok((rx, ry)) = Orientation::get_right(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(rx, ry)) && map[ry][rx] != 'r' {
                        map[ry][rx] = 'r';
                        right += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'r' {
                        map[uy][ux] = 'r';
                        right += 1;
                    }
                }
            },
            ('└', Orientation::Down) => {
                if let Ok((rx, ry)) = Orientation::get_right(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(rx, ry)) && map[ry][rx] != 'r' {
                        map[ry][rx] = 'r';
                        right += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'r' {
                        map[uy][ux] = 'r';
                        right += 1;
                    }
                }
            },
            ('└', Orientation::Left) => {
                if let Ok((lx, ly)) = Orientation::get_left(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(lx, ly)) && map[ly][lx] != 'l' {
                        map[ly][lx] = 'l';
                        left += 1;
                    }
                }
                if let Ok((ux, uy)) = Orientation::get_up(orientation, (*x, *y), max) {
                    if !loop_set.contains(&(ux, uy)) && map[uy][ux] != 'l' {
                        map[uy][ux] = 'l';
                        left += 1;
                    }
                }
            },
            ('S', _) => break,
            _ => panic!("Unreachable")
        }
        orientation = match (map[*y][*x], orientation) {
            ('S', _) => break,
            ('─', Orientation::Right) => Orientation::Right,
            ('─', Orientation::Left) => Orientation::Left,
            ('│', Orientation::Up) => Orientation::Up,
            ('│', Orientation::Down) => Orientation::Down,
            ('┌', Orientation::Up) => Orientation::Right,
            ('┌', Orientation::Left) => Orientation::Down,
            ('┐', Orientation::Up) => Orientation::Left,
            ('┐', Orientation::Right) => Orientation::Down,
            ('┘', Orientation::Down) => Orientation::Left,
            ('┘', Orientation::Right) => Orientation::Up,
            ('└', Orientation::Down) => Orientation::Right,
            ('└', Orientation::Left) => Orientation::Up,
            _ => panic!("Uncreachable"),
        };
    }

    let mut changed;
    loop {
        changed = false;
        for y in 0..max.1 {
            for x in 0..max.0 {
                match map[y][x] {
                    'l' => {
                        if x + 1 < max.0 && !loop_set.contains(&(x + 1, y)) && map[y][x + 1] != 'l' {
                            map[y][x + 1] = 'l';
                            left += 1;
                            changed = true;
                        }
                        if x > 0 && !loop_set.contains(&(x - 1, y)) && map[y][x - 1] != 'l' {
                            map[y][x - 1] = 'l';
                            left += 1;
                            changed = true;
                        }
                        if y + 1 < max.1 && !loop_set.contains(&(x, y + 1)) && map[y + 1][x] != 'l' {
                            map[y + 1][x] = 'l';
                            left += 1;
                            changed = true;
                        }
                        if y > 0 && !loop_set.contains(&(x, y - 1)) && map[y - 1][x] != 'l' {
                            map[y - 1][x] = 'l';
                            left += 1;
                            changed = true;
                        }
                    },
                    'r' => {
                        if x + 1 < max.0 && !loop_set.contains(&(x + 1, y)) && map[y][x + 1] != 'r' {
                            map[y][x + 1] = 'r';
                            right += 1;
                            changed = true;
                        }
                        if x > 0 && !loop_set.contains(&(x - 1, y)) && map[y][x - 1] != 'r' {
                            map[y][x - 1] = 'r';
                            right += 1;
                            changed = true;
                        }
                        if y + 1 < max.1 && !loop_set.contains(&(x, y + 1)) && map[y + 1][x] != 'r' {
                            map[y + 1][x] = 'r';
                            right += 1;
                            changed = true;
                        }
                        if y > 0 && !loop_set.contains(&(x, y - 1)) && map[y - 1][x] != 'r' {
                            map[y - 1][x] = 'r';
                            right += 1;
                            changed = true;
                        }
                    },
                    _ => ()
                }
            }
        }
        if !changed { break; }
    }

    let mut part2 = 0;
    for x in 0..max.0 {
        if map[0][x] == 'l' {
            part2 = right;
            break;
        } else if map[0][x] == 'r' {
            part2 = left;
            break;
        } else if map[max.1 - 1][x] == 'l' {
            part2 = right;
            break;
        } else if map[max.1 - 1][x] == 'r' {
            part2 = left;
            break;
        }
    }
    for y in 0..max.1 {
        if map[y][0] == 'l' {
            part2 = right;
            break;
        } else if map[y][0] == 'r' {
            part2 = left;
            break;
        } else if map[y][max.0 - 1] == 'l' {
            part2 = right;
            break;
        } else if map[y][max.0 - 1] == 'r' {
            part2 = left;
            break;
        }
    }

    // print_map(&map, &biggest_loop);

    println!("Year 2023 day 10 part 2: {}", part2);
}

fn get_loop(map: &Vec<Vec<char>>, start: (usize, usize)) -> (bool, Vec<(usize, usize)>) {
    let mut current_loop: Vec<(usize, usize)> = Vec::new();
    let mut current = start;
    let mut next = start;
    let mut prev = start;
    let mut is_loop = false;

    loop {
        if let Some(p) = current_loop.last() { prev = *p; }
        current_loop.push(current);
        // println!("({}, {}), {} {:?}", current.0, current.1, map[current.1][current.0], current_loop);
        match map[current.1][current.0] {
            'S' => break,
            '─' => {
                if current.0 == 0 || current.0 + 1 == map[0].len() { break; }
                if (current.0 + 1, current.1) != prev {
                    match map[current.1][current.0 + 1] {
                        'S' => {
                            if current_loop.len() != 1
                            {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┐' | '┘' => next = (current.0 + 1, current.1),
                        _ => break,
                    }
                }
                if (current.0 - 1, current.1) != prev {
                    match map[current.1][current.0 - 1] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┌' | '└' => next = (current.0 - 1, current.1),
                        _ => break,
                    }
                }
            },
            '│' => {
                if current.1 == 0 || current.1 + 1 == map.len() { break; }
                if (current.0, current.1 + 1) != prev {
                    match map[current.1 + 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '└' | '┘' => next = (current.0, current.1 + 1),
                        _ => break,
                    }
                }
                if (current.0, current.1 - 1) != prev {
                    match map[current.1 - 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '┌' | '┐' => next = (current.0, current.1 - 1),
                        _ => break,
                    }
                }
            },
            '┌' => {
                if current.0 + 1 == map[0].len() || current.1 + 1 == map.len() { break; }
                if (current.0 + 1, current.1) != prev {
                    match map[current.1][current.0 + 1] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┐' | '┘' => next = (current.0 + 1, current.1),
                        _ => break,
                    }
                }
                if (current.0, current.1 + 1) != prev {
                    match map[current.1 + 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '└' | '┘' => next = (current.0, current.1 + 1),
                        _ => break,
                    }
                }
            },
            '┐' => {
                if current.0 == 0 || current.1 + 1 == map.len() { break; }
                if (current.0 - 1, current.1) != prev {
                    match map[current.1][current.0 - 1] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┌' | '└' => next = (current.0 - 1, current.1),
                        _ => break,
                    }
                }
                if (current.0, current.1 + 1) != prev {
                    match map[current.1 + 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '└' | '┘' => next = (current.0, current.1 + 1),
                        _ => break,
                    }
                }
            },
            '┘' => {
                if current.0 == 0 || current.1 == 0 { break; }
                if (current.0 - 1, current.1) != prev {
                    match map[current.1][current.0 - 1] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┌' | '└' => next = (current.0 - 1, current.1),
                        _ => break,
                    }
                }
                if (current.0, current.1 - 1) != prev {
                    match map[current.1 - 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '┌' | '┐' => next = (current.0, current.1 - 1),
                        _ => break,
                    }
                }
            },
            '└' => {
                if current.0 + 1 == map[0].len() || current.1 == 0 { break; }
                if (current.0 + 1, current.1) != prev {
                    match map[current.1][current.0 + 1] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '─' | '┐' | '┘' => next = (current.0 + 1, current.1),
                        _ => break,
                    }
                }
                if (current.0, current.1 - 1) != prev {
                    match map[current.1 - 1][current.0] {
                        'S' => {
                            if current_loop.len() != 1 {
                                is_loop = true;
                                break;
                            }
                        },
                        '│' | '┌' | '┐' => next = (current.0, current.1 - 1),
                        _ => break,
                    }
                }
            },
            _ => panic!("Unreachable"),
        }
        current = next;
    }

    (is_loop, current_loop)
}


#[allow(dead_code)]
fn print_map(map: &Vec<Vec<char>>, print_loop: &Vec<(usize, usize)>) {
    let print_loop = print_loop.into_iter().collect::<HashSet<_>>();

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            let c = map[y][x];
            if c == 'S' || print_loop.contains(&(x, y)) {
                print!("\x1b[32m{}\x1b[0m", c)
            } else if c == 'l' || c == 'r' {
                print!("\x1b[31m{}\x1b[0m", c)
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

#[derive(Clone, Copy)]
enum Orientation {
    Up,
    Down,
    Left,
    Right,
}
impl Orientation {
    fn get_left(orientation: Orientation, loc: (usize, usize), max: (usize, usize)) -> Result<(usize, usize), ()> {
        match orientation {
            Orientation::Up => if loc.0 > 0 { Ok((loc.0 - 1, loc.1)) } else { Err(()) },
            Orientation::Down => if loc.0 + 1 < max.0 { Ok((loc.0 + 1, loc.1)) } else { Err(()) },
            Orientation::Left => if loc.1 + 1 < max.1 { Ok((loc.0, loc.1 + 1)) } else { Err(()) },
            Orientation::Right => if loc.1 > 0 { Ok((loc.0, loc.1 - 1)) } else { Err(()) },
        }
    }
    fn get_right(orientation: Orientation, loc: (usize, usize), max: (usize, usize)) -> Result<(usize, usize), ()> {
        match orientation {
            Orientation::Up => if loc.0 + 1 < max.0 { Ok((loc.0 + 1, loc.1)) } else { Err(()) },
            Orientation::Down => if loc.0 > 0 { Ok((loc.0 - 1, loc.1)) } else { Err(()) },
            Orientation::Left => if loc.1 > 0 { Ok((loc.0, loc.1 - 1)) } else { Err(()) },
            Orientation::Right => if loc.1 + 1 < max.1 { Ok((loc.0, loc.1 + 1)) } else { Err(()) },
        }
    }
    fn get_up(orientation: Orientation, loc: (usize, usize), max: (usize, usize)) -> Result<(usize, usize), ()> {
        match orientation {
            Orientation::Up => if loc.1 > 0 { Ok((loc.0, loc.1 - 1)) } else { Err(()) },
            Orientation::Down => if loc.1 + 1 < max.1 { Ok((loc.0, loc.1 + 1)) } else { Err(()) },
            Orientation::Left => if loc.0 > 0 { Ok((loc.0 - 1, loc.1)) } else { Err(()) },
            Orientation::Right => if loc.0 + 1 < max.0 { Ok((loc.0 + 1, loc.1)) } else { Err(()) },
        }
    }
}
