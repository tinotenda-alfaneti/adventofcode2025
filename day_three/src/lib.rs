use std::num::ParseIntError;

pub fn largest_two_digit(s: &str) -> u32 {
    let digit_count = s.chars().filter(|c| c.is_ascii_digit()).count();
    if digit_count < 2 {
        return 0;
    }
    largest_n_digit(s, 2).unwrap_or(0) as u32
}

pub fn largest_twelve_digit(s: &str) -> u64 {
    largest_n_digit(s, 12).unwrap_or(0)
}


pub fn largest_n_digit(s: &str, keep: usize) -> Result<u64, ParseIntError> {
    let digits: Vec<char> = s.chars().filter(|c| c.is_ascii_digit()).collect();
    if keep == 0 {
        return Err("empty".parse::<u64>().unwrap_err());
    }
    let n = digits.len();
    if n <= keep {
        let collected: String = digits.into_iter().collect();
        return collected.parse::<u64>();
    }

    let mut remove = n - keep;
    let mut stack: Vec<char> = Vec::with_capacity(keep);

    for d in digits {
        while remove > 0 && stack.last().map_or(false, |&c| c < d) {
            stack.pop();
            remove -= 1;
        }
        stack.push(d);
    }

    stack.truncate(keep);
    let s: String = stack.into_iter().collect();
    s.parse::<u64>()
}

