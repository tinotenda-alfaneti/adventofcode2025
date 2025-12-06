use aoc_common as common;
use day_six as lib;

fn main() {
    let file_path = "input.txt";
    let mut lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let answer = lib::part_one::solve(&mut lines);
    println!("Grand total = {:?}", answer);

    let answer = lib::part_two::solve(&mut lines);
    println!("Part 2 grand total = {:?}", answer);
}
