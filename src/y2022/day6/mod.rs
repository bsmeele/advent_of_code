use std::fs::read_to_string;

pub fn tuning_trouble() {
    let filename = "src/day6/input";
    let input: Vec<char> = read_to_string(filename).unwrap().chars().collect();
    let mut part1 = false;
    let mut part2 = false;

    for i in 0..(input.len() - 3) {
        if !part1 {
            part1 = true;
            for j in i..i+3 {
                if input[j+1..=i+3].contains(&input[j]) {
                    part1 = false;
                }
            }
            if part1 { println!("{}", i + 4); }
        }
        if !part2 {
            part2 = true;
            for j in i..i+13 {
                if input[j+1..=i+13].contains(&input[j]) {
                    part2 = false;
                }
            }
            if part2 { println!("{}", i + 14); }
        }
        if part1 && part2 { break; }
    }
}
