use aoc_common as common;

pub fn part_one(lines: &[String]) -> i32 {
    let mut dial_start = 50;
    let mut password = 0;

    for line in lines {
        if let Ok((letter, number)) = common::split_alpha_num(line) {
            dial_start = match letter.as_str() {
                "L" => (dial_start - number).rem_euclid(100),
                "R" => (dial_start + number).rem_euclid(100),
                _ => dial_start,
            };

            if dial_start == 0 {
                password += 1;
            }
        } else {
            eprintln!("Error parsing line: {}", line);
        }
    }

    password
}

pub fn part_two(lines: &[String]) -> i32 {
    let dial_size = 100;
    let mut dial_start = 50;
    let mut password = 0;

    for line in lines {
        if let Ok((letter, steps)) = common::split_alpha_num(line) {
            let direction = letter.chars().next().unwrap_or('R');
            let clockwise = direction == 'R';
            let abs_steps = steps.abs();

            let steps_to_first_zero = if dial_start == 0 {
                dial_size
            } else if clockwise {
                dial_size - dial_start
            } else {
                dial_start
            };

            let zeros = if abs_steps >= steps_to_first_zero {
                1 + (abs_steps - steps_to_first_zero) / dial_size
            } else {
                0
            };

            password += zeros;

            dial_start = if clockwise {
                (dial_start + steps).rem_euclid(dial_size)
            } else {
                (dial_start - steps).rem_euclid(dial_size)
            };
        } else {
            eprintln!("Error parsing line: {}", line);
        }
    }

    password
}
