use std::collections::HashSet;
use std::convert::TryFrom;

pub fn part_a(contents: &str) -> i32 {
    // parse map
    let cave_map = contents
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|coord| {
                    coord
                        .split(",")
                        .map(|part| part.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let direction = [(0, 1), (-1, 1), (1, 1)];

    // initilize start
    let sand_source: Vec<usize> = vec![500, 0];

    // calculate max and min boundaries of cave
    let mut min_col = usize::MAX;
    let mut max_col = 0;
    let mut min_row = usize::MAX;
    let mut max_row = 0;
    min_col = std::cmp::min(min_col, sand_source[0]);
    max_col = std::cmp::max(max_col, sand_source[0]);
    min_row = std::cmp::min(min_row, sand_source[1]);
    max_row = std::cmp::max(max_row, sand_source[1]);
    for path in &cave_map {
        for point in path {
            min_col = std::cmp::min(min_col, point[0]);
            max_col = std::cmp::max(max_col, point[0]);
            min_row = std::cmp::min(min_row, point[1]);
            max_row = std::cmp::max(max_row, point[1]);
        }
    }

    // plot cave from cave_map
    let mut cave: HashSet<(usize, usize, &str)> = HashSet::new();
    for path in &cave_map {
        let segment = path.windows(2);
        for part in segment {
            let minc = std::cmp::min(part[0][0], part[1][0]);
            let maxc = std::cmp::max(part[0][0], part[1][0]);
            for col in minc..=maxc {
                let minr = std::cmp::min(part[0][1], part[1][1]);
                let maxr = std::cmp::max(part[0][1], part[1][1]);
                for row in minr..=maxr {
                    cave.insert((col, row, "#"));
                }
            }
        }
    }

    let mut particles_at_rest = 0;
    let mut at_rest = false;
    let mut search_pos = 0;
    let mut sand = sand_source.clone();

    loop {
        if at_rest {
            particles_at_rest += 1;
            cave.insert((sand[0], sand[1], "o"));
            sand = sand_source.clone();
        }

        for (col, row) in direction.iter().clone() {
            let potential_col = usize::try_from(i32::try_from(sand[0]).unwrap() + col).unwrap();
            let potential_row = usize::try_from(i32::try_from(sand[1]).unwrap() + row).unwrap();

            let can_move = !(cave.contains(&(potential_col, potential_row, "#"))
                || cave.contains(&(potential_col, potential_row, "o")));

            if can_move {
                sand[0] = potential_col;
                sand[1] = potential_row;
                break;
            }
            search_pos += 1
        }

        at_rest = search_pos == 3;
        search_pos = 0;

        // println!("{:?} {} {} {}", sand, max_col, max_row, search_pos);
        // println!("{cave:?}");

        if sand[1] >= max_row {
            break;
        }
    }

    return particles_at_rest;
}
