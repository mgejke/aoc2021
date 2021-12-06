use std::fs;

use itertools::Itertools;

struct Board {
    rows: Vec<Option<i32>>,
    bingo: bool,
}

impl Board {
    fn sum_row(&self, start: usize, step: usize, count: usize) -> i32 {
        self.rows[start..std::cmp::min(start + count * step, 25)]
            .iter()
            .step_by(step)
            .map(|b_value| match b_value {
                Some(value) => *value,
                None => 0,
            })
            .sum()
    }

    fn check_bingo(&mut self) -> bool {
        for i in 0..5 {
            if self.sum_row(i * 5, 1, 5) == 0 || self.sum_row(i, 5, 5) == 0 {
                self.bingo = true;
                return true;
            }
        }
        false
    }

    fn table_sum(&self) -> i32 {
        self.sum_row(0, 1, 25)
    }

    fn check_number(&mut self, number: i32) {
        let f = self.rows.iter_mut().find(|b_value| {
            if let Some(value) = b_value {
                *value == number
            } else {
                false
            }
        });
        if let Some(a) = f {
            *a = None;
        }
    }
}

fn p1(input: String) -> String {
    let (numbers, mut boards) = get_numbers_and_boards(input);

    for number in numbers {
        for board in boards.iter_mut() {
            board.check_number(number);
            if board.check_bingo() {
                return format!("{}", board.table_sum() * number);
            }
        }
    }
    format!("{}", 0)
}

fn get_numbers_and_boards(input: String) -> (Vec<i32>, Vec<Board>) {
    let parts = input.split("\n\n").collect_vec();
    let numbers = parts[0]
        .split(",")
        .map(|value| value.parse::<i32>().unwrap())
        .collect_vec();
    let mut boards: Vec<Board> = Vec::new();
    for table in &parts[1..] {
        let board: Board = Board {
            rows: table
                .split_whitespace()
                .map(|value| Some(value.parse::<i32>().unwrap()))
                .collect(),
            bingo: false,
        };
        boards.push(board);
    }
    (numbers, boards)
}

fn p2(input: String) -> String {
    let (numbers, mut boards) = get_numbers_and_boards(input);

    for number in numbers {
        let board_count = boards.len();
        for board in boards.iter_mut() {
            board.check_number(number);
            if board.check_bingo() {
                if board_count == 1 {
                    return format!("{}", board.table_sum() * number);
                }
            }
        }
        boards = boards
            .into_iter()
            .filter(|board| !board.bingo)
            .collect_vec();
    }
    format!("{}", 0)
}

fn main() {
    if let Ok(text) = fs::read_to_string("./data/input_day_04.txt") {
        println!("P1 - {}", p1(text));
    }

    if let Ok(text) = fs::read_to_string("./data/input_day_04.txt") {
        println!("P2 - {}", p2(text));
    }
}

#[cfg(test)]
mod tests {
    use crate::p1;
    use crate::p2;
    use std::fs;

    #[test]

    fn test_p1() {
        if let Ok(text) = fs::read_to_string("./data/test_day_04.txt") {
            assert_eq!(p1(text), "4512");
        }
    }
    #[test]
    fn test_p2() {
        if let Ok(text) = fs::read_to_string("./data/test_day_04.txt") {
            assert_eq!(p2(text), "1924");
        }
    }
}
