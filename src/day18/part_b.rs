use std::collections::{HashMap, HashSet, VecDeque};

pub fn part_b(contents: &str) -> i32 {
    let directions: [(i32, i32, i32); 6] = [
        (0, -1, 0),
        (1, 0, 0),
        (0, 1, 0),
        (-1, 0, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut memo: HashMap<(i32, i32, i32), i32> = HashMap::new();
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

    let minmax_xyz = droplet.iter().fold(
        ((i32::MAX, i32::MAX, i32::MAX), (0, 0, 0)),
        |acc, &(x, y, z)| {
            (
                (acc.0 .0.min(x), acc.0 .1.min(y), acc.0 .2.min(z)),
                (acc.1 .0.max(x), acc.1 .1.max(y), acc.1 .2.max(z)),
            )
        },
    );

    let mut uncovered_surfaces = 0;
    for &(x, y, z) in droplet.iter() {
        for &(dx, dy, dz) in directions.iter() {
            uncovered_surfaces += bfs(
                &(x + dx, y + dy, z + dz),
                &minmax_xyz,
                &directions,
                &droplet,
                &mut memo,
            );
        }
    }

    return uncovered_surfaces;
}

fn bfs(
    src: &(i32, i32, i32),
    minmax_xyz: &((i32, i32, i32), (i32, i32, i32)),
    directions: &[(i32, i32, i32); 6],
    droplet: &HashSet<(i32, i32, i32)>,
    memo: &mut HashMap<(i32, i32, i32), i32>,
) -> i32 {
    let mut explore = VecDeque::from([*src]);
    let mut visited: HashSet<(i32, i32, i32)> = HashSet::from([*src]);

    if memo.contains_key(&src) {
        return *memo.get(&src).unwrap();
    }

    while explore.len() > 0 {
        let (x, y, z) = explore.pop_front().unwrap();

        if x > minmax_xyz.1 .0
            || y > minmax_xyz.1 .1
            || z > minmax_xyz.1 .2
            || x < minmax_xyz.0 .0
            || y < minmax_xyz.0 .1
            || z < minmax_xyz.0 .2
        {
            memo.entry((x, y, z)).or_insert(1);
            return 1;
        }

        if droplet.contains(&(x, y, z)) {
            continue;
        }

        for neighbour in directions
            .iter()
            .map(|&(dx, dy, dz)| (x + dx, y + dy, z + dz))
        {
            if !visited.contains(&neighbour) {
                explore.push_back(neighbour);
                visited.insert(neighbour);
            }
        }
    }

    memo.entry(*src).or_insert(0);
    return 0;
}
