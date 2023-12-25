pub fn fully_contains(pair: &[(u8, u8)]) -> bool {
    pair[0].0 <= pair[1].0 && pair[0].1 >= pair[1].1
    || pair[1].0 <= pair[0].0 && pair[1].1 >= pair[0].1
}

pub fn partially_contains(pair: &[(u8, u8)]) -> bool {
    pair[0].0 <= pair[1].0 && pair[0].1 >= pair[1].0
    || pair[0].0 <= pair[1].1 && pair[0].1 >= pair[1].1

    || pair[1].0 <= pair[0].0 && pair[1].1 >= pair[0].0
    || pair[1].0 <= pair[0].1 && pair[1].1 >= pair[0].1
}

pub fn run(input: &str) {
    let elf_pairs = input.lines().map(|line| {
        line.split(',').map(|range_string| {
            let split: Vec<_> = range_string.split('-').collect();
            let start: u8 = split[0].parse().unwrap();
            let end: u8 = split[1].parse().unwrap();
            (start, end)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let count1 = elf_pairs.iter().map(|pair| {
        fully_contains(pair)
    }).filter(|fully_contained| *fully_contained).count();
    println!("{:#?}", count1);

    let count2 = elf_pairs.iter().map(|pair| {
        partially_contains(pair)
    }).filter(|fully_contained| *fully_contained).count();
    println!("{:#?}", count2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fully_contains_1() {
        let vector = vec!((1, 4), (2, 3));
        assert!(fully_contains(&vector));
    }

    #[test]
    fn fully_contains_2() {
        let vector = vec!((2, 8), (3, 7));
        assert!(fully_contains(&vector));
    }

    #[test]
    fn fully_contains_3() {
        let vector = vec!((6, 6), (4, 6));
        assert!(fully_contains(&vector));
    }
}
