fn get_choice(choice: &str) -> u8 {
    match choice {
        "A" | "X" => 0,
        "B" | "Y" => 1,
        "C" | "Z" => 2,
        _ => panic!("nuh uh!"),
    }
}

fn evaluate_game(them: u8, me: u8) -> u32 {
    let outcome = (3 + them - me) % 3;
    me as u32 + 1 + [3, 0, 6][outcome as usize]
}

fn evaluate_move(them: u8, outcome: u8) -> u8 {
    (2 + them + outcome) % 3
}

pub fn run(input: &str) {
    let strategy: Vec<(u8, u8)> = input.lines()
        .map(|line| {
            line.split(' ').collect::<Vec<_>>()
        })
        .map(|vector| {
            (get_choice(vector[0]), get_choice(vector[1]))
        }).collect();

    let score: u32 = strategy.iter().map(|game| {
        evaluate_game(game.0, game.1)
    }).sum();
    println!("{}", score);

    let score2: u32 = strategy.iter().map(|game| {
        evaluate_game(game.0, evaluate_move(game.0, game.1))
    }).sum();
    println!("{}", score2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_draw() {
        assert_eq!(evaluate_game(0, 0), 4);
        assert_eq!(evaluate_game(1, 1), 5);
        assert_eq!(evaluate_game(2, 2), 6);
    }

    #[test]
    fn eval_rock_scissors() {
        // they are rock, i am scissors
        assert_eq!(evaluate_game(0, 2), 3);
    }

    #[test]
    fn eval_scissors_rock() {
        // they are scissors, i am rock
        assert_eq!(evaluate_game(2, 0), 7);
    }
}
