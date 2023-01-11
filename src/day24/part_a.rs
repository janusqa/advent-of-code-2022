use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct ValleyState {
    pos: (usize, usize),
    mins: usize,
    blizzards: usize,
}

impl ValleyState {
    fn new(pos: (usize, usize), mins: usize, blizzards: usize) -> Self {
        return ValleyState {
            pos,
            mins,
            blizzards,
        };
    }

    fn neighbours(
        &self,
        directions: &[(i32, i32); 5],
        walls: &HashSet<(usize, usize)>,
        blizzard_states: &HashMap<usize, HashSet<(usize, usize)>>,
        valley_area: &(usize, usize, usize, usize),
    ) -> Vec<(ValleyState, usize)> {
        let r = self.pos.0;
        let c = self.pos.1;
        let mins = self.mins;
        let blizzards = self.blizzards;

        return directions
            .iter()
            .map(|&(dr, dc)| {
                (
                    i32::try_from(r).unwrap() + dr,
                    i32::try_from(c).unwrap() + dc,
                )
            })
            .filter(|&(r, c)| {
                i32::try_from(valley_area.0).unwrap() <= r
                    && r < i32::try_from(valley_area.1 + 1).unwrap()
                    && i32::try_from(valley_area.2).unwrap() <= c
                    && c < i32::try_from(valley_area.3 + 1).unwrap()
            })
            .map(|(r, c)| (usize::try_from(r).unwrap(), usize::try_from(c).unwrap()))
            .filter(|f| {
                let next_blizzards = (((blizzards + 1) - 1) + blizzard_states.len() + 1)
                    .rem_euclid(blizzard_states.len());
                clear_ground(f, walls, blizzard_states.get(&next_blizzards).unwrap())
            })
            .map(|s| (Self::new(s, mins + 1, blizzards + 1), 1))
            .collect::<Vec<(ValleyState, usize)>>();
    }
}

pub fn part_a(contents: &str) -> i32 {
    let mut walls: HashSet<(usize, usize)> = HashSet::new();
    let mut blizzards: HashMap<(usize, usize, char), usize> = HashMap::new();
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1), (0, 0)];

    for line in contents.lines().enumerate() {
        for tile in line.1.chars().enumerate() {
            match tile.1 {
                '#' => {
                    walls.insert((line.0, tile.0));
                }
                '.' => {
                    ();
                }
                _ => {
                    blizzards.entry((line.0, tile.0, tile.1)).or_insert(1);
                }
            }
        }
    }

    // calculate the bounds of the valley
    let valley_area = walls.iter().fold(
        (usize::MAX, usize::MIN, usize::MAX, usize::MIN),
        |acc, &(r, c)| (acc.0.min(r), acc.1.max(r), acc.2.min(c), acc.3.max(c)),
    );

    // Precompute all blizzard states. They will repeat after a while
    // Store them in a hash table with the key being the minute they occur.
    // So we can access a blizzard state at any min by moduoloing the current time and
    // accessing that time in the blizzard states.
    let blizzard_states = get_blizzard_states(&mut blizzards, &valley_area);

    // calculate start and end
    let start = (valley_area.0, valley_area.2 + 1);
    let end = (valley_area.1, valley_area.3 - 1);

    // find least amount of mins to go from start to end using a* search
    let (_, mins) = astar(
        &ValleyState::new(start, 0, 0),
        |p| p.neighbours(&directions, &walls, &blizzard_states, &valley_area),
        |p| manhattan(&(p.pos.0, p.pos.1), &end),
        |p| p.pos == end,
    )
    .unwrap();

    return i32::try_from(mins).unwrap();
}

fn get_blizzard_states(
    blizzards: &mut HashMap<(usize, usize, char), usize>,
    valley_area: &(usize, usize, usize, usize),
) -> HashMap<usize, HashSet<(usize, usize)>> {
    let mut bs: HashMap<usize, HashMap<(usize, usize, char), usize>> = HashMap::new();
    let mut bv: Vec<HashMap<(usize, usize, char), usize>> = Vec::new();
    bs.entry(0).or_insert(blizzards.clone());
    bv.push(blizzards.clone());

    blow(blizzards, &valley_area);
    let mut i = 1;
    while !bv.contains(&blizzards) {
        bs.entry(i).or_insert(blizzards.clone());
        bv.push(blizzards.clone());
        i += 1;
        blow(blizzards, &valley_area);
    }

    return bs
        .iter()
        .map(|(k, v)| {
            (
                *k,
                v.keys()
                    .map(|k| (k.0, k.1))
                    .collect::<HashSet<(usize, usize)>>(),
            )
        })
        .collect::<HashMap<_, _>>();
    // return bs;
}

