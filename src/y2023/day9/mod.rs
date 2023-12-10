use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn mirage_maintenance() {
    let test = false;
    let filename = if test { "src/y2023/day9/test" } else { "src/y2023/day9/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut histories: Vec<Vec<isize>> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line
            .split(' ').
            collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<isize>().unwrap())
            .collect::<Vec<isize>>();
        histories.push(line);
    }

    let mut acc1 = 0;
    let mut acc2 = 0;
    for h in histories {
        let (l, r) = extrapolate_history(h);
        acc1 += r;
        acc2 += l;
    }

    println!("Year 2023 day 9 part 1: {}", acc1);
    println!("Year 2023 day 9 part 2: {}", acc2);
}

fn extrapolate_history(h: Vec<isize>) -> (isize, isize) {
    let mut all_zeros = true;
    for e in &h {
        if *e != 0 {
            all_zeros = false;
            break;
        }
    }
    if all_zeros { return (0, 0); }

    let dif = h
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .collect::<Vec<isize>>();

    let (l, r) = extrapolate_history(dif);
    (h.first().unwrap() - l, h.last().unwrap() + r)
}
