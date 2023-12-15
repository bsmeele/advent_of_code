use std::fs::File;
use std::io::{BufRead, BufReader};

const EMPTY_VEC: Vec<(String, usize)> = Vec::new();

pub fn lens_library() {
    let test = false;
    let filename = if test { "src/y2023/day15/test" } else { "src/y2023/day15/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut acc = 0;
    let mut boxes: [Vec<(String, usize)>; 256] = [EMPTY_VEC; 256];

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(',').collect::<Vec<&str>>();
        for s in line {
            let tmp = hash(s);
            acc += tmp;

            let mut label = String::new();
            for c in s.chars() {
                match c {
                    '=' => {
                        let lens = s.chars().last().unwrap().to_digit(10).unwrap() as usize;
                        update_lens(&mut boxes[hash(&label)], &label, lens)
                    },
                    '-' => remove_lens(&mut boxes[hash(&label)], &label),
                    _ => label.push(c),
                }
            }
        }
    }

    // for b in boxes {
    //     println!("{:?}", b);
    // }

    println!("Year 2023 day 15 part 1: {}", acc);
    println!("Year 2023 day 15 part 2: {}", score(&boxes));
}

fn score(boxes:&[Vec<(String, usize)>; 256]) -> usize {
    let mut acc = 0;
    for b in 1..=boxes.len() {
        for s in 1..=boxes[b-1].len() {
            acc += b * s * boxes[b-1][s-1].1;
        }
    }

    acc
}

fn update_lens(b: &mut Vec<(String, usize)>, label: &str, lens: usize) {
    let mut id = None;
    for i in 0..b.len() {
        if b[i].0 == label {
            id = Some(i);
            break;
        }
    }
    if let Some(i) = id { b[i].1 = lens; }
    else { b.push((String::from(label), lens)); }
}

fn remove_lens(b: &mut Vec<(String, usize)>, label: &str) {
    let mut id = None;
    for i in 0..b.len() {
        if b[i].0 == label {
            id = Some(i);
            break;
        }
    }
    if let Some(i) = id { b.remove(i); }
}

fn hash(s: &str) -> usize {
    let mut current_value = 0;
    for c in s.chars() {
        current_value += (c as u8) as usize;
        current_value *= 17;
        current_value %= 256;
    }
    current_value
}