use std::collections::HashSet;

fn prasing(bytes: &[u8], window_size: usize) -> usize {
    for index in 0..bytes.len() {
        let mut set = HashSet::new();
        if bytes[index..index+window_size].iter().all(|byte| set.insert(byte)) {
            return index + window_size;
        }
    }
    panic!();
}

pub fn run(input: &str) {
    let bytes: Vec<u8> = input.bytes().collect();
    println!("{}\n{}", prasing(&bytes, 4), prasing(&bytes, 14));
}
