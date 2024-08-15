use std::collections::HashSet;

const ROW: i64 = 2_000_000;
const TUNING_NUMBER: i64 = 4_000_000;

fn solve1(sensor_beacons: &[(i64, i64, i64, i64)]) -> u64 {
    let mut unmerged_ranges: Vec<(i64, i64)> = sensor_beacons
        .iter()
        .filter_map(|(sx, sy, bx, by)| {
            let beacon_distance = sx.abs_diff(*bx) + sy.abs_diff(*by);
            let row_distance = sy.abs_diff(ROW);

            if row_distance > beacon_distance {
                return None;
            }

            let spread = beacon_distance - row_distance;
            let lower_bound = sx.saturating_sub_unsigned(spread);
            let upper_bound = sx.saturating_add_unsigned(spread);
            Some((lower_bound, upper_bound))
        })
        .collect();
    unmerged_ranges.sort_by(|(a, _), (b, _)| a.cmp(b));

    let mut merged_ranges: Vec<(i64, i64)> = vec![];
    for unmerged_range in unmerged_ranges {
        if let Some(range) = merged_ranges.last_mut() {
            if range.1 >= unmerged_range.0 && unmerged_range.1 >= range.0 {
                range.0 = range.0.min(unmerged_range.0);
                range.1 = range.1.max(unmerged_range.1);
            }
        } else {
            merged_ranges.push(unmerged_range);
        }
    }

    let mut x = 0;
    for (lower_bound, upper_bound) in merged_ranges {
        x += lower_bound.abs_diff(upper_bound) + 1;
    }

    let count = sensor_beacons
        .iter()
        .filter_map(|(_, _, bx, by)| (ROW == *by).then_some(*bx))
        .collect::<HashSet<i64>>()
        .len();

    x - count as u64
}

fn solve2(sensor_beacons: &[(i64, i64, i64, i64)]) -> i64 {
    let sensor_distances: Vec<_> = sensor_beacons
        .iter()
        .map(|(sx, sy, bx, by)| (*sx, *sy, (sx - bx).abs() + (sy - by).abs())).collect();

    for (sx, sy, distance) in &sensor_distances {
        for ry in -(distance+1)..=distance+1 {
            let rx = distance + 1 - ry.abs();
            
            let y = *sy + ry;
            if y < 0 || y > TUNING_NUMBER {
                continue;
            }

            if (*sx-rx) >= 0 && (*sx-rx) <= TUNING_NUMBER && out_of_range(&sensor_distances, *sx - rx, y) {
                return (*sx - rx) * TUNING_NUMBER + y;
            }
            if (*sx+rx) >= 0 && (*sx+rx) <= TUNING_NUMBER && out_of_range(&sensor_distances, *sx + rx, y) {
                return (*sx + rx) * TUNING_NUMBER + y;
            }
        }
    }
    unreachable!();
}

fn out_of_range(sensor_distances: &[(i64, i64, i64)], x: i64, y: i64) -> bool {
    for (sx, sy, distance) in sensor_distances {
        if (sx - x).abs() + (sy - y).abs() <= *distance {
            return false;
        }
    }
    true
}

pub fn run(input: &str) {
    let sensor_beacons: Vec<(i64, i64, i64, i64)> = input
        .lines()
        .filter_map(|line| {
            let [sx, sy, bx, by] = line
                .split('=')
                .skip(1)
                .map(parse_number)
                .collect::<Vec<_>>()[..]
            else {
                return None;
            };
            Some((sx, sy, bx, by))
        })
        .collect();

    println!("{}", solve1(&sensor_beacons));
    println!("{}", solve2(&sensor_beacons));
}

fn parse_number(slice: &str) -> i64 {
    if let Some(i) = slice.find(',') {
        let (s, _) = slice.split_at(i);
        s.parse().unwrap()
    } else if let Some(i) = slice.find(':') {
        let (s, _) = slice.split_at(i);
        s.parse().unwrap()
    } else {
        slice.parse().unwrap()
    }
}
