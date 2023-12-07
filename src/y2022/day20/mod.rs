use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn grove_positioning_system() {
    let test = false;
    let filename = if test { "src/y2022/day20/test" } else { "src/y2022/day20/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut list: Vec<(isize, usize)> = Vec::new();
    let mut list2: Vec<(isize, usize)> = Vec::new();
    let mut id = 0;
    let key = 811589153;
    let mix_amount = 10;

    for line in reader.lines() {
        let line = line.unwrap();
        list.push((line.parse::<isize>().unwrap(), id));
        list2.push((line.parse::<isize>().unwrap() * key, id));
        id += 1;
    }

    mix(&mut list);

    for i in 0..list.len() {
        if list[i].0 == 0 {
            id = i;
            break;
        }
    }

    let a = list[(id + 1000)%list.len()].0;
    let b = list[(id + 2000)%list.len()].0;
    let c = list[(id + 3000)%list.len()].0;

    println!("Year 2022 day 20 part 1: {}", a + b + c);

    for _ in 0..mix_amount {
        mix(&mut list2);
    }

    for i in 0..list2.len() {
        if list2[i].0 == 0 {
            id = i;
            break;
        }
    }

    let a = list2[(id + 1000)%list.len()].0;
    let b = list2[(id + 2000)%list.len()].0;
    let c = list2[(id + 3000)%list.len()].0;

    println!("Year 2022 day 20 part 2: {}", a + b + c);
}

fn mix(list: &mut Vec<(isize, usize)>) {
    for i in 0..list.len() {
        for j in 0..list.len() {
            if i == list[j].1 {
                let tmp = list.remove(j);
                let mut k = j as isize + tmp.0;
                k = k%(list.len() as isize);
                if k <= 0 { k += list.len() as isize; }
                list.insert(k as usize, tmp);
                break;
            }
        }
    }
}
