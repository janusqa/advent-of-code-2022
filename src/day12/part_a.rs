extern crate queues;

use queues::*;
use std::collections::HashSet;
use std::convert::TryFrom;

pub fn part_a(contents: &str) -> i32 {
    let mut start: (usize, usize, usize) = (0, 0, 0);
    let mut end: (usize, usize, usize) = (0, 0, 0);
    let mut visited: HashSet<String> = HashSet::new();
    let mut explore: Queue<(usize, usize, usize)> = queue![];
    let children: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    let mut height_map = contents
        .lines()
        .map(|line| line.chars().map(|c| c as usize).collect::<Vec<usize>>())
        .collect::<Vec<_>>();

    for r in 0..height_map.len() {
        for c in 0..height_map[0].len() {
            if height_map[r][c] == 83 {
                start.0 = r;
                start.1 = c;
                start.2 = 0;
                height_map[r][c] = 97;
            } else if height_map[r][c] == 69 {
                end.0 = r;
                end.1 = c;
                end.2 = 0;
                height_map[r][c] = 122;
            }
        }
    }

    explore.add(start).unwrap();
    visited.insert(format!("{},{}", start.0, start.1));

    while explore.size() > 0 {
        let current = explore.remove().unwrap();

        if current.0 == end.0 && current.1 == end.1 {
            return i32::try_from(current.2).unwrap();
        }

        for direction in children {
            let r = isize::try_from(current.0).unwrap() + direction.0;
            let c = isize::try_from(current.1).unwrap() + direction.1;

            let row_inbounds = if 0 <= r && r < isize::try_from(height_map.len()).unwrap() {
                true
            } else {
                false
            };

            let col_inbounds = if 0 <= c && c < isize::try_from(height_map[0].len()).unwrap() {
                true
            } else {
                false
            };

            if !row_inbounds || !col_inbounds {
                continue;
            }

            if i32::try_from(height_map[usize::try_from(r).unwrap()][usize::try_from(c).unwrap()])
                .unwrap()
                - i32::try_from(height_map[current.0][current.1]).unwrap()
                >= 2
            {
                continue;
            }

            if visited.contains(&format!("{},{}", r, c)) {
                continue;
            }

            explore
                .add((
                    usize::try_from(r).unwrap(),
                    usize::try_from(c).unwrap(),
                    current.2 + 1,
                ))
                .unwrap();

            visited.insert(format!("{},{}", r, c));
        }
    }

    return -1;
}
