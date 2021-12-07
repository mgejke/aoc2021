use std::collections::HashMap;

use aoc2021lib::read_lines_to_strings;
use itertools::Itertools;
use rusttype::Point;

fn p1(input: Vec<String>, diagonal: bool) -> String {
    let rows: Vec<(&str, &str)> = input
        .iter()
        .map(|row| row.split_once(" -> ").unwrap())
        .collect();

    let coords = rows
        .iter()
        .map(|pair| {
            let p1 = pair.0.split_once(",").unwrap();
            let p2 = pair.1.split_once(",").unwrap();
            (
                Point {
                    x: p1.0.parse::<i32>().unwrap(),
                    y: p1.1.parse::<i32>().unwrap(),
                },
                Point {
                    x: p2.0.parse::<i32>().unwrap(),
                    y: p2.1.parse::<i32>().unwrap(),
                },
            )
        })
        .collect_vec();

    let h_v_lines = coords
        .iter()
        .filter(|(p1, p2)| p1.x == p2.x || p1.y == p2.y)
        .collect_vec();

    let d_lines = coords
        .iter()
        .filter(|(p1, p2)| (diagonal && (p2.x - p1.x).abs() == (p2.y - p1.y).abs()))
        .collect_vec();

    let mut o_floor: HashMap<(i32, i32), i32> = HashMap::new();

    for (p1, p2) in &h_v_lines {
        if p1.x == p2.x {
            for y in std::cmp::min(p1.y, p2.y)..std::cmp::max(p1.y, p2.y) + 1 {
                *o_floor.entry((p1.x, y)).or_insert(0) += 1;
            }
        } else {
            for x in std::cmp::min(p1.x, p2.x)..std::cmp::max(p1.x, p2.x) + 1 {
                *o_floor.entry((x, p1.y)).or_insert(0) += 1;
            }
        }
    }
    if diagonal {
        for (p1, p2) in &d_lines {
            let (dx, dy) = ((p2.x - p1.x).signum(), (p2.y - p1.y).signum());
            for i in 0..(p1.x - p2.x).abs() {
                *o_floor.entry((p1.x + i * dx, p1.y + i * dy)).or_insert(0) += 1;
            }
        }
    }
    format!("{}", o_floor.values().filter(|v| *v >= &2).count())
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_05.txt") {
        println!("P1 - {}", p1(lines, false));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_05.txt") {
        println!("P2 - {}", p1(lines, true));
    }
}

#[test]
fn test_p1() {
    if let Ok(lines) = read_lines_to_strings("./data/test_day_05.txt") {
        assert_eq!(p1(lines, false), "5");
    }
}
#[test]
fn test_p2() {
    if let Ok(lines) = read_lines_to_strings("./data/test_day_05.txt") {
        assert_eq!(p1(lines, true), "12");
    }
}
