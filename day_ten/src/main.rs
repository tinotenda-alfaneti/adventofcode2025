use aoc_common as common;
use day_ten::indicator_presses::solve as solve_indicator;
use day_ten::joltage_presses::solve as solve_joltage;

fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let total = solve_indicator(&lines);

    println!("Fewest total presses (indicator lights): {total}");

    let total_joltage = solve_joltage(&lines);

    println!("Fewest total presses (joltage): {total_joltage}");
}
