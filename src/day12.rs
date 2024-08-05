use core::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Coordinate = (usize, usize);
type Neighbors = Vec<Coordinate>;

#[derive(Debug, PartialEq)]
struct TileMap {
    area: Vec<Vec<Tile>>,
    graph: HashMap<Coordinate, Neighbors>,
    start: Coordinate,
    end: Coordinate,
}

impl TileMap {
    fn new(area: Vec<Vec<Tile>>, graph: HashMap<Coordinate, Neighbors>) -> TileMap {
        let (start, end) = Self::find(&area);
        TileMap {
            area,
            graph,
            start,
            end,
        }
    }

    fn with_start(
        area: Vec<Vec<Tile>>,
        graph: HashMap<Coordinate, Neighbors>,
        start: Coordinate,
    ) -> TileMap {
        let (_, end) = Self::find(&area);
        TileMap {
            area,
            graph,
            start,
            end,
        }
    }

    fn find(area: &[Vec<Tile>]) -> (Coordinate, Coordinate) {
        let mut start = (0, 0);
        let mut end = (0, 0);
        for (x, row) in area.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Start => start = (x, y),
                    Tile::End => end = (x, y),
                    _ => continue,
                }
            }
        }
        (start, end)
    }
}

#[derive(Debug, PartialEq, Clone)]
enum Tile {
    Start, // elevation 0
    End,   // elevation 25
    Elevation(u8),
}

impl Tile {
    fn elevation(&self) -> u8 {
        match self {
            Tile::Start => 0,
            Tile::End => 25,
            Tile::Elevation(e) => *e,
        }
    }
    fn can_reach(&self, other: &Tile) -> bool {
        other.elevation().saturating_sub(self.elevation()) <= 1
    }
}

pub fn run(input: &str) {
    let area: Vec<Vec<Tile>> = input
        .split('\n')
        .map(|line| {
            line.bytes()
                .filter_map(|byte| match byte {
                    b'S' => Some(Tile::Start),
                    b'E' => Some(Tile::End),
                    b'a'..=b'z' => Some(Tile::Elevation(byte - b'a')),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let rows = input.split('\n').count();
    let columns = input.split('\n').next().unwrap().len();

    let tile_count = area.iter().flatten().count();
    let mut graph: HashMap<Coordinate, Neighbors> = HashMap::with_capacity(tile_count);
    for (ix, row) in area.iter().enumerate() {
        for (iy, tile) in row.iter().enumerate() {
            let mut vector = vec![(ix, iy + 1), (ix + 1, iy)];
            if iy > 0 {
                vector.push((ix, iy - 1));
            }
            if ix > 0 {
                vector.push((ix - 1, iy));
            }

            let directions = vector
                .iter()
                .filter(|(x, y)| *x < rows && *y < columns && tile.can_reach(&area[*x][*y]))
                .map(|(x, y)| (*x, *y))
                .collect();

            graph.insert((ix, iy), directions);
        }
    }

    let tile_map = TileMap::new(area.clone(), graph.clone());
    let result = explore(tile_map).unwrap();
    println!("RESULT {}", result);

    let mut a_list: Vec<Coordinate> = vec![];
    for (ix, row) in area.iter().enumerate() {
        for (iy, tile) in row.iter().enumerate() {
            if tile.elevation() == 0 {
                a_list.push((ix, iy));
            }
        }
    }

    let result2 = a_list
        .iter()
        .filter_map(|start| explore(TileMap::with_start(area.clone(), graph.clone(), *start)))
        .min()
        .unwrap();
    println!("{}", result2);
}

fn explore(tile_map: TileMap) -> Option<u32> {
    let mut open = BinaryHeap::from([(Reverse(0), tile_map.start)]);
    let mut steps = HashMap::from([(tile_map.start, 0)]);
    while let Some((_, pos)) = open.pop() {
        if pos == tile_map.end {
            return steps.get(&pos).copied();
        }

        let Some(neighbors) = tile_map.graph.get(&pos) else {
            continue;
        };
        for neighbor in neighbors {
            let next_steps = steps.get(&pos).unwrap() + 1;
            let curr_steps = *steps.get(neighbor).unwrap_or(&u32::MAX);
            if next_steps >= curr_steps {
                continue;
            }
            open.push((Reverse(next_steps), *neighbor));
            steps.insert(*neighbor, next_steps);
        }
    }
    None
}
