use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use aoc2021lib::{get_map, read_lines_to_strings, DIRECTIONS};
use itertools::iproduct;
use rusttype::Point;

type P = Point<i32>;

#[derive(Copy, Clone, Eq, PartialEq)]
struct MapPos {
    cost: usize,
    position: P,
}

impl Ord for MapPos {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for MapPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn find_a_path<F>(cost_func: F, pos: P, goal: P, map_size: P) -> usize
where
    F: Fn(&P) -> Option<i32>,
{
    let mut open = BinaryHeap::new();
    open.push(MapPos {
        position: pos,
        cost: 0,
    });

    let mut dist = iproduct!(0..map_size.x, 0..map_size.y)
        .map(|(x, y)| (Point { x, y }, usize::MAX))
        .collect::<HashMap<_, _>>();
    *dist.get_mut(&pos).unwrap() = 0;

    while let Some(MapPos { cost, position }) = open.pop() {
        if position == goal {
            return cost;
        };

        if cost > *dist.get(&position).unwrap() {
            continue;
        }

        for direction in &DIRECTIONS {
            let next_position = position + *direction;
            if let Some(next_cost) = cost_func(&next_position) {
                let next_mappos = MapPos {
                    cost: cost + (next_cost as usize),
                    position: next_position,
                };

                let current_cost = dist.get_mut(&next_position).unwrap();
                if next_mappos.cost < *current_cost {
                    *current_cost = next_mappos.cost;
                    open.push(next_mappos);
                }
            }
        }
    }
    usize::MAX
}

fn p1(input: Vec<String>) -> String {
    let map = get_map(input);

    let goal = map.iter().fold(Point { x: 0, y: 0 }, |p, (k, _v)| Point {
        x: std::cmp::max(p.x, k.x),
        y: std::cmp::max(p.y, k.y),
    });

    let pos = Point { x: 0, y: 0 };

    let cost_func = |p: &P| match map.get(p) {
        Some(v) => Some(*v),
        _ => None,
    };
    let shortest = find_a_path(
        &cost_func,
        pos,
        goal,
        P {
            x: goal.x + 1,
            y: goal.y + 1,
        },
    );

    format!("{}", shortest)
}

fn p2(input: Vec<String>) -> String {
    let map = get_map(input);

    let max = map.iter().fold(Point { x: 0, y: 0 }, |p, (k, _v)| Point {
        x: std::cmp::max(p.x, k.x),
        y: std::cmp::max(p.y, k.y),
    });
    let map_size = P {
        x: (max.x + 1) * 5,
        y: (max.y + 1) * 5,
    };
    let goal = P {
        x: map_size.x - 1,
        y: map_size.y - 1,
    };

    let pos = Point { x: 0, y: 0 };

    let cost_func = |p: &P| {
        if p.x > goal.x || p.y > goal.y {
            return None;
        }
        let added_val = p.x / (max.x + 1) + p.y / (max.y + 1);
        let newp = P {
            x: p.x % (max.x + 1),
            y: p.y % (max.y + 1),
        };
        match map.get(&newp) {
            Some(v) => Some((*v + added_val - 1) % 9 + 1),
            _ => None,
        }
    };
    let shortest = find_a_path(&cost_func, pos, goal, map_size);
    format!("{}", shortest)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_15.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_15.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    const TEST_DATA: &[&str] = &[
        "1163751742",
        "1381373672",
        "2136511328",
        "3694931569",
        "7463417111",
        "1319128137",
        "1359912421",
        "3125421639",
        "1293138521",
        "2311944581",
    ];

    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_DATA.iter().map(|s| s.to_string()).collect()), "40");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_DATA.iter().map(|s| s.to_string()).collect()), "315");
    }
}
