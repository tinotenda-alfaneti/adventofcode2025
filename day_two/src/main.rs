use aoc_common as common;
use day_two as lib;

fn main() {
    
    let file_path = "input.txt";
    let lines = match common::read_file_to_string(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return;
        }
    };

    let part1 = lib::sum_invalid_ids(&lines, 1, false);
    let part2 = lib::sum_invalid_ids(&lines, 2, false);

    println!("Part One Result: {}", part1);
    println!("Part Two Result: {}", part2);
}

