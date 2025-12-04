use aoc_common as common;

fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };



    // Part One
    let accessible = count_accessible(&lines);
    println!("Accessible: {}", accessible);

    // Part Two
    let removed = removed_total(&lines);
    println!("Total Removed: {}", removed);
}

fn count_accessible(grid: &Vec<String>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    let map: Vec<Vec<char>> = grid.iter().map(|row| row.chars().collect()).collect();

    let mut accessible = 0;

    for r in 0..h {
        for c in 0..w {
            if is_accessible(&map, r, c) {
                accessible += 1;
            }
        }
    }

    accessible
}

/// Repeatedly remove all accessible '@'s
/// until no more can be removed.
pub fn removed_total(input: &Vec<String>) -> usize {
    if input.is_empty() { return 0; }

    let h = input.len();
    let w = input[0].len();

    let mut map: Vec<Vec<char>> = input.iter()
        .map(|row| row.chars().collect())
        .collect();


    let mut total_removed = 0usize;

    loop {
        let mut to_remove: Vec<(usize, usize)> = Vec::new();

        for r in 0..h {
            for c in 0..w {
                if is_accessible(&map, r, c) {
                    to_remove.push((r, c));
                }
            }
        }

        let removed_this_round = to_remove.len();
        if removed_this_round == 0 { break; }

        for (r,c) in to_remove {
            map[r][c] = '.';
        }

        total_removed += removed_this_round;
    }


    total_removed
}

fn is_accessible(map: &Vec<Vec<char>>, r: usize, c: usize) -> bool {
    if map[r][c] != '@' {
        return false;
    }

    let h = map.len();
    let w = map[0].len();

    let dirs = [
        (-1,-1), (-1, 0), (-1, 1),
        ( 0,-1),          ( 0, 1),
        ( 1,-1), ( 1, 0), ( 1, 1),
    ];

    let mut adj = 0;

    for (dr, dc) in dirs {
        let nr = r as isize + dr;
        let nc = c as isize + dc;

        if nr >= 0 && nr < h as isize && nc >= 0 && nc < w as isize {
            if map[nr as usize][nc as usize] == '@' {
                adj += 1;
                if adj >= 4 {
                    return false; // no need to keep counting
                }
            }
        }
    }

    adj < 4
}


