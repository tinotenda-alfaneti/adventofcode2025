Day Four — Learning notes

What I solved
- Parse a grid of characters and compute accessibility and iterative removals of accessible cells.

Rust concepts used
- Nested collections: `Vec<Vec<char>>` to represent a 2D grid.
- Indexing and bounds checks: careful conversions between `usize` and `isize` for neighbor checks.
- Iteration over grid coordinates with `for r in 0..h { for c in 0..w { ... } }`.
- Mutating a shared grid: building `to_remove` lists and then mutating `map[r][c] = '.'`.
- Unit tests for small grids.

Approach
- Parse input lines into a 2D character matrix and inspect neighbors for each cell to decide accessibility.
- Iterate rounds of removals until no accessible cells remain.

Notes / study tips
- Using `Vec<Vec<char>>` is convenient and straightforward. For very large grids consider contiguous storage (one Vec) for performance.
- Keep careful track of index conversions and avoid panics by checking bounds before indexing.
