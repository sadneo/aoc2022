pub fn run(input: &str) {
    let mut elves: Vec<u32> = vec![0];
    let mut current_index = 0;

    for line in input.lines() {
        if line.is_empty() {
            current_index += 1;
            elves.push(0);
            continue;
        }
        
        let calories = line.parse::<u32>().unwrap();
        elves[current_index] += calories;
    }

    elves.sort();
    let solution: &u32 = &elves.as_slice()[elves.len() - 3..].iter().sum();
    println!("{}", solution);
}
