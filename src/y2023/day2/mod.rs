use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn cube_conundrum() {
    let test = false;
    let filename = if test { "src/y2023/day2/test" } else { "src/y2023/day2/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let max_colors = [12, 13, 14];
    let mut acc1 = 0;
    let mut acc2 = 0;
    let mut possible;
    let mut min_colors;

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(':').collect::<Vec<&str>>();
        let id = line[0][5..].parse::<usize>().unwrap();

        possible = true;
        min_colors = [0, 0, 0];

        for set in line[1].split(';').collect::<Vec<&str>>() {
            for pair in set.split(',').collect::<Vec<&str>>() {
                let pair = pair.split(' ').collect::<Vec<&str>>();
                let num = pair[1].parse::<usize>().unwrap();
                match pair[2] {
                    "red" => {
                        if min_colors[0] < num { min_colors[0] = num; }
                        if num > max_colors[0] { possible = false; }
                    },
                    "green" => {
                        if min_colors[1] < num { min_colors[1] = num; }
                        if num > max_colors[1] { possible = false; }
                    },
                    "blue" => {
                        if min_colors[2] < num { min_colors[2] = num; }
                        if num > max_colors[2] { possible = false; }
                    },
                    _ => panic!("Unreachable"),
                };
            }
        }
        if possible { acc1 += id; }
        acc2 += min_colors[0] * min_colors[1] * min_colors[2];
    }

    println!("Day 2 part 1: {}", acc1);
    println!("Day 2 part 2: {}", acc2);
}
