#[derive(Debug)]
struct Instruction {
    pub count: u8,
    pub from: usize,
    pub to: usize,
}

pub fn run(input: &str) {
    let split = input.split("\n\n").collect::<Vec<_>>();
    let rows: Vec<Vec<char>> = split[0].lines().rev().skip(1).map(|line| {
        line.as_bytes().chunks(4).map(|crate_element| {
            crate_element[1] as char
        }).collect::<Vec<char>>()
    }).collect();
    
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); rows[0].len()];
    for row in rows {
        for (index, value) in row.iter().enumerate() {
            if *value == ' ' { continue }
            stacks[index].push(*value);
        }
    }
    let mut stacks2 = stacks.clone();

    let instructions: Vec<Instruction> = split[1].lines().map(|instruction| {
        let split: Vec<&str> = instruction.split(' ').collect();
        Instruction {
            count: split[1].parse().unwrap(),
            from: split[3].parse::<usize>().unwrap() - 1,
            to: split[5].parse::<usize>().unwrap() - 1,
        }
    }).collect();

    for instruction in &instructions {
        for _ in 0..instruction.count {
            let item = stacks[instruction.from].pop().expect("i trust you aoc");
            stacks[instruction.to].push(item);
        }
    }

    for instruction in &instructions {
        let mut queue = Vec::with_capacity(instruction.count.into());

        for _ in 0..instruction.count {
            let item = stacks2[instruction.from].pop().unwrap();
            queue.push(item);
        }
        for _ in 0..instruction.count {
            let item = queue.pop().unwrap();
            stacks2[instruction.to].push(item);
        }
    }

    for stack in stacks {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
    for stack in stacks2 {
        print!("{}", stack[stack.len() - 1]);
    }
    println!();
}
