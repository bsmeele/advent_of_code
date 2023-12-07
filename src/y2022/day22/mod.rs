use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn monkey_map() {
    let test = false;
    let filename = if test { "src/y2022/day21/test" } else { "src/y2022/day21/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for _line in reader.lines().map(|x| x.unwrap()) {

    }
}