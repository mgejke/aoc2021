use itertools::Itertools;
use std::vec;

use aoc2021lib::read_lines_to_strings;

fn to_vec_vec(v: Vec<String>) -> Vec<Vec<i32>> {
    v.iter()
        .map(|s| {
            s.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect_vec()
        })
        .collect_vec()
}

fn get_sum(v: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut sums: Vec<i32> = vec![0; v[0].len()];

    for i in 0..sums.len() {
        for row in v {
            sums[i] += row[i] as i32;
        }
    }
    sums
}

fn bin_vec_to_dec(vec: &Vec<i32>) -> i32 {
    let (dec, _) = vec
        .iter()
        .rev()
        .fold((0, 1), |(val, pow), &c| (val + pow * c as i32, pow * 2));
    dec
}

fn reduce_from_sum(vec: &Vec<i32>, f: &dyn Fn(&i32) -> i32) -> Vec<i32> {
    vec.iter().map(f).collect_vec()
}

fn calculate_life_support_rating(vecvec: &Vec<Vec<i32>>, magic: i32) -> i32 {
    let mut vec = vecvec.clone();

    let mut i = 0;
    while vec.len() > 1 {
        let veclen = vec.len() as i32;
        let sums = get_sum(&vec);

        let pattern = reduce_from_sum(&sums, &|s| {
            (magic - (*s >= (veclen - s)) as i32).abs() as i32
        });

        vec = vec
            .into_iter()
            .filter(|val| val[i] == pattern[i] as i32)
            .collect_vec();
        i += 1;
    }
    bin_vec_to_dec(&vec[0])
}

fn p1(input: Vec<String>) -> String {
    let total = input.len() as i32;
    let b = get_sum(&to_vec_vec(input));
    let c = reduce_from_sum(&b, &|s| (*s >= (total - s)) as i32);

    let gamma = bin_vec_to_dec(&c);
    let epsilon = bin_vec_to_dec(&c.iter().map(|v| 1 - v).collect_vec());
    format!("{}", gamma * epsilon)
}

fn p2(input: Vec<String>) -> String {
    let vecvec = to_vec_vec(input);
    let oxygen = calculate_life_support_rating(&vecvec, 0);
    let c02 = calculate_life_support_rating(&vecvec, 1);
    format!("{}", oxygen * c02)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_03.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_03.txt") {
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
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(
            p1(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "198"
        );
    }
    #[test]
    fn test_p2() {
        let test_data = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "230"
        );
    }
}
