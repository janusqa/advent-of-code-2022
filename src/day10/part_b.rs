use lazy_static::lazy_static;
use regex::Regex;

pub fn part_b(contents: &str) -> i32 {
    lazy_static! {
        static ref RE_INSTRUCTIONS: Regex = Regex::new(r"([a-z]+) ?(-?\d{1,})?").unwrap();
    }

    let mut instructions = contents.lines();
    let mut crt = [' '; 240];
    let mut cycle: usize = 1;
    let mut operator = "";
    let mut instruction = None;
    let mut sprite: [i32; 3] = [0, 1, 2];
    let mut register_x: i32 = 1;
    let mut addx_cycle_counter = 1;

    loop {
        // Get a new instuction each cycle unless its a addx that spans 2 cycles
        if (((addx_cycle_counter - 1) % 2) + 1 == 1 && operator == "addx")
            || (operator == "noop")
            || (cycle == 1)
        {
            instruction = instructions.next();
        }

        if instruction == None {
            break;
        }

        let cmd = RE_INSTRUCTIONS.captures(instruction.unwrap()).unwrap();
        operator = cmd.get(1).unwrap().as_str();

        // draw a pixel. If any part of sprite is at position being drawn
        // draw a light pixel otherwise draw a dark pixel
        let mut pixel = ' ';
        if sprite.contains(&(((cycle - 1) as i32) % 40)) {
            pixel = '#';
        } else {
            pixel = '.';
        }
        crt[cycle - 1] = pixel;

        // println!(
        //     "instruction: {}, cycle: {}, sprite: {:?}, addxcyc: {}, paint: {},  pixel: {}",
        //     instruction.unwrap(),
        //     cycle,
        //     sprite,
        //     addx_cycle_counter,
        //     cycle - 1,
        //     pixel
        // );
        // println!("crt: {:?}", &crt,);

        // 1. complete addx operation after 2nd cycle

        if operator == "addx" && ((addx_cycle_counter - 1) % 2) + 1 == 2 {
            register_x += cmd.get(2).unwrap().as_str().parse::<i32>().unwrap();

            // 2. move sprite at end of 2nd cycle of addx operation

            sprite[1] = register_x;
            sprite[0] = sprite[1] - 1;
            sprite[2] = sprite[1] + 1;
        }

        if operator == "addx" {
            addx_cycle_counter += 1;
        }

        cycle += 1
    }

    display(&crt);

    return 0;
}

fn display(crt: &[char; 240]) -> () {
    for pixel in crt.iter().enumerate() {
        print!("{}", pixel.1);
        if (pixel.0 + 1) % 40 == 0 {
            print!("\n");
        }
    }
}
