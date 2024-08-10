#[derive(Clone, PartialEq)]
enum Tile {
    Air,
    Sand,
    Stone,
}

type Coord = (usize, usize);

const SAND_START: Coord = (500, 0);
const STOP_X: usize = 700;
const STOP_LINE_Y: usize = 160;

pub fn run(input: &str) {
    let mut map: Vec<Vec<Tile>> = vec![vec![Tile::Air; STOP_LINE_Y]; STOP_X];

    let obstacles: Vec<Vec<Coord>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    let xy: Vec<&str> = coord.split(',').collect();
                    (xy[0].parse().unwrap(), xy[1].parse().unwrap())
                })
                .collect::<Vec<Coord>>()
        })
        .collect();

    for line in obstacles.iter() {
        for i in 0..line.len() - 1 {
            let start = line[i];
            let next = line[i + 1];
            let (direction, magnitude) = unit(
                next.0 as isize - start.0 as isize,
                next.1 as isize - start.1 as isize,
            );

            for m in 0..=magnitude {
                let (x, y) = (
                    (start.0 as isize + direction.0 * m) as usize,
                    (start.1 as isize + direction.1 * m) as usize,
                );
                map[x][y] = Tile::Stone;
            }
        }
    }


    let mut unit_count = 0;
    let mut first_map = map.clone();
    while drop_sand(&mut first_map, SAND_START) {
        unit_count += 1;
    }

    println!("{}", unit_count);

    let floor = obstacles
        .iter()
        .flatten()
        .map(|coord| coord.1)
        .max()
        .unwrap()
        + 2;

    let mut second_map = map.clone();
    for line in second_map.iter_mut() {
        line[floor] = Tile::Stone;
    }

    unit_count = 0;
    while drop_sand(&mut second_map, SAND_START) {
        unit_count += 1
    }
    println!("{}", unit_count + 1);
}

fn unit(x: isize, y: isize) -> ((isize, isize), isize) {
    if x == 0 {
        ((0, y.signum()), y.abs())
    } else if y == 0 {
        ((x.signum(), 0), x.abs())
    } else {
        panic!("x or y should be zero");
    }
}

fn drop_sand(map: &mut [Vec<Tile>], start: Coord) -> bool {
    if start.1 + 1 == STOP_LINE_Y {
        return false;
    }

    if map[start.0][start.1 + 1] == Tile::Air {
        return drop_sand(map, (start.0, start.1 + 1));
    } else if map[start.0 - 1][start.1 + 1] == Tile::Air {
        return drop_sand(map, (start.0 - 1, start.1 + 1));
    } else if map[start.0 + 1][start.1 + 1] == Tile::Air {
        return drop_sand(map, (start.0 + 1, start.1 + 1));
    } else {
        map[start.0][start.1] = Tile::Sand;
        if start == SAND_START {
            return false;
        }
    }
    true
}
