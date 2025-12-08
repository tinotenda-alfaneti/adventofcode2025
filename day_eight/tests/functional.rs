use day_eight as lib;
use aoc_common as common;

#[test]
fn example_counts_and_last_pair() {
    let lines = common::read_file_to_vec("example.txt").unwrap();
    let mut points: Vec<lib::Point> = Vec::new();
    for l in &lines {
        let nums: Vec<f64> = l.split(',').map(|n| n.trim().parse().unwrap()).collect();
        points.push(lib::Point { x: nums[0], y: nums[1], z: nums[2] });
    }

    let n = points.len();
    let mut dsu = lib::DisjointSetUnion::new(n);
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            pairs.push((lib::distance(&points[i], &points[j]), i, j));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    // first 10 connections
    for (idx, &(_, i, j)) in pairs.iter().enumerate() {
        if idx >= 10 { break; }
        let _ = dsu.union(i, j);
    }

    let mut components = Vec::new();
    for i in 0..n {
        if dsu.find(i) == i { components.push(dsu.size[i]); }
    }
    components.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(components[0] * components[1] * components[2], 40);


    let mut dsu2 = lib::DisjointSetUnion::new(n);
    let mut left = n;
    let mut last = None;
    for &(_, i, j) in &pairs {
        if dsu2.union(i, j) {
            left -= 1;
            if left == 1 { last = Some((i, j)); break; }
        }
    }
    let (i, j) = last.expect("should have merged to one component");
    let xs: Vec<i128> = lines.iter().map(|l| l.split(',').next().unwrap().trim().parse().unwrap()).collect();
    assert_eq!(xs[i] * xs[j], 25272i128);
}

#[test]
fn puzzle_input_results() {
    let lines = common::read_file_to_vec("input.txt").unwrap();
    let mut points: Vec<lib::Point> = Vec::new();
    let mut xs: Vec<i128> = Vec::new();
    for l in &lines {
        let nums_int: Vec<i128> = l.split(',').map(|n| n.trim().parse().unwrap()).collect();
        xs.push(nums_int[0]);
        points.push(lib::Point { x: nums_int[0] as f64, y: nums_int[1] as f64, z: nums_int[2] as f64 });
    }

    let n = points.len();
    let mut dsu = lib::DisjointSetUnion::new(n);
    let mut pairs: Vec<(f64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            pairs.push((lib::distance(&points[i], &points[j]), i, j));
        }
    }
    pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (idx, &(_, i, j)) in pairs.iter().enumerate() {
        if idx >= 1000 { break; }
        let _ = dsu.union(i, j);
    }

    let mut components = Vec::new();
    for i in 0..n { if dsu.find(i) == i { components.push(dsu.size[i]); } }
    components.sort_unstable_by(|a, b| b.cmp(a));
    assert_eq!(components[0] * components[1] * components[2], 129564);

    let mut dsu2 = lib::DisjointSetUnion::new(n);
    let mut left = n;
    let mut last = None;
    for &(_, i, j) in &pairs {
        if dsu2.union(i, j) {
            left -= 1;
            if left == 1 { last = Some((i, j)); break; }
        }
    }
    let (i, j) = last.expect("should have merged to one component");
    assert_eq!(xs[i] * xs[j], 42047840i128);
}
