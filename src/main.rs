use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let days: Vec<u32> = match args.len() {
        1 => (1..=7).collect(),
        _ => args.iter().skip(1).map(|day| day.parse().unwrap()).collect(),
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
            _ => unreachable!(),
        };

        day_func(input.unwrap().trim_end());
    }
}
