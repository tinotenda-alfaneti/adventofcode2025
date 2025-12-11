use aoc_common as common;
use day_eleven::day_to_out::solve as solve_part_one;
use day_eleven::svr_to_out::solve as solve_part_two;

fn main() {
    let lines = common::read_file_to_vec("input.txt").unwrap();

    let result = solve_part_one(&lines);
    println!("Valid paths from you = {}", result);

    let result = solve_part_two(&lines);
    println!("Valid paths from svr = {}", result);
}
