use std::collections::HashSet;

fn find_start(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (r, row) in grid.iter().enumerate() {
        for (c, &ch) in row.iter().enumerate() {
            if ch == 'S' {
                return Some((r, c));
            }
        }
    }
    None
}

pub fn count_splits(grid: &[Vec<char>]) -> usize {
    let w = grid[0].len();

    let (sr, sc) = find_start(grid).expect("No S found in grid");
    let mut splits = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    if sr + 1 < grid.len() {
        beams.insert(sc);
    }

    for (_r, row) in grid.iter().enumerate().skip(sr + 1) {
        if beams.is_empty() {
            break;
        }
        let mut next: HashSet<usize> = HashSet::new();
        for &c in &beams {
            if row[c] == '^' {
                splits += 1;
                if c > 0 {
                    next.insert(c - 1);
                }
                if c + 1 < w {
                    next.insert(c + 1);
                }
            } else {
                next.insert(c);
            }
        }
        beams = next;
    }

    splits
}

pub fn count_timelines(grid: &[Vec<char>]) -> u128 {
    let w = grid[0].len();

    let (sr, sc) = find_start(grid).expect("No S found in grid");

    if sr + 1 >= grid.len() {
        return 1;
    }

    let mut counts = vec![0u128; w];
    counts[sc] = 1;

    for (_r, row) in grid.iter().enumerate().skip(sr + 1) {
        let mut next = vec![0u128; w];
        for (c, &cell) in row.iter().enumerate() {
            let cnt = counts[c];
            if cnt == 0 {
                continue;
            }
            if cell == '^' {
                if c > 0 {
                    next[c - 1] = next[c - 1].saturating_add(cnt);
                }
                if c + 1 < w {
                    next[c + 1] = next[c + 1].saturating_add(cnt);
                }
            } else {
                next[c] = next[c].saturating_add(cnt);
            }
        }
        counts = next;
    }

    counts.into_iter().sum()
}
