# Day 8 — Junction Boxes & Disjoint Set Union (DSU)

This crate solves Advent of Code Day 8. The code demonstrates a compact, slightly OOP-style DSU (Disjoint Set Union) implementation in Rust and shows how to structure a small crate with `lib.rs` + `main.rs` while keeping the logic testable.

This README explains:
- the problem and high-level approach
- how the DSU works and a Rust implementation pattern
- how the crate is organized (modules, `pub use`, `crate`)
- Rust ownership and borrowing considerations used here
- usage / CLI examples
- `Option` usage and error choices
- alternative approaches (BinaryHeap streaming, KD-tree, partial selection)

---

## Problem (short)

- Input: lines of comma-separated integers `X,Y,Z` (one junction box per line).
- Part 1: consider all pairs of boxes, compute Euclidean distance between them, sort all pairs by increasing distance, then take the first `N` pairs (where `N` is a configurable "merge limit"). For these `N` connections perform unions in that order using a DSU. After applying the `N` connections, compute the sizes of all connected components and return the product of the three largest component sizes.
- Part 2: continue processing connections in sorted order until all boxes are in one connected component. The last pair that actually reduced the component count is used to compute a product of the `X` coordinates of those two boxes.

This is effectively a Kruskal-like process where edges are considered in increasing order, but we stop early for Part 1 and keep going for Part 2.

---

## High-level approach used in this crate

1. `prepare(lines)` — parse input, build `Point` values and an `xs: Vec<i128>` of X coordinates, generate all `(distance, i, j)` pairs and sort them.
2. `prod_of_top_three(pairs, n, merge_limit)` — run DSU unions for the first `merge_limit` pairs and compute the top-3 component-size product.
3. `prod_of_last_con_x(pairs, xs, n)` — run DSU unions until exactly one component remains, returning the product of the X coordinates for the last union that merged two components.
4. `solve(lines, merge_limit)` — convenience wrapper that calls `prepare` once and delegates to the two part helper functions. This avoids duplicating parsing/sorting logic.

This design keeps the heavy O(n^2) work (pair generation + sorting) in one place and allows both parts to reuse the precomputed `pairs` and `xs` data.

---

## DSU (Disjoint Set Union) — concept and Rust implementation pattern

A DSU maintains a forest of trees where each node points to a parent. Each tree represents a connected component. The two important operations are:

- `find(x)`: return the root of `x`. With *path compression*, we update the parent pointers along the way so subsequent `find` calls are fast.
- `union(a, b)`: attach one tree to another (usually attach smaller tree to larger by size or rank). Return `true` if a merge happened (roots were different), `false` if already connected.

Rust pattern (short):

```rust
pub struct DisjointSetUnion {
    pub parent: Vec<usize>,
    pub size: Vec<usize>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> Self {
        Self { parent: (0..n).collect(), size: vec![1; n] }
    }

    // Path compression alters structure, so &mut self
    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root; // compress
            root
        } else {
            x
        }
    }

    // Union-by-size; returns true if merged (roots different)
    pub fn union(&mut self, a: usize, b: usize) -> bool {
        let ra = self.find(a);
        let rb = self.find(b);
        if ra == rb { return false; }
        if self.size[ra] < self.size[rb] {
            self.parent[ra] = rb;
            self.size[rb] += self.size[ra];
        } else {
            self.parent[rb] = ra;
            self.size[ra] += self.size[rb];
        }
        true
    }
}
```

Notes on the implementation above:
- `find` uses recursion and path compression. Because it mutates `parent`, it requires `&mut self`. If you prefer `&self`, you'd need a non-destructive `find` that does not compress, which is slower.
- `union` also takes `&mut self` since it mutates `parent` and `size`.
- We store `size` as `usize` and expose it for reporting component sizes.

### Complexity
DSU with path compression + union-by-size achieves near-constant amortized time per `find`/`union` (inverse-Ackermann), which is effectively constant for any realistic `n`.

---

## Module system and re-exports

In `lib.rs` we use the module system like this:

```rust
mod dsu; // include dsu.rs
pub use dsu::{Point, DisjointSetUnion};
```

This keeps `dsu.rs` as an internal module but re-exports `Point` and `DisjointSetUnion` so callers can use `day_eight::DisjointSetUnion` directly. The binary crate (`main.rs`) stays tiny and only handles CLI and I/O.

Why re-export?
- It provides a clean public API at the crate root. Tests and `main.rs` do not need to know internal file layout.

---

## Ownership & borrowing notes (why signatures look like they do)

- Parsing — `prepare(lines: &[String])` accepts a borrowed slice `&[String]`. We borrow because callers (tests or `main`) usually keep ownership of the `Vec<String>` and we don't want to move it when parsing.
- Passing around large vectors — `pairs` is `Vec<(f64, usize, usize)>`. After `prepare` returns it, we pass `&pairs` to `part1_from_pairs` and `part2_from_pairs` so they can iterate without taking ownership.
- DSU methods mutate their internal state; they therefore take `&mut self`.
- Returning `Option<T>` allows the function to express "no answer" (for example, empty input). `None` is returned when there is no meaningful numeric result.

