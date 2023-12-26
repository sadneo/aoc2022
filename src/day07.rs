use std::collections::HashMap;

// i used a lot of help on this one...
#[derive(Debug)]
struct DirectoryContent {
    total_file_sizes: u64,
    subdirs: Vec<String>,
}

pub fn run(input: &str) {
    let commands: Vec<&str> = input.lines().collect();

    let mut directory_contents: HashMap<Vec<String>, DirectoryContent> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();
    let mut index = 0;

    while index < commands.len() {
        let command = commands[index];
        
        match command {
            "$ cd /" => {
                current_path.clear();
                index += 1;
            }
            "$ cd .." => {
                current_path.pop();
                index += 1;
            }
            "$ ls" => {
                index += 1;
                let mut total_file_sizes: u64 = 0;
                let mut subdirs: Vec<String> = Vec::new();

                while index < commands.len() && !commands[index].starts_with('$') {
                    let entry: Vec<&str> = commands[index].split(' ').collect();
                    if let Ok(size) = entry[0].parse::<u64>() {
                        total_file_sizes += size;
                    } else {
                        subdirs.push(entry[1].to_owned());
                    }
                    index += 1;
                }

                let content = DirectoryContent { total_file_sizes, subdirs };
                directory_contents.insert(current_path.clone(), content);
            }
            _ => {
                let dir = command.split(' ').last().unwrap();
                current_path.push(dir.to_owned());
                index += 1;
            }
        }
    }

    let directory_sizes: Vec<u64> = directory_contents.keys().map(|path| calculate_size(&directory_contents, path)).collect();

    let sum_of_total_small_sizes: u64 = directory_sizes.iter().filter(|size| size <= &&100_000).sum();
    println!("{}", sum_of_total_small_sizes);

    let size_needed: u64 = 30000000 - (70000000 - calculate_size(&directory_contents, &Vec::new()));
    let size_of_deletion: &u64 = directory_sizes.iter().filter(|size| size > &&size_needed).min().unwrap();
    println!("{}", size_of_deletion);
}

fn calculate_size(directory_contents: &HashMap<Vec<String>, DirectoryContent>, path: &Vec<String>) -> u64 {
    let content = directory_contents.get(path).unwrap();
    content.subdirs.iter().map(|subdir_name| {
        let mut subdir_path = path.clone();
        subdir_path.push(subdir_name.to_owned());
        calculate_size(directory_contents, &subdir_path)
    }).sum::<u64>() + content.total_file_sizes
}
