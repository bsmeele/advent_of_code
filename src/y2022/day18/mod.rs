use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn boiling_boulders() {
    let test = false;
    let filename = if test { "src/y2022/day18/test" } else { "src/y2022/day18/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut blob: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut water: HashSet<(isize, isize, isize)> = HashSet::new();
    let mut bounds: ((isize, isize), (isize, isize), (isize, isize)) = ((isize::MAX, 0), (isize::MAX, 0), (isize::MAX, 0));
    let mut area = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(',').collect();
        let (x, y, z) = (line[0].parse::<isize>().unwrap(), line[1].parse::<isize>().unwrap(), line[2].parse::<isize>().unwrap());
        if x < bounds.0.0 { bounds.0.0 = x; }
        if x > bounds.0.1 { bounds.0.1 = x; }
        if y < bounds.1.0 { bounds.1.0 = y; }
        if y > bounds.1.1 { bounds.1.1 = y; }
        if z < bounds.2.0 { bounds.2.0 = z; }
        if z > bounds.2.1 { bounds.2.1 = z; }
        blob.insert((x, y, z));
    }

    // println!("{:?}", bounds);
    // println!("Volume: {}", (bounds.0.1 - bounds.0.0) * (bounds.1.1 - bounds.1.0) * (bounds.2.1 - bounds.2.0));

    for (x, y, z) in &blob {
        if !blob.contains(&(x + 1, *y, *z)) { area += 1; }
        if !blob.contains(&(x - 1, *y, *z)) { area += 1; }
        if !blob.contains(&(*x, y + 1, *z)) { area += 1; }
        if !blob.contains(&(*x, y - 1, *z)) { area += 1; }
        if !blob.contains(&(*x, *y, z + 1)) { area += 1; }
        if !blob.contains(&(*x, *y, z - 1)) { area += 1; }
    }
    println!("Year 2022 day 18 part 1: {}", area);

    water.insert((bounds.0.0 - 1, bounds.1.0 - 1, bounds.2.0 - 1));
    loop {
        let mut to_add: Vec<(isize, isize, isize)> = Vec::new();
        for (x, y, z) in &water {
            if x <= &bounds.0.1 { if !blob.contains(&(x + 1, *y, *z)) && !water.contains(&(x + 1, *y, *z)) { to_add.push((x + 1, *y, *z)); } }
            if x >= &bounds.0.0 { if !blob.contains(&(x - 1, *y, *z)) && !water.contains(&(x - 1, *y, *z)) { to_add.push((x - 1, *y, *z)); } }
            if y <= &bounds.1.1 { if !blob.contains(&(*x, y + 1, *z)) && !water.contains(&(*x, y + 1, *z)) { to_add.push((*x, y + 1, *z)); } }
            if y >= &bounds.1.0 { if !blob.contains(&(*x, y - 1, *z)) && !water.contains(&(*x, y - 1, *z)) { to_add.push((*x, y - 1, *z)); } }
            if z <= &bounds.2.1 { if !blob.contains(&(*x, *y, z + 1)) && !water.contains(&(*x, *y, z + 1)) { to_add.push((*x, *y, z + 1)); } }
            if z >= &bounds.2.0 { if !blob.contains(&(*x, *y, z - 1)) && !water.contains(&(*x, *y, z - 1)) { to_add.push((*x, *y, z - 1)); } }
        }
        if to_add.is_empty() { break; }
        for e in to_add { water.insert(e); }
    }

    // draw(&blob, &water, bounds);

    area = 0;
    for (x, y, z) in blob {
        if water.contains(&(x + 1, y, z)) { area += 1; }
        if water.contains(&(x - 1, y, z)) { area += 1; }
        if water.contains(&(x, y + 1, z)) { area += 1; }
        if water.contains(&(x, y - 1, z)) { area += 1; }
        if water.contains(&(x, y, z + 1)) { area += 1; }
        if water.contains(&(x, y, z - 1)) { area += 1; }
    }
    println!("Year 2022 day 18 part 2: {}", area);
}

#[allow(dead_code)]
fn draw(blob: &HashSet<(isize, isize, isize)>, water: &HashSet<(isize, isize, isize)>, bounds: ((isize, isize), (isize, isize), (isize, isize))) {
    for x in bounds.0.0..=bounds.0.1 {
        for y in bounds.1.0..=bounds.1.1 {
            for z in bounds.2.0..=bounds.2.1 {
                if  blob.contains(&(x, y, z)) && water.contains(&(x, y, z)) { panic!("help") }
                else if blob.contains(&(x, y, z)) { print!("B", ); }
                else if water.contains(&(x, y, z)) { print!("W", ); }
                else { print!("."); }
            }
            println!();
        }
        println!("\n");
    }
}