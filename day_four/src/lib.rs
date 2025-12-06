
pub fn parse_grid(lines: &[String]) -> Vec<Vec<char>> {
    lines.iter().map(|row| row.chars().collect()).collect()
}

/// Return true if the cell at (r,c) is an '@' and has fewer than 4 adjacent '@'s.
pub fn is_accessible(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if map[r][c] != '@' {
        return false;
    }

    let h = map.len();
    let w = map[0].len();

    let dirs = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let mut adj = 0usize;
    for (dr, dc) in dirs {
        let nr = r as isize + dr;
        let nc = c as isize + dc;
        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            if map[nr as usize][nc as usize] == '@' {
                adj += 1;
                if adj >= 4 {
                    return false;
                }
            }
        }
    }

    adj < 4
}


pub fn count_accessible(lines: &[String]) -> usize {
    if lines.is_empty() { return 0; }
    let map = parse_grid(lines);
    let h = map.len();
    let w = map[0].len();

    let mut accessible = 0usize;
    for r in 0..h {
        for c in 0..w {
            if is_accessible(&map, r, c) {
                accessible += 1;
            }
        }
    }
    accessible
}

pub fn removed_total(lines: &[String]) -> usize {
    if lines.is_empty() { return 0; }
    let h = lines.len();
    let w = lines[0].len();

    let mut map: Vec<Vec<char>> = parse_grid(lines);
    let mut total_removed = 0usize;

    loop {
        let mut to_remove = Vec::new();
        for r in 0..h {
            for c in 0..w {
                if is_accessible(&map, r, c) {
                    to_remove.push((r, c));
                }
            }
        }
        let removed_this_round = to_remove.len();
        if removed_this_round == 0 { break; }
        for (r, c) in to_remove {
            map[r][c] = '.';
        }
        total_removed += removed_this_round;
    }

    total_removed
}

pub fn solve(lines: Vec<String>) -> (usize, usize) {
    let part1 = count_accessible(&lines);
    let part2 = removed_total(&lines);
    (part1, part2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_accessible_simple() {
        let lines = vec!["@@@".to_string(), "...".to_string(), "..@".to_string()];
        // accessible positions: all top row (3) and bottom-right (1) => 4
        assert_eq!(count_accessible(&lines), 4);
    }

    #[test]
    fn test_removed_total_simple() {
        let lines = vec!["@@@".to_string(), "...".to_string()];
        // all three in top row are accessible and removed in one round
        assert_eq!(removed_total(&lines), 3);
    }
}
