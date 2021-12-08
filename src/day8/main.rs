use std::collections::{HashMap, HashSet};

use aoc2021lib::read_lines_to_strings;
use itertools::Itertools;

fn p1(input: Vec<String>) -> String {
    let mut right_words: Vec<&str> = Vec::new();
    for line in &input {
        let (_left, right) = line.split_once(" | ").unwrap();
        right_words.extend(right.split(" "));
    }

    let counts = (0..8)
        .map(|f| {
            right_words
                .iter()
                .filter(|w| f == w.chars().count())
                .count()
        })
        .collect::<Vec<usize>>();

    format!("{}", counts[2] + counts[4] + counts[3] + counts[7])
}

fn p2(input: Vec<String>) -> String {
    let mut sum: i32 = 0;

    for line in &input {
        let (left, right) = line.split_once(" | ").unwrap();

        let mut all: Vec<&str> = left.split(" ").collect::<Vec<&str>>();
        all.extend(right.split(" "));

        let words = all
            .iter()
            .map(|w| w.chars().collect::<HashSet<char>>())
            .collect_vec();

        let one = words.iter().filter(|w| w.len() == 2).collect_vec()[0].clone();
        let seven = words.iter().filter(|w| w.len() == 3).collect_vec()[0].clone();
        let four = words.iter().filter(|w| w.len() == 4).collect_vec()[0].clone();
        let eight = words.iter().filter(|w| w.len() == 7).collect_vec()[0].clone();

        let two = words
            .iter()
            .filter(|w| w.len() == 5)
            .map(|w| (w, &(w - &seven) - &four))
            .filter(|(_, l)| l.len() == 2)
            .collect_vec()[0]
            .0
            .clone();

        let five = words
            .iter()
            .filter(|w| w.len() == 5)
            .map(|w| (w, &(w - &two) - &one))
            .filter(|(_, l)| l.len() == 1)
            .collect_vec()[0]
            .0
            .clone();

        let three = words
            .iter()
            .filter(|w| w.len() == 5 && (*w != &two && *w != &five))
            .next()
            .unwrap()
            .clone();

        let zero = words
            .iter()
            .filter(|w| w.len() == 6)
            .map(|w| (w, &five - w))
            .filter(|(_, l)| l.len() == 1)
            .collect_vec()[0]
            .0
            .clone();

        let nine = words
            .iter()
            .filter(|w| w.len() == 6)
            .map(|w| (w, (w - &four)))
            .filter(|(_, l)| l.len() == 2)
            .collect_vec()[0]
            .0
            .clone();

        let six = words
            .iter()
            .filter(|w| w.len() == 6 && (*w != &nine && *w != &zero))
            .next()
            .unwrap()
            .clone();

        let mut decipher: HashMap<String, i32> = HashMap::new();
        decipher.insert(zero.iter().sorted().join(""), 0);
        decipher.insert(one.iter().sorted().join(""), 1);
        decipher.insert(two.iter().sorted().join(""), 2);
        decipher.insert(three.iter().sorted().join(""), 3);
        decipher.insert(four.iter().sorted().join(""), 4);
        decipher.insert(five.iter().sorted().join(""), 5);
        decipher.insert(six.iter().sorted().join(""), 6);
        decipher.insert(seven.iter().sorted().join(""), 7);
        decipher.insert(eight.iter().sorted().join(""), 8);
        decipher.insert(nine.iter().sorted().join(""), 9);

        let numbers = right
            .split(" ")
            .map(|v| v.chars().sorted().join(""))
            .collect_vec();

        let val = decipher.get(&numbers[0]).unwrap() * 1000
            + decipher.get(&numbers[1]).unwrap() * 100
            + decipher.get(&numbers[2]).unwrap() * 10
            + decipher.get(&numbers[3]).unwrap();
        sum += val;
    }
    format!("{}", sum)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_08.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_08.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use aoc2021lib::read_lines_to_strings;
    use itertools::Itertools;

    use crate::p1;
    use crate::p2;

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(read_lines_to_strings("./data/test_day_08.txt").unwrap()),
            "26"
        );
    }
    #[test]
    fn test_p2_1() {
        let test_data = vec![
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf",
        ];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "5353"
        );
    }
    #[test]
    fn test_p2_2() {
        assert_eq!(
            p2(read_lines_to_strings("./data/test_day_08.txt").unwrap()),
            "61229"
        );
    }
    #[test]
    fn test_p2_3() {
        let test_data = vec![
            "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb",
        ];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "9361"
        );
    }
}
