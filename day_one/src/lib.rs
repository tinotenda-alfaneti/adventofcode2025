mod solutions;

pub fn solve_part_one(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    solutions::part_one(&lines)
}

pub fn solve_part_two(input: &str) -> i32 {
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    solutions::part_two(&lines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let input = "L50\nR50\n";
        let _ = solve_part_one(input);
        let _ = solve_part_two(input);
    }
}
