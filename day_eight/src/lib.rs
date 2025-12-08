mod dsu;

pub use dsu::{DisjointSetUnion, Point};

pub fn distance(a: &Point, b: &Point) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}

pub fn solve(lines: &[String], merge_limit: usize) -> (Option<usize>, Option<i128>) {
    let (pairs, xs, n) = prepare(lines);
    if n == 0 {
        return (None, None);
    }

    let part1_solution = prod_of_top_three(&pairs, n, merge_limit);
    let part2_solution = prod_of_last_con_x(&pairs, &xs, n);
    (part1_solution, part2_solution)
}

/// Parse lines and prepare sorted distance pairs and xs vector
pub fn prepare(lines: &[String]) -> (Vec<(f64, usize, usize)>, Vec<i128>, usize) {
    let mut points: Vec<Point> = Vec::new();
    let mut xs: Vec<i128> = Vec::new();
    for l in lines {
        let nums_int: Vec<i128> = l
            .split(',')
            .map(|n| n.trim().parse::<i128>().unwrap())
            .collect();
        xs.push(nums_int[0]);
        points.push(Point {
            x: nums_int[0] as f64,
            y: nums_int[1] as f64,
            z: nums_int[2] as f64,
        });
    }

    let n = points.len();
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            pairs.push((distance(&points[i], &points[j]), i, j));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    (pairs, xs, n)
}

pub fn prod_of_top_three(
    pairs: &[(f64, usize, usize)],
    n: usize,
    merge_limit: usize,
) -> Option<usize> {
    let mut dsu = DisjointSetUnion::new(n);
    for (idx, &(_, i, j)) in pairs.iter().enumerate() {
        if idx >= merge_limit || idx >= pairs.len() {
            break;
        }
        let _ = dsu.union(i, j);
    }
    let mut components = Vec::new();
    for i in 0..n {
        if dsu.find(i) == i {
            components.push(dsu.size[i]);
        }
    }
    components.sort_unstable_by(|a, b| b.cmp(a));
    if components.len() >= 3 {
        Some(components[0] * components[1] * components[2])
    } else {
        None
    }
}

pub fn prod_of_last_con_x(pairs: &[(f64, usize, usize)], xs: &[i128], n: usize) -> Option<i128> {
    let mut dsu = DisjointSetUnion::new(n);
    let mut components_left = n;
    for &(_, i, j) in pairs {
        if dsu.union(i, j) {
            components_left -= 1;
            if components_left == 1 {
                return Some(xs[i] * xs[j]);
            }
        }
    }
    None
}
