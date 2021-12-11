use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use rusttype::Point;
use rusttype::Vector;

type V = Vector<i32>;

pub static NEIGHBOURS: [V; 8] = [
    V { x: -1, y: -1 },
    V { x: 1, y: -1 },
    V { x: -1, y: 1 },
    V { x: 1, y: 1 },
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];

pub static DIRECTIONS: [V; 4] = [
    V { x: 0, y: -1 },
    V { x: 1, y: 0 },
    V { x: 0, y: 1 },
    V { x: -1, y: 0 },
];

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_lines_to_strings<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file)
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect())
}

pub fn words_by_line<'a>(s: &'a Vec<String>) -> impl Iterator<Item = Vec<&'a str>> {
    s.iter().map(|line| line.split_whitespace().collect())
}

pub fn get_map(input: Vec<String>) -> HashMap<Point<i32>, i32> {
    let mut map: HashMap<Point<i32>, i32> = HashMap::new();
    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.insert(
                Point {
                    x: x as i32,
                    y: y as i32,
                },
                c.to_digit(10).unwrap() as i32,
            );
        }
    }
    map
}
