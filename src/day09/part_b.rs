use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub fn part_b(contents: &str) -> usize {
    let mut start: (usize, usize) = (0, 0);
    let mut pen: (usize, usize) = (0, 0);
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

    let mut rope: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    //  let mut grid: Vec<Vec<usize>> = vec![vec![10; max_col + 1]; max_row + 1];

    for _ in 0..10 {
        rope.push((start.0, start.1));
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

        for _ in 0..steps {
            match direction {
                "R" => {
                    rope[0].1 += 1;
                }
                "L" => {
                    rope[0].1 -= 1;
                }
                "U" => {
                    rope[0].0 -= 1;
                }
                "D" => {
                    rope[0].0 += 1;
                }
                _ => panic!("{}", "Invalid instruction encountered"),
            };

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let tail = &mut rope[i];

                let row_dis = head.0 as i32 - tail.0 as i32;
                let col_dis = head.1 as i32 - tail.1 as i32;

                // Nothing to do. Head and tail are adjacent
                if row_dis.abs() <= 1 && col_dis.abs() <= 1 {
                    break;
                }

                // Directly above or below.
                if row_dis.abs() > 1 && head.1 == tail.1 {
                    tail.0 = (tail.0 as i32 + row_dis.signum()) as usize;
                    continue;
                }

                // Directly left or right
                if col_dis.abs() > 1 && head.0 == tail.0 {
                    tail.1 = (tail.1 as i32 + col_dis.signum()) as usize;
                    continue;
                }

                // Diagonal
                if row_dis.abs() != 0 && col_dis.abs() != 0 {
                    tail.0 = (tail.0 as i32 + row_dis.signum()) as usize;
                    tail.1 = (tail.1 as i32 + col_dis.signum()) as usize;
                    continue;
                }
            }

            visited.insert(*rope.last().unwrap());

            // for row in &mut grid {
            //     for col in row {
            //         *col = 10;
            //     }
            // }

            // for i in 0..rope.len() {
            //     grid[rope[i].0][rope[i].1] = i;
            // }

            // for row in &grid {
            //     for col in row {
            //         if col == &10 {
            //             print!(".");
            //         } else {
            //             print!("{}", col);
            //         }
            //     }
            //     println!();
            // }
            // println!();
            // println!();
        }
    }

    return visited.len();
}
