use std::char;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn treetop_tree_house() {
    let filename = "src/day8/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut forest: Vec<Vec<u8>> = Vec::new();

    for line in reader.lines() {
        let line: Vec<char> = line.unwrap().chars().collect();
        let mut row: Vec<u8> = Vec::new();

        for t in line {
            row.push(t.to_digit(10).unwrap() as u8);
        }

        forest.push(row);
    }

    let len = forest.len();
    let width = forest[0].len();
    let mut total = len * 2 + (width - 2) * 2;
    let mut best_view = 0;

    for r in 1..len-1 {
        for c in 1..width-1 {
            let tree = forest[r][c];

            // Up
            let mut up: bool = true;
            let mut up_view = 0;
            for i in (0..r).rev() {
                if forest[i][c] >= tree {
                    up = false;
                    up_view += 1;
                    break;
                }
                up_view += 1;
            }

            // Down
            let mut down: bool = true;
            let mut down_view = 0;
            for i in r+1..len {
                if forest[i][c] >= tree {
                    down = false;
                    down_view += 1;
                    break;
                }
                down_view += 1;
            }

            // Left
            let mut left: bool = true;
            let mut left_view = 0;
            for i in (0..c).rev() {
                if forest[r][i] >= tree {
                    left = false;
                    left_view += 1;
                    break;
                }
                left_view += 1;
            }

            // Right
            let mut right: bool = true;
            let mut right_view = 0;
            for i in c+1..width {
                if forest[r][i] >= tree {
                    right = false;
                    right_view += 1;
                    break;
                }
                right_view += 1;
            }

            if up || down || left || right {
                total += 1;
            }

            let view = up_view * down_view * left_view * right_view;
            if view > best_view { best_view = view; }
        }
    }

    println!("Day 8 part 1: {}", total);
    println!("Day 8 part 2: {}", best_view);
}