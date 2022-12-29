use std::collections::HashSet;

pub fn part_a(contents: &str) -> i32 {
    let directions: [(i32, i32, i32); 6] = [
        (0, -1, 0),
        (1, 0, 0),
        (0, 1, 0),
        (-1, 0, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let droplet: HashSet<(i32, i32, i32)> = HashSet::from_iter(
        contents
            .split("\n")
            .map(|line| {
                let position = line
                    .split(",")
                    .map(|index| index.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>();
                return (position[0], position[1], position[2]);
            })
            .collect::<Vec<(i32, i32, i32)>>(),
    );

    let mut uncovered_surfaces = 0;
    for surface in droplet.iter() {
        for direction in directions.iter() {
            if !droplet.contains(&(
                surface.0 + direction.0,
                surface.1 + direction.1,
                surface.2 + direction.2,
            )) {
                uncovered_surfaces += 1;
            }
        }
    }

    return uncovered_surfaces;
}
