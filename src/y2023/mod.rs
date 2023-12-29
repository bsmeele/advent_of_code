use std::time::Instant;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

pub fn y2023(day: u8) {
    if day > 25 {
        println!("Invalid day");
        return;
    }

    let start_time = Instant::now();

    if day == 0 {
        for day in 1..=25 {
            y2023(day);
        }
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Total time: {:.2?}", elapsed_time);
        return;
    }
    else {
        match day {
            1 => day1::trebuchet(),
            2 => day2::cube_conundrum(),
            3 => day3::gear_ratios(),
            4 => day4::scratchcards(),
            5 => day5::if_you_give_a_seed_fertilizer(),
            6 => day6::wait_for_it(),
            7 => day7::camel_cards(),
            8 => day8::haunted_wasteland(),
            9 => day9::mirage_maintenance(),
            10 => day10::pipe_maze(),
            11 => day11::cosmic_expansion(),
            12 => day12::hot_springs(),
            13 => day13::point_of_incidence(),
            14 => day14::parabolic_reflector_dish(),
            15 => day15::lens_library(),
            16 => day16::the_floor_will_be_lava(),
            17 => day17::clumsy_crucible(),
            18 => day18::lavaduct_lagoon(),
            19 => day19::aplenty(),
            20 => day20::pulse_propagation(),
            21 => day21::step_counter(),
            22 => day22::sand_slabs(),
            23 => println!("Not yet implemented"),
            24 => println!("Not yet implemented"),
            25 => println!("Not yet implemented"),
            _ => println!("Invalid day"),
        }

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Year 2023 day {} executed in {:.2?}", day, elapsed_time);
    }
}
