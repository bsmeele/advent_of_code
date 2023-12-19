use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lavaduct_lagoon() {
    let test = false;
    let filename = if test { "src/y2023/day18/test" } else { "src/y2023/day18/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut loc1 = (0, 0);
    let mut loc2 = (0, 0);

    let mut coords_part1 = vec![loc1];
    let mut coords_part2 = vec![loc2];

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(' ').collect::<Vec<&str>>();
        let hex = line[2].replace('(', "").replace(')', "").replace('#', "");

        let dist = line[1].parse::<isize>().unwrap();
        match line[0] {
            "U" => loc1.1 -= dist,
            "R" => loc1.0 += dist,
            "D" => loc1.1 += dist,
            "L" => loc1.0 -= dist,
            _ => panic!("Unreachable"),
        }
        coords_part1.push(loc1);

        let dist = isize::from_str_radix(&hex[0..5], 16).unwrap();
        match hex.chars().last().unwrap().to_digit(10).unwrap() {
            0 => loc2.0 += dist,
            1 => loc2.1 += dist,
            2 => loc2.0 -= dist,
            3 => loc2.1 -= dist,
            _ => panic!("Unreachable"),
        }
        coords_part2.push(loc2);
    }

    // draw_map(&coords_part1);
    let area1 = shoelace(&coords_part1);
    println!("Year 2023 day 18 part 1: {}", area1);

    let area2 = shoelace(&coords_part2);
    println!("Year 2023 day 18 part 2: {}", area2);
}

fn shoelace(coords: &Vec<(isize, isize)>) -> usize {
    let mut acc1 = 0;
    let mut acc2 = 0;

    for i in 1..coords.len() {
        acc1 += coords[i].1 * coords[i-1].0;
        acc2 += coords[i].0 * coords[i-1].1;
    }

    let mut circumference = 0;
    let mut prev = coords[0];
    for c in coords {
        circumference += (c.0 - prev.0 + c.1 - prev.1).abs();
        prev = *c;
    }

    // Shoelace gets the area as from the middle of the squares
    // To adjust for this
    //   Add 1/2 for each square on a line
    //   Add 1/4 for each inner corner
    //   Add 3/4 for each outer corner
    // Since it is a loop, there are exactly for more outer corner s than inner corners
    // Thus the area to correct the shoelace area simplifies to perimeter/2 + 1

    (((acc1 - acc2).abs() + circumference)/2 + 1)  as usize
}

#[allow(dead_code)]
fn draw_map(coords: &Vec<(isize, isize)>) {
    let mut prev = coords[0];
    let mut map: HashSet<(isize, isize)> = HashSet::new();
    let mut bounds = ((isize::MAX, isize::MAX), (isize::MIN, isize::MIN));

    for c in coords {
        let y_range = if prev.1 < c.1 { prev.1..=c.1 } else { c.1..=prev.1 };
        for y in y_range {
            if y > bounds.1.1 { bounds.1.1 = y; }
            if y < bounds.0.1 { bounds.0.1 = y; }
            let x_range = if prev.0 < c.0 { prev.0..=c.0 } else { c.0..=prev.0 };
            for x in x_range {
                if x > bounds.1.0 { bounds.1.0 = x; }
                if x < bounds.0.0 { bounds.0.0 = x; }
                map.insert((x, y));
            }
        }
        prev = *c;
    }

    for y in bounds.0.1..=bounds.1.1 {
        for x in bounds.0.0..=bounds.1.0 {
            if map.contains(&(x, y)) { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
    println!();
}
