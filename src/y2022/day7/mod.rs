use std::fs::File;
use std::io::{BufRead, BufReader};

struct Dir {
    name: String,
    dir_contents: Vec<Dir>,
    file_contents: Vec<FileStruct>,
}
impl Dir {
    fn parse (&mut self, input: &Vec<String>, line: usize) -> usize {
        let mut line = line;

        'outer: loop {
            if line == input.len() { break; }
            let i: Vec<&str> = input[line].split(' ').collect();
            if i[0] == "$" {
                if i[1] == "cd" {
                    if i[2] == ".." {
                        return line + 1;
                    } else {
                        for e in &mut self.dir_contents {
                            if e.name == i[2] {
                                line = e.parse(input, line + 1);
                                continue 'outer;
                            }
                        }
                    }
                }
            } else if i[0] == "dir" {
                self.dir_contents.push(Dir {
                    name: String::from(i[1]),
                    dir_contents: Vec::new(),
                    file_contents: Vec::new(),
                });
            } else {
                self.file_contents.push(FileStruct {
                    name: String::from(i[1]),
                    size: i[0].parse::<u32>().unwrap(),
                })
            }
            line += 1;
        }
        line
    }
    #[allow(dead_code)]
    fn print(&self, offset: u32) {
        for _ in 0..offset {
            print!("-");
        }
        // println!("{} {}", self.name, self.size());
        println!("{}", self.name);

        for d in &self.dir_contents {
            d.print(offset + 1);
        }
        for f in &self.file_contents {
            for _ in 0..=offset {
                print!("-");
            }
            // println!("{} {}", f.name, f.size);
            println!("{}", f.name);
        }
    }
    fn size(&self) -> u32 {
        let mut size = 0;
        for d in &self.dir_contents {
            size += d.size();
        }
        for f in &self.file_contents {
            size += f.size;
        }
        size
    }
    fn part1(&self) -> u32 {
        let mut total = 0;
        let size = self.size();
        if size <= 100000 { total += size; }
        for d in &self.dir_contents {
            total += d.part1()
        }
        total
    }

    fn part2(&self, necessary: u32) -> u32 {
        let mut smallest = 70000000;
        let size = self.size();
        if size >= necessary { smallest = size; }
        for d in &self.dir_contents {
            let size = d.part2(necessary);
            if size >= necessary && size < smallest {
                smallest = size;
            }
        }
        smallest
    }
}
struct FileStruct {
    name: String,
    size: u32,
}

pub fn no_space_left_on_device() {
    let filename = "src/day7/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut input: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        input.push(line);
    }
    let mut root  = Dir {
        name: String::from("/"),
        dir_contents: Vec::new(),
        file_contents: Vec::new(),
    };

    root.parse(&input, 0);
    // root.print(0);
    // root.print2();
    println!("{}", root.part1());

    let necessary = 30000000 - (70000000 - root.size());
    println!("{}", root.part2(necessary));
}

