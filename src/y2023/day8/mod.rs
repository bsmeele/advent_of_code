use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::mem::swap;

pub fn haunted_wasteland() {
    let test = false;
    let verbose = false;
    let mut filename = if test { "src/y2023/day8/test1" } else { "src/y2023/day8/input" };
    let mut file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);

    let mut network: HashMap<String, (String, String)> = HashMap::new();
    let mut instructions: Vec<char> = Vec::new();
    let mut ins_parsed = false;

    for line in reader.lines().map(|x| x.unwrap()) {
        let tmp = line.chars().collect::<Vec<char>>();
        if !line.is_empty() {
            if !ins_parsed && (tmp[0] == 'L' || tmp[0] == 'R') {
                instructions = tmp;
                ins_parsed = true;
            } else {
                let line = line.split(' ').collect::<Vec<&str>>();
                let node = line[0];
                let l = line[2].replace('(', "").replace(',', "");
                let r = line[3].replace('(', "").replace(')', "");
                network.insert(String::from(node), (l, r));
            }
        }
    }

    let mut id = 0;
    let mut steps = 0;
    let mut current_node = "AAA";
    let dest_node = "ZZZ";
    loop {
        if current_node == dest_node { break; }
        let (l, r) = &network[current_node];
        match instructions[id] {
            'L' => current_node = l,
            'R' => current_node = r,
            _ => panic!("Unreachable")
        }
        id += 1;
        if id == instructions.len() { id = 0; }
        steps += 1;
    }

    println!("Year 2023 day 8 part 1: {}", steps);

    network.clear();
    ins_parsed = false;
    let mut current_nodes: Vec<String> = Vec::new();
    let mut end_nodes: Vec<String> = Vec::new();

    if test {
        filename = "src/y2023/day8/test2";
    }

    file = File::open(filename).unwrap();
    reader = BufReader::new(file);
    for line in reader.lines().map(|x| x.unwrap()) {
        let tmp = line.chars().collect::<Vec<char>>();
        if !line.is_empty() {
            if !ins_parsed && (tmp[0] == 'L' || tmp[0] == 'R') {
                instructions = tmp;
                ins_parsed = true;
            } else {
                let line = line.split(' ').collect::<Vec<&str>>();
                let node = line[0];
                let l = line[2].replace('(', "").replace(',', "");
                let r = line[3].replace('(', "").replace(')', "");
                if node.chars().collect::<Vec<char>>().last().unwrap() == &'A' {
                    current_nodes.push(String::from(node));
                }
                if node.chars().collect::<Vec<char>>().last().unwrap() == &'Z' {
                    end_nodes.push(String::from(node));
                }
                network.insert(String::from(node), (l, r));
            }
        }
    }

    // Notes:
    // - from any start to some end takes a multiple of the instruction length amount of steps
    // - from any end to some end takes a multiple of the instruction length amount of steps
    // - from any start to some end takes the same amount of steps as from that end to some end
    // - Concluding: the solution is the least common multiple of each individual start to end

    if verbose {
        for i in 0..current_nodes.len() {
            id = 0;
            steps = 0;
            current_node = &current_nodes[i];
            loop {
                if current_node.chars().collect::<Vec<char>>().last().unwrap() == &'Z' {
                    break;
                }
                let (l, r) = &network[current_node];
                match instructions[id] {
                    'L' => current_node = l,
                    'R' => current_node = r,
                    _ => panic!("Unreachable")
                }
                id += 1;
                if id == instructions.len() { id = 0; }
                steps += 1;
            }
            println!("Start: {}, end: {}, number of steps: {}, instruction number: {}", current_nodes[i], current_node, steps, id);
        }

        for i in 0..end_nodes.len() {
            id = 0;
            steps = 0;
            current_node = &current_nodes[i];
            loop {
                let (l, r) = &network[current_node];
                match instructions[id] {
                    'L' => current_node = l,
                    'R' => current_node = r,
                    _ => panic!("Unreachable")
                }
                id += 1;
                if id == instructions.len() { id = 0; }
                steps += 1;

                if current_node.chars().collect::<Vec<char>>().last().unwrap() == &'Z' {
                    break;
                }
            }
            println!("Start: {}, end: {}, number of steps: {}, instruction number: {}", end_nodes[i], current_node, steps, id);
        }
    }

    let mut steps_list: Vec<usize> = Vec::new();
    for node in &current_nodes {
        id = 0;
        steps = 0;
        current_node = node;
        loop {
            if current_node.chars().collect::<Vec<char>>().last().unwrap() == &'Z' { break; }
            let (l, r) = &network[current_node];
            match instructions[id] {
                'L' => current_node = l,
                'R' => current_node = r,
                _ => panic!("Unreachable")
            }

            id += 1;
            if id == instructions.len() { id = 0; }
            steps += 1;
        }
        steps_list.push(steps);
    }

    let mut num = 1;
    for n in &steps_list {
        num = lcm(num, *n);
    }

    // ----- Brute force approach -----
    // id = 0;
    // steps = 0;
    // let mut finished;
    // loop {
    //     finished = true;
    //     for n in &current_nodes {
    //         if n.chars().collect::<Vec<char>>().last().unwrap() != &'Z' {
    //             finished = false;
    //             break;
    //         }
    //     }
    //     if finished { break; }
    //     for i in 0..current_nodes.len() {
    //         let (l, r) = &network[&current_nodes[i]];
    //         match instructions[id] {
    //             'L' => current_nodes[i] = l.clone(),
    //             'R' => current_nodes[i] = r.clone(),
    //             _ => panic!("Unreachable")
    //         }
    //     }
    //     id += 1;
    //     if id == instructions.len() { id = 0; }
    //     steps += 1;
    // }

    println!("Year 2023 day 8 part 2: {}", num);
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    if a < b {
        swap(&mut a, &mut b);
    }
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}
