use std::num::ParseIntError;
use std::str::FromStr;

use aoc2021lib::read_lines_to_strings;

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Direction, Self::Err> {
        let (direction, value) = s.split_once(" ").unwrap();
        match direction {
            "forward" => Ok(Direction::Forward(value.parse()?)),
            "up" => Ok(Direction::Up(value.parse()?)),
            "down" => Ok(Direction::Down(value.parse()?)),
            _ => panic!("OJ - {}", direction),
        }
    }
}

fn p1(input: Vec<String>) -> String {
    let commands = input.iter().map(|f| f.parse::<Direction>().unwrap());

    let (h_pos, depth) = commands.fold((0, 0), |(h_pos, depth), command| match command {
        Direction::Forward(v) => (h_pos + v, depth),
        Direction::Up(v) => (h_pos, depth - v),
        Direction::Down(v) => (h_pos, depth + v),
    });
    format!("{}", h_pos * depth)
}

fn p2(input: Vec<String>) -> String {
    let commands = input.iter().map(|f| f.parse::<Direction>().unwrap());

    let (h_pos, depth, _) =
        commands.fold((0, 0, 0), |(h_pos, depth, aim), command| match command {
            Direction::Forward(v) => (h_pos + v, depth + aim * v, aim),
            Direction::Up(v) => (h_pos, depth, aim - v),
            Direction::Down(v) => (h_pos, depth, aim + v),
        });
    format!("{}", h_pos * depth)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_02.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_02.txt") {
        println!("P2 - {}", p2(lines));
    }
}
