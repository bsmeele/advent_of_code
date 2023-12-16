use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn the_floor_will_be_lava() {
    let test = false;
    let filename = if test { "src/y2023/day16/test" } else { "src/y2023/day16/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        map.push(line.chars().collect())
    }
    // draw_map(&map);

    let mut map_part1 = map.clone();
    let tiles = simulate_beam(&mut map_part1, (0, 0), Direction::East, HashSet::new());
    // draw_map(&map_part1);

    println!("Year 2023 day 16 part 1: {}", tiles.len());

    let mut map_part2;
    let mut res;
    let mut max_energized = 0;
    let mut max_map = map.clone();
    for x in 0..map[0].len() {
        map_part2 = map.clone();
        res = simulate_beam(&mut map_part2, (x, 0), Direction::South, HashSet::new()).len();
        if res > max_energized {
            max_energized = res;
            max_map = map_part2.clone();
        }
        map_part2 = map.clone();
        res = simulate_beam(&mut map_part2, (x, map.len()-1), Direction::North, HashSet::new()).len();
        if res > max_energized {
            max_energized = res;
            max_map = map_part2.clone();
        }
    }

    for y in 0..map.len() {
        map_part2 = map.clone();
        res = simulate_beam(&mut map_part2, (0, y), Direction::East, HashSet::new()).len();
        if res > max_energized {
            max_energized = res;
            max_map = map_part2.clone();
        }
        map_part2 = map.clone();
        res = simulate_beam(&mut map_part2, (map[0].len()-1, y), Direction::West, HashSet::new()).len();
        if res > max_energized {
            max_energized = res;
            max_map = map_part2.clone();
        }
    }
    // draw_map(&max_map);

    println!("Year 2023 day 16 part 2: {}", max_energized);
}

fn simulate_beam(map: &mut Vec<Vec<char>>, start: (usize, usize), direction: Direction, tiles: HashSet<(usize, usize)>) -> HashSet<(usize, usize)> {
    let mut loc = start;
    let mut direction = direction;
    let mut tiles = tiles;

    loop {
        tiles.insert(loc);

        match (map[loc.1][loc.0], direction) {
            ('^', Direction::North) => break,
            ('>', Direction::East) => break,
            ('v', Direction::South) => break,
            ('<', Direction::West) => break,
            _ => (),
        }

        match map[loc.1][loc.0] {
            '\\' | '/' | '-' | '|' | '*' => (),
            '.' => {
                match direction {
                    Direction::North => map[loc.1][loc.0] = '^',
                    Direction::East => map[loc.1][loc.0] = '>',
                    Direction::South => map[loc.1][loc.0] = 'v',
                    Direction::West => map[loc.1][loc.0] = '<'
                }
            },
            '^' | '>' | 'v' | '<' => map[loc.1][loc.0] = '2',
            c if c.is_numeric() => map[loc.1][loc.0] = if let Some(c) = char::from_digit(c.to_digit(10).unwrap() + 1, 10) { c } else { '*' },
            _ => panic!("Unreachable"),
        }

        match direction {
            Direction::North => {
                match map[loc.1][loc.0] {
                    '\\' => direction = Direction::West,
                    '/' => direction = Direction::East,
                    '-' => {
                        if loc.0 > 0 { tiles.extend(simulate_beam(map, (loc.0 - 1, loc.1), Direction::West, tiles.clone())); }
                        if loc.0 + 1 < map[0].len() { tiles.extend(simulate_beam(map, loc, Direction::East, tiles.clone())); }
                        break;
                    },
                    _ => (),
                }
            },
            Direction::East => {
                match map[loc.1][loc.0] {
                    '\\' => direction = Direction::South,
                    '/' => direction = Direction::North,
                    '|' => {
                        if loc.1 > 0 { tiles.extend(simulate_beam(map, (loc.0, loc.1 - 1), Direction::North, tiles.clone())); }
                        if loc.1 + 1 < map.len() { tiles.extend(simulate_beam(map, (loc.0, loc.1 + 1), Direction::South, tiles.clone())); }
                        break;
                    },
                    _ => (),
                }
            },
            Direction::South => {
                match map[loc.1][loc.0] {
                    '\\' => direction = Direction::East,
                    '/' => direction = Direction::West,
                    '-' => {
                        if loc.0 > 0 { tiles.extend(simulate_beam(map, (loc.0 - 1, loc.1), Direction::West, tiles.clone())); }
                        if loc.0 + 1 < map[0].len() { tiles.extend(simulate_beam(map, loc, Direction::East, tiles.clone())); }
                        break;
                    }
                    _ => (),
                }
            },
            Direction::West => {
                match map[loc.1][loc.0] {
                    '\\' => direction = Direction::North,
                    '/' => direction = Direction::South,
                    '|' => {
                        if loc.1 > 0 { tiles.extend(simulate_beam(map, (loc.0, loc.1 - 1), Direction::North, tiles.clone())); }
                        if loc.1 + 1 < map.len() { tiles.extend(simulate_beam(map, (loc.0, loc.1 + 1), Direction::South, tiles.clone())); }
                        break;
                    },
                    _ => (),
                }
            },
        }
        match direction {
            Direction::North => if loc.1 > 0 { loc.1 -= 1; } else { break; },
            Direction::East => if loc.0 + 1 < map[0].len() { loc.0 += 1; } else { break; },
            Direction::South => if loc.1 + 1 < map.len() { loc.1 +=1; } else { break; },
            Direction::West => if loc.0 > 0 { loc.0 -= 1; } else { break; },
                    }
    }

    tiles
}

#[allow(dead_code)]
fn draw_map(map: &Vec<Vec<char>>) {
    for r in map {
        for c in r {
            print!("{}", c);
        }
        println!();
    }
    println!();
}

#[derive(Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}