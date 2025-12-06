use aoc_common as common;
use day_three as lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input.txt";
    let lines = common::read_file_to_vec(file_path)?;

    let part1: u32 = lines.iter().map(|l| lib::largest_two_digit(l)).sum();
    println!("Part One Result: {}", part1);

    let part2: u64 = lines.iter().map(|l| lib::largest_twelve_digit(l)).sum();
    println!("Part Two Result: {}", part2);

    Ok(())
}

