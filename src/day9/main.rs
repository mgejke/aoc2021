use std::collections::{HashMap, HashSet, VecDeque};

use aoc2021lib::{get_map, read_lines_to_strings};
use itertools::Itertools;
use rusttype::{Point, Vector};

type P = Point<i32>;
type V = Vector<i32>;

static DIRECTIONS: [V; 4] = [
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];

fn get_lows(map: &HashMap<Point<i32>, i32>) -> Vec<(&Point<i32>, &i32)> {
    let lows = map
        .iter()
        .filter(|(p, v)| {
            DIRECTIONS.iter().all(|d| match map.get(&(**p - *d)) {
                Some(m_v) => *v < m_v,
                None => true,
            })
        })
        .collect_vec();
    lows
}

fn p1(input: Vec<String>) -> String {
    let map = get_map(input);
    let lows = get_lows(&map);
    format!("{}", lows.iter().map(|(_, v)| *v + 1).sum::<i32>())
}

fn p2(input: Vec<String>) -> String {
    let map = get_map(input);
    let lows = get_lows(&map);

    let mut basin_sizes: Vec<usize> = Vec::new();
    for (basin, _) in lows {
        let mut to_explore: VecDeque<P> = VecDeque::from([*basin]);
        let mut explored: HashSet<P> = HashSet::new();
        while !to_explore.is_empty() {
            let p = to_explore.pop_front().unwrap();
            for d in DIRECTIONS {
                let np = p + d;
                if match map.get(&np) {
                    Some(v) => *v != 9,
                    None => false,
                } && !explored.contains(&np)
                {
                    to_explore.push_back(np);
                }
                explored.insert(p);
            }
        }
        basin_sizes.push(explored.len());
    }

    basin_sizes.sort();
    basin_sizes.reverse();
    format!("{:?}", basin_sizes.iter().take(3).product::<usize>())
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_09.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_09.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::p1;
    use crate::p2;

    #[test]
    fn test_p1() {
        let test_data = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        assert_eq!(
            p1(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "15"
        );
    }
    #[test]
    fn test_p2() {
        let test_data = vec![
            "2199943210",
            "3987894921",
            "9856789892",
            "8767896789",
            "9899965678",
        ];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "1134"
        );
    }
}
