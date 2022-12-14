use lazy_static::lazy_static;
use regex::Regex;

pub fn part_a(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_INSTRUCTIONS: Regex = Regex::new(r"([a-z]+) ?(-?\d{1,})?").unwrap();
    }

    let mut register_x: i32 = 1;
    let mut cycle_counter = 0;
    let mut prev_cycle = (0, 0);
    let mut total_signal_strength = 0;

    for instruction in contents.lines() {
        let cmd = RE_INSTRUCTIONS.captures(instruction).unwrap();
        let operator = cmd.get(1).unwrap().as_str();

        prev_cycle.0 = cycle_counter;
        prev_cycle.1 = register_x;

        register_x = match operator {
            "noop" => {
                cycle_counter += 1;
                register_x
            }
            "addx" => {
                cycle_counter += 2;
                register_x += cmd.get(2).unwrap().as_str().parse::<i32>().unwrap();
                register_x
            }
            _ => register_x,
        };

        if (cycle_counter % 20 == 0) && ((cycle_counter / 20) % 2 != 0) {
            total_signal_strength += cycle_counter * prev_cycle.1;
            // println!(
            //     "Instruction: {}, Cycle: {}, X: {}",
            //     instruction, cycle_counter, prev_cycle.1
            // );
        } else if (cycle_counter % 20 == 1)
            && (((cycle_counter - 1) / 20) % 2 != 0)
            && (operator != "noop")
        {
            total_signal_strength += (cycle_counter - 1) * prev_cycle.1;
            // println!(
            //     "Instruction: {}, Cycle: {}, X: {}",
            //     instruction,
            //     cycle_counter - 1,
            //     prev_cycle.1
            // );
        }
    }

    return total_signal_strength;
}
