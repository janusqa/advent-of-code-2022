use std::collections::{HashMap, HashSet, VecDeque};

struct Side<'a> {
    s1: &'a (i32, i32),
    s2: &'a (i32, i32),
    s3: &'a (i32, i32),
}

pub fn part_a(contents: &str) -> i32 {
    let compass: [(i32, i32); 8] = [
        (-1, -1), // nw
        (-1, 0),  // n
        (-1, 1),  // ne
        (0, 1),   // e
        (1, 1),   // se
        (1, 0),   // s
        (1, -1),  // sw
        (0, -1),  // w
    ];

    let north_side = Side {
        s1: &compass[0],
        s2: &compass[1],
        s3: &compass[2],
    };

    let south_side = Side {
        s1: &compass[6],
        s2: &compass[5],
        s3: &compass[4],
    };

    let west_side = Side {
        s1: &compass[0],
        s2: &compass[7],
        s3: &compass[6],
    };

    let east_side = Side {
        s1: &compass[2],
        s2: &compass[3],
        s3: &compass[4],
    };

    let mut sides: VecDeque<&Side> =
        VecDeque::from([&north_side, &south_side, &west_side, &east_side]);

    let mut elves: HashSet<(i32, i32)> = HashSet::new();

    for line in contents.lines().enumerate() {
        for pos in line.1.chars().enumerate() {
            if pos.1 == '#' {
                elves.insert((
                    i32::try_from(line.0).unwrap(),
                    i32::try_from(pos.0).unwrap(),
                ));
            }
        }
    }

    let mut round = 0;
    loop {
        // first half of round
        let mut proposals: HashMap<(i32, i32), Vec<&(i32, i32)>> = HashMap::new();
        let elves_copy = elves.clone();
        for elf in elves_copy.iter() {
            if should_move(elf, &elves, &compass) {
                let proposed_move = propose(elf, &elves, &sides);
                if proposals.contains_key(&proposed_move) {
                    proposals.get_mut(&proposed_move).unwrap().push(elf);
                } else {
                    proposals.entry(proposed_move).or_insert(vec![elf]);
                }
            }
        }

        // second half of round
        if proposals.len() > 0 {
            for proposal in proposals.iter() {
                if proposal.1.len() == 1 {
                    elves.remove(proposal.1[0]);
                    elves.insert(*proposal.0);
                }
            }
        } else {
            break;
        }

        let side = sides.pop_front().unwrap();
        sides.push_back(side);

        round += 1;
        if round > 9 {
            break;
        }
    }

    // calculate bounds of rectangle
    let rectangle = elves.iter().fold(
        ((i32::MAX, i32::MIN), (i32::MAX, i32::MIN)),
        |mut acc, &(r, c)| {
            acc.0 .0 = acc.0 .0.min(r);
            acc.0 .1 = acc.0 .1.max(r);
            acc.1 .0 = acc.1 .0.min(c);
            acc.1 .1 = acc.1 .1.max(c);
            acc
        },
    );

    return ((rectangle.0 .1 - rectangle.0 .0 + 1) * (rectangle.1 .1 - rectangle.1 .0 + 1))
        - i32::try_from(elves.len()).unwrap();
}

fn should_move(elf: &(i32, i32), elves: &HashSet<(i32, i32)>, compass: &[(i32, i32); 8]) -> bool {
    let mut can_move = false;

    for direction in compass {
        if elves.contains(&(elf.0 + direction.0, elf.1 + direction.1)) {
            can_move = true;
            break;
        }
    }

    return can_move;
}

fn propose(elf: &(i32, i32), elves: &HashSet<(i32, i32)>, sides: &VecDeque<&Side>) -> (i32, i32) {
    for i in 0..sides.len() {
        if !elves.contains(&(elf.0 + sides[i].s1.0, elf.1 + sides[i].s1.1))
            && !elves.contains(&(elf.0 + sides[i].s2.0, elf.1 + sides[i].s2.1))
            && !elves.contains(&(elf.0 + sides[i].s3.0, elf.1 + sides[i].s3.1))
        {
            return (elf.0 + sides[i].s2.0, elf.1 + sides[i].s2.1);
        }
    }

    return *elf;
}
