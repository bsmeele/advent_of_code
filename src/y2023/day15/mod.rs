use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn lens_library() {
    let test = false;
    let filename = if test { "src/y2023/day15/test" } else { "src/y2023/day15/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().map(|x| x.unwrap()) {

    }
}