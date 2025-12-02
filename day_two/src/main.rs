use aoc_common as common;

trait PatternChecker {
    fn is_invalid(&self, id_str: &str) -> bool;
}

struct Part1Checker;
struct Part2Checker;

impl PatternChecker for Part1Checker {
    fn is_invalid(&self, id_str: &str) -> bool {
        has_repeated_pattern_part1(id_str)
    }
}

impl PatternChecker for Part2Checker {
    fn is_invalid(&self, id_str: &str) -> bool {
        has_repeated_pattern_part2(id_str)
    }
}

fn main() {
    // CLI: cargo run -- <input-file> [--part 1|2]
    let args: Vec<String> = std::env::args().collect();
    let file_path = args.get(1).map(|s| s.as_str()).unwrap_or("input.txt");
    // default to part 2 (the broader rule)
    let mut part = 2;
    if let Some(idx) = args.iter().position(|a| a == "--part") {
        if let Some(p) = args.get(idx + 1) {
            if let Ok(n) = p.parse::<u32>() {
                part = n as i32;
            }
        }
    }

    let lines = match common::read_file_to_string(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };

    let id_ranges = common::split_and_trim(&lines, ',');

    let checker: Box<dyn PatternChecker> = match part {
        1 => Box::new(Part1Checker),
        _ => Box::new(Part2Checker),
    };

    let sum_invalid_ids = get_sum_of_invalid_ids_with_checker(&id_ranges, file_path == "example.txt", &*checker);
    println!("Sum of invalid IDs: {}", sum_invalid_ids);
}

fn get_sum_of_invalid_ids_with_checker(id_ranges: &[String], debug: bool, checker: &dyn PatternChecker) -> i64 {
    let mut sum: i64 = 0;

    for range in id_ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        if bounds.len() != 2 {
            continue; // skip malformed range
        }

        // Parse the two bounds as i64; skip this range if parsing fails
        let lower: i64 = match bounds[0].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let upper: i64 = match bounds[1].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if lower > upper {
            continue; // skip invalid ranges
        }

        // Loop from lower to upper inclusive
        for id in lower..=upper {
            if checker.is_invalid(&id.to_string()) {
                if debug {
                    println!("Invalid ID found: {} (range {}-{})", id, lower, upper);
                }

                sum = sum.saturating_add(id);
            }
        }
    }

    sum
}

/// Part 1 rule: ID is invalid if it's made of a sequence repeated exactly twice
fn has_repeated_pattern_part1(num_str: &str) -> bool {
    let len = num_str.len();
    if len % 2 != 0 || len == 0 {
        return false;
    }

    let half = len / 2;
    &num_str[..half] == &num_str[half..]
}

/// Part 2 rule: ID is invalid if it's made of some sequence repeated at least twice
fn has_repeated_pattern_part2(num_str: &str) -> bool {
    let len = num_str.len();
    if len == 0 {
        return false;
    }

    // Check all possible substring lengths where the substring repeats at least twice
    for sub_len in 1..=len / 2 {
        if len % sub_len != 0 {
            continue;
        }

        let times = len / sub_len;
        if times < 2 {
            continue;
        }

        let pattern = &num_str[..sub_len];
        if pattern.repeat(times) == num_str {
            return true;
        }
    }

    false
}

