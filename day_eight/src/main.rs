use std::env;
use std::io;
use aoc_common as common;
use day_eight as lib;

fn main() -> io::Result<()> {

    let mut args = env::args().skip(1);
    let file_path = args.next().unwrap_or_else(|| "input.txt".to_string());
    let merge_limit: usize = match args.next() {
        Some(s) => s.parse().unwrap_or(1000),
        None => 1000,
    };

    let lines = common::read_file_to_vec(&file_path)?;

    let (part1, part2) = lib::solve(&lines, merge_limit);

    println!("Part 1 (product of top 3 sizes after {} connections) = {:?}", merge_limit, part1);
    println!("Part 2 (product of X coords of last connection) = {:?}", part2);

    Ok(())
}
