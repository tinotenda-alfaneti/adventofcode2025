use aoc_common as common;
use day_four as lib;

fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let (part1, part2) = lib::solve(lines);
    println!("Accessible: {}", part1);
    println!("Total Removed: {}", part2);
}
