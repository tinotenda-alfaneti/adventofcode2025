use aoc_common as common;
use day_seven as lib;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let file_path = "input.txt";
    let lines = common::read_file_to_vec(file_path)?;
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    
    // Part 1
    let splits = lib::count_splits(&grid);
    println!("Total splits: {}", splits);

    // Part 2
    let timelines = lib::count_timelines(&grid);
    println!("Total timelines: {}", timelines);

    Ok(())
}
