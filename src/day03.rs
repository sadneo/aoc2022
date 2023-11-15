use std::collections::HashSet;

fn search_group(group: &[&str]) -> char {
    *group
        .iter()
        .map(|r| r.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
        .unwrap()
        .iter()
        .next()
        .unwrap()
}

fn char_value(character: char) -> u8 {
    match character {
        'a'..='z' => character as u8 - b'a' + 1,
        'A'..='Z' => character as u8 - b'A' + 27,
        _ => panic!("unexpected character input")
    }
}

// map addiction mode
pub fn run(input: &str) {
    let lines = input.lines().collect::<Vec<_>>();

    let score: u32 = lines.iter()
        .map(|line| {
            let tuple = line.split_at(line.len() / 2);
            vec!(tuple.0, tuple.1)
        }).map(|bag| {
            char_value(search_group(&bag)) as u32
        }).sum();
    println!("{}", score);

    let score2: u32 = lines.chunks(3)
        .map(|group| {
            char_value(search_group(group)) as u32
        }).sum();
    println!("{:#?}", score2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_value_works() {
        assert_eq!(char_value('a'), 1);
    }
}
