use std::collections::VecDeque;

use aoc2021lib::read_lines_to_strings;
use num_traits::FromPrimitive;

#[derive(PartialEq, Eq, Debug, num_derive::FromPrimitive)]
enum TypeId {
    Sum = 0,
    Product = 1,
    Min = 2,
    Max = 3,
    Literal = 4,
    Greater = 5,
    Less = 6,
    Equal = 7,
}

fn to_binary_str(hex: &String) -> VecDeque<char> {
    let binary_str = hex
        .chars()
        .flat_map(|c| {
            format!("{:04b}", c.to_digit(16).unwrap())
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<VecDeque<_>>();
    binary_str
}

fn parse_packet(binary: &mut VecDeque<char>, version_sum: &mut i64) -> Option<i64> {
    let packet_version: i64 = take_n_to_value(binary, 3);
    *version_sum += packet_version;

    let type_id: TypeId = FromPrimitive::from_i32(take_n_to_value(binary, 3)).unwrap();
    match type_id {
        TypeId::Sum => Some(parse_operator(binary, version_sum).iter().sum()),
        TypeId::Product => Some(parse_operator(binary, version_sum).iter().product()),
        TypeId::Min => Some(*parse_operator(binary, version_sum).iter().min().unwrap()),
        TypeId::Max => Some(*parse_operator(binary, version_sum).iter().max().unwrap()),
        TypeId::Literal => Some(parse_literal(binary)),
        TypeId::Greater => {
            let v = parse_operator(binary, version_sum);
            Some((v[0] > v[1]) as i64)
        }
        TypeId::Less => {
            let v = parse_operator(binary, version_sum);
            Some((v[0] < v[1]) as i64)
        }
        TypeId::Equal => {
            let v = parse_operator(binary, version_sum);
            Some((v[0] == v[1]) as i64)
        }
    }
}

fn parse_operator(binary: &mut VecDeque<char>, version_sum: &mut i64) -> Vec<i64> {
    let mode = binary.pop_front();

    let mut values: Vec<i64> = Vec::new();
    match mode {
        Some('0') => {
            let length: usize = take_n_to_value(binary, 15);
            let before = binary.len();
            loop {
                if let Some(v) = parse_packet(binary, version_sum) {
                    values.push(v);
                }
                if length == before - binary.len() {
                    break;
                }
            }
        }
        Some('1') => {
            let packets = take_n_to_value(binary, 11);
            for _ in 0..packets {
                if let Some(v) = parse_packet(binary, version_sum) {
                    values.push(v);
                }
            }
        }
        Some(_) => {
            panic!("Unknown mode");
        }
        None => {
            panic!("no operator?");
        }
    }
    values
}

fn parse_literal(binary: &mut VecDeque<char>) -> i64 {
    let mut value = String::new();
    loop {
        let b = binary.pop_front().unwrap();
        value.push_str(&binary.drain(0..4).collect::<String>());
        if b == '0' {
            break;
        }
    }
    i64::from_str_radix(&value, 2).unwrap()
}

fn take_n_to_value<T>(binary: &mut VecDeque<char>, n: usize) -> T
where
    T: num_traits::Num,
    <T as num_traits::Num>::FromStrRadixErr: std::fmt::Debug,
{
    T::from_str_radix(&binary.drain(0..n).collect::<String>(), 2).unwrap()
}

fn p1(input: Vec<String>) -> String {
    let hex = &input[0];
    let mut binary = to_binary_str(hex);
    let mut version_sum: i64 = 0;
    parse_packet(&mut binary, &mut version_sum);
    format!("{}", version_sum)
}

fn p2(input: Vec<String>) -> String {
    let hex = &input[0];
    let mut binary = to_binary_str(hex);
    let mut version_sum: i64 = 0;

    if let Some(v) = parse_packet(&mut binary, &mut version_sum) {
        return format!("{}", v);
    }

    "-1".to_string()
}

fn main() {
    if let Ok(lines) = read_lines_to_strings("./data/input_day_16.txt") {
        println!("P1 - {}", p1(lines));
    }

    if let Ok(lines) = read_lines_to_strings("./data/input_day_16.txt") {
        println!("P2 - {}", p2(lines));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    const TEST_DATA1: &[&str] = &["D2FE28"];
    const TEST_DATA2: &[&str] = &["38006F45291200"];
    const TEST_DATA3: &[&str] = &["EE00D40C823060"];
    const TEST_DATA4: &[&str] = &["8A004A801A8002F478"];
    const TEST_DATA5: &[&str] = &["620080001611562C8802118E34"];
    const TEST_DATA6: &[&str] = &["C0015000016115A2E0802F182340"];
    const TEST_DATA7: &[&str] = &["A0016C880162017C3686B18A3D4780"];

    const TEST_DATA8: &[&str] = &["C200B40A82"];
    const TEST_DATA9: &[&str] = &["04005AC33890"];

    #[test]
    fn test_p1_1() {
        assert_eq!(p1(TEST_DATA1.iter().map(|s| s.to_string()).collect()), "6");
    }

    #[test]
    fn test_p1_2() {
        assert_eq!(p1(TEST_DATA2.iter().map(|s| s.to_string()).collect()), "9");
    }

    #[test]
    fn test_p1_3() {
        assert_eq!(p1(TEST_DATA3.iter().map(|s| s.to_string()).collect()), "14");
    }

    #[test]
    fn test_p1_4() {
        assert_eq!(p1(TEST_DATA4.iter().map(|s| s.to_string()).collect()), "16");
    }
    #[test]
    fn test_p1_5() {
        assert_eq!(p1(TEST_DATA5.iter().map(|s| s.to_string()).collect()), "12");
    }
    #[test]
    fn test_p1_6() {
        assert_eq!(p1(TEST_DATA6.iter().map(|s| s.to_string()).collect()), "23");
    }
    #[test]
    fn test_p1_7() {
        assert_eq!(p1(TEST_DATA7.iter().map(|s| s.to_string()).collect()), "31");
    }

    #[test]
    fn test_p2_1() {
        assert_eq!(p2(TEST_DATA8.iter().map(|s| s.to_string()).collect()), "3");
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(p2(TEST_DATA9.iter().map(|s| s.to_string()).collect()), "54");
    }
}
