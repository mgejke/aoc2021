use aoc2021lib::read_lines_to_strings;
use itertools::Itertools;

fn calc_cost(subs: Vec<i32>, f: &dyn Fn(&i32) -> i32) -> i32 {
    let mut lowest = i32::MAX;

    for i in *subs.iter().min().unwrap()..*subs.iter().max().unwrap() {
        let cost_vec = subs.iter().map(|v| (*v - i).abs()).collect_vec();
        let cost: i32 = cost_vec.iter().map(f).sum();
        if cost < lowest {
            lowest = cost;
        }
    }
    lowest
}

fn p1(input: Vec<String>) -> String {
    let subs: Vec<i32> = input[0]
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect_vec();

    format!("{:?}", calc_cost(subs, &|v: &i32| *v))
}

fn p2(input: Vec<String>) -> String {
    let subs: Vec<i32> = input[0]
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect_vec();

    format!("{:?}", calc_cost(subs, &|v: &i32| *v * (1 + *v) / 2))
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_07.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_07.txt") {
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
        let test_data = vec!["16,1,2,0,4,2,7,1,2,14"];

        assert_eq!(
            p1(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "37"
        );
    }
    #[test]
    fn test_p2() {
        let test_data = vec!["16,1,2,0,4,2,7,1,2,14"];

        assert_eq!(
            p2(test_data.iter().map(|s| s.to_string()).collect_vec()),
            "168"
        );
    }
}
