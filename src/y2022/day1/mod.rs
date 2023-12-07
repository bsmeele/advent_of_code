use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn calorie_counting() {
    let test = false;
    let file = if test { "src/y2022/day1/test1" } else { "src/y2022/day1/input" };
    let file = File::open(file).expect("Issue opening file");
    let reader = BufReader::new(file);

    let mut first = 0;
    let mut second = 0;
    let mut third = 0;
    let mut current = 0;

    for (_, line) in reader.lines().enumerate() {
        if let Ok(num) = line.unwrap().parse::<u32>() {
            current += num;
        } else {
            if current > first {
                third = second;
                second = first;
                first = current;
            }
            else if current > second {
                third = second;
                second = current;
            }
            else if current > third { third = current; }
            current = 0;
        };
    }

    println!("Day 1 part 1: {}", first);
    println!("Day 1 part 2: {}", first + second + third);
}

// #[cfg(test1)]
// mod test1 {
//     #[test1]
//     fn
// }