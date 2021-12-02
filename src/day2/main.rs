use aoc2021lib::read_lines_to_strings;
use aoc2021lib::words_by_line;

fn p1(input: Vec<String>) -> String {
    let mut h_pos = 0;
    let mut depth = 0;

    let commands =
        words_by_line(&input).map(|line| (line[0].clone(), line[1].parse::<i32>().unwrap()));

    for command in commands {
        match command.0.as_ref() {
            "forward" => h_pos += command.1,
            "up" => depth -= command.1,
            "down" => depth += command.1,
            _ => println!("error"),
        }
    }
    format!("{}", h_pos * &depth)
}

fn p2(input: Vec<String>) -> String {
    let mut depth = 0;
    let mut h_pos = 0;
    let mut aim = 0;

    let commands =
        words_by_line(&input).map(|line| (line[0].clone(), line[1].parse::<i32>().unwrap()));

    for command in commands {
        match command.0.as_ref() {
            "forward" => {
                h_pos += command.1;
                depth += aim * command.1;
            }
            "up" => aim -= command.1,
            "down" => aim += command.1,
            _ => println!("error"),
        }
    }
    format!("{}", h_pos * &depth)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_02.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_02.txt") {
        println!("P2 - {}", p2(lines));
    }
}
