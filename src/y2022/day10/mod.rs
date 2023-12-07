use std::fs::File;
use std::io::{BufRead, BufReader};

struct Cpu {
    x: i32,
    cycle: u32,
}
impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cycle: 1,
        }
    }
}

pub fn cathode_ray_tube() {
    let filename = "src/day10/input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut cpu = Cpu::new();
    let mut total = 0;
    let mut screenbuf: [char; 240] = ['.'; 240];

    for line in reader.lines() {
        let line = line.unwrap();
        let command: Vec<&str> = line.split(' ').collect();

        total += check_signal(&cpu);
        draw_pixel(&cpu, &mut screenbuf);
        cpu.cycle += 1;

        match command[0] {
            "noop" => (),
            "addx" => {
                total += check_signal(&cpu);
                draw_pixel(&cpu, &mut screenbuf);

                cpu.x += command[1].parse::<i32>().unwrap();
                cpu.cycle += 1;
            }
            _ => (),
        }
    }

    println!("Day 10 part 1: {}", total);
    println!("Day 10 part 2:");
    draw_buf(&screenbuf);
}

fn check_signal(cpu: &Cpu) -> i32 {
    if cpu.cycle == 20 || (cpu.cycle as i32 - 20) % 40 == 0 {
        // println!("{} {} {}", cpu.cycle, cpu.x, cpu.cycle as i32 * cpu.x);
        cpu.cycle as i32 * cpu.x
    } else { 0 }
}

fn draw_pixel(cpu: &Cpu, screenbuf: &mut [char; 240]) {
    let pixel = cpu.cycle as i32 - 1;
    let ds = cpu.x- (pixel - (pixel/40)*40);
    if ds == -1 || ds == 0 || ds == 1 { screenbuf[pixel as usize] = '#'; }

    println!("{} {}", cpu.cycle, cpu.x);
}

fn draw_buf(screenbuf: &[char; 240]) {
    for y in 0..6 {
        for x in 0..40 {
            print!("{}", screenbuf[(x + y*40) as usize]);
        }
        println!();
    }
}