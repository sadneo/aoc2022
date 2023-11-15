use std::collections::HashMap;

fn search_group(group: &[&str]) -> char {
    let mut map: HashMap<char, usize> = HashMap::new();

    // might be able to make this part more functional
    for item in group {
        for char in item.chars() {
            *map.entry(char).or_insert(0) += 1;
        }
    }
    
    *map.iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k)
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
