use std::collections::HashSet;

fn adjacent(head: (i16, i16), tail: (i16, i16)) -> bool {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    (-1..=1).contains(&dx) && (-1..=1).contains(&dy)
}

fn simulate(instructions: &[((i16, i16), i16)], rope_nodes: usize) -> usize {
    let mut rope = vec![(0, 0); rope_nodes];
    let mut set: HashSet<(i16, i16)> = HashSet::new();
    set.insert(rope[rope.len()-1]);

    for ((dx, dy), magnitude) in instructions {
        for _ in 0..*magnitude {
            rope[0].0 += dx;
            rope[0].1 += dy;

            for i in 1..rope.len() {
                if !adjacent(rope[i-1], rope[i]) {
                    rope[i].0 += (rope[i-1].0 - rope[i].0).signum();
                    rope[i].1 += (rope[i-1].1 - rope[i].1).signum();
                }
            }

            set.insert(rope[rope.len()-1]);
        }
    }

    set.len()
}

pub fn run(input: &str) {
    let instructions: Vec<((i16, i16), i16)> = input.lines().map(|line| {
        let mut split = line.split(' ');
        let direction = match split.next().unwrap() {
            "U" => (0, 1),
            "D" => (0, -1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => unreachable!(),
        };
        let magnitude = split.next().unwrap().parse::<i16>().unwrap();

        (direction, magnitude)
    }).collect();

    println!("{}", simulate(&instructions, 2));
    println!("{}", simulate(&instructions, 10));
}

#[cfg(test)]
mod tests {
    use super::adjacent;

    #[test]
    fn adjacency() {
        let head = (0, 0);
        let tail = (-1, 1);
        assert!(adjacent(head, tail));
    }
}
