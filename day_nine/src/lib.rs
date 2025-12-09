pub mod compressed;

pub fn solve(lines: &[String]) -> (u64, u64) {
    let mut points: Vec<(i32, i32)> = Vec::new();

    for line in lines {
        if let Some((x, y)) = parse_point(line) {
            points.push((x, y));
        }
    }

    (
        get_largest_rectangle(points.clone()),
        get_largest_green_rectangle(points.clone()),
    )
}

fn get_largest_rectangle(points: Vec<(i32, i32)>) -> u64 {
    let mut max_area = 0u64;
    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points.iter().skip(i + 1) {
            let width = (x1 - x2).unsigned_abs() as u64 + 1;
            let height = (y1 - y2).unsigned_abs() as u64 + 1;
            let area = width * height;

            if area > max_area {
                max_area = area;
            }
        }
    }

    max_area
}

fn parse_point(line: &str) -> Option<(i32, i32)> {
    let mut split = line.split(',');
    let x = split.next()?.trim().parse().ok()?;
    let y = split.next()?.trim().parse().ok()?;
    Some((x, y))
}

fn get_largest_green_rectangle(points: Vec<(i32, i32)>) -> u64 {
    let (xs, ys, x_to_idx, y_to_idx) = compressed::compress_coords(&points);
    let (mut green, width, height) =
        compressed::build_compressed_grid(&points, &xs, &ys, &x_to_idx, &y_to_idx);
    compressed::flood_fill_compressed(&mut green);
    compressed::mark_red_tiles(&mut green, &points, &x_to_idx, &y_to_idx, width, height);
    let ps = compressed::build_weighted_prefix_sum(&green, &xs, &ys, width, height);

    let mut max_area = 0u64;
    let ctx = compressed::QueryHelper {
        ps: &ps,
        x_to_idx: &x_to_idx,
        y_to_idx: &y_to_idx,
        width,
    };
    let rect_sum = |x1: i32, y1: i32, x2: i32, y2: i32| -> u64 {
        compressed::rect_allowed_area_ctx(x1, y1, x2, y2, &ctx)
    };

    for (i, &(x1, y1)) in points.iter().enumerate() {
        for &(x2, y2) in points.iter().skip(i + 1) {
            let width_r = (x1 - x2).unsigned_abs() as u64 + 1;
            let height_r = (y1 - y2).unsigned_abs() as u64 + 1;
            let area = width_r * height_r;
            if area <= max_area {
                continue;
            }

            if !x_to_idx.contains_key(&x1)
                || !x_to_idx.contains_key(&x2)
                || !y_to_idx.contains_key(&y1)
                || !y_to_idx.contains_key(&y2)
            {
                continue;
            }

            if rect_sum(x1, y1, x2, y2) == area {
                max_area = area;
            }
        }
    }

    max_area
}
