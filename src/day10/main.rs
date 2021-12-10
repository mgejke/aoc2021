use std::collections::{HashMap, HashSet};

use aoc2021lib::read_lines_to_strings;

fn p1(input: Vec<String>) -> String {
    let matching = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let opening = HashSet::from(['(', '[', '{', '<']);
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    let mut sum = 0;
    for line in input {
        let mut stack: Vec<char> = Vec::new();
        for c in line.chars() {
            if opening.contains(&c) {
                stack.push(c);
            } else {
                let opening = stack.pop().unwrap();
                if *matching.get(&c).unwrap() != opening {
                    sum += scores.get(&c).unwrap();
                }
            }
        }
    }
    format!("{}", sum)
}

fn p2(input: Vec<String>) -> String {
    let scores = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);

    let mut sums: Vec<i64> = Vec::new();
    for line in input {
        if let Some(stack) = get_incomplete(line) {
            let sum: i64 = stack
                .iter()
                .rev()
                .fold(0, |sum, c| sum * 5 + scores.get(&c).unwrap());
            sums.push(sum);
        }
    }
    sums.sort();
    format!("{}", sums[sums.len() / 2])
}

fn get_incomplete(line: String) -> Option<Vec<char>> {
    let matching = HashMap::from([('}', '{'), (')', '('), (']', '['), ('>', '<')]);
    let opening = HashSet::from(['(', '[', '{', '<']);

    let mut stack: Vec<char> = Vec::new();
    for c in line.chars() {
        if opening.contains(&c) {
            stack.push(c);
        } else {
            let opening = stack.pop().unwrap();
            if *matching.get(&c).unwrap() != opening {
                return None;
            }
        }
    }
    Some(stack)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_10.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_10.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    const TEST_DATA: &[&str] = &[
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(TEST_DATA.iter().map(|s| s.to_string()).collect()),
            "26397"
        );
    }

    #[test]
    fn test_p2() {
        assert_eq!(
            p2(TEST_DATA.iter().map(|s| s.to_string()).collect()),
            "288957"
        );
    }
}
