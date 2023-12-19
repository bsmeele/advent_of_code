use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn clumsy_crucible() {
    let test = false;
    let filename = if test { "src/y2023/day17/test" } else { "src/y2023/day17/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: Vec<Vec<usize>> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        map.push(line.chars().map(|x| x.to_digit(10).unwrap() as usize).collect());
    }

    let start = (0, 0);
    let dest = (map[0].len()-1, map.len()-1);
    let mut heat_map: HashMap<((usize, usize), Direction, usize), usize> = HashMap::new();

    // draw_map(&map);
    minimize_heat_loos(&map, &mut heat_map, start, 3, 0);

    let mut min_heat = usize::MAX;
    for ((e, _, _), d) in heat_map {
        if e == dest && d < min_heat { min_heat = d; }
    }
    println!("Year 2023 day 17 part 1: {}", min_heat);

    heat_map = HashMap::new();
    minimize_heat_loos(&map, &mut heat_map, start, 10, 4);
    min_heat = usize::MAX;
    for ((e, _, _), d) in heat_map {
        if e == dest && d < min_heat { min_heat = d; }
    }
    println!("Year 2023 day 17 part 2: {}", min_heat);

}

fn minimize_heat_loos(map: &Vec<Vec<usize>>, heat_map: &mut HashMap<((usize, usize), Direction, usize), usize>, start: (usize, usize), max_straight: usize, min_straight: usize) {
    let mut heat;
    let bounds = (map[0].len(), map.len());
    let mut q: VecDeque<((usize, usize), Direction, usize)> = VecDeque::from([(start, Direction::East, 0)]);
    heat_map.insert((start, Direction::East, 0), 0);
    let mut next_s: usize;
    let mut next_loc: (usize, usize);

    while let Some(((x, y), d, s)) = q.pop_front() {
        heat = heat_map[&((x, y), d, s)];

        if s < min_straight {
            next_loc = match d {
                Direction::North if y > 0 => (x, y-1),
                Direction::East if x + 1 < bounds.0 => (x+1, y),
                Direction::South if y + 1 < bounds.1 => (x, y+1),
                Direction::West if x > 0 => (x-1, y),
                _ => continue,
            };
            if !heat_map.contains_key(&(next_loc, d, s+1)) || heat + map[next_loc.1][next_loc.0] < heat_map[&(next_loc, d, s+1)] {
                heat_map.insert((next_loc, d, s+1), heat + map[next_loc.1][next_loc.0]);
                q.push_back((next_loc, d, s+1));
            }
            continue;
        }

        if y > 0 && d != Direction::South && !(d == Direction::North && s >= max_straight) {
            next_s = if d == Direction::North { s + 1 } else { 1 };
            if !heat_map.contains_key(&((x, y-1), Direction::North, next_s)) || heat + map[y-1][x] < heat_map[&((x, y-1), Direction::North, next_s)] {
                heat_map.insert(((x, y-1), Direction::North, next_s), heat + map[y-1][x]);
                q.push_back(((x, y-1), Direction::North, next_s));
            }
        }
        if x + 1 < bounds.0 && d != Direction::West && !(d == Direction::East && s >= max_straight) {
            next_s = if d == Direction::East { s + 1 } else { 1 };
            if !heat_map.contains_key(&((x+1, y), Direction::East, next_s)) || heat + map[y][x+1] < heat_map[&((x+1, y), Direction::East, next_s)] {
                heat_map.insert(((x+1, y), Direction::East, next_s), heat + map[y][x+1]);
                q.push_back(((x+1, y), Direction::East, next_s));
            }
        }
        if y + 1 < bounds.1 && d != Direction::North && !(d == Direction::South && s >= max_straight) {
            next_s = if d == Direction::South { s + 1 } else { 1 };
            if !heat_map.contains_key(&((x, y+1), Direction::South, next_s)) || heat + map[y+1][x] < heat_map[&((x, y+1), Direction::South, next_s)] {
                heat_map.insert(((x, y+1), Direction::South, next_s), heat + map[y+1][x]);
                q.push_back(((x, y+1), Direction::South, next_s));
            }
        }
        if x > 0 && d != Direction::East && !(d == Direction::West && s >= max_straight) {
            next_s = if d == Direction::West { s + 1 } else { 1 };
            if !heat_map.contains_key(&((x-1, y), Direction::West, next_s)) || heat + map[y][x-1] < heat_map[&((x-1, y), Direction::West, next_s)] {
                heat_map.insert(((x-1, y), Direction::West, next_s), heat + map[y][x-1]);
                q.push_back(((x-1, y), Direction::West, next_s));
            }
        }
    }
}

#[allow(dead_code)]
fn draw_map(map: &Vec<Vec<usize>>) {
    let mut largest = 0;
    for r in map {
        for c in r {
            if *c > largest { largest = *c; }
        }
    }

    let padding = (largest as f32).log10().floor() as usize + 1;

    for y in 0..map.len() {
        for x in 0..map[0].len() {
            print!("{}", map[y][x]);
            for _ in ((map[y][x] as f32).log10().floor() as usize + 1)..=padding {
                print!(" ");
            }
        }
        println!();
    }
    println!();
}

#[derive(Eq, PartialEq, Debug, Hash, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
