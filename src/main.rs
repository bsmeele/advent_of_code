mod y2022;
mod y2023;
mod web_stuff;
mod new_year;

use clap::Parser;
use std::{fs, io};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Use test input
    #[arg(short, long, default_value_t = false)]
    test: bool,

    // Year to execute
    #[arg(short, long, default_value_t = 2023)]
    year: u16,

    // Day to execute
    #[arg(short, long, default_value_t = 0)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    match args.year {
        2023 => y2023::y2023(args.day),
        2022 => y2022::y2022(args.day),
        year => {
             if let Ok(metadata) = fs::metadata(format!("src/y{}", year)) {
                 if metadata.is_dir() {
                     println!("Year {} exists but has not been added attached to main", year);
                     return;
                 }
             }
             println!("Year {} does not exist. Generate a template year {}?", year, year);
             loop {
                 let mut user_input = String::new();
                 io::stdin().read_line(&mut user_input).expect("Failed to read line");
                 match user_input.replace('\n', "").replace('\r', "").as_str() {
                     "y" | "year" => {
                         new_year::new_year(year).unwrap();
                         println!("Template generated for year {}", year);
                         break;
                     },
                     "exit" | "n" | "no" => break,
                     _ => println!("Input not recognized"),
                 }
             }
         },
    }
}







