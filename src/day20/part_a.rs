pub fn part_a(contents: &str) -> i32 {
    let encrypted_file = contents
        .lines()
        .map(|n| n.parse::<i32>().unwrap())
        .enumerate()
        .map(|(i, n)| (i32::try_from(i).unwrap(), n))
        .collect::<Vec<(i32, i32)>>();

    let mut encrypted_file_shadow = encrypted_file.clone();
    let buffer_len = i32::try_from(encrypted_file_shadow.len()).unwrap();

    for (pos, value) in encrypted_file.iter() {
        let target_pos = encrypted_file_shadow
            .iter()
            .position(|&m| m.0 == *pos && m.1 == *value)
            .unwrap();

        encrypted_file_shadow.remove(target_pos);

        // modulo previous next, start at 0
        let new_pos = if *value > 0 {
            (i32::try_from(target_pos).unwrap() + *value - 1).rem_euclid(buffer_len - 1) + 1
        } else if *value < 0 {
            (i32::try_from(target_pos).unwrap() - value.abs() - 1).rem_euclid(buffer_len - 1) + 1
        } else {
            i32::try_from(target_pos).unwrap()
        };

        encrypted_file_shadow.insert(usize::try_from(new_pos).unwrap(), (*pos, *value));
    }

    let zero_pos = i32::try_from(
        encrypted_file_shadow
            .iter()
            .position(|&m| m.1 == 0)
            .unwrap(),
    )
    .unwrap();

    return encrypted_file_shadow
        [usize::try_from((1000 + zero_pos).rem_euclid(buffer_len)).unwrap()]
    .1 + encrypted_file_shadow
        [usize::try_from((2000 + zero_pos).rem_euclid(buffer_len)).unwrap()]
    .1 + encrypted_file_shadow
        [usize::try_from((3000 + zero_pos).rem_euclid(buffer_len)).unwrap()]
    .1;
}
