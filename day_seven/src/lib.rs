use std::collections::HashSet;

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    let h = grid.len();
    for r in 0..h {
        for c in 0..grid[r].len() {
            if grid[r][c] == 'S' {
                return Some((r, c));
            }
        }
    }
    None
}

pub fn count_splits(grid: &Vec<Vec<char>>) -> usize {
    let h = grid.len();
    let w = grid[0].len();

    let (sr, sc) = find_start(grid).expect("No S found in grid");
    let start_beam = (sr + 1, sc);
    let mut splits = 0;
    let mut beams: HashSet<usize> = HashSet::new();
    if start_beam.0 < h {
        beams.insert(start_beam.1);
    }

    for r in (sr + 1)..h {
        if beams.is_empty() {
            break;
        }
        let mut next: HashSet<usize> = HashSet::new();
        for &c in &beams {
            if grid[r][c] == '^' {
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


pub fn count_timelines(grid: &Vec<Vec<char>>) -> u128 {
    let h = grid.len();
    let w = grid[0].len();

    let (sr, sc) = find_start(grid).expect("No S found in grid");

    if sr + 1 >= h {
        return 1;
    }

    let mut counts = vec![0u128; w];
    counts[sc] = 1;

    for r in (sr + 1)..h {
        let mut next = vec![0u128; w];
        for c in 0..w {
            let cnt = counts[c];
            if cnt == 0 {
                continue;
            }
            if grid[r][c] == '^' {
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
