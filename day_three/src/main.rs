use aoc_common as common;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path = "input.txt";
    let lines = common::read_file_to_vec(file_path)?;

    let part1: u32 = lines.iter().map(|l| largest_two_digit(l)).sum();
    println!("Part One Result: {}", part1);

    let part2: u64 = lines.iter().map(|l| largest_n_digit(l, 12).unwrap_or(0)).sum();
    println!("Part Two Result: {}", part2);

    Ok(())
}

/// Find the largest two-digit number that can be formed by choosing
/// a digit for the tens place and a later digit for the ones place.
/// Non-digit characters are ignored; if there aren't enough digits,
/// returns 0.
fn largest_two_digit(s: &str) -> u32 {
    // collect digits as u32 (0..9)
    let digits: Vec<u32> = s
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();

    // Keep track of the best ones digit seen to the right while scanning
    // from right to left.
    let mut best = 0u32;
    let mut best_ones = 0u32; // max digit to the right of current position

    for &d in digits.iter().rev().skip(1) {
        // d is the digit at position i (scanning right-to-left). The next
        // digit to the left is a candidate tens digit; update best_ones
        // first then compute candidate when moving left in the outer loop.
        if d > best_ones {
            best_ones = d;
        }
    }

    // Simpler two-pass: build max-right array but with safe handling.
    if digits.len() < 2 {
        return 0;
    }

    let n = digits.len();
    let mut max_right = vec![0u32; n];
    let mut current_max = 0u32;
    for i in (0..n).rev() {
        max_right[i] = current_max;
        if digits[i] > current_max {
            current_max = digits[i];
        }
    }

    for i in 0..n - 1 {
        let val = digits[i] * 10 + max_right[i];
        if val > best {
            best = val;
        }
    }

    best
}

/// Return the largest number (as u64) obtainable by deleting digits from
/// `s` such that exactly `keep` digits remain and the resulting number is
/// as large as possible. Returns a parse error if the resulting string
/// cannot be parsed as u64 (e.g. empty or too large).
fn largest_n_digit(s: &str, keep: usize) -> Result<u64, ParseIntError> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if keep == 0 {
        return Err("empty".parse::<u64>().unwrap_err());
    }
    let n = digits.len();
    if n <= keep {
        // If there are fewer-or-equal digits than keep, just parse what's there
        let collected: String = digits.into_iter().collect();
        return collected.parse::<u64>();
    }

    let mut remove = n - keep;
    let mut stack: Vec<char> = Vec::with_capacity(keep);

    for d in digits {
        while remove > 0 && stack.last().map(|&c| c) < Some(d) {
            stack.pop();
            remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(keep);
    let s: String = stack.into_iter().collect();
    s.parse::<u64>()
}

#[cfg(test)]
mod tests {
    use super::{largest_n_digit, largest_two_digit};

    #[test]
    fn test_largest_two_digit_basic() {
        assert_eq!(largest_two_digit("123"), 23);
        assert_eq!(largest_two_digit("9081726354"), 98);
    }

    #[test]
    fn test_largest_two_digit_nondigits() {
        assert_eq!(largest_two_digit("a1b2c3"), 23);
        assert_eq!(largest_two_digit("x"), 0);
        assert_eq!(largest_two_digit(""), 0);
    }

    #[test]
    fn test_largest_n_digit_basic() {
        // From "123456" keep=3 -> choose 456
        assert_eq!(largest_n_digit("123456", 3).unwrap(), 456);
        // keep equals length -> full parse
        assert_eq!(largest_n_digit("007", 3).unwrap(), 7);
    }

    #[test]
    fn test_largest_n_digit_with_nondigits_and_short() {
        assert_eq!(largest_n_digit("a9b8c7d6", 2).unwrap(), 98);
        // fewer digits than keep -> parse as-is ("12" parsed as 12)
        assert_eq!(largest_n_digit("1a2", 5).unwrap(), 12);
    }

    #[test]
    fn test_largest_n_digit_zero_keep() {
        // keep == 0 returns parse error; ensure it doesn't panic
        assert!(largest_n_digit("12345", 0).is_err());
    }
}

