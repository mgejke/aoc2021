use aoc2021lib::read_lines_to_strings;

fn p1(input: Vec<String>) -> String {
    format!("{}", 0)
}

fn p2(_input: Vec<String>) -> String {
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

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    const TEST_DATA: &[&str] = &[
        "DATADATADATADATA",
        "DATADATADATADATA",
        "DATADATADATADATA",
        "DATADATADATADATA",
    ];

    #[test]
    fn test_p1() {
        assert_eq!(p1(TEST_DATA.iter().map(|s| s.to_string()).collect()), "-1");
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2(TEST_DATA.iter().map(|s| s.to_string()).collect()), "-1");
    }
}
