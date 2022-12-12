pub fn part_b(contents: &str) -> usize {
    let patch = contents
        .lines()
        .map(|line| {
            return line
                .chars()
                .map(|tree| tree.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
        })
        .collect::<Vec<_>>();

    // let max_row = patch.len() - 1;
    // let max_col = patch[0].len() - 1;

    // the edge trees are already visible so we can already count them
    let mut scenic_score = 0;

    for row in patch.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            // look left
            let scenic_left = match row.1[..col.0].iter().rev().position(|x| x >= col.1) {
                None => row.1[..col.0].len(),
                Some(a) => a + 1,
            };

            //look right
            let scenic_right = match row.1[col.0 + 1..].iter().position(|x| x >= col.1) {
                None => row.1[col.0 + 1..].len(),
                Some(a) => a + 1,
            };

            // look up
            let scenic_up = match &patch[..row.0]
                .iter()
                .map(|x| x[col.0])
                .rev()
                .position(|y| &y >= col.1)
            {
                None => patch[..row.0].len(),
                Some(a) => a + 1,
            };

            // look down
            let scenic_down = match &patch[row.0 + 1..]
                .iter()
                .map(|x| x[col.0])
                .position(|y| &y >= col.1)
            {
                None => patch[row.0 + 1..].len(),
                Some(a) => a + 1,
            };
            scenic_score = std::cmp::max(
                scenic_score,
                scenic_left * scenic_right * scenic_up * scenic_down,
            );
        }
    }

    return scenic_score;
}
