use std::fs::{create_dir, File, OpenOptions};
use std::io::{self, Read, Write};
use tera::Tera;

pub fn new_year(year: u16) -> io::Result<()> {
    // Create year directory
    create_dir(format!("src/y{}", year).as_str())?;

    // Create year mod file
    let mut context = tera::Context::new();
    context.insert("year", &year);
    let mut template = String::new();
    File::open("src/new_year/year_template")?.read_to_string(&mut template)?;
    let render = Tera::one_off(template.as_str(), &context, false).unwrap();
    let mut file = File::create(format!("src/y{}/mod.rs", year))?;
    file.write_all(render.as_bytes())?;

    // Create day directories and mod files
    let mut template = String::new();
    File::open("src/new_year/day_template")?.read_to_string(&mut template)?;
    for day in 1..=25 {
        context.insert("day", &day);
        create_dir(format!("src/y{}/day{}", year, day).as_str())?;
        let render = Tera::one_off(template.as_str(), &context, false).unwrap();
        let mut file = File::create(format!("src/y{}/day{}/mod.rs", year, day).as_str())?;
        file.write_all(render.as_bytes())?;
    }

    let mut file_content = String::new();
    let mut file = OpenOptions::new().read(true).open("src/main.rs")?;
    file.read_to_string(&mut file_content)?;

    // Add year to match statement
    let position = file_content.find("       year => {").unwrap_or_else(|| file_content.len());
    file_content.insert_str(position, format!("       {} => y{}::y{}(args.day, args.test),\n ", year, year, year).as_str());

    // Add "mod {year}"
    let mut file = OpenOptions::new().write(true).truncate(true).open("src/main.rs")?;
    writeln!(file, "mod y{};\n{}", year, file_content)?;

    Ok(())
}
