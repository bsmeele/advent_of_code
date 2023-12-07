use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn if_you_give_a_seed_fertilizer() {
    let test = true;
    let filename = if test { "src/y2023/day5/test" } else { "src/y2023/day5/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut seeds: Vec<usize> = Vec::new();
    let mut seed_to_soil: Vec<Map> = Vec::new();
    let mut soil_to_fertilizer: Vec<Map> = Vec::new();
    let mut fertilizer_to_water: Vec<Map> = Vec::new();
    let mut water_to_light: Vec<Map> = Vec::new();
    let mut light_to_temperature: Vec<Map> = Vec::new();
    let mut temperature_to_humidity: Vec<Map> = Vec::new();
    let mut humidity_to_location: Vec<Map> = Vec::new();
    let mut map_id = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        let line = line.split(' ').collect::<Vec<&str>>();
        match line[0] {
            "seeds:" => for i in 1..line.len() {
                    seeds.push(line[i].parse::<usize>().unwrap());
            },
            "seed-to-soil" => map_id = 1,
            "soil-to-fertilizer" => map_id = 2,
            "fertilizer-to-water" => map_id = 3,
            "water-to-light" => map_id = 4,
            "light-to-temperature" => map_id = 5,
            "temperature-to-humidity" => map_id = 6,
            "humidity-to-location" => map_id = 7,
            _ => (),
        }

        if line[0].parse::<usize>().is_err() { continue; }
        match map_id {
            1 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                seed_to_soil.push(Map{dest, source, range});
            },
            2 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                soil_to_fertilizer.push(Map{dest, source, range});
            },
            3 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                fertilizer_to_water.push(Map{dest, source, range});
            },
            4 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                water_to_light.push(Map{dest, source, range});
            },
            5 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                light_to_temperature.push(Map{dest, source, range});
            },
            6 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                temperature_to_humidity.push(Map{dest, source, range});
            },
            7 => {
                let source = line[1].parse::<usize>().unwrap();
                let dest = line[0].parse::<usize>().unwrap();
                let range = line[2].parse::<usize>().unwrap();
                humidity_to_location.push(Map{dest, source, range});
            },
            _ => (),
        }
    }

    let mut lowest = usize::MAX;
    for seed in &seeds {
        let mut num = seed.clone();
        num = map(num, &seed_to_soil);
        num = map(num, &soil_to_fertilizer);
        num = map(num, &fertilizer_to_water);
        num = map(num, &water_to_light);
        num = map(num, &light_to_temperature);
        num = map(num, &temperature_to_humidity);
        num = map(num, &humidity_to_location);
        if num < lowest { lowest = num; }
    }

    println!("Year 2023 day 5 part 1: {}", lowest);

    let mut i = 0;
    'inv_loop: loop {
        let mut tmp = i;
        tmp = map_reverse(tmp, &humidity_to_location);
        tmp = map_reverse(tmp, &temperature_to_humidity);
        tmp = map_reverse(tmp, &light_to_temperature);
        tmp = map_reverse(tmp, &water_to_light);
        tmp = map_reverse(tmp, &fertilizer_to_water);
        tmp = map_reverse(tmp, &soil_to_fertilizer);
        tmp = map_reverse(tmp, &seed_to_soil);
        for pair in seeds.chunks(2) {
            if let &[s, r] = pair {
                if tmp >= s && tmp < s + r {
                    lowest = i;
                    break 'inv_loop;
                }
            }
        }
        i += 1;
    }

    // lowest = usize::MAX;
    // for pair in seeds.chunks(2) {
    //     if let &[s, r] = pair {
    //         let mut tmp = vec![(s, s+r)];
    //     }
    // }

    println!("Year 2023 day 5 part 2: {}", lowest);
}

#[derive(Debug)]
struct Map {
    dest: usize,
    source: usize,
    range: usize,
}

fn map(n: usize, map: &Vec<Map>) -> usize {
    for m in map {
        if n >= m.source && n < m.source + m.range {
            return m.dest + n - m.source;
        }
    }

    n
}

#[allow(dead_code)]
fn map_range(seeds: (usize, usize), map: &Vec<Map>) -> Vec<(usize, usize)> {
    let mut start = seeds.0;
    let end = seeds.1;
    let mut out: Vec<(usize, usize)> = Vec::new();

    for m in map {
        if start >= m.source && start < m.source + m.range {
            if end < m.source + m.range {
                out.push((m.dest + start - m.source, m.dest + end - m.source));
                return out;
            } else {
                out.push((m.dest + start - m.source, m.dest + m.dest - m.source));
                start = m.source + m.range;
            }
        }
    }

    out
}

fn map_reverse(n: usize, map: &Vec<Map>) -> usize {
    for m in map {
        if n >= m.dest && n < m.dest + m.range {
            return m.source + n - m.dest;
        }
    }

    n
}