---

## Why `Option` is used here

- `solve_lines` returns `(Option<usize>, Option<i128>)` so `main.rs` can print friendly messages when a part has no result (e.g., empty input). It also makes tests explicit by forcing callers to unwrap or assert on `Some`.
- Using `Result` would be appropriate for I/O or parse failures. For AoC puzzles with known-good input, `Option` is fine for indicating logical absence of a value.

---

## Practical tips in this codebase

- Use squared distance (`dx*dx + dy*dy + dz*dz`) if you only need ordering; avoids floating `sqrt` and tiny inaccuracies. We keep `f64` distances in pairs here for clarity.
- Sorting `Vec<(f64, usize, usize)>` uses `partial_cmp` for `f64`:
  ```rust
  pairs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
  ```
  This unwrap is safe as distances are finite and not NaN for valid input.
- Use `i128` for the `X` coordinate product to avoid overflow for unexpected puzzle inputs. Adjust to `i64` if you know the bounds are small.

---

## CLI / Running

From the `day_eight` directory you can run:

```powershell
cargo run -- input.txt 1000
```

- The binary expects two positional args: the input file path and an optional `merge_limit` (default used if missing).
- `main.rs` reads the file into `Vec<String>` then calls `day_eight::solve_lines(&lines, merge_limit)` and prints both part results, taking care to pretty-print `None`.

---

## Alternative approaches (when n is large)

The simple approach here enumerates all O(n^2) pairs, sorts them, then processes them. That's easy and robust, but it's O(n^2) memory/time for pair generation.

Below are several alternatives for performance or lower-memory usage.

### 1) Binary Heap streaming (keep only smallest K edges)

If you only need the first `K` smallest edges (Part 1 with small `merge_limit`), you can avoid sorting all O(n^2) edges. Two variants:

- Use a max-heap of size `K` to keep the current K smallest edges seen so far while streaming pairs; for each new edge, if it's smaller than the max, replace it.
- Or use a min-heap to lazily generate the next edge in increasing order by maintaining neighbor candidates for each vertex.

Pseudocode (max-heap keeping K smallest):

```text
heap = BinaryHeap::new(); // max-heap by distance
for i in 0..n {
  for j in i+1..n {
    d = distance_sq(i, j);
    if heap.len() < K { heap.push((d,i,j)); }
    else if d < heap.peek().distance { heap.pop(); heap.push((d,i,j)); }
  }
}
// heap contains K smallest edges in no particular order
```

This approach reduces memory to O(K) while still requiring O(n^2) time to examine pairs; useful if K << n^2 and memory is the bottleneck.

A more advanced min-heap approach (lazy K-smallest from a Cartesian product) can reduce time, but it is more complex.

### 2) KD-Tree / Spatial partitioning

For geometric nearest-neighbor problems, KD-trees or spatial hashing can find nearest neighbors in O(n log n) or similar, then expand to multiple neighbors. If the graph of interest is the Delaunay triangulation or something similar, you might avoid considering all pairs. Implementing KD-tree in Rust is non-trivial, but crates like `kiddo` or `kdtree` exist.

### 3) Partial selection / nth_element

If you only need the smallest `M` edges but still need them sorted, you can use a selection algorithm (`std::slice::select_nth_unstable_by`) to partition and then sort the smallest block. That can be faster than sorting the entire O(n^2) list when `M` is small.

### 4) Parallelization

Pair generation and distance computation are embarrassingly parallel. Use `rayon` to parallelize the pair generation and then merge results. Sorting can also be parallelized with `par_sort_by` from `rayon`.

---

## Example: how Part 1/Part 2 are split in the source

```rust
let (pairs, xs, n) = prepare(&lines);
let p1 = prod_of_top_three(&pairs, n, merge_limit);
let p2 = prod_of_last_con_x(&pairs, &xs, n);
// or use the convenience wrapper:
let (p1_alt, p2_alt) = solve(&lines, merge_limit);
```

This ensures `prepare` runs once and both parts reuse the same `pairs` and `xs` without duplication.

---

## Final notes / learning goals

- This problem is an excellent exercise in:
  - implementing a DSU in Rust and reasoning about `&mut self` and path compression;
  - working with floating point ordering safely;
  - using `Option` to express possibly-missing answers;
  - understanding Rust ownership and borrowing in a practical pipeline (parsing -> prepare -> parts).
- Once you understand this implementation, try replacing the pair-sorted pipeline with a streaming BinaryHeap or KD-tree and measure performance for larger inputs.

---

If you'd like, I can also:
- add a small benchmark harness under `day_eight/benches` to compare full pair-sort vs. heap streaming for configurable `n`;
- add a `README` section that shows the actual outputs for example/puzzle inputs and embed a tiny visual explanation of DSU unions.

Happy to continue — tell me which follow-up you'd like next.
