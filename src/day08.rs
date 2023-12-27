fn visible_direction(trees: &[u8], columns: usize, index: usize, dr: i8, dc: i8, count: i8) -> bool {
    let row = (index / columns) as i8 + dr * count;
    let column = (index % columns) as i8 + dc * count;

    if row < 0 || row >= columns as i8 || column < 0 || column >= columns as i8 {
        return true;
    }

    let offset_index = row as usize * columns + column as usize;
    if trees[index] > trees[offset_index] {
        visible_direction(trees, columns, index, dr, dc, count + 1)
    } else {
        false
    }
}

fn score_direction(trees: &[u8], columns: usize, index: usize, dr: i8, dc: i8) -> u32 {
    let mut row = (index / columns) as i8 + dr;
    let mut column = (index % columns) as i8 + dc;

    let mut acc = 0;

    while row >= 0 && row < columns as i8 && column >= 0 && column < columns as i8 {
        acc += 1;

        let offset_index = row as usize * columns + column as usize;
        if trees[index] <= trees[offset_index] {
            return acc;
        }

        row += dr;
        column += dc;
    }

    acc
}

pub fn run(input: &str) {
    let columns: usize = input.find('\n').unwrap();
    let trees: Vec<u8> = input.replace('\n', "").chars().map(|c| c.to_digit(10).unwrap() as u8).collect();

    let visible_trees: Vec<bool> = (0..trees.len()).map(|i| {
        visible_direction(&trees, columns, i, 0, 1, 1) ||
        visible_direction(&trees, columns, i, 0, -1, 1) ||
        visible_direction(&trees, columns, i, 1, 0, 1) ||
        visible_direction(&trees, columns, i, -1, 0, 1)
    }).collect();

    let solution1 = visible_trees.iter().filter(|&&value| value).count();
    println!("{}", solution1);

    let scored_trees = (0..trees.len()).map(|i| {
        score_direction(&trees, columns, i, 1, 0) *
        score_direction(&trees, columns, i, -1, 0) *
        score_direction(&trees, columns, i, 0, 1) *
        score_direction(&trees, columns, i, 0, -1)
    }).max().unwrap();
    println!("{}", scored_trees);
}
