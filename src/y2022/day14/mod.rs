use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn regolith_reservoir() {
    let test = false;
    let filename = if test { "src/day14/test" } else { "src/day14/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut cave: HashMap<(u16, u16), char> = HashMap::new();
    let mut start_coords: (u16, u16) = (0xffff, 0xffff);
    let mut end_coords: (u16, u16) = (0, 0);

    for line in reader.lines() {
        let line = line.unwrap();

        let line: Vec<&str> = line.split(" -> ").collect();
        for i in 0..line.len()-1 {
            let start: Vec<&str> = line[i].split(',').collect();
            let mut xs = start[0].parse::<u16>().unwrap();
            let mut ys = start[1].parse::<u16>().unwrap();

            let end: Vec<&str> = line[i + 1].split(',').collect();
            let xe = end[0].parse::<u16>().unwrap();
            let ye = end[1].parse::<u16>().unwrap();

            if xs < start_coords.0 { start_coords.0 = xs; }
            if xe < start_coords.0 { start_coords.0 = xe; }
            if xs > end_coords.0 { end_coords.0 = xs; }
            if xe > end_coords.0 { end_coords.0 = xs; }
            if ys < start_coords.1 { start_coords.1 = ys; }
            if ye < start_coords.1 { start_coords.1 = ye; }
            if ys > end_coords.1 { end_coords.1 = ys; }
            if ye > end_coords.1 { end_coords.1 = ys; }

            if xs > xe {
                while xs >= xe {
                    cave.insert((xs, ys), '#');
                    xs -= 1;
                }
            } else if xs < xe {
                while xs <= xe {
                    cave.insert((xs, ys), '#');
                    xs += 1;
                }
            } else if ys > ye {
                while ys >= ye {
                    cave.insert((xs, ys), '#');
                    ys -= 1;
                }
            } else if ys < ye {
                while ys <= ye {
                    cave.insert((xs, ys), '#');
                    ys += 1;
                }
            }
        }
    }
    start_coords.0 -= 1;
    start_coords.1 = 0;
    end_coords.0 += 1;
    end_coords.1 += 2;

    // -----Part 1-----
    let mut num_sand = 0;
    'drop_loop: loop {
        let mut sand_coords = (500, 0);
        loop {
            if sand_coords.1 > end_coords.1 { break 'drop_loop; }
            if cave.get(&(sand_coords.0, sand_coords.1 + 1)).is_none() {
                sand_coords.1 += 1;
            } else if cave.get(&(sand_coords.0 - 1, sand_coords.1 + 1)).is_none() {
                sand_coords.0 -= 1;
                sand_coords.1 += 1;
            } else if cave.get(&(sand_coords.0 + 1, sand_coords.1 + 1)).is_none() {
                sand_coords.0 += 1;
                sand_coords.1 += 1;
            } else {
                cave.insert(sand_coords, 'o');
                break;
            }
        }
        num_sand += 1;
    }

    let mut cave1 = cave.clone();
    let mut sand_coords = (500, 0);
    loop {
        cave1.insert(sand_coords, '~');
        if sand_coords.1 > end_coords.1 { break; }
        if cave1.get(&(sand_coords.0, sand_coords.1 + 1)).is_none() {
            sand_coords.1 += 1;
        } else if cave1.get(&(sand_coords.0 - 1, sand_coords.1 + 1)).is_none() {
            sand_coords.0 -= 1;
            sand_coords.1 += 1;
        } else if cave1.get(&(sand_coords.0 + 1, sand_coords.1 + 1)).is_none() {
            sand_coords.0 += 1;
            sand_coords.1 += 1;
        }
    }

    // draw_cave(start_coords, end_coords, &cave1);
    println!("Day 14 part 1: {}", num_sand);

    // -----Part 2-----
    'drop_loop: loop {
        let mut sand_coords = (500, 0);
        loop {
            if sand_coords.1 + 1 == end_coords.1 {
                cave.insert(sand_coords, 'o');
                cave.insert((sand_coords.0, sand_coords.1 + 1), '#');
                if sand_coords.0 < start_coords.0 { start_coords.0 = sand_coords.0 }
                else if sand_coords.0 > end_coords.0 { end_coords.0 = sand_coords.0 }
                break;
            }
            else if cave.get(&(sand_coords.0, sand_coords.1 + 1)).is_none() {
                sand_coords.1 += 1;
            } else if cave.get(&(sand_coords.0 - 1, sand_coords.1 + 1)).is_none() {
                sand_coords.0 -= 1;
                sand_coords.1 += 1;
            } else if cave.get(&(sand_coords.0 + 1, sand_coords.1 + 1)).is_none() {
                sand_coords.0 += 1;
                sand_coords.1 += 1;
            } else {
                cave.insert(sand_coords, 'o');
                if sand_coords == (500, 0) {
                    num_sand += 1;
                    break 'drop_loop;
                }
                break;
            }
        }
        num_sand += 1;
    }

    // draw_cave(start_coords, end_coords, &cave);
    println!("Day 14 part 2: {}", num_sand);
}

#[allow(dead_code)]
fn draw_cave(start: (u16, u16), end: (u16, u16), cave: &HashMap<(u16, u16), char>) {
    for y in start.1..=end.1 {
        for x in start.0..=end.0 {
            let c = match cave.get(&(x, y)) {
                Some(c) => c,
                _ => &'.',
            };
            print!("{}", c);
        }
        println!();
    }
}
