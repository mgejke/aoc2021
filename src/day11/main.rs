use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, Write},
};

use aoc2021lib::{get_map, read_lines_to_strings};
use rusttype::{Point, Vector};

type V = Vector<i32>;

static NEIGHBOURS: [V; 8] = [
    V { x: -1, y: -1 },
    V { x: 1, y: -1 },
    V { x: -1, y: 1 },
    V { x: 1, y: 1 },
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];

#[derive(Debug, Clone, Copy)]
struct Octopus {
    value: i32,
    flashed: bool,
}

fn p1(input: Vec<String>, p2: bool, iteration: i32) -> String {
    let map = get_map(input);

    let mut map = map
        .into_iter()
        .map(|(p, v)| {
            (
                p,
                RefCell::new(Octopus {
                    value: v,
                    flashed: false,
                }),
            )
        })
        .collect::<HashMap<Point<i32>, RefCell<Octopus>>>();

    let mut flash_count = 0;
    for i in 0..iteration {
        let mut flash_per_iteration = 0;
        for (_, v) in map.iter() {
            v.borrow_mut().value += 1;
        }

        let mut to_be_flashed: Vec<(Point<i32>, &RefCell<Octopus>)> = Vec::new();
        for (p, r_o) in map.iter() {
            if r_o.borrow().value > 9 {
                r_o.borrow_mut().flashed = true;
                flash_per_iteration += 1;
                to_be_flashed.push((*p, r_o));
            }
        }
        while !to_be_flashed.is_empty() {
            let (p, _) = to_be_flashed.pop().unwrap();

            for v in &NEIGHBOURS {
                let np = p + *v;
                if let Some(a) = map.get(&np) {
                    let mut b = a.borrow_mut();
                    b.value += 1;
                    if b.value > 9 && !b.flashed {
                        b.flashed = true;
                        flash_per_iteration += 1;
                        to_be_flashed.push((np, a));
                    }
                }
            }
        }
        map = map
            .iter()
            .map(|(p, r_o)| {
                if r_o.borrow().flashed {
                    let mut o = r_o.borrow_mut();
                    o.flashed = false;
                    o.value = 0;
                }
                (*p, r_o.clone())
            })
            .collect();
        flash_count += flash_per_iteration;

        if p2 && flash_per_iteration == 100 {
            return format!("{:?}", i + 1);
        }
    }
    let _ = io::stdout().flush();

    format!("{:?}", flash_count)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_11.txt") {
        println!("P1 - {}", p1(lines, false, 100));
    }
    if let Ok(lines) = read_lines_to_strings("./data/input_day_11.txt") {
        println!("P2 - {}", p1(lines, true, i32::MAX));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    const TEST_DATA: &[&str] = &[
        "5483143223",
        "2745854711",
        "5264556173",
        "6141336146",
        "6357385478",
        "4167524645",
        "2176841721",
        "6882881134",
        "4846848554",
        "5283751526",
    ];

    const TEST_DATA_2: &[&str] = &["11111", "19991", "19191", "19991", "11111"];

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(
                TEST_DATA.iter().map(|s| s.to_string()).collect(),
                false,
                100
            ),
            "1656"
        );
    }

    #[test]
    fn test_p1_1() {
        assert_eq!(
            p1(
                TEST_DATA_2.iter().map(|s| s.to_string()).collect(),
                false,
                2
            ),
            "9"
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p1(
                TEST_DATA.iter().map(|s| s.to_string()).collect(),
                true,
                10000
            ),
            "195"
        );
    }
}
