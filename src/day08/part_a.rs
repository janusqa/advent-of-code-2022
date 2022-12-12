pub fn part_a(contents: &str) -> i32 {
    let patch = contents
        .lines()
        .map(|line| {
            return line
                .chars()
                .map(|tree| tree.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
        })
        .collect::<Vec<_>>();

    let max_row = patch.len() - 1;
    let max_col = patch[0].len() - 1;

    // the edge trees are already visible so we can already count them
    let mut num_visible_trees = ((2 * ((max_row + 1) - 2)) + (2 * ((max_col + 1) - 2))) + 4;

    for row in patch.iter().enumerate() {
        if row.0 > 0 && row.0 < max_row {
            for col in row.1.iter().enumerate() {
                if col.0 > 0 && col.0 < max_col {
                    // look left
                    if !row.1[..col.0].iter().any(|x| x >= col.1) {
                        num_visible_trees += 1;
                        continue;
                    }

                    //look right
                    if !row.1[col.0 + 1..].iter().any(|x| x >= col.1) {
                        num_visible_trees += 1;
                        continue;
                    }

                    // look up
                    if !(&patch[..row.0].iter().map(|x| x[col.0]).any(|x| &x >= col.1)) {
                        num_visible_trees += 1;
                        continue;
                    }

                    // look down
                    if !(&patch[row.0 + 1..]
                        .iter()
                        .map(|x| x[col.0])
                        .any(|x| &x >= col.1))
                    {
                        num_visible_trees += 1;
                        continue;
                    }
                }
            }
        }
    }

    return num_visible_trees as i32;
}

// ***FAILED ATTEMPTS BELOW
// ***
// use std::collections::HashMap;

// pub fn part_a(contents: &str) -> i32 {
//     let mut patch = contents
//         .lines()
//         .map(|line| {
//             line.chars()
//                 .map(|tree| (tree.to_digit(10).unwrap() as i32, [-1, -1, -1, -1]))
//                 .collect::<Vec<(i32, [i32; 4])>>()
//         })
//         .collect::<Vec<_>>();

//     let mut memo: HashMap<String, i32> = HashMap::new();
//     let max_row = patch.len() - 1;
//     let max_col = patch[0].len() - 1;
//     // let tree_pos = (((0 + (max_row - 0)) / 2) + 1, (0 + (max_col - 0)) / 2);
//     let tree_pos = (0, 0);
//     let mut direction = 0;

//     map_visible_trees(
//         &mut patch,
//         tree_pos,
//         &mut memo,
//         &mut direction,
//         max_row,
//         max_col,
//     );

//     println!("{:?}", patch);
//     println!("size: {}, {}", max_row + 1, max_col + 1);

//     let mut num_visible_trees = 0;
//     for row in patch.iter().enumerate() {
//         for col in row.1.iter().enumerate() {
//             if row.0 == 0 || row.0 == max_row || col.0 == 0 || col.0 == max_col {
//                 num_visible_trees += 1;
//             } else if col.1 .0 > *col.1 .1.iter().min().unwrap() {
//                 num_visible_trees += 1;
//             }
//         }
//     }

//     return num_visible_trees;
// }

// *** Total failure again! It looks like it should work but no bueno. Unto brute forcing.
// fn map_visible_trees(
//     patch: &mut Vec<Vec<(i32, [i32; 4])>>,
//     tree_pos: (usize, usize),
//     memo: &mut HashMap<String, i32>,
//     direction: &mut usize,
//     max_row: usize,
//     max_col: usize,
// ) {
//     // css ordering. Top, right, bottom, left
//     let children: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
//     let mut stack = Vec::new();
//     stack.push((tree_pos, *direction));

//     while !stack.is_empty() {
//         let (tree_pos, direction) = stack.pop().unwrap();
//         // println!("pop: {:?} -> {}", tree_pos, direction);

//         // check to see if visited
//         if memo.contains_key(&format!("{}-{}-{}", tree_pos.0, tree_pos.1, direction)) {
//             // println!("seen: {:?} -> {}", tree_pos, direction);
//             continue;
//         } else {
//             memo.entry(format!("{}-{}-{}", tree_pos.0, tree_pos.1, direction))
//                 .or_insert(0);
//         }

//         // if tree is on an edge set it to visible
//         if tree_pos.0 == 0 || tree_pos.0 == max_row || tree_pos.1 == 0 || tree_pos.1 == max_col {
//             if tree_pos.0 == 0 {
//                 patch[tree_pos.0][tree_pos.1].1[0] = patch[tree_pos.0][tree_pos.1].0;
//             }
//             if tree_pos.1 == max_col {
//                 patch[tree_pos.0][tree_pos.1].1[1] = patch[tree_pos.0][tree_pos.1].0;
//             }
//             if tree_pos.0 == max_row {
//                 patch[tree_pos.0][tree_pos.1].1[2] = patch[tree_pos.0][tree_pos.1].0;
//             }
//             if tree_pos.1 == 0 {
//                 patch[tree_pos.0][tree_pos.1].1[3] = patch[tree_pos.0][tree_pos.1].0;
//             }
//             continue;
//         }

//         let mut dir = 0;
//         for child in children {
//             let r = (tree_pos.0 as isize + child.0) as usize;
//             let c = (tree_pos.1 as isize + child.1) as usize;
//             if (max_row >= r) && (max_col >= c) {
//                 stack.push(((r, c), dir));
//                 // println!("push: {:?} -> {}", (r, c), dir);
//                 patch[tree_pos.0][tree_pos.1].1[dir] =
//                     std::cmp::max(patch[r][c].0, patch[r][c].1[dir]);
//             }
//             dir += 1;
//         }
//     }
// }

// *** recursive - total failure - the stack overflowed
// fn map_visible_trees_recursion(
//     patch: &mut Vec<Vec<(i32, [i32; 4])>>,
//     tree_pos: (usize, usize),
//     memo: &mut HashMap<String, i32>,
//     dir: usize,
//     max_row: usize,
//     max_col: usize
// ) {
//     // check to see if visited
//     if memo.contains_key(&format!("{}-{}-{}", tree_pos.0, tree_pos.1, dir)) {
//         return;
//     } else {
//         memo.entry(format!("{}-{}-{}", tree_pos.0, tree_pos.1, dir))
//             .or_insert(0);
//     }

//     // short circuit if you see a 9 which is the tallest tree
//     if patch[tree_pos.0][tree_pos.1].0 == 9 {
//         patch[tree_pos.0][tree_pos.1].1[dir] = 9;
//         return;
//     }

//     // if tree is on an edge set it to visible
//     if tree_pos.0 == 0
//         || tree_pos.0 == (max_row)
//         || tree_pos.1 == 0
//         || tree_pos.1 == max_col
//     {
//         if tree_pos.0 == 0 {
//             patch[tree_pos.0][tree_pos.1].1[0] = patch[tree_pos.0][tree_pos.1].0;
//         }
//         if tree_pos.1 == max_col {
//             patch[tree_pos.0][tree_pos.1].1[1] = patch[tree_pos.0][tree_pos.1].0;
//         }
//         if tree_pos.0 == (max_row) {
//             patch[tree_pos.0][tree_pos.1].1[2] = patch[tree_pos.0][tree_pos.1].0;
//         }
//         if tree_pos.1 == 0 {
//             patch[tree_pos.0][tree_pos.1].1[3] = patch[tree_pos.0][tree_pos.1].0;
//         }

//         return;
//     }

//     // css ordering. Top, right, bottom, left
//     let neighbours: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

//     let mut dir = 0;
//     for neighbour in neighbours {
//         let r = (tree_pos.0 as isize + neighbour.0) as usize;
//         let c = (tree_pos.1 as isize + neighbour.1) as usize;
//         if (last_row >= r) && (last_col >= c) {
//             map_visible_trees_recursion(patch, (r, c), memo, dir, max_row, max_col);
//             patch[tree_pos.0][tree_pos.1].1[dir] = std::cmp::max(
//                 patch[r][c].0,
//                 patch[r][c].1[dir],
//             );
//         }
//         dir += 1;
//     }
// }
