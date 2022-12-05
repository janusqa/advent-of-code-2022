pub fn part_b(contents: &str) -> i32 {
    let mut cleanup_plan = contents
        .lines()
        .map(|range_pair| {
            range_pair
                .split(",")
                .map(|range| {
                    let r = range
                        .split("-")
                        .map(|boundary| boundary.parse::<i32>().unwrap())
                        .collect::<Vec<i32>>();
                    return (r[0], r[1]);
                })
                .collect::<Vec<(i32, i32)>>()
        })
        .fold(Vec::new(), |mut acc, item| {
            acc.push(item[0]);
            acc.push(item[1]);
            acc
        });

    let mut overlapping_pairs = 0;
    while cleanup_plan.len() > 0 {
        let assignment = cleanup_plan.drain(0..2).collect::<Vec<(i32, i32)>>();
        if !((assignment[0].1 < assignment[1].0) || (assignment[1].1 < assignment[0].0)) {
            overlapping_pairs += 1
        }
    }

    return overlapping_pairs;
}
