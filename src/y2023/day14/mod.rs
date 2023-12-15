use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parabolic_reflector_dish() {
    let test = false;
    let filename = if test { "src/y2023/day14/test" } else { "src/y2023/day14/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut round: HashSet<(usize, usize)> = HashSet::new();
    let mut cube: HashSet<(usize, usize)> = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    let cycles = 1000000000;

    for line in reader.lines().map(|x| x.unwrap()) {
        x = 0;
        for c in line.chars() {
            match c {
                'O' => { round.insert((x, y)); },
                '#' => { cube.insert((x, y)); },
                '.' => (),
                _ => panic!("Unreachable"),
            }
            x += 1;
        }
        y += 1;
    }

    // draw_map(&round, &cube, (x, y));
    tilt(&mut round, &cube, (x, y), Direction::North);
    // draw_map(&round, &cube, (x, y));

    let part1 = get_load(&round, (x, y));
    println!("Year 2023 day 14 part 1: {}", part1);

    let mut enc: usize;
    let mut enc_map: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut part2 = 0;
    for c in 0..cycles {
        // println!("----- Cycle {} -----", c);
        tilt(&mut round, &cube, (x, y), Direction::North);
        tilt(&mut round, &cube, (x, y), Direction::West);
        tilt(&mut round, &cube, (x, y), Direction::South);
        tilt(&mut round, &cube, (x, y), Direction::East);
        // draw_map(&round, &cube, (x, y));

        enc = encode(&round, (x, y));
        // println!("{} {}", enc, get_load(&round, (x, y)));
        if enc_map.contains_key(&enc) {
            let id = (cycles - enc_map[&enc].0 - 1)%(c - enc_map[&enc].0) + enc_map[&enc].0;
            for (c, l) in enc_map.values() { if *c == id { part2 = *l; } }
            break;
        }
        enc_map.insert(enc, (c, get_load(&round, (x, y))));
    }

    // let part2 = get_load(&round, (x, y));
    println!("Year 2023 day 14 part 2: {}", part2);
}

fn encode(round: &HashSet<(usize, usize)>, bounds: (usize, usize)) -> usize {
    let mut acc = 0;
    for (x, y) in round.iter() {
        acc += x + y * bounds.0;
    }

    acc
}

fn tilt(round: &mut HashSet<(usize, usize)>, cube: &HashSet<(usize, usize)>, bounds: (usize, usize), direction: Direction) {
    let mut to_update: Vec<(usize, usize)> = Vec::new();
    loop {
        for (x, y) in round.iter() {
            match direction {
                Direction::North => if *y > 0 && !round.contains(&(*x, y-1)) && !cube.contains(&(*x, y-1)) { to_update.push((*x, *y))},
                Direction::East => if x+1 < bounds.0 && !round.contains(&(x+1, *y)) && !cube.contains(&(x+1, *y)) { to_update.push((*x, *y)); },
                Direction::South => if y+1 < bounds.1 && !round.contains(&(*x, y+1)) && !cube.contains(&(*x, y+1)) { to_update.push((*x, *y)); },
                Direction::West => if *x > 0 && !round.contains(&(x-1, *y)) && !cube.contains(&(x-1, *y)) { to_update.push((*x, *y)); },
            }
        }
        if to_update.is_empty() { break; }
        for (x, y) in &to_update {
            round.remove(&(*x, *y));
            match direction {
                Direction::North => round.insert((*x, y-1)),
                Direction::East => round.insert((x+1, *y)),
                Direction::South => round.insert((*x, y+1)),
                Direction::West => round.insert((x-1, *y)),
            };
        }
        to_update.clear();
    }
}

fn get_load(round: &HashSet<(usize, usize)>, bounds: (usize, usize)) -> usize {
    let mut acc = 0;
    for (_, y) in round.iter() {
        acc += bounds.1 - y;
    }

    acc
}

#[allow(dead_code)]
fn draw_map(round: &HashSet<(usize, usize)>, cube: &HashSet<(usize, usize)>, bounds: (usize, usize)) {
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            if round.contains(&(x, y)) { print!("O"); }
            else if cube.contains(&(x, y)) { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
    println!();
}

enum Direction {
    North,
    East,
    South,
    West,
}