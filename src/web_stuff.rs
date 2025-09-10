use std::error::Error;
use std::fs::{self, File};
use std::io::Write;

pub fn get_input(year: usize, day: u8) -> Result<(), Box<dyn Error>> {
    let session_cookie = fs::read_to_string("src/session_cookie.txt").expect("Issue opening file").trim().to_string();

    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day);

    let response = ureq::get(&url).set("Cookie", format!("session={}", session_cookie).as_str()).call()?;

    let input_data = response.into_string()?;
    let output_file_path = format!("src/y{}/day{}/input", year, day);
    let mut output_file = File::create(&output_file_path)?;
    output_file.write_all(&input_data.as_bytes())?;
    println!("Input data copied successfully to: {}", output_file_path);
    Ok(())
}
