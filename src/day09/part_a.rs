use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn part_a(contents: &str) -> usize {
    let mut start: (usize, usize) = (0, 0);
    let mut pen: (usize, usize) = (0, 0);
    let mut head: (usize, usize) = (0, 0);
    let mut tail: (usize, usize) = (0, 0);
    let mut max_row: usize = 0;
    let mut max_col: usize = 0;

    lazy_static! {
        static ref RE_INSTRUCTIONS: Regex = Regex::new(r"([A-Z]) (\d{1,})").unwrap();
    }

    for instruction in contents.lines() {
        let direction = RE_INSTRUCTIONS
            .captures(instruction)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let steps = RE_INSTRUCTIONS
            .captures(instruction)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        match direction {
            "R" => {
                if steps > max_col - pen.1 {
                    max_col = pen.1 + steps;
                }
                pen.1 += steps;
            }
            "L" => {
                if steps > pen.1 {
                    max_col += steps - pen.1;
                    start.1 += steps - pen.1;
                    pen.1 = 0;
                } else {
                    pen.1 -= steps;
                }
            }
            "U" => {
                if steps > pen.0 {
                    max_row += steps - pen.0;
                    start.0 += steps - pen.0;
                    pen.0 = 0;
                } else {
                    pen.0 -= steps;
                }
            }
            "D" => {
                if steps > max_row - pen.0 {
                    max_row = pen.0 + steps
                }
                pen.0 += steps;
            }
            _ => panic!("{}", "Invalid instruction encountered"),
        };
    }

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    head.0 = start.0;
    head.1 = start.1;
    tail.0 = head.0;
    tail.1 = head.1;

    for instruction in contents.lines() {
        let direction = RE_INSTRUCTIONS
            .captures(instruction)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str();

        let steps = RE_INSTRUCTIONS
            .captures(instruction)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        for _ in 0..steps {
            match direction {
                "R" => {
                    head.1 += 1;
                }
                "L" => {
                    head.1 -= 1;
                }
                "U" => {
                    head.0 -= 1;
                }
                "D" => {
                    head.0 += 1;
                }
                _ => panic!("{}", "Invalid instruction encountered"),
            };

            let row_dis = head.0 as i32 - tail.0 as i32;
            let col_dis = head.1 as i32 - tail.1 as i32;

            // Nothing to do. Head and tail are adjacent
            if row_dis.abs() <= 1 && col_dis.abs() <= 1 {
                continue;
            }

            if row_dis.abs() > 1 && head.1 == tail.1 {
                // Directly above or below.
                tail.0 = (tail.0 as i32 + row_dis.signum()) as usize;
            } else if col_dis.abs() > 1 && head.0 == tail.0 {
                // Directly left or right
                tail.1 = (tail.1 as i32 + col_dis.signum()) as usize;
            } else if row_dis.abs() != 0 && col_dis.abs() != 0 {
                // Diagonal
                tail.0 = (tail.0 as i32 + row_dis.signum()) as usize;
                tail.1 = (tail.1 as i32 + col_dis.signum()) as usize;
            }
            visited.insert(tail);
        }
    }

    return visited.len();
}
