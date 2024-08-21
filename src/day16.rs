use std::collections::HashMap;

const START: &str = "AA";
const TURNS: u32 = 30;

#[derive(Debug, PartialEq)]
struct Location {
    flow_rate: u64,
    tunnels: Vec<String>,
}

pub fn run(input: &str) {
    let locations: HashMap<String, Location> = input
        .lines()
        .map(|line| {
            let key = line[6..=7].to_owned();

            let semicolon_index = line.find(';').unwrap();
            let es_index = line.find("valve").unwrap() + 6;
            let flow_rate = line[23..semicolon_index].parse::<u64>().unwrap();
            let tunnels: Vec<String> = line[es_index..].split(", ").map(str::trim).map(str::to_owned).collect();

            let location = Location { flow_rate, tunnels };
            (key, location)
        })
        .collect();

    let pressure = simulate(&locations, START, 0, &mut 0, &mut vec![], &mut vec![]);
    println!("{pressure}");
}

// check the current location
// if the current location is closed, then open it and then continue
// else, for each tunnel in the current location's tunnels, do something

fn simulate<'a>(
    locations: &'a HashMap<String, Location>,
    current_name: &str,
    mut turns_taken: u32,
    pressure: &mut u64,
    open_valves: &mut Vec<&'a Location>,
    path_taken: &mut Vec<&'a Location>,
) -> u64 {
    if turns_taken == TURNS {
        return *pressure;
    }

    let flow_rate: u64 = open_valves
        .iter()
        .map(|location| location.flow_rate)
        .sum();
    turns_taken += 1;
    *pressure += flow_rate;

    let current_valve = &locations[current_name];
    if !open_valves.contains(&current_valve) && current_valve.flow_rate > 0 {
        open_valves.push(current_valve);
        simulate(
            locations,
            current_name,
            turns_taken,
            pressure,
            open_valves,
            path_taken,
        )
    } else {
        path_taken.push(current_valve);
        let mut pressurepressure = 0;
        for new_name in &current_valve.tunnels {
            let test_pressure = simulate(
                locations,
                new_name,
                turns_taken,
                pressure,
                open_valves,
                path_taken,
            );
            if test_pressure > pressurepressure {
                pressurepressure = test_pressure
            }
        }
        *pressure
    }
}
