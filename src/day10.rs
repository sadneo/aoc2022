pub fn run(input: &str) {
    let instructions: Vec<Option<i32>> = input.lines().map(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        match split[0] {
            "noop" => None,
            "addx" => Some(split[1].parse::<i32>().unwrap()),
            _ => unreachable!(),
        }
    }).collect();

    let mut clock = 0;
    let mut x = 1;
    let mut sum = 0;

    for instruction in &instructions {
        if let Some(value) = instruction {
            clock += 1;
            if [20, 60, 100, 140, 180, 220].contains(&clock) { sum += clock * x; }
            clock += 1;
            if [20, 60, 100, 140, 180, 220].contains(&clock) { sum += clock * x; }
            x += value;
        } else {
            clock += 1;
            if [20, 60, 100, 140, 180, 220].contains(&clock) { sum += clock * x; }
        }
    }

    let mut x = 1;
    let mut x_state = vec![1];
    for instruction in &instructions {
        x_state.push(x);
        if let Some(value) = instruction {
            x += value;
            x_state.push(x);
        }
    }

    for (i, value) in x_state.iter().enumerate() {
        let horizontal = (i as i32) % 40;
        if i != 0 && horizontal == 0 {
            println!();
        }

        if (value-1..=value+1).contains(&horizontal) {
            print!("#");
        } else {
            print!(" ");
        }
    }

    println!("{}", sum);
}
