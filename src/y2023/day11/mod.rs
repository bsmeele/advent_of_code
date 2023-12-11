use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn cosmic_expansion() {
    let test = false;
    let filename = if test { "src/y2023/day11/test" } else { "src/y2023/day11/input" };
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut universe: HashSet<(usize, usize)> = HashSet::new();
    let mut expansion = 2;
    let mut empty;
    let mut x = 0;
    let mut y = 0;
    let mut empty_row: Vec<usize> = Vec::new();
    let mut empty_col: Vec<usize> = Vec::new();

    for line in reader.lines().map(|x| x.unwrap()) {
        x = 0;
        empty = true;
        for c in line.chars() {
            if c == '#' {
                universe.insert((x, y));
                empty = false;
            }
            x += 1;
        }
        if empty { empty_row.push(y); }
        y += 1;
    }

    for rx in 0..x {
        empty = true;
        for ry in 0..y {
            if universe.contains(&(rx, ry)) {
                empty = false;
                break;
            }
        }
        if empty {
            empty_col.push(rx);
        }
    }

    let mut universe1 = universe.clone().into_iter().collect::<Vec<(usize, usize)>>();
    for r in empty_row.iter().rev() {
        for (_, y) in universe1.iter_mut() {
            if *y > *r { *y += expansion - 1; }
        }
    }
    for c in empty_col.iter().rev() {
        for (x, _) in universe1.iter_mut() {
            if *x > *c { *x += expansion - 1; }
        }
    }

    #[allow(unused_assignments)]
    {
        x += empty_col.len() * (expansion - 1);
        y += empty_row.len() * (expansion - 1);
    }

    // draw_universe(&universe, (x, y));

    let mut acc = 0;
    for g1 in 0..universe1.len() {
        for g2 in g1+1..universe1.len() {
            let g1 = universe1[g1];
            let g2 = universe1[g2];
            let dist = (g1.0 as isize - g2.0 as isize).abs() + (g1.1 as isize - g2.1 as isize).abs();
            acc += dist;
        }
    }

    println!("Year 2023 day 11 part 1: {}", acc);

    expansion = 1000000;
    let mut universe2 = universe.into_iter().collect::<Vec<(usize, usize)>>();
    for r in empty_row.iter().rev() {
        for (_, y) in universe2.iter_mut() {
            if *y > *r { *y += expansion - 1; }
        }
    }
    for c in empty_col.iter().rev() {
        for (x, _) in universe2.iter_mut() {
            if *x > *c { *x += expansion - 1; }
        }
    }

    acc = 0;
    for g1 in 0..universe2.len() {
        for g2 in g1+1..universe2.len() {
            let g1 = universe2[g1];
            let g2 = universe2[g2];
            let dist = (g1.0 as isize - g2.0 as isize).abs() + (g1.1 as isize - g2.1 as isize).abs();
            acc += dist;
        }
    }

    println!("Year 2023 day 11 part 2: {}", acc);
}

#[allow(dead_code)]
fn draw_universe(universe: &Vec<(usize, usize)>, bounds: (usize, usize)) {
    let mut tmp: HashSet<(usize, usize)> = HashSet::new();
    for (x, y) in universe {
        tmp.insert((*x, *y));
    }
    for y in 0..bounds.1 {
        for x in 0..bounds.0 {
            if universe.contains(&(x, y)) { print!("#"); }
            else { print!("."); }
        }
        println!();
    }
}