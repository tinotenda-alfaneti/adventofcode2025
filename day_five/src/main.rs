use aoc_common as common;
use day_five as lib;

fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    match lib::solve(lines) {
        Ok((fresh_count, total_fresh_ids)) => {
            println!("Part 1 - Total fresh count: {}", fresh_count);
            println!("Part 2 - Total fresh ingredient IDs: {}", total_fresh_ids);
        }
        Err(e) => eprintln!("Error solving: {}", e),
    }
}
