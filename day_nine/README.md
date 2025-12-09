**Problem Summary**
- **Goal (Part 1):** Given a list of red tile coordinates on an integer grid, choose any two red tiles as opposite corners of an axis-aligned rectangle and compute the maximum area possible. Coordinates are inclusive (a rectangle from x=2 to x=11 has width 10 if counting tiles, i.e., |11-2| + 1).
- **Goal (Part 2):** Same as Part 1, but the rectangle interior (all tiles it contains) must be either red or green. The input guarantees that red tiles are connected in order by straight lines of green tiles (wrapping from last to first), and the area inside that red+green loop is also green. All other tiles are neither red nor green.

**Repository Files (relevant)**
- `src/lib.rs` — solver logic for part 1 and part 2 (refactored into helpers).
- `src/main.rs` — runner that reads `input.txt` and prints results.
- `example.txt` — small example (used in tests).
- `input.txt` — full puzzle input (used in tests).
- `tests/functional.rs`, `tests/input.rs` — example and input tests.

**Part 1 — Approach (largest rectangle with red corners)**
- Observation: Any valid rectangle is determined by two red tiles that lie at opposite corners. The area depends only on the absolute differences of their x and y coordinates (inclusive count).
- Implementation:
  - Parse red tile coordinates into a `Vec<(i32,i32)>`.
  - Check every pair (i, j) of points (O(n^2) pairs).
  - Compute inclusive width = |x1 - x2| + 1 and height = |y1 - y2| + 1, area = width * height.
  - Track maximum area.

**Part 1 — Pseudocode**
- Input: points = list of (x,y)
- max_area = 0
- for i in 0..points.len:
  - for j in i+1..points.len:
    - width = abs(points[i].x - points[j].x) + 1
    - height = abs(points[i].y - points[j].y) + 1
    - max_area = max(max_area, width * height)
- return max_area

**Complexity (Part 1)**
- Time: O(n^2) where n = number of red tiles (pairs checked).
- Memory: O(n) storing the points list.

**Part 2 — Naive (slow) idea and why it was too slow**
- Naive: For each red pair (rectangle), scan every tile inside the rectangle and verify each tile is red or green.
- Problem: A rectangle can be very large (many tiles). For each pair we might iterate over area tiles, producing O(n^2 * area) worst-case behavior — impractical for large inputs.

