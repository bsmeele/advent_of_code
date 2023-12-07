use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn cube_conundrum() {
    let test = false;
    let filename = if test { "src/y2023/day2/test" } else { "src/y2023/day2/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let max_colors = [12, 13, 14];
    let mut acc1 = 0;
    let mut acc2 = 0;
    // let re = Regex::new(r"Game (\d+): ([^;]+)").unwrap();
    let re = Regex::new(r"Game (\d+): (.+)").unwrap();

    for line in reader.lines() {
        let line = line.unwrap();

        if let Some(captures) = re.captures(&line) {
            let id = captures.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            let sets: Vec<&str> = captures.get(2).map_or("", |m| m.as_str()).split(';').collect();

            let mut possible = true;
            let mut min_colors = [0, 0, 0];
            for set in sets {
                let set: Vec<&str> = set.split(',').collect();
                let colors = count_set(&set);
                if !((colors[0] <= max_colors[0]) && (colors[1] <= max_colors[1]) && (colors[2] <= max_colors[2])) {
                    possible = false;
                }
                if colors[0] > min_colors[0] { min_colors[0] = colors[0]; }
                if colors[1] > min_colors[1] { min_colors[1] = colors[1]; }
                if colors[2] > min_colors[2] { min_colors[2] = colors[2]; }
            }

            if possible { acc1 += id; }
            acc2 += min_colors[0] * min_colors[1] * min_colors[2]

        }
    }

    println!("Day 2 part 1: {}", acc1);
    println!("Day 2 part 2: {}", acc2);
}

fn count_set(set: &Vec<&str>) -> [usize; 3] {
    let re = Regex::new(r"(\d+) (\w+)").unwrap();
    let mut colors = [0, 0, 0];

    for pair in set {
        if let Some(captures) = re.captures(pair) {
            let num = captures.get(1).map_or("", |m| m.as_str()).parse::<usize>().unwrap();
            match captures.get(2).map_or("", |m| m.as_str()) {
                "red" => colors[0] += num,
                "green" => colors[1] += num,
                "blue" => colors[2] += num,
                _ => panic!()
            };
        }
    }

    colors
}
