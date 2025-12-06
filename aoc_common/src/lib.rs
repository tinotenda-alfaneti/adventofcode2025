use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;

/// Read a file into a Vec<String> where each element is a line.
pub fn read_file_to_vec(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

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

/// Split a string into leading alphabetic part and trailing integer part.
/// Returns (alpha_part, number). If no digits are present the number is 0.
pub fn split_alpha_num(input: &str) -> Result<(String, i32), ParseIntError> {
    let idx = input
        .find(|c: char| c.is_ascii_digit())
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

pub fn read_file_to_string(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path)
}

/// Split `input` by the given delimiter character, trimming whitespace from each piece.
pub fn split_and_trim(input: &str, delim: char) -> Vec<String> {
    input.split(delim).map(|s| s.trim().to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_split_alpha_num() {
        assert_eq!(split_alpha_num("L10").unwrap(), ("L".to_string(), 10));
        assert_eq!(split_alpha_num("R").unwrap(), ("R".to_string(), 0));
        assert_eq!(split_alpha_num("FOO123").unwrap(), ("FOO".to_string(), 123));
    }

    #[test]
    fn test_read_one_line_file() {
        let path = "one_line.txt";
        fs::write(path, "hello").unwrap();

        let content = read_file_to_string(path).unwrap();
        assert_eq!(content, "hello");

        fs::remove_file(path).unwrap();
    }
}
