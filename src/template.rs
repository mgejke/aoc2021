use aoc2021lib::read_lines_to_strings;

fn p1(input: Vec<String>) -> String {
    format!("{}", 0)
}

fn p2(input: Vec<String>) -> String {
    format!("{}", 0)
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_X.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_X.txt") {
        println!("P2 - {}", p2(lines));
    }
}
