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

    let mut tiles: HashSet<((usize, usize), Direction)> = HashSet::new();
    simulate_beam(&map, (0, 0), Direction::East, &mut tiles);
    // draw_map(&map);
    // draw_tiles(&tiles, (map[0].len(), map.len()));

    println!("Year 2023 day 16 part 1: {}", get_score(&tiles));

    let mut res: usize;
    let mut max_energized = 0;
    // let mut max_tiles = HashSet::new();
    for x in 0..map[0].len() {
        tiles = HashSet::new();
        simulate_beam(&map, (x, 0), Direction::South, &mut tiles);
        res = get_score(&tiles);
        if res > max_energized {
            max_energized = res;
            // max_tiles = tiles.clone();
        }

        tiles = HashSet::new();
        simulate_beam(&map, (x, map.len()-1), Direction::North, &mut tiles);
        res = get_score(&tiles);
        if res > max_energized {
            max_energized = res;
            // max_tiles = tiles.clone();
        }
    }

    for y in 0..map.len() {
        tiles = HashSet::new();
        simulate_beam(&map, (0, y), Direction::East, &mut tiles);
        res = get_score(&tiles);
        if res > max_energized {
            max_energized = res;
            // max_tiles = tiles.clone();
        }

        tiles = HashSet::new();
        simulate_beam(&map, (map[0].len()-1, y), Direction::West, &mut tiles);
        res = get_score(&tiles);
        if res > max_energized {
            max_energized = res;
            // max_tiles = tiles.clone();
        }
    }
    // draw_map(&max_map);
    // draw_tiles(&max_tiles, (map[0].len(), map.len()));

    println!("Year 2023 day 16 part 2: {}", max_energized);
}

fn get_score(tiles: &HashSet<((usize, usize), Direction)>) -> usize {
    let mut tmp: HashSet<(usize, usize)> = HashSet::new();
    for (e, _) in tiles {
        tmp.insert(*e);
    }
    tmp.len()
}

#[allow(dead_code)]
fn draw_tiles(tiles: &HashSet<(usize, usize)>, bounds: (usize, usize)) {
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            if tiles.contains(&(x, y)) { print!("#"); }
            else { print!(".") };
        }
        println!();
    }
    println!();
}

fn simulate_beam(map: &Vec<Vec<char>>, start: (usize, usize), direction: Direction, tiles: &mut HashSet<((usize, usize), Direction)>) {
    let mut loc = start;
    let mut direction = direction;

    loop {
        if tiles.contains(&(loc, direction)) { break; };

        tiles.insert((loc, direction));

        match direction {
            Direction::North => {
                match map[loc.1][loc.0] {
                    '\\' => direction = Direction::West,
                    '/' => direction = Direction::East,
                    '-' => {
                        if loc.0 > 0 { simulate_beam(map, (loc.0 - 1, loc.1), Direction::West, tiles); }
                        if loc.0 + 1 < map[0].len() { simulate_beam(map, loc, Direction::East, tiles); }
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
                        if loc.1 > 0 { simulate_beam(map, (loc.0, loc.1 - 1), Direction::North, tiles); }
                        if loc.1 + 1 < map.len() { simulate_beam(map, (loc.0, loc.1 + 1), Direction::South, tiles); }
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
                        if loc.0 > 0 { simulate_beam(map, (loc.0 - 1, loc.1), Direction::West, tiles); }
                        if loc.0 + 1 < map[0].len() { simulate_beam(map, loc, Direction::East, tiles); }
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
                        if loc.1 > 0 { simulate_beam(map, (loc.0, loc.1 - 1), Direction::North, tiles); }
                        if loc.1 + 1 < map.len() { simulate_beam(map, (loc.0, loc.1 + 1), Direction::South, tiles); }
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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}