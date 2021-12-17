use std::collections::HashSet;

use aoc2021lib::read_lines_to_strings;
use regex::Regex;

fn sum_to_n(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn pos_after_n_iterations(initial: i32, iteration: i32, gravity: bool) -> i32 {
    if !gravity && iteration >= initial {
        sum_to_n(initial)
    } else {
        sum_to_n(initial) - sum_to_n(initial - iteration)
    }
}

fn parse_input(input: Vec<String>) -> Vec<i32> {
    let re = Regex::new(r"-?\d+").unwrap();
    re.find_iter(&input[0])
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .collect()
}

fn p1(input: Vec<String>) -> String {
    let v = parse_input(input);
    let (y_min, y_max) = (v[2], v[3]);

    let mut heights: Vec<i32> = Vec::new();
    for v in 0..-y_min {
        for y in 0..500 {
            let pos_y = pos_after_n_iterations(v, y, true);
            if pos_y >= y_min && pos_y <= y_max {
                heights.push(sum_to_n(v));
            }
        }
    }
    format!("{}", heights.iter().max().unwrap())
}

fn p2(input: Vec<String>) -> String {
    let v = parse_input(input);
    let (x_min, x_max, y_min, y_max) = (v[0], v[1], v[2], v[3]);

    let mut all: HashSet<(i32, i32)> = HashSet::new();
    for y in y_min..-y_min {
        for i in 0..500 {
            let y_pos = pos_after_n_iterations(y, i, true);
            if y_pos >= y_min && y_pos <= y_max {
                for x in 0..x_max + 1 {
                    let x_pos = pos_after_n_iterations(x, i, false);
                    if x_pos >= x_min && x_pos <= x_max {
                        all.insert((x, y));
                    }
                }
            }
        }
    }
    format!("{}", all.len())
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_17.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_17.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    const TEST_DATA: &[&str] = &["target area: x=20..30, y=-10..-5"];

    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_DATA.iter().map(|s| s.to_string()).collect()), "45");
    }
    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_DATA.iter().map(|s| s.to_string()).collect()), "112");
    }
}
