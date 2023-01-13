use std::collections::HashMap;

pub fn part_a(contents: &str) -> String {
    let fuel_requirements = contents.lines().collect::<Vec<&str>>();

    let base: i64 = 5;
    let snafu_system: HashMap<char, i64> =
        HashMap::from([('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)]);

    let total_fuel_requirements = fuel_requirements
        .iter()
        .map(|snafu_number| to_decimal(snafu_number, base, &snafu_system))
        .sum::<i64>();

    return to_snafu(total_fuel_requirements, base, &snafu_system);
}

fn to_decimal(snafu_number: &str, base: i64, snafu_system: &HashMap<char, i64>) -> i64 {
    return snafu_number
        .chars()
        .enumerate()
        .fold(0, |acc, (place, snafu_digit)| {
            acc + (snafu_system.get(&snafu_digit).unwrap()
                * (base.pow(u32::try_from(snafu_number.len() - 1 - place).unwrap())))
        });
}

fn to_snafu(decimal_number: i64, base: i64, snafu_system: &HashMap<char, i64>) -> String {
    let snafu_system_reverse_lookup = snafu_system
        .iter()
        .map(|(k, v)| (*v, *k))
        .collect::<HashMap<i64, char>>();

    let mut snafu_number: Vec<char> = Vec::new();
    let mut quotient = Some(decimal_number);
    while let Some(part) = quotient {
        let mut q = part;
        let r = q % base;
        match r {
            0..=2 => {
                snafu_number.push(*snafu_system_reverse_lookup.get(&r).unwrap());
            }
            _ => {
                snafu_number.push(*snafu_system_reverse_lookup.get(&(r - base)).unwrap());
                q += r;
            }
        }

        quotient = Some(q / base);
        if q < base {
            quotient = None;
        }
    }

    snafu_number.reverse();
    return String::from_iter(snafu_number);
}
