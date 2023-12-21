use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn step_counter() {
    let test = true;
    let filename = if test { "src/y2023/day21/test" } else { "src/y2023/day21/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut steps = if test { 6 } else { 64 };
    let mut rocks: HashSet<(usize, usize)> = HashSet::new();
    let mut plots: HashSet<(usize, usize)> = HashSet::new();
    let mut start = (0, 0);
    let mut x = 0;
    let mut y = 0;

    for line in reader.lines().map(|x| x.unwrap()) {
        x = 0;
        for c in line.chars() {
            match c {
                'S' => {
                    start = (x, y);
                    plots.insert((x, y));
                },
                '.' => { plots.insert((x, y)); },
                '#' => { rocks.insert((x, y)); },
                _ => panic!("Unreachable"),
            }
            x += 1;
        }
        y += 1;
    }

    // draw_map(&plots, &HashSet::new(), start, (x, y));
    let mut reachable = walk(&plots, start, steps, (x, y), &mut HashSet::new(), false);
    // draw_map(&plots, &reachable, start, (x, y));

    println!("Year 2023 day 21 part 1: {}", reachable.len());

    steps = if test { 5000 } else { 26501365 };
    reachable = walk(&plots, start, steps, (x, y), &mut HashSet::new(), false);
    println!("Year 2023 day 21 part 2: {}", reachable.len());
    // draw_map(&plots, &reachable, start, (x, y));
}

fn walk(plots: &HashSet<(usize, usize)>, current: (usize, usize), steps: usize, bounds: (usize, usize), cache: &mut HashSet<((usize, usize), usize)>, part2: bool) -> HashSet<(usize, usize)> {
    if steps == 0 { return HashSet::from([current]); }
    if cache.contains(&(current, steps)) { return HashSet::new(); }

    let mut acc: HashSet<(usize, usize)> = HashSet::new();

    if plots.contains(&(current.0+1, current.1)) { acc.extend(walk(plots, (current.0+1, current.1), steps-1, bounds, cache, part2)); }
    if current.0 > 0 && plots.contains(&(current.0-1, current.1)) { acc.extend(walk(plots, (current.0-1, current.1), steps-1, bounds, cache, part2));  }
    if plots.contains(&(current.0, current.1+1)) { acc.extend(walk(plots, (current.0, current.1+1), steps-1, bounds, cache, part2)); }
    if current.1 > 0 && plots.contains(&(current.0, current.1-1)) { acc.extend(walk(plots, (current.0, current.1-1), steps-1, bounds, cache, part2)); }

    cache.insert((current, steps));

    acc
}

#[allow(dead_code)]
fn draw_map(plots: &HashSet<(isize, isize)>, reachable: &HashSet<(isize, isize)>, start: (isize, isize), bounds: (isize, isize)) {
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            if reachable.contains(&(x, y)) { print!("O"); }
            else if (x, y) == start { print!("S"); }
            // else if rocks.contains(&(x, y)) { print!("#"); }
            else if plots.contains(&(x, y)) { print!(".") }
            else { print!("#"); }
        }
        println!();
    }
    println!();
}