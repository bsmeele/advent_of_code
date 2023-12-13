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

        simplify(&mut condition_record, &alternate_record);
        simplify(&mut condition_record2, &alternate_record2);

        let tmp = get_pos(&condition_record, &alternate_record);
        acc1 += tmp;

        let tmp = get_pos(&condition_record2, &alternate_record2);
        println!("{}", tmp);
        acc2 += tmp;
    }

    println!("Year 2023 day 12 part 1: {}", acc1);
    println!("Year 2023 day 12 part 2: {}", acc2);
}

fn simplify(condition_record: &mut Vec<char>, alternate_record: &Vec<usize>) {
    let alternate_id = 0;
    // let mut contains_q = false;
    let mut l_id = 0;
    let mut r_id = 0;

    loop {
        for i in r_id..condition_record.len() {
            match condition_record[i] {
                '?' => {
                    // contains_q = true;
                    l_id = i;
                    break;
                }
                '#' => {
                    l_id = i;
                    break;
                }
                '.' => continue,
                _ => panic!("Unreachable"),
            }
        }
        for i in l_id+1..condition_record.len() {
            match condition_record[i] {
                // '?' => contains_q = true,
                '#' => continue,
                '.' => {
                    r_id = i - 1;
                    break;
                },
                _ => panic!("Unreachable"),
            }
        }

        if r_id > l_id && (r_id - l_id + 1) < alternate_record[alternate_id] {
            for i in l_id..=r_id {
                condition_record[i] = '.';
            }
        } else { break; }
    }
}

fn get_pos(condition_record: &Vec<char>, alternate_record: &Vec<usize>) -> usize {
    let mut num_q = 0;

    for c in condition_record {
        if *c == '?' { num_q += 1; }
    }

    gen_perm(num_q, &mut Vec::new(), condition_record, alternate_record)
}

fn gen_perm(length: usize, current: &mut Vec<char>, condition_record: &Vec<char>, alternate_record: &Vec<usize>) -> usize {
    if current.len() == length {
        let mut perm: Vec<char> = Vec::new();
        let mut id = 0;
        for c in condition_record {
            if *c == '?' {
                perm.push(current[id]);
                id += 1;
            } else { perm.push(*c); }
        }
        return if is_pos(&perm, alternate_record) { 1 } else { 0 };
    }

    current.push('.');
    let mut acc = gen_perm(length, current, condition_record, alternate_record);
    current.pop();

    current.push('#');
    acc += gen_perm(length, current, condition_record, alternate_record);
    current.pop();

    acc
}

fn is_pos(condition_record: &Vec<char>, alternate_record: &Vec<usize>) -> bool {
    let mut id = 0;
    let mut concurrent_damaged = 0;

    for c in condition_record {
        match c {
            '.' => {
                if concurrent_damaged > 0 {
                    if id >= alternate_record.len() || alternate_record[id] != concurrent_damaged { return false; }
                    id += 1;
                    concurrent_damaged = 0;
                }
            },
            '#' => concurrent_damaged += 1,
            _ => panic!("Unreachable"),
        }
    }

    if concurrent_damaged > 0 {
        if id >= alternate_record.len() || alternate_record[id] != concurrent_damaged { return false; }
        id += 1;
    }

    if id < alternate_record.len() { return false; }

    true
}
