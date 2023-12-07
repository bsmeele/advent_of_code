use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

pub fn hill_climbing_algorithm() {
    let test = false;

    let filename = if test { "src/day12/test" } else { "src/day12/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut map: HashMap<(u8, u8), (u8, u16)> = HashMap::new();
    let mut dists: Vec<u16> = Vec::new();
    let mut start: Vec<(u8, u8)> = vec![(0, 0)];
    let mut end: (u8, u8) = (0, 0);


    let mut r = 0;
    let mut c = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<char> = line.chars().collect();

        c = 0;
        for i in line {
            if i == 'S' {
                start[0] = (c, r);
                map.insert((c, r), (0, 0));
            } else if i == 'E' {
                end = (c, r);
                map.insert((c, r), (25, 0xffff));
            } else {
                if i == 'a' { start.push((c, r)); }
                let n = if let Some(n) = i.to_digit(36) { n } else { break; } - 10;
                map.insert((c, r), (n as u8, 0xffff));
            }
            c += 1;
        }
        r += 1;
    }

    for start in start {
        let mut map = map.clone();
        map.insert(start, (0, 0));
        let mut q = vec![start];

        while let Some(p) = q.pop() {
            let h = map[&p].0;
            let d = map[&p].1;

            // Up
            if p.1 > 0 {
                let t = (p.0, p.1 - 1);
                let ht = map[&t].0;
                let dh = ht as i8 - h as i8;
                if dh <= 1 && map[&t].1 > d + 1 {
                    map.insert(t, (ht, d + 1));
                    q.push(t);
                }
            }

            // Down
            if p.1 < r - 1 {
                let t = (p.0, p.1 + 1);
                let ht = map[&t].0;
                let dh = ht as i8 - h as i8;
                if dh <= 1 && map[&t].1 > d + 1 {
                    map.insert(t, (ht, d + 1));
                    q.push(t);
                }
            }

            // Left
            if p.0 > 0 {
                let t = (p.0 - 1, p.1);
                let ht = map[&t].0;
                let dh = ht as i8 - h as i8;
                if dh <= 1 && map[&t].1 > d + 1 {
                    map.insert(t, (ht, d + 1));
                    q.push(t);
                }
            }

            // Right
            if p.0 < c - 1 {
                let t = (p.0 + 1, p.1);
                let ht = map[&t].0;
                let dh = ht as i8 - h as i8;
                if dh <= 1 && map[&t].1 > d + 1 {
                    map.insert(t, (ht, d + 1));
                    q.push(t);
                }
            }
        }
        dists.push(map[&end].1);
        println!("Finished ({}, {})", start.0, start.1);
    }

    let mut shortest = 0xfff;
    for dist in &dists { if *dist < shortest { shortest = *dist; } }

    println!("Day 12 part 1: {}", dists[0]);
    println!("Day 12 part 2: {}", shortest);
}
