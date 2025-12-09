use criterion::BatchSize;
use criterion::Criterion;

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

fn main() {
    let s = include_str!("../input.txt");
    let points = parse_points(s);

    let mut c = Criterion::default();

    c.bench_function("compress_coords", |b| {
        b.iter_batched(
            || points.clone(),
            |pts| {
                let _ = day_nine::compressed::compress_coords(&pts);
            },
            BatchSize::SmallInput,
        )
    });

    let (xs, ys, x_to_idx, y_to_idx) = day_nine::compressed::compress_coords(&points);

    c.bench_function("build_grid_and_fill", |b| {
        b.iter(|| {
            let (mut green, width, height) = day_nine::compressed::build_compressed_grid(
                &points, &xs, &ys, &x_to_idx, &y_to_idx,
            );
            day_nine::compressed::flood_fill_compressed(&mut green);
            day_nine::compressed::mark_red_tiles(
                &mut green, &points, &x_to_idx, &y_to_idx, width, height,
            );
        })
    });

    let (mut green, width, height) =
        day_nine::compressed::build_compressed_grid(&points, &xs, &ys, &x_to_idx, &y_to_idx);
    day_nine::compressed::flood_fill_compressed(&mut green);
    day_nine::compressed::mark_red_tiles(&mut green, &points, &x_to_idx, &y_to_idx, width, height);

    c.bench_function("build_weighted_prefix_sum", |b| {
        b.iter(|| {
            let _ps =
                day_nine::compressed::build_weighted_prefix_sum(&green, &xs, &ys, width, height);
        })
    });

    c.bench_function("full_solve", |b| {
        b.iter(|| {
            let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();
            let _ = day_nine::solve(&lines);
        })
    });
}
