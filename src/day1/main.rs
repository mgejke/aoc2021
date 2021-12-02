use aoc2021lib::read_lines_to_strings;
use itertools::Itertools;

fn counter<I>(numbers: I) -> usize
where
    I: Iterator<Item = i32>,
{
    numbers.tuple_windows().filter(|(a, b)| b > a).count()
}

fn p1(input: Vec<String>) -> String {
    let a = input.iter().map(|s| s.parse::<i32>().unwrap());
    format!("{}", counter(a))
}

fn p2(input: Vec<String>) -> String {
    let numbers: Vec<i32> = input.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let windowed = numbers.windows(3).map(|window| window.iter().sum());
    format!("{}", counter(windowed))
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_01.txt") {
        println!("P1 - {}", p1(lines))
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_01.txt") {
        println!("P2 - {}", p2(lines))
    }
}
