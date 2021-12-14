use std::{collections::HashMap, fs};

fn p1(input: String, iterations: i32) -> String {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .trim_end()
        .split("\n")
        .map(|s| {
            let (f, t) = s.split_once(" -> ").unwrap();
            let to = format!("{}{}", f.chars().nth(0).unwrap(), t);
            (f, to)
        })
        .collect::<HashMap<&str, String>>();

    let mut template = template.to_owned();
    for _ in 0..iterations {
        let mut new: String = String::new();
        for (p1, p2) in template.chars().zip(template[1..].chars()) {
            let combo = format!("{}{}", p1, p2);
            new.push_str(rules.get(&combo.as_str()).unwrap());
        }
        new.push(template.chars().last().unwrap());
        template = new;
    }

    let letter_counts: HashMap<u8, i32> =
        template
            .as_bytes()
            .iter()
            .fold(HashMap::new(), |mut map, c| {
                *map.entry(*c).or_insert(0) += 1;
                map
            });

    let (max, min) = letter_counts
        .iter()
        .fold((0, i32::MAX), |(max, min), (_, v)| {
            (std::cmp::max(max, *v), std::cmp::min(min, *v))
        });

    format!("{}", max - min)
}

fn p2(input: String, iterations: i32) -> String {
    let (template, rules) = input.split_once("\n\n").unwrap();

    let rules = rules
        .trim_end()
        .split("\n")
        .map(|s| {
            let (f, t) = s.split_once(" -> ").unwrap();
            let chars = f.as_bytes();
            ((chars[0], chars[1]), t.as_bytes()[0])
        })
        .collect::<HashMap<(u8, u8), u8>>();

    let bytes = template.as_bytes();
    let mut counts: HashMap<(u8, u8), u64> = bytes.windows(2).fold(HashMap::new(), |mut map, p| {
        *map.entry((p[0], p[1])).or_insert(0) += 1;
        map
    });

    for _ in 0..iterations {
        let new_counts: HashMap<(u8, u8), u64> =
            counts.iter().fold(HashMap::new(), |mut map, (k, v)| {
                let t = rules.get(&k).unwrap();
                *map.entry((k.0, *t)).or_insert(0) += v;
                *map.entry((*t, k.1)).or_insert(0) += v;
                map
            });
        counts = new_counts;
    }

    let mut letter_counts: HashMap<u8, u64> =
        counts
            .into_iter()
            .fold(HashMap::new(), |mut map, ((c1, c2), count)| {
                *map.entry(c1).or_insert(0) += count;
                *map.entry(c2).or_insert(0) += count;

                map
            });

    *letter_counts.entry(*bytes.first().unwrap()).or_insert(0) += 1;
    *letter_counts.entry(*bytes.last().unwrap()).or_insert(0) += 1;

    let (max, min) = letter_counts
        .iter()
        .fold((0, u64::MAX), |(max, min), (_, v)| {
            (std::cmp::max(max, *v / 2), std::cmp::min(min, *v / 2))
        });

    format!("{:?}", max - min)
}

fn main() {
    if let Ok(lines) = fs::read_to_string("./data/input_day_14.txt") {
        println!("P1 - {}", p1(lines, 10));
    }

    if let Ok(lines) = fs::read_to_string("./data/input_day_14.txt") {
        println!("P2 - {}", p2(lines, 40));
    }
}

#[cfg(test)]
mod tests {
    use crate::{p1, p2};
    use std::fs;

    #[test]
    fn test_p1() {
        assert_eq!(
            p1(fs::read_to_string("./data/test_day_14.txt").unwrap(), 10),
            "1588"
        );
    }

    #[test]
    fn test_p2_1() {
        assert_eq!(
            p2(fs::read_to_string("./data/test_day_14.txt").unwrap(), 10),
            "1588"
        );
    }

    #[test]
    fn test_p2_2() {
        assert_eq!(
            p2(fs::read_to_string("./data/test_day_14.txt").unwrap(), 40),
            "2188189693529"
        );
    }

    #[test]
    fn test_p2_3() {
        assert_eq!(
            p1(fs::read_to_string("./data/input_day_14.txt").unwrap(), 10),
            p2(fs::read_to_string("./data/input_day_14.txt").unwrap(), 10)
        );
    }
}
