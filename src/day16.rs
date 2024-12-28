use std::collections::{HashMap, HashSet};

const START: &str = "AA";
const TURNS: u32 = 30;

// graph of nodes
// move to nodes and open them or don't open them
// (prim or dijkstra) + 0/1 knapsack
//
// prim's algorithm: start with how many nodes there are
// HashMap of nodes, contain whether they've been went through before
// take node, go next
// to go next, spawn new path for each open/close, handle those spawned paths in a queue

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
            let tunnels: Vec<String> = line[es_index..]
                .split(", ")
                .map(str::trim)
                .map(str::to_owned)
                .collect();

            let location = Location { flow_rate, tunnels };
            (key, location)
        })
        .collect();

    let pressure = simulate2(&locations, &HashSet::new(), locations.get(START).unwrap(), TURNS, 0);
    println!("{pressure}");
}

fn simulate2(
    locations: &HashMap<String, Location>,
    visited_locations: &HashSet<String>,
    cur_location: &Location,
    turn: u32,
    total: u64,
) -> u64 {
    if turn == 0 {
        return total;
    }

    let mut best_total = 0;
    for location_name in cur_location.tunnels.iter() {
        let new_location = locations.get(location_name).unwrap();
        let mut new_visited = visited_locations.clone();
        new_visited.insert(location_name.to_owned());

        // try each path
        let result1 = simulate2(locations, &new_visited, new_location, turn - 1, total);
        best_total = std::cmp::max(best_total, result1);

        if turn >= 2 {
            let result2 = simulate2(locations, &new_visited, new_location, turn - 2, total + new_location.flow_rate);
            best_total = std::cmp::max(best_total, result2);
        }
    }

    best_total
}
