use aoc_common as common;

pub fn sum_invalid_ids(input: &str, part: u8, debug: bool) -> i64 {
    let id_ranges = common::split_and_trim(input, ',');

    let mut sum: i64 = 0;

    for range in id_ranges {
        let bounds: Vec<&str> = range.split('-').collect();
        if bounds.len() != 2 {
            continue; // skip malformed range
        }

        let lower: i64 = match bounds[0].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        let upper: i64 = match bounds[1].trim().parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        if lower > upper {
            continue;
        }

        for id in lower..=upper {
            let s = id.to_string();
            let invalid = match part {
                1 => has_repeated_pattern_twice(&s),
                _ => has_more_than_two_repeated_patterns(&s),
            };

            if invalid {
                if debug {
                    println!("Invalid ID found: {} (range {}-{})", id, lower, upper);
                }
                sum = sum.saturating_add(id);
            }
        }
    }

    sum
}

pub fn has_repeated_pattern_twice(num_str: &str) -> bool {
    let len = num_str.len();
    if len % 2 != 0 || len == 0 {
        return false;
    }

    let half = len / 2;
    &num_str[..half] == &num_str[half..]
}

pub fn has_more_than_two_repeated_patterns(num_str: &str) -> bool {
    let len = num_str.len();
    if len == 0 {
        return false;
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_pattern_examples() {
        assert!(has_repeated_pattern_twice("1212"));
        assert!(!has_repeated_pattern_twice("123"));
        assert!(!has_repeated_pattern_twice("111"));
    }

    #[test]
    fn part2_pattern_examples() {
        assert!(has_more_than_two_repeated_patterns("1212"));
        assert!(has_more_than_two_repeated_patterns("1111"));
        assert!(!has_more_than_two_repeated_patterns("12312"));
    }

    #[test]
    fn sum_invalid_ids_small_range() {
        let input = "10-12,1212-1212";
        assert_eq!(sum_invalid_ids(input, 2, false), 1223);
    }
}
