use std::collections::{HashMap, HashSet, VecDeque};

pub type CoordMap = HashMap<i32, usize>;

pub fn compress_coords(points: &[(i32, i32)]) -> (Vec<i32>, Vec<i32>, CoordMap, CoordMap) {
    let mut xs_set: HashSet<i32> = HashSet::new();
    let mut ys_set: HashSet<i32> = HashSet::new();
    let n = points.len();
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        xs_set.insert(x1);
        xs_set.insert(x2);
        xs_set.insert(x1 + 1);
        xs_set.insert(x2 + 1);
        ys_set.insert(y1);
        ys_set.insert(y2);
        ys_set.insert(y1 + 1);
        ys_set.insert(y2 + 1);
    }
    for &(x, y) in points {
        xs_set.insert(x);
        xs_set.insert(x + 1);
        ys_set.insert(y);
        ys_set.insert(y + 1);
    }
    let mut xs: Vec<i32> = xs_set.into_iter().collect();
    let mut ys: Vec<i32> = ys_set.into_iter().collect();
    xs.sort_unstable();
    ys.sort_unstable();
    let mut x_to_idx: CoordMap = HashMap::new();
    let mut y_to_idx: CoordMap = HashMap::new();
    for (i, &v) in xs.iter().enumerate() {
        x_to_idx.insert(v, i);
    }
    for (i, &v) in ys.iter().enumerate() {
        y_to_idx.insert(v, i);
    }
    (xs, ys, x_to_idx, y_to_idx)
}

pub fn build_compressed_grid(
    points: &[(i32, i32)],
    xs: &[i32],
    ys: &[i32],
    x_to_idx: &CoordMap,
    y_to_idx: &CoordMap,
) -> (Vec<Vec<bool>>, usize, usize) {
    let width = xs.len().saturating_sub(1);
    let height = ys.len().saturating_sub(1);
    let mut green = vec![vec![false; width]; height];
    let n = points.len();
    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];
        if x1 == x2 {
            let ix = *x_to_idx.get(&x1).unwrap();
            let a = y1.min(y2);
            let b = y1.max(y2);
            let iy1 = *y_to_idx.get(&a).unwrap();
            let iy2 = *y_to_idx.get(&(b + 1)).unwrap();
            if ix < width {
                for row in green.iter_mut().take(iy2).skip(iy1) {
                    row[ix] = true;
                }
            }
        } else {
            let iy = *y_to_idx.get(&y1).unwrap();
            let a = x1.min(x2);
            let b = x1.max(x2);
            let ix1 = *x_to_idx.get(&a).unwrap();
            let ix2 = *x_to_idx.get(&(b + 1)).unwrap();
            if iy < height {
                let row = &mut green[iy];
                for cell in row.iter_mut().take(ix2).skip(ix1) {
                    *cell = true;
                }
            }
        }
    }
    (green, width, height)
}

pub fn flood_fill_compressed(green: &mut [Vec<bool>]) {
    let height = green.len();
    if height == 0 {
        return;
    }
    let width = green[0].len();
    if width == 0 {
        return;
    }
    let start_x = width / 2;
    let start_y = height / 2;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    if !green[start_y][start_x] {
        green[start_y][start_x] = true;
        q.push_back((start_x, start_y));
    }
    while let Some((cx, cy)) = q.pop_front() {
        let neigh = [(-1isize, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in neigh {
            let nx = cx as isize + dx;
            let ny = cy as isize + dy;
            if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                let (nxu, nyu) = (nx as usize, ny as usize);
                if !green[nyu][nxu] {
                    green[nyu][nxu] = true;
                    q.push_back((nxu, nyu));
                }
            }
        }
    }
}

pub fn mark_red_tiles(
    green: &mut [Vec<bool>],
    points: &[(i32, i32)],
    x_to_idx: &CoordMap,
    y_to_idx: &CoordMap,
    width: usize,
    height: usize,
) {
    for &(x, y) in points {
        match (x_to_idx.get(&x), y_to_idx.get(&y)) {
            (Some(&ix), Some(&iy)) if ix < width && iy < height => green[iy][ix] = true,
            _ => {}
        }
    }
}

pub fn build_weighted_prefix_sum(
    green: &[Vec<bool>],
    xs: &[i32],
    ys: &[i32],
    width: usize,
    height: usize,
) -> Vec<u64> {
    let mut ps = vec![0u64; (width + 1) * (height + 1)];
    for iy in 0..height {
        for ix in 0..width {
            let cell_area = ((xs[ix + 1] - xs[ix]) as i64 * (ys[iy + 1] - ys[iy]) as i64) as u64;
            let v = if green[iy][ix] { cell_area } else { 0 };
            ps[(iy + 1) * (width + 1) + (ix + 1)] = v;
        }
    }
    for y in 1..=height {
        for x in 1..=width {
            let idx = y * (width + 1) + x;
            let above = ps[(y - 1) * (width + 1) + x];
            let left = ps[y * (width + 1) + (x - 1)];
            let diag = ps[(y - 1) * (width + 1) + (x - 1)];
            ps[idx] += above + left - diag;
        }
    }
    ps
}

pub struct QueryHelper<'a> {
    pub ps: &'a [u64],
    pub x_to_idx: &'a CoordMap,
    pub y_to_idx: &'a CoordMap,
    pub width: usize,
}

pub fn rect_allowed_area_ctx(x1: i32, y1: i32, x2: i32, y2: i32, ctx: &QueryHelper) -> u64 {
    let min_xx = x1.min(x2);
    let max_xx = x1.max(x2);
    let min_yy = y1.min(y2);
    let max_yy = y1.max(y2);
    let ix1 = *ctx.x_to_idx.get(&min_xx).unwrap();
    let iy1 = *ctx.y_to_idx.get(&min_yy).unwrap();
    let ix2 = *ctx.x_to_idx.get(&(max_xx + 1)).unwrap();
    let iy2 = *ctx.y_to_idx.get(&(max_yy + 1)).unwrap();
    let a = ctx.ps[iy2 * (ctx.width + 1) + ix2];
    let b = ctx.ps[iy1 * (ctx.width + 1) + ix2];
    let c = ctx.ps[iy2 * (ctx.width + 1) + ix1];
    let d = ctx.ps[iy1 * (ctx.width + 1) + ix1];
    a + d - b - c
}
