use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn sand_slabs() {
    let test = false;
    let filename = if test { "src/y2023/day22/test" } else { "src/y2023/day22/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut bricks: Vec<((usize, usize, usize), (usize, usize, usize))> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split('~').collect::<Vec<&str>>();
        let b1 = line[0].split(',').collect::<Vec<&str>>();
        let b2 = line[1].split(',').collect::<Vec<&str>>();
        let b1 = (b1[0].parse::<usize>().unwrap(), b1[1].parse::<usize>().unwrap(), b1[2].parse::<usize>().unwrap());
        let b2 = (b2[0].parse::<usize>().unwrap(), b2[1].parse::<usize>().unwrap(), b2[2].parse::<usize>().unwrap());

        bricks.push((b1, b2));
    }

    for b in &bricks {
        println!("{:?}", b);
    }
    println!();

    bricks.sort_by(|a, b| a.0.2.cmp(&b.0.2));
    for b in &bricks {
        println!("{:?}", b);
    }
    println!();
}