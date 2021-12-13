use std::{
    collections::HashSet,
    fs,
    io::{self, Write},
};

use rusttype::Point;

fn p1(input: String, fold_count: usize, p2: bool) -> String {
    let (points, folds) = input.split_once("\n\n").unwrap();

    let points = points
        .lines()
        .map(|f| f.split_once(",").unwrap())
        .map(|(p1, p2)| Point {
            x: p1.parse::<i32>().unwrap(),
            y: p2.parse::<i32>().unwrap(),
        });
    let mut paper: HashSet<Point<i32>> = HashSet::from_iter(points);

    let folds = folds
        .lines()
        .map(|s| {
            let (k, v) = s.split_once("=").unwrap();
            let v = v.parse::<i32>().unwrap();
            if k.chars().last().unwrap() == 'x' {
                Point { x: v, y: 0 }
            } else {
                Point { x: 0, y: v }
            }
        })
        .collect::<Vec<_>>();

    for (i, fold) in folds.iter().enumerate() {
        if i == fold_count {
            break;
        }
        if fold.x > 0 {
            paper = fold_x(fold.x, paper);
        } else {
            paper = fold_y(fold.y, paper);
        }
    }

    let maxx = paper.iter().map(|p| p.x).max().unwrap();
    let maxy = paper.iter().map(|p| p.y).max().unwrap();
    if p2 {
        for y in 0..maxy + 1 {
            for x in 0..maxx + 1 {
                if paper.contains(&Point { x, y }) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
        let _ = io::stdout().flush();
    }

    format!("{}", paper.len())
}

fn fold_x(foldx: i32, paper: HashSet<Point<i32>>) -> HashSet<Point<i32>> {
    paper
        .iter()
        .map(|p| Point {
            x: foldx - i32::abs(p.x - foldx),
            y: p.y,
        })
        .collect()
}

fn fold_y(foldy: i32, paper: HashSet<Point<i32>>) -> HashSet<Point<i32>> {
    paper
        .iter()
        .map(|p| Point {
            x: p.x,
            y: foldy - i32::abs(p.y - foldy),
        })
        .collect()
}

fn main() {
    if let Ok(lines) = fs::read_to_string("./data/input_day_13.txt") {
        println!("P1 - {}", p1(lines, 1, false));
    }

    if let Ok(lines) = fs::read_to_string("./data/input_day_13.txt") {
        println!("P2 - {}", p1(lines, usize::MAX, true));
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::p1;

    #[test]
    fn test_p1() {
        if let Ok(text) = fs::read_to_string("./data/test_day_13.txt") {
            assert_eq!(p1(text, 1, false), "17");
        }
    }
}
