use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn hot_springs() {
    let test = false;
    let filename = if test { "src/y2023/day12/test" } else { "src/y2023/day12/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut condition_record: Vec<char>;
    let mut alternate_record: Vec<usize>;
    let mut acc1 = 0;

    let mut condition_record2: Vec<char>;
    let mut alternate_record2: Vec<usize>;
    let mut acc2 = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(' ').collect::<Vec<&str>>();
        condition_record = line[0].chars().collect();
        alternate_record = line[1].split(',').map(|n| n.parse::<usize>().unwrap()).collect();

        condition_record2 = condition_record.clone();
        condition_record2.push('?');
        condition_record2.extend(condition_record.clone());
        condition_record2.push('?');
        condition_record2.extend(condition_record.clone());
        condition_record2.push('?');
        condition_record2.extend(condition_record.clone());
        condition_record2.push('?');
        condition_record2.extend(condition_record.clone());

        alternate_record2 = alternate_record.clone();
        alternate_record2.extend(alternate_record.clone());
        alternate_record2.extend(alternate_record.clone());
        alternate_record2.extend(alternate_record.clone());
        alternate_record2.extend(alternate_record.clone());

        let tmp = cached_pos(condition_record, alternate_record, &mut HashMap::new());
        acc1 += tmp;

        let tmp2 = cached_pos(condition_record2, alternate_record2, &mut HashMap::new());
        acc2 += tmp2;
    }

    println!("Year 2023 day 12 part 1: {}", acc1);
    println!("Year 2023 day 12 part 2: {}", acc2);
}

fn cached_pos(condition_record: Vec<char>, alternate_record: Vec<usize>, cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>) -> usize {
    if condition_record.is_empty() {
        return if alternate_record.is_empty() || (alternate_record.len() == 1 && alternate_record[0] == 0) { 1 } else { 0 }
    }
    if alternate_record.is_empty() {
        for c in &condition_record {
            if *c == '#' { return 0; }
        }
        return 1;
    }
    if cache.contains_key(&(condition_record.clone(), alternate_record.clone())) { return cache[&(condition_record.clone(), alternate_record.clone())] }

    let mut condition_record = condition_record;
    let mut alternate_record = alternate_record;

    let tmp_c = condition_record.clone();
    let tmp_a = alternate_record.clone();
    let tmp = match condition_record.pop().unwrap() {
        '#' => {
            if *alternate_record.last().unwrap() == 0 { return 0; }

            *alternate_record.last_mut().unwrap() -= 1;
            for _ in 0..*alternate_record.last().unwrap() {
                if condition_record.is_empty() { return 0; }
                if condition_record.pop() == Some('.') { return 0; }
                if *alternate_record.last().unwrap() == 0 { return 0; }
                *alternate_record.last_mut().unwrap() -= 1;
            }

            cached_pos(condition_record, alternate_record, cache)
        },
        '.' => {
            if *alternate_record.last().unwrap() == 0 { alternate_record.pop(); }
            cached_pos(condition_record, alternate_record, cache)
        },
        '?' => {
            let pos = {
                let mut tmp = alternate_record.clone();
                if *tmp.last().unwrap() == 0 { tmp.pop(); }
                cached_pos(condition_record.clone(), tmp, cache)
            };
            if *alternate_record.last().unwrap() == 0 { pos }
            else {
                *alternate_record.last_mut().unwrap() -= 1;
                for _ in 0..*alternate_record.last().unwrap() {
                    if condition_record.is_empty() { return 0; }
                    if condition_record.pop() == Some('.') { return pos; }
                    if *alternate_record.last().unwrap() == 0 { return pos; }
                    *alternate_record.last_mut().unwrap() -= 1;
                }
                pos + cached_pos(condition_record, alternate_record, cache)
            }
        },
        _ => panic!("Unreachable"),
    };

    cache.insert((tmp_c, tmp_a), tmp);

    tmp
}
