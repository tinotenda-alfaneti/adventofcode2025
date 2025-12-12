use aoc_common as common;

mod christmas_tree_farm;
use christmas_tree_farm::solve;

fn main() {
    let lines = common::read_file_to_vec("input.txt").unwrap();
    let result = solve(&lines);
    println!("Regions that can be tiled = {}", result);
}
