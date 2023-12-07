use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;
use std::cmp::{max, min};

pub fn beacon_exclusion_zone() {
    let test = false;
    let y_part_1 = if test { 10 } else { 2000000 };
    let range_part_2 = if test { (0, 20) } else { (0, 4000000) };
    let filename = if test { "src/y2022/day15/test" } else { "src/y2022/day15/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    let mut sensors: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut beacons: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut start: (i32, i32) = (2_147_483_647, 2_147_483_647);
    let mut end: (i32, i32) = (-2_147_483_647, -2_147_483_647);

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(' ').collect();

        let xs = line[2].replace("x=", "").replace(',', "").parse::<i32>().unwrap();
        let ys = line[3].replace("y=", "").replace(':', "").parse::<i32>().unwrap();
        let xb = line[8].replace("x=", "").replace(',', "").parse::<i32>().unwrap();
        let yb = line[9].replace("y=", "").parse::<i32>().unwrap();

        sensors.entry(ys)
            .and_modify(|entry| entry.push(xs))
            .or_insert_with(|| vec![xs]);
        beacons.entry(yb)
            .and_modify(|entry| if !entry.contains(&xb) { entry.push(xb); })
            .or_insert_with(|| vec![xb]);

        let dist = (xs - xb).abs() + (ys - yb).abs();
        if start.0 > xs - dist { start.0 = xs - dist; }
        else if end.0 < xs + dist { end.0 = xs + dist; }
        if start.0 > xb - dist { start.0 = xb - dist; }
        else if end.0 < xb + dist { end.0 = xb + dist; }
        if start.1 > ys - dist { start.1 = ys - dist; }
        else if end.1 < ys + dist { end.1 = ys + dist; }
        if start.1 > yb - dist { start.1 = yb - dist; }
        else if end.1 < yb + dist { end.1 = yb + dist; }

        for dy in -dist..=dist {
            let dx = dist - dy.abs();
            map.entry(ys + dy)
                .and_modify(|entry| entry.push((xs - dx, xs + dx)))
                .or_insert_with(|| vec![(xs - dx, xs + dx)]);
        }
        // println!("Completed sensor ({}, {})", xs, ys);
    }

    for y in start.1..=end.1 {
        let v = match map.get(&y) {
            Some(v) => v,
            None => continue,
        };
        let ret = reduce(v.clone());
        map.insert(y, ret);
    }

    let mut total = 0;
    for r in map.get(&y_part_1).unwrap() {
        total += r.1 - r.0 + 1;
    }
    if let Some(s) = sensors.get(&y_part_1) { total -= s.len() as i32; }
    if let Some(b) = beacons.get(&y_part_1) { total -= b.len() as i32; }

    println!("Day 15 part 1: {}", total);

    // draw_map(&map, &sensors, &beacons, start, end);

    let mut total = 0;
    'find_empty: for y in range_part_2.0..=range_part_2.1 {
        let mut num_empty = range_part_2.1 - range_part_2.0;
        if let Some(v) = map.get(&y) {
            for rx in v {
                num_empty -= min(rx.1, range_part_2.1) - max(rx.0, range_part_2.0);
            }
        } else { num_empty = 0; }
        if let Some(s) = sensors.get(&y) { num_empty -= s.len() as i32; }
        if let Some(b) = beacons.get(&y) { num_empty -= b.len() as i32; }
        if num_empty > 0 {
            for x in range_part_2.0..=range_part_2.1 {
                let mut contains = false;
                for rx in map.get(&y).unwrap() {
                    if x >= rx.0 && x <= rx.1 { contains = true; }
                }
                if !contains {
                    total = x as i64 * 4000000 + y as i64;
                    break 'find_empty;
                }
            }
        }
    }

    println!("Day 15 part 2: {}", total);

    // draw_map(&map, &sensors, &beacons, start, end);
}

fn reduce(ranges: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut iter = 0;
    let mut ranges = ranges;
    loop {
        if ranges.len() == 1 { break; }
        let mut converged = true;
        let mut eval = ranges.remove(0);
        for _ in 0..ranges.len() {
            let mut changed = false;
            let cmp = ranges.remove(0);
            if eval.0 <= cmp.0 && eval.1 >= cmp.1 {
                converged = false;
                continue;
            }
            if eval.1 + 1 == cmp.0 {
                eval.1 = cmp.1;
                changed = true;
                converged = false;
            }
            if eval.0 > cmp.0 && eval.0 <= cmp.1 {
                eval.0 = cmp.0;
                changed = true;
                converged = false;
            }
            if eval.1 < cmp.1 && eval.1 >= cmp.0 {
                eval.1 = cmp.1;
                changed = true;
                converged = false;
            }
            if !changed { ranges.push(cmp); }
        }
        ranges.push(eval);
        iter += 1;
        if converged && iter >= ranges.len() { break; }
    }
    ranges
}

#[allow(dead_code)]
fn draw_map(map: &HashMap<i32, Vec<(i32, i32)>>, sensors: &HashMap<i32, Vec<i32>>, beacons: &HashMap<i32, Vec<i32>>, start: (i32, i32), end: (i32, i32)) {
    println!("Start: ({}, {}), end: ({}, {})", start.0, start.1, end.0, end.1);
    for y in start.1..=end.1 {
        let mut row = vec!['.'; (end.0 - start.0) as usize];
        if let Some(dx) = map.get(&y) {
            for dx in dx {
                for x in dx.0..=dx.1 {
                    row[(x - start.0) as usize] = '#';
                }
            }
        }
        if let Some(s) = sensors.get(&y) {
            for s in s { row[(s - start.0) as usize] = 'S'; }
        }
        if let Some(b) = beacons.get(&y) {
            for b in b { row[(b - start.0) as usize] = 'B'; }
        }
        for c in row { print!("{}", c); }
        println!();
    }
}
