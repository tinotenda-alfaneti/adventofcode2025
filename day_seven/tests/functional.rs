use aoc_common as common;
use day_seven as lib;

#[test]
fn example_counts_21() {
    let lines = common::read_file_to_vec("example.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    assert_eq!(lib::count_splits(&grid), 21);
}

#[test]
fn puzzle_input_result() {
    let lines = common::read_file_to_vec("input.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    assert_eq!(lib::count_splits(&grid), 1649);
}

#[test]
fn example_timelines_40() {
    let lines = common::read_file_to_vec("example.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    assert_eq!(lib::count_timelines(&grid), 40u128);
}

#[test]
fn puzzle_input_timelines() {
    let lines = common::read_file_to_vec("input.txt").unwrap();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    let val = lib::count_timelines(&grid);
    println!("puzzle timelines = {}", val);
    assert_eq!(val, 16937871060075u128);
}
