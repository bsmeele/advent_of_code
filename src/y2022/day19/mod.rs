use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn not_enough_minerals() {
    let test = false;
    let filename = if test { "src/y2022/day19/test" } else { "src/y2022/day19/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut blueprints: Vec<Blueprint> = Vec::new();
    let mut time = 24;
    let robots = (1, 0, 0);
    let resources = (0, 0, 0);
    let mut acc = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let line: Vec<&str> = line.split(':').collect();
        let id = line[0].split(' ').collect::<Vec<&str>>()[1].replace(':', " ").parse::<usize>().unwrap();
        let line: Vec<&str> = line[1].split('.').collect();
        let ore = get_cost(line[0]);
        let clay = get_cost(line[1]);
        let obsidian = get_cost(line[2]);
        let geode = get_cost(line[3]);
        blueprints.push(Blueprint{ id, ore, clay, obsidian, geode });
    }

    for blueprint in &blueprints {
        let geodes = max_geodes(blueprint, robots, resources, time, 0, 0);
        // println!("Blueprint {} cracks {} geodes in {} minutes", blueprint.id, geodes, time);
        acc += geodes * blueprint.id;
    }

    println!("Year 2022 day 19 part 1: {}", acc);

    time = 32;
    acc = 1;
    let range = if test { 0..2 } else { 0..3 };
    for i in range {
        let geodes = max_geodes(&blueprints[i], robots, resources, time, 0, 0);
        // println!("Blueprint {} cracks {} geodes in {} minutes", blueprints[i].id, geodes, time);
        acc *= geodes;
    }

    println!("Year 2022 day 19 part 2: {}", acc);
}

fn get_cost(s: &str) -> (usize, usize,usize) {
    let mut cost = (0, 0, 0);
    let s_list: Vec<&str> = s.split(' ').collect();
    for (i, w) in s_list.iter().enumerate() {
        if w == &"ore" { if let Ok(c) = s_list[i - 1].parse::<usize>() { cost.0 = c; } }
        if w == &"clay" { if let Ok(c) = s_list[i - 1].parse::<usize>() { cost.1 = c; } }
        if w == &"obsidian" { if let Ok(c) = s_list[i - 1].parse::<usize>() { cost.2 = c; } }
    }
    cost
}

struct Blueprint {
    id: usize,
    ore: (usize, usize, usize),
    clay: (usize, usize, usize),
    obsidian: (usize, usize, usize),
    geode: (usize, usize, usize),
}

fn max_geodes(blueprint: &Blueprint, robots: (usize, usize, usize), resources: (usize, usize, usize), time: usize, current_geodes: usize, current_max: usize) -> usize {
    if time == 0 { return 0; }

    let time_sum = (time - 1) * time / 2;
    let max_pos = time_sum + current_geodes;
    if current_max >= max_pos { return 0; }

    let mut geodes = 0;
    let mut current_max = current_max;

    if can_afford(resources, &blueprint.ore) {
        let mut tmp_robots = robots;
        tmp_robots.0 += 1;
        let mut tmp_res = tuple_add(&resources, &robots);
        tmp_res = sub_cost(&tmp_res, &blueprint.ore);
        let alt_geodes = max_geodes(blueprint, tmp_robots, tmp_res, time - 1, current_geodes, current_max);
        if alt_geodes > geodes { geodes = alt_geodes; }
        if geodes > current_max { current_max = geodes; }
    }
    if can_afford(resources, &blueprint.clay) {
        let mut tmp_robots = robots;
        tmp_robots.1 += 1;
        let mut tmp_res = tuple_add(&resources, &robots);
        tmp_res = sub_cost(&tmp_res, &blueprint.clay);
        let alt_geodes = max_geodes(blueprint, tmp_robots, tmp_res, time - 1, current_geodes, current_max);
        if alt_geodes > geodes { geodes = alt_geodes; }
        if geodes > current_max { current_max = geodes; }
    }
    if can_afford(resources, &blueprint.obsidian) {
        let mut tmp_robots = robots;
        tmp_robots.2 += 1;
        let mut tmp_res = tuple_add(&resources, &robots);
        tmp_res = sub_cost(&tmp_res, &blueprint.obsidian);
        let alt_geodes = max_geodes(blueprint, tmp_robots, tmp_res, time - 1, current_geodes, current_max);
        if alt_geodes > geodes { geodes = alt_geodes; }
        if geodes > current_max { current_max = geodes; }
    }
    if can_afford(resources, &blueprint.geode) {
        let mut tmp_res = tuple_add(&resources, &robots);
        tmp_res = sub_cost(&tmp_res, &blueprint.geode);
        let alt_geodes = time - 1 + max_geodes(blueprint, robots, tmp_res, time - 1, current_geodes + time - 1, current_max);
        if alt_geodes > geodes { geodes = alt_geodes; }
        if geodes > current_max { current_max = geodes; }
    }

    if !(can_afford(resources, &blueprint.ore) && can_afford(resources, &blueprint.clay) && can_afford(resources, &blueprint.obsidian) && can_afford(resources, &blueprint.geode)){
        let alt_geodes = max_geodes(blueprint, robots, tuple_add(&resources, &robots), time - 1, current_geodes, current_max);
        if alt_geodes > geodes { geodes = alt_geodes; }
    }

    geodes
}

fn can_afford(res: (usize, usize, usize), cost: &(usize, usize, usize)) -> bool {
    if res.0 < cost.0 { false }
    else if res.1 < cost.1 { false }
    else if res.2 < cost.2 { false }
    else { true }
}

fn tuple_add(t1: &(usize, usize, usize), t2: &(usize, usize, usize)) -> (usize, usize, usize) {
    (t1.0 + t2.0, t1.1 + t2.1, t1.2 + t2.2)
}

fn sub_cost(res: &(usize, usize, usize), cost: &(usize, usize, usize)) -> (usize, usize, usize) {
    (res.0 - cost.0, res.1 - cost.1, res.2 - cost.2)
}
