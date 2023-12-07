use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn proboscidea_volcanium() {
    let test = false;
    let filename = if test { "src/y2022/day16/test" } else { "src/y2022/day16/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map: HashMap<String, Valve> = HashMap::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(' ').collect();
        let valve = String::from(line[1]);
        let flow = line[4]
            .replace("rate=", "")
            .replace(';', "")
            .parse::<u8>().unwrap();
        let mut neighbours: Vec<(String, usize)> = Vec::new();
        for i in 9..line.len() {
            neighbours.push((String::from(line[i].replace(',', "")), 1));
        }
        map.insert(valve.clone(), Valve{flow, neighbours});
    }

    // println!("----- Old -----");
    // for (k, v) in &map {
    //     println!("Valve {} with flow rate {} and connected to {:?}", k, v.flow, v.neighbours);
    // }
    // println!();

    // println!("{}", map.len());
    while reduce(&mut map) {}
    // println!("{}", map.len());

    // println!("----- New -----");
    // for (k, v) in &map {
    //     println!("Valve {} with flow rate {} and connected to {:?}", k, v.flow, v.neighbours);
    // }
    // println!();

    let mut highest_flow = 0;
    for valve in map.values() {
        if valve.flow > highest_flow { highest_flow = valve.flow; }
    }

    println!("Day 16 part 1: {}", max_flow(&map, Vec::new(), "AA", "AA", 30));
    println!("Day 16 part 2: {}", part_2(&map, Vec::new(), "AA", "AA", 0, "AA", "AA", 0, 26, highest_flow, 0, 0));
}

#[derive(Clone)]
struct Valve {
    flow: u8,
    neighbours: Vec<(String, usize)>,
}

fn reduce(map: &mut HashMap<String, Valve>) -> bool {
    let mut changed = false;

    let mut to_remove: Vec<String> = Vec::new();

    for valve in &mut *map {
        if valve.1.neighbours.len() == 2 && valve.1.flow == 0 {
            changed = true;

            to_remove.push(String::from(valve.0));
        }
    }

    for valve in to_remove {
        let n1 = map[&valve].neighbours[0].clone();
        let n2 = map[&valve].neighbours[1].clone();

        if let Some(v) = map.get_mut(&n1.0) {
            v.neighbours.push((n2.0.clone(), n1.1 + n2.1));
            v.neighbours.retain(|x| x.0 != valve);
        }
        if let Some(v) = map.get_mut(&n2.0) {
            v.neighbours.push((n1.0, n1.1 + n2.1));
            v.neighbours.retain(|x| x.0 != valve);
        }

        map.remove(&valve);
    }

    changed
}

fn max_flow(map: &HashMap<String, Valve>, open: Vec<&str>, current: &str, prev: &str, time: isize) -> usize {
    if time <= 0 { return 0; }

    let mut flow = 0;

    let neighbours = map[current].neighbours.clone();
    for valve in neighbours {
        if valve.0 == prev { continue; }
        let alt_flow = max_flow(map, open.clone(), &valve.0, current, time - valve.1 as isize);
        if alt_flow > flow { flow = alt_flow; }
    }

    if !open.contains(&current) && map[current].flow != 0 {
        let mut open = open;
        open.push(current);
        let alt_flow = (time as usize - 1) * map[current].flow as usize + max_flow(map, open, current, current, time - 1);
        if alt_flow > flow { flow = alt_flow; }
    }

    flow
}

