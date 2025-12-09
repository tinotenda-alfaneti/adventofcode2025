use criterion::{Criterion, criterion_group, criterion_main};
use day_nine as lib;

fn bench_part2_full(c: &mut Criterion) {
    let s = include_str!("../input.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    c.bench_function("part2_solve", |b| {
        b.iter(|| {
            let (_p1, p2) = lib::solve(&lines);
            p2
        })
    });
}

fn parse_points(s: &str) -> Vec<(i32, i32)> {
    s.lines()
        .filter_map(|line| {
            let mut split = line.split(',');
            let x = split.next()?.trim().parse().ok()?;
            let y = split.next()?.trim().parse().ok()?;
            Some((x, y))
        })
        .collect()
}

fn bench_compress_and_grid(c: &mut Criterion) {
    let s = include_str!("../input.txt");
    let points = parse_points(s);

    c.bench_function("compress_coords", |b| {
        b.iter(|| {
            let _ = lib::compressed::compress_coords(&points);
        })
    });

    // prepare xs, ys, maps once for downstream benches
    let (xs, ys, x_to_idx, y_to_idx) = lib::compressed::compress_coords(&points);

    c.bench_function("build_grid_and_fill", |b| {
        b.iter(|| {
            let (mut green, width, height) =
                lib::compressed::build_compressed_grid(&points, &xs, &ys, &x_to_idx, &y_to_idx);
            lib::compressed::flood_fill_compressed(&mut green);
            lib::compressed::mark_red_tiles(
                &mut green, &points, &x_to_idx, &y_to_idx, width, height,
            );
        })
    });

    // bench weighted prefix sum construction separately
    let (mut green, width, height) =
        lib::compressed::build_compressed_grid(&points, &xs, &ys, &x_to_idx, &y_to_idx);
    lib::compressed::flood_fill_compressed(&mut green);
    lib::compressed::mark_red_tiles(&mut green, &points, &x_to_idx, &y_to_idx, width, height);

    c.bench_function("build_weighted_prefix_sum", |b| {
        b.iter(|| {
            let _ps = lib::compressed::build_weighted_prefix_sum(&green, &xs, &ys, width, height);
        })
    });
}

criterion_group!(benches, bench_part2_full, bench_compress_and_grid);
criterion_main!(benches);
