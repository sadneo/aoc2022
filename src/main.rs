use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<u32> = match args.len() {
        1 => (1..=16).collect(),
        _ => args
            .iter()
            .skip(1)
            .map(|day| day.parse().unwrap())
            .collect(),
    };

    for day in &days {
        println!("Day {}:", day);
        let path = format!("./data/day{:02}.txt", day);
        let input = fs::read_to_string(path);

        if let Err(error) = input {
            println!("Error! No data for day {}:\n{}", day, error);
            continue;
        }

        let day_func = match day {
            1 => aoc2022::day01::run,
            2 => aoc2022::day02::run,
            3 => aoc2022::day03::run,
            4 => aoc2022::day04::run,
            5 => aoc2022::day05::run,
            6 => aoc2022::day06::run,
            7 => aoc2022::day07::run,
            8 => aoc2022::day08::run,
            9 => aoc2022::day09::run,
            10 => aoc2022::day10::run,
            11 => aoc2022::day11::run,
            12 => aoc2022::day12::run,
            13 => aoc2022::day13::run,
            14 => aoc2022::day14::run,
            16 => aoc2022::day16::run,
            _ => unreachable!(),
        };

        day_func(input.unwrap().trim_end());
    }
}
