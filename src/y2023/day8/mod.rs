use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn haunted_wasteland() {
    let test = false;
    let mut filename = if test { "src/y2023/day8/test1" } else { "src/y2023/day8/input" };
    let mut file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

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

    if test {
        filename = "src/y2023/day8/test2";
        file = File::open(filename).unwrap();
    }

    network.clear();
    ins_parsed = false;
    let mut current_nodes: Vec<String> = Vec::new();

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
                if node.chars().collect::<Vec<char>>().last().unwrap() == 'A' {
                    current_nodes.push(String::from(node));
                }
                network.insert(String::from(node), (l, r));
            }
        }
    }

    id = 0;
    steps = 0;

    println!("Year 2023 day 8 part 1: {}", steps);
}