fn clear_ground(
    pos: &(usize, usize),
    walls: &HashSet<(usize, usize)>,
    blizzards: &HashSet<(usize, usize)>,
) -> bool {
    return !walls.contains(&pos) && !blizzards.contains(&pos);
}

fn blow(
    blizzards: &mut HashMap<(usize, usize, char), usize>,
    valley_area: &(usize, usize, usize, usize),
) {
    let r_wrap_at = (valley_area.1 - valley_area.0 + 1) - 2;
    let c_wrap_at = (valley_area.3 - valley_area.2 + 1) - 2;

    for (k, _) in blizzards.clone().iter() {
        match k.2 {
            '>' => {
                let c = ((k.1 - 1) + c_wrap_at + 1).rem_euclid(c_wrap_at) + 1;
                if blizzards.contains_key(&(k.0, c, k.2)) {
                    blizzards.insert((k.0, c, k.2), blizzards.get(&(k.0, c, k.2)).unwrap() + 1);
                } else {
                    blizzards.insert((k.0, c, k.2), 1);
                }
            }
            '<' => {
                let c = ((k.1 - 1) + c_wrap_at - 1).rem_euclid(c_wrap_at) + 1;
                if blizzards.contains_key(&(k.0, c, k.2)) {
                    blizzards.insert((k.0, c, k.2), blizzards.get(&(k.0, c, k.2)).unwrap() + 1);
                } else {
                    blizzards.insert((k.0, c, k.2), 1);
                }
            }
            '^' => {
                let r = ((k.0 - 1) + r_wrap_at - 1).rem_euclid(r_wrap_at) + 1;
                if blizzards.contains_key(&(r, k.1, k.2)) {
                    blizzards.insert((r, k.1, k.2), blizzards.get(&(r, k.1, k.2)).unwrap() + 1);
                } else {
                    blizzards.insert((r, k.1, k.2), 1);
                }
            }
            'v' => {
                let r = ((k.0 - 1) + r_wrap_at + 1).rem_euclid(r_wrap_at) + 1;
                if blizzards.contains_key(&(r, k.1, k.2)) {
                    blizzards.insert((r, k.1, k.2), blizzards.get(&(r, k.1, k.2)).unwrap() + 1);
                } else {
                    blizzards.insert((r, k.1, k.2), 1);
                }
            }
            _ => panic!("This is not a blizzard"),
        }
        if blizzards.contains_key(&(k.0, k.1, k.2)) {
            if blizzards.get(&(k.0, k.1, k.2)).unwrap() > &1 {
                blizzards.insert(
                    (k.0, k.1, k.2),
                    blizzards.get(&(k.0, k.1, k.2)).unwrap() - 1,
                );
            } else {
                blizzards.remove(&(k.0, k.1, k.2));
            }
        }
    }
}

fn manhattan(a: &(usize, usize), b: &(usize, usize)) -> usize {
    return a.0.abs_diff(b.0) + a.1.abs_diff(b.1);
}

fn _display(
    pos: &(usize, usize),
    walls: &HashSet<(usize, usize)>,
    blizzards: &HashMap<(usize, usize, char), usize>,
    valley_area: &(usize, usize, usize, usize),
) {
    let mut screen = vec![vec![String::from("."); valley_area.3 + 1]; valley_area.1 + 1];
    for wall in walls.iter() {
        screen[wall.0][wall.1] = String::from("#");
    }
    for (k, _) in blizzards.iter() {
        if screen[k.0][k.1] == "." {
            screen[k.0][k.1] = k.2.to_string();
        } else if screen[k.0][k.1] == ">"
            || screen[k.0][k.1] == "<"
            || screen[k.0][k.1] == "^"
            || screen[k.0][k.1] == "v"
        {
            screen[k.0][k.1] = String::from("2");
        } else {
            screen[k.0][k.1] = (screen[k.0][k.1].parse::<usize>().unwrap() + 1).to_string();
        }
    }
    screen[pos.0][pos.1] = String::from("E");

    for row in screen.iter().enumerate() {
        for col in row.1.iter().enumerate() {
            print!("{}", col.1);
        }
        println!();
    }
    println!()
}
