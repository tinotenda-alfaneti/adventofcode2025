use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_day_one(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_one/input.txt").unwrap();

    c.bench_function("day_01", |b| {
        b.iter(|| {
            let p1 = day_one::solve_part_one(black_box(&input));
            let p2 = day_one::solve_part_two(black_box(&input));
            (p1, p2)
        })
    });
}

fn benchmark_day_two(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_two/input.txt").unwrap();

    c.bench_function("day_02", |b| {
        b.iter(|| {
            let p1 = day_two::sum_invalid_ids(black_box(&input), 1, false);
            let p2 = day_two::sum_invalid_ids(black_box(&input), 2, false);
            (p1, p2)
        })
    });
}

fn benchmark_day_three(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_three/input.txt").unwrap();

    c.bench_function("day_03", |b| {
        b.iter(|| {
            let p1 = day_three::largest_two_digit(black_box(&input));
            let p2 = day_three::largest_twelve_digit(black_box(&input));
            (p1, p2)
        })
    });
}

fn benchmark_day_four(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_four/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_04", |b| {
        b.iter(|| day_four::solve(black_box(lines.clone())))
    });
}

fn benchmark_day_five(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_five/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_05", |b| {
        b.iter(|| day_five::solve(black_box(lines.clone())).unwrap())
    });
}

fn benchmark_day_six(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_six/input.txt").unwrap();

    c.bench_function("day_06", |b| {
        b.iter(|| {
            let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
            let p1 = day_six::part_one::solve(black_box(&mut lines)).unwrap();
            let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
            let p2 = day_six::part_two::solve(black_box(&mut lines)).unwrap();
            (p1, p2)
        })
    });
}

fn benchmark_day_seven(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_seven/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();

    c.bench_function("day_07", |b| {
        b.iter(|| day_seven::count_splits(black_box(&grid)))
    });
}

fn benchmark_day_eight(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_eight/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_08", |b| {
        b.iter(|| day_eight::solve(black_box(&lines), 2))
    });
}

fn benchmark_day_nine(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_nine/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_09", |b| b.iter(|| day_nine::solve(black_box(&lines))));
}

fn benchmark_day_ten(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_ten/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_10", |b| {
        b.iter(|| {
            let p1 = day_ten::indicator_presses::solve(black_box(&lines));
            let p2 = day_ten::joltage_presses::solve(black_box(&lines));
            (p1, p2)
        })
    });
}

fn benchmark_day_eleven(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_eleven/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_11", |b| {
        b.iter(|| {
            let p1 = day_eleven::day_to_out::solve(black_box(&lines));
            let p2 = day_eleven::svr_to_out::solve(black_box(&lines));
            (p1, p2)
        })
    });
}

fn benchmark_day_twelve(c: &mut Criterion) {
    let input = std::fs::read_to_string("day_twelve/input.txt").unwrap();
    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    c.bench_function("day_12", |b| {
        b.iter(|| day_twelve::christmas_tree_farm::solve(black_box(&lines)))
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().sample_size(10);
    targets =
        benchmark_day_one,
        benchmark_day_two,
        benchmark_day_three,
        benchmark_day_four,
        benchmark_day_five,
        benchmark_day_six,
        benchmark_day_seven,
        benchmark_day_eight,
        benchmark_day_nine,
        benchmark_day_ten,
        benchmark_day_eleven,
        benchmark_day_twelve
}

criterion_main!(benches);