fn part_2(map: &HashMap<String, Valve>, open: Vec<&str>, current_you: &str, prev_you: &str, wait_you: usize, current_el: &str, prev_el: &str, wait_el: usize, time: isize, highest_flow: u8, current_flow: usize, current_max: usize) -> usize {
    if time <= 0 { return 0; }
    if open.len() == map.len() { return 0; }
    let time_sum = (time - 1) * time / 2;
    let max_pos = highest_flow as usize * time_sum as usize + current_flow;
    if current_max > max_pos { return 0; }

    let mut flow = 0;
    let mut current_max = current_max;

    if wait_you == 0 && wait_el == 0 {
        let ny = map[current_you].neighbours.clone();
        let ne = map[current_el].neighbours.clone();
        for n1 in &ny {
            if n1.0 == prev_you { continue; }
            for n2 in &ne {
                if n2.0 == prev_el { continue; }
                let alt_flow = part_2(map, open.clone(), &n1.0, current_you, n1.1 - 1, &n2.0, current_el, n2.1 - 1,time - 1, highest_flow, current_flow, current_max);
                if alt_flow > flow { flow = alt_flow; }
                if flow + current_flow > current_max { current_max = flow + current_flow; }
            }
        }

        for n in &ne {
            if n.0 == prev_el { continue; }
            if !open.contains(&current_you) && map[current_you].flow != 0 {
                let mut open = open.clone();
                open.push(current_you);
                let local_flow = (time as usize - 1) * map[current_you].flow as usize;
                let alt_flow = local_flow + part_2(map, open, current_you, current_you, 0, &n.0, current_el, n.1 - 1, time - 1, highest_flow, current_flow + local_flow, current_max);
                if alt_flow > flow { flow = alt_flow; }
                if flow + current_flow > current_max { current_max = flow + current_flow; }
            }
        }

        for n in &ny {
            if n.0 == prev_you { continue; }
            if !open.contains(&current_el) && map[current_el].flow != 0 {
                let mut open = open.clone();
                open.push(current_el);
                let local_flow = (time as usize - 1) * map[current_el].flow as usize;
                let alt_flow = local_flow + part_2(map, open, &n.0, current_you, n.1 - 1, current_el, current_el, 0, time - 1, highest_flow, current_flow + local_flow, current_max);
                if alt_flow > flow { flow = alt_flow; }
                if flow + current_flow > current_max { current_max = flow + current_flow; }
            }
        }

        if !open.contains(&current_you) && !open.contains(&current_el) && map[current_you].flow != 0 && map[current_el].flow != 0 && current_you != current_el {
            let mut open = open.clone();
            open.push(current_you);
            open.push(current_el);
            let local_flow = (time as usize - 1) * map[current_you].flow as usize + (time as usize - 1) * map[current_el].flow as usize;
            let alt_flow = local_flow + part_2(map, open, current_you, current_you, 0, current_el, current_el, 0, time - 1, highest_flow, current_flow + local_flow, current_max);
            if alt_flow > flow { flow = alt_flow; }
        }
    } else if wait_you == 0 {
        let ny = map[current_you].neighbours.clone();
        for n in ny {
            if n.0 == prev_you { continue; }
            let alt_flow = part_2(map, open.clone(), &n.0, current_you, n.1 - 1, current_el, prev_el, wait_el - 1, time - 1, highest_flow, current_flow, current_max);
            if alt_flow > flow { flow = alt_flow; }
            if flow + current_flow > current_max { current_max = flow + current_flow; }
        }

        if !open.contains(&current_you) && map[current_you].flow != 0 {
            let mut open = open.clone();
            open.push(current_you);
            let local_flow = (time as usize - 1) * map[current_you].flow as usize;
            let alt_flow = local_flow + part_2(map, open, current_you, current_you, 0, current_el, prev_el, wait_el - 1, time - 1, highest_flow, current_flow + local_flow, current_max);
            if alt_flow > flow { flow = alt_flow; }
        }
    } else if wait_el == 0 {
        let ne = map[current_el].neighbours.clone();
        for n in ne {
            if n.0 == prev_el { continue; }
            let alt_flow = part_2(map, open.clone(), current_you, prev_you, wait_you - 1, &n.0, current_el, n.1 - 1, time - 1, highest_flow, current_flow, current_max);
            if alt_flow > flow { flow = alt_flow; }
            if flow + current_flow > current_max { current_max = flow + current_flow; }
        }

        if !open.contains(&current_el) && map[current_el].flow != 0 {
            let mut open = open.clone();
            open.push(current_el);
            let local_flow = (time as usize - 1) * map[current_el].flow as usize;
            let alt_flow = local_flow + part_2(map, open, current_you, prev_you, wait_you - 1, current_el, current_el, 0, time - 1, highest_flow, current_flow + local_flow, current_max);
            if alt_flow > flow { flow = alt_flow; }
        }
    } else {
        flow = part_2(map, open, current_you, prev_you, wait_you - 1, current_el, prev_el, wait_el - 1, time - 1, highest_flow, current_flow, current_max);
    }

    flow
}
