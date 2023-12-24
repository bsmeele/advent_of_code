use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn step_counter() {
    let test = true;
    let filename = if test { "src/y2023/day21/test2" } else { "src/y2023/day21/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut steps = if test { 6 } else { 64 };
    let mut rocks: HashSet<(isize, isize)> = HashSet::new();
    let mut plots: HashSet<(isize, isize)> = HashSet::new();
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

    // Notes:
    //   After a certain amount of steps, the base (non expanded) map alternates between two possible reach sets
    //   After this, if the number of steps is even, only the even coordinates are reachable
    //   For odd steps, only the odd coordinates
    //   A coordinate being even or odd based on whether the sum of the coordinates is even or odd
    //   For the test map this state is reached after 13 steps
    //   For the input map it is 129

    // Todo:
    //   Find the worst case amount of steps it takes to reach a cycle state
    //   Find the shortest path from the start to each edge
    //   Find the shortest path from the shortest edge entry point to each other edge

    steps = if test { 5000 } else { 26_501_365 };
    steps = 17;
    reachable = HashSet::new();
    let mut cache: HashSet<((isize, isize), usize)> = HashSet::new();
    let mut q: Vec<((isize, isize), usize)> = Vec::from([(start, steps)]);
    while let Some((loc, step)) = q.pop() {
        if step == 0 {
            reachable.insert(loc);
            continue;
        }
        if cache.contains(&(loc, step)) { continue; }

        if plots.contains(&(loc.0+1, loc.1)) { q.push(((loc.0+1, loc.1), step-1)); }
        if plots.contains(&(loc.0-1, loc.1)) { q.push(((loc.0-1, loc.1), step-1)); }
        if plots.contains(&(loc.0, loc.1+1)) { q.push(((loc.0, loc.1+1), step-1)); }
        if plots.contains(&(loc.0, loc.1-1)) { q.push(((loc.0, loc.1-1), step-1)); }

        cache.insert((loc, step));
    }

    println!("Year 2023 day 21 part 2: {}", reachable.len());
    draw_map(&plots, &reachable, start, (x, y));
}

fn walk(plots: &HashSet<(isize, isize)>, current: (isize, isize), steps: usize, bounds: (isize, isize), cache: &mut HashSet<((isize, isize), usize)>, part2: bool) -> HashSet<(isize, isize)> {
    if steps == 0 { return HashSet::from([current]); }
    if cache.contains(&(current, steps)) { return HashSet::new(); }

    let mut acc: HashSet<(isize, isize)> = HashSet::new();

    if plots.contains(&(current.0+1, current.1)) { acc.extend(walk(plots, (current.0+1, current.1), steps-1, bounds, cache, part2)); }
    if current.0 > 0 && plots.contains(&(current.0-1, current.1)) { acc.extend(walk(plots, (current.0-1, current.1), steps-1, bounds, cache, part2));  }
    if plots.contains(&(current.0, current.1+1)) { acc.extend(walk(plots, (current.0, current.1+1), steps-1, bounds, cache, part2)); }
    if current.1 > 0 && plots.contains(&(current.0, current.1-1)) { acc.extend(walk(plots, (current.0, current.1-1), steps-1, bounds, cache, part2)); }

    cache.insert((current, steps));

    acc
}

#[allow(dead_code)]
fn draw_map(plots: &HashSet<(isize, isize)>, reachable: &HashSet<(isize, isize)>, start: (isize, isize), bounds: (isize, isize), expand: usize) {
    for y in 0..bounds.1*expand {
        // if y == 11 || y == 22 { println!("-----------------------------------")}
        for x in 0..bounds.0*expand {
            // if x == 11 || x == 22 { print!("|"); }
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