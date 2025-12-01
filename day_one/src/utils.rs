use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

pub fn read_file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Collect each line into a Vec<String>
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| match line {
            Ok(l) => Some(l),
            Err(e) => {
                eprintln!("Warning: Could not read a line: {}", e);
                None
            }
        })
        .collect();

    Ok(lines)
}

pub fn split_alpha_num(input: &str) -> Result<(String, i32), ParseIntError> {
    let idx = input.find(|c: char| c.is_ascii_digit())
        .unwrap_or(input.len());

    let alpha_part = input[..idx].to_string();
    let num_part_str = &input[idx..];

    let num_part = if num_part_str.is_empty() {
        0
    } else {
        num_part_str.parse::<i32>()?
    };

    Ok((alpha_part, num_part))
}