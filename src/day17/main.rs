use std::collections::HashSet;

use aoc2021lib::read_lines_to_strings;
use regex::Regex;

fn sum_to_n(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn p1_p2(input: Vec<String>) -> String {
    let re = Regex::new(r"-?\d+").unwrap();
    let v: Vec<_> = re
        .find_iter(&input[0])
        .map(|v| v.as_str().parse::<i32>().unwrap())
        .collect();
    let (x_min, x_max, y_min, y_max) = (v[0], v[1], v[2], v[3]);

    let mut all: HashSet<(i32, i32)> = HashSet::new();
    let mut heights: Vec<i32> = Vec::new();
    for v in y_min..-y_min {
        let height = std::cmp::max(0, sum_to_n(v));

        for y in 0..200 {
            let p = height - sum_to_n(y);

            if p >= y_min && p <= y_max {
                let iterations = v + y + 1;

                for x in 0..x_max + 1 {
                    if iterations > x {
                        let x_pos = sum_to_n(x);
                        if x_pos >= x_min && x_pos <= x_max {
                            heights.push(height);
                            all.insert((x, v));
                        }
                    } else {
                        let x_pos = sum_to_n(x) - sum_to_n(x - iterations);
                        if x_pos >= x_min && x_pos <= x_max {
                            heights.push(height);
                            all.insert((x, v));
                        }
                    }
                }
            }
        }
    }
    println!("P1 - {:?}", heights.iter().max().unwrap());
    format!("{}", all.len())
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_17.txt") {
        println!("P1 - {}", p1_p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1_p2;
    // use crate::p2;
    const TEST_DATA: &[&str] = &["target area: x=20..30, y=-10..-5"];

    #[test]
    fn test_p1() {
        assert_eq!(
            p1_p2(TEST_DATA.iter().map(|s| s.to_string()).collect()),
            "112"
        );
    }
}
