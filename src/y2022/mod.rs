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

pub fn y2022(day: u8) {
    if day > 25 {
        println!("Invalid day");
        return;
    }

    let start_time = Instant::now();

    if day == 0 {
        for day in 1..=25 {
            y2022(day);
        }
        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Total time: {:.2?}", elapsed_time);
        return;
    }
    else {
        match day {
            1 => day1::calorie_counting(),
            2 => day2::rock_paper_scissors(),
            3 => day3::rucksack_reorganization(),
            4 => day4::camp_cleanup(),
            5 => day5::supply_stacks(),
            6 => day6::tuning_trouble(),
            7 => day7::no_space_left_on_device(),
            8 => day8::treetop_tree_house(),
            9 => day9::rope_bridge(),
            10 => day10::cathode_ray_tube(),
            11 => day11::monkey_in_the_middle(),
            12 => day12::hill_climbing_algorithm(),
            13 => day13::distress_signal(),
            14 => day14::regolith_reservoir(),
            15 => day15::beacon_exclusion_zone(),
            16 => day16::proboscidea_volcanium(),
            17 => day17::pyroclastic_flow(),
            18 => day18::boiling_boulders(),
            19 => day19::not_enough_minerals(),
            20 => day20::grove_positioning_system(),
            21 => day21::monkey_math(),
            22 => day22::monkey_map(),
            23 => println!("Not yet implemented"),
            24 => println!("Not yet implemented"),
            25 => println!("Not yet implemented"),
            _ => println!("Invalid day"),
        }

        let end_time = Instant::now();
        let elapsed_time = end_time - start_time;
        println!("Year 2022 day {} executed in {:.2?}", day, elapsed_time);
    }
}
