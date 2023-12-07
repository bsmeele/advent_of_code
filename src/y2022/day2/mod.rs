use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn rock_paper_scissors() {
    let test = false;
    let file = if test { "src/y2022/day2/test" } else { "src/y2022/day2/input" };
    let file = File::open(file).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut total1 = 0;
    let mut total2 = 0;

    for (_, line) in reader.lines().enumerate() {
        let line: Vec<char> = line.unwrap().chars().collect();
        match line[2] {
            'X' => { // Rock or lose
                total1 += 1;
                total2 += 0;
                match line[0] {
                    'A' => { // Rock
                        total1 += 3;
                        total2 += 3;
                    },
                    'B' => { // Paper
                        total1 += 0;
                        total2 += 1;
                    },
                    'C' => { // Scissors
                        total1 += 6;
                        total2 += 2;
                    },
                    _ => (),
                }
            },
            'Y' => { // Paper or draw
                total1 += 2;
                total2 += 3;
                match line[0] {
                    'A' => { // Rock
                        total1 += 6;
                        total2 += 1;
                    },
                    'B' => { // Paper
                        total1 += 3;
                        total2 += 2;
                    },
                    'C' => { // Scissors
                        total1 += 0;
                        total2 += 3;
                    },
                    _ => (),
                }
            },
            'Z' => { // Scissors or win
                total1 += 3;
                total2 += 6;
                match line[0] {
                    'A' => { // Rock
                        total1 += 0;
                        total2 += 2;
                    },
                    'B' => { // Paper
                        total1 += 6;
                        total2 += 3;
                    },
                    'C' => { // Scissors
                        total1 += 3;
                        total2 += 1;
                    },
                    _ => (),
                }
            }
            _ => (),
        }
    }
    println!("Day 2 part 1: {}", total1);
    println!("Day 2 part 2: {}", total2);

}

// A for rock
// B for paper
// C for scissors

// X for rock or lose
// Y for paper or draw
// Z for scissors or win

// 1 for rock
// 2 for paper
// 3 for scissors

// 0 for loss
// 3 for draw
// 6 for win