**Optimized Approach (what's implemented)**
1. Build the boundary of green tiles by connecting adjacent red points along rows/columns (the puzzle guarantees these connections).
2. Fill the interior of that boundary to mark all green tiles (the "loop interior" is green).
3. Rather than storing a dense map at full coordinate resolution (which may be huge), do coordinate compression so that consecutive empty ranges are combined into single compressed cells whose area we can compute.
4. Work on a compressed 2D grid (cells correspond to intervals between consecutive unique x and y coordinates). Mark boundary edges and interior cells as green in compressed grid; also mark red cells allowed.
5. Build a weighted 2D prefix-sum (summed-area table) over the compressed grid where each compressed cell contributes its true tile count (cell_width * cell_height) if allowed (green or red), 0 otherwise.
6. For each red pair (still O(n^2) pairs), test if the rectangle's allowed tile area equals its full tile area (computed using inclusive coordinate differences). Using prefix-sum, this check is O(1).

**Part 2 — Key steps & why they help**
- Coordinate compression: avoids allocating a huge W x H dense boolean grid when most coordinates are sparse (e.g., large empty regions). The number of compressed cells is based on unique x/y boundaries derived from red point positions and their connecting edges.
- Weighted prefix-sum: lets us query the total count of allowed tiles inside any rectangle in O(1). If the query equals the rectangle area, the rectangle is fully allowed.
- Compressed flood-fill: flood fill interior on compressed grid to mark inside green cells, but flood-fill cost is proportional to compressed grid size (much smaller than full resolution grid).

**Part 2 — Pseudocode (high level)**
- Input: points = ordered red points (list wraps to first)
- (xs, ys, x_to_idx, y_to_idx) = compress_coords(points)
- (green_grid, width, height) = build_compressed_grid(points, xs, ys, x_to_idx, y_to_idx)
- flood_fill_compressed(green_grid) // mark interior as green
- mark_red_tiles(green_grid, points, x_to_idx, y_to_idx)
- ps = build_weighted_prefix_sum(green_grid, xs, ys, width, height)
- max_area = 0
- for each pair of red points (p1, p2):
  - area = (abs(p1.x - p2.x) + 1) * (abs(p1.y - p2.y) + 1)
  - if area <= max_area: continue
  - if not all coordinates align to compression (should usually align): continue
  - allowed = rect_allowed_area(p1, p2, ps, x_to_idx, y_to_idx)
  - if allowed == area: max_area = area
- return max_area

**Complexity (Optimized Part 2)**
- Let m be number of compressed cells (roughly unique_x_count * unique_y_count)
- Building compressed grid & flood-fill: O(m)
- Building prefix-sum: O(m)
- Pair checks: O(n^2) checks, each O(1) via prefix-sum -> O(n^2)
- Total: O(m + n^2)
- Memory: O(m) for compressed grid and prefix sums (plus O(n) for points)

**Practical notes**
- If the compressed bounding box still grows large (e.g., when red/green perimeter spans a huge area), memory may still be significant. In that case consider further compression strategies or sweep-line approaches (see alternatives below).

**Alternative optimizations**
- Sweep-line per x-pair: For each pair of left/right x coordinates where red tiles exist, project red y positions and use scanning to find max y-span where interior is allowed (reduces checks in some distributions).
- Use spatial indexing or segment trees to test allowed vertical runs faster.
- Multi-thread the O(n^2) pair checks (requires careful synchronization of max-area update).

**Why the compression uses +1 boundaries**
- We represent cells by the half-open intervals between unique coordinates. Including `x` and `x+1` as boundaries ensures single-tile widths are expressible and that when we compute a rectangle from x=a to x=b inclusive we can identify which compressed cells sum up to exactly that inclusive area.

**Rust-specific learnings (key takeaways from the implementation)**
- Data structures used:
  - `Vec<(i32,i32)>`: compact list of points; efficient indexed access and iteration.
  - `HashSet<(i32,i32)>`: used earlier in naive implementations to quickly check membership for green/red tiles.
  - `HashMap<i32,usize>`: coordinate → compressed-index map for fast lookups.
  - `Vec<Vec<bool>>`: 2D compressed boolean grid (green allowed cells).
  - `Vec<u64>`: flattened 2D prefix-sum array.
  - `VecDeque<(usize,usize)>`: queue used for flood-fill (BFS) over compressed grid.

- Ownership and borrowing:
  - Pass `&[T]` or `&Vec<T>` to helpers to avoid unnecessary cloning; `Vec` clones only when necessary.
  - Use `.iter().cloned().collect::<HashSet<_>>()` to build a `HashSet` of owned tuples from references.

- Syntactic features & idioms used:
  - `Option` and `?` operator to parse points: concise error handling while parsing lines.
  - `include_str!("../input.txt")` in tests to embed example/input text at compile time.
  - Iterator adaptors like `.lines().map(|l| l.to_string()).collect()` for building test input vectors.
  - `for i in 0..n` loops when index access is required; `for (i, &v) in xs.iter().enumerate()` when building maps.
  - Use of closures for small helpers (where appropriate) and small helper functions for readability.
  - Flattened 2D arrays (a single `Vec` indexed by `y*(width+1)+x`) for prefix sums — faster and simpler than nested `Vec`s for this use.
  - `saturating_sub(1)` to guard against small length underflow when computing cell counts.

- Type safety and conversions:
  - Be explicit about integer widths and areas: cast `i32` diffs to `u64` when computing area to avoid negative or overflow surprises.
  - Mixed signed/unsigned arithmetic requires careful casts: `(xs[ix+1] - xs[ix]) as i64` to compute cell areas safely and then to `u64`.

- Testing & tooling:
  - Add example and input tests in `tests/` so `cargo test` verifies correctness quickly.
  - Use `cargo run` to print final answers; `cargo test` runs tests; use `--release` for performance measurements on full input.

**Examples — how to run**
- Run the solver on full input (local):

```
cd day_nine
cargo run
```

- Run tests (example + input checks):

```
cd day_nine
cargo test
```

- Run in release mode for performance profiling on `input.txt`:

```
cd day_nine
cargo run --release
```

**Key pitfalls encountered & fixes**
- Off-by-one: initial area computation used `abs(dx) * abs(dy)` which counts gaps rather than inclusive tile counts. Fix: add `+1` to each difference.
- Slow part 2: naive per-rectangle interior scan was too slow. Fix: coordinate compression + weighted prefix-sum gave O(1) rectangle checks.
- Memory vs. correctness tradeoff: dense grid easiest to implement but may blow memory for wide ranges. Compression trades some implementation complexity for memory savings.

**Next actions / improvements**
- Add more unit tests for helper functions (e.g., coordinate compression correctness, prefix-sum queries) so regressions are caught earlier.
- Consider a sweep-line or event-based approach to reduce O(n^2) pairs if `n` (red points) becomes large.
- Add benchmarks (Cargo `bench` or criterion) to track runtime improvements.

