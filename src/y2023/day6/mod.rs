use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn wait_for_it() {
    let test = false;
    let filename = if test { "src/y2023/day6/test" } else { "src/y2023/day6/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut time: Vec<usize> = Vec::new();
    let mut time2 = 0;
    let mut distance: Vec<usize> = Vec::new();
    let mut distance2 = 0;
    let mut acc = 1;

    for line in reader.lines().map(|x| x.unwrap()) {
        let split_line: Vec<&str> = line.split(' ').collect();
        match split_line[0] {
            "Time:" => for e in split_line {
                if let Ok(n) = e.parse::<usize>() { time.push(n); }
                time2 = line.replace("Time:", "").replace(' ', "").parse::<usize>().unwrap();
            },
            "Distance:" => for e in split_line {
                if let Ok(n) = e.parse::<usize>() { distance.push(n); }
                distance2 = line.replace("Distance:", "").replace(' ', "").parse::<usize>().unwrap();
            },
            _ => (),
        }
    }

    for race in 0..time.len() {
        let d = (time[race].pow(2) - 4 * distance[race]) as f32;
        let t1 = (((time[race] as f32 - d.sqrt()) / 2.) + 1.).floor() as usize;
        let t2 = (((time[race] as f32 + d.sqrt()) / 2.) - 1.).ceil() as usize;
        acc *= t2 - t1 + 1;
    }

    println!("year 2023 day 6 part 1: {}", acc);

    let d = (time2.pow(2) - 4 * distance2) as f64;
    let t1 = (((time2 as f64 - d.sqrt()) / 2.) + 1.).floor() as usize;
    let t2 = (((time2 as f64 + d.sqrt()) / 2.) - 1.).ceil() as usize;

    println!("Year 2023 day 6 part 2: {}", t2 - t1 + 1)
}
