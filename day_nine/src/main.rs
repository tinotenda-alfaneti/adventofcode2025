use aoc_common as common;
use day_nine as lib;

fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let (max_area, all_green_max_area) = lib::solve(&lines);

    println!("Largest rectangle area: {}", max_area);
    println!("Largest rectangle area (all green): {}", all_green_max_area);
}
