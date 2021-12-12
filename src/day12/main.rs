use std::collections::{HashMap, HashSet};

use aoc2021lib::read_lines_to_strings;

fn p1(input: Vec<String>, p1: bool) -> String {
    let mut caves: HashMap<String, HashSet<String>> = HashMap::new();

    for line in &input {
        let (a, b) = line.split_once("-").unwrap();

        let a_con = caves.entry(a.to_string()).or_insert_with(HashSet::new);
        a_con.insert(b.to_string());

        let b_con = caves.entry(b.to_string()).or_insert_with(HashSet::new);
        b_con.insert(a.to_string());
    }

    let mut v_count: HashMap<String, i32> = HashMap::new();

    let count;
    if p1 {
        count = find_path(&caves, "start".to_string(), &mut v_count);
    } else {
        count = find_path_deluxe(&caves, "start".to_string(), &mut v_count, false);
    }

    format!("{}", count)
}

fn find_path(
    caves: &HashMap<String, HashSet<String>>,
    current: String,
    v_count: &mut HashMap<String, i32>,
) -> i32 {
    if current == "end" {
        return 1;
    }
    let c = v_count
        .entry(current.to_string())
        .and_modify(|c| *c += 1)
        .or_insert(1);
    if current.to_lowercase() == current && *c > 1 {
        return 0;
    }
    let mut paths: i32 = 0;
    for next in caves.get(&current).unwrap().iter() {
        paths += find_path(caves, next.to_string(), &mut v_count.clone());
    }
    paths
}

fn find_path_deluxe(
    caves: &HashMap<String, HashSet<String>>,
    current: String,
    v_count: &mut HashMap<String, i32>,
    done_2: bool,
) -> i32 {
    if current == "end" {
        return 1;
    }
    let c = v_count
        .entry(current.to_string())
        .and_modify(|c| *c += 1)
        .or_insert(1);

    let lowercase = current.to_lowercase() == current;
    if lowercase && (*c > 2 || *c == 2 && done_2) {
        return 0;
    }

    let done_2 = done_2 || lowercase && *c == 2;
    let mut paths: i32 = 0;
    for next in caves.get(&current).unwrap().iter() {
        if next != "start" {
            paths += find_path_deluxe(caves, next.to_string(), &mut v_count.clone(), done_2);
        }
    }
    paths
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_12.txt") {
        println!("P1 - {}", p1(lines, true));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_12.txt") {
        println!("P2 - {}", p1(lines, false));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    const TEST_DATA: &[&str] = &["start-A", "start-b", "A-c", "A-b", "b-d", "A-end", "b-end"];

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(TEST_DATA.iter().map(|s| s.to_string()).collect(), true),
            "10"
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p1(TEST_DATA.iter().map(|s| s.to_string()).collect(), false),
            "36"
        );
    }
}
