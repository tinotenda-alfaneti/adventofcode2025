use aoc_common as common;
use day_one as lib;

fn main() {
    let file_path = "input.txt";

    let input = match common::read_file_to_string(file_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let part1_password = lib::solve_part_one(&input);
    let part2_password = lib::solve_part_two(&input);

    println!("Part 1 Password: {}", part1_password);
    println!("Part 2 Password: {}", part2_password);
}