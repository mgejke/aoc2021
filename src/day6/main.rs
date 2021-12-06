use std::collections::VecDeque;

use aoc2021lib::read_lines_to_strings;
use itertools::Itertools;

fn brute_force(mut fish: Vec<i32>, iterations: i32) -> usize {
    for _ in 0..iterations {
        fish = fish.iter().map(|v| *v - 1).collect_vec();
        let (zeroes, mut others): (Vec<i32>, Vec<i32>) = fish.iter().partition(|v| **v < 0);
        others.extend(vec![8; zeroes.len()]);
        others.extend(vec![6; zeroes.len()]);
        fish = others;
    }
    fish.len()
}

fn p1(input: Vec<String>) -> String {
    let fish = input[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect_vec();

    format!("{}", brute_force(fish, 80))
}

fn p2(input: Vec<String>) -> String {
    let fish = input[0]
        .split(",")
        .map(|v| v.parse::<i32>().unwrap())
        .collect_vec();

    format!("{}", count_fish_smart(fish))
}

fn count_fish_smart(fish: Vec<i32>) -> usize {
    let mut counts = (0..9)
        .map(|v| fish.iter().filter(|f| **f == v).count())
        .collect::<VecDeque<usize>>();
    for _ in 0..256 {
        let zeroes = counts.pop_front().unwrap();
        counts[6] += zeroes;
        counts.push_back(zeroes);
    }
    counts.iter().sum::<usize>()
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_06.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_06.txt") {
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
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(
            p1(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "5934"
        );
    }
    #[test]
    fn test_p2() {
        let test_data = vec!["3,4,3,1,2"];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "26984457539"
        );
    }
}
