# Day Seven — Tachyon Manifold (learning notes)

Short notes and reminders about how I solved Day 7 and what I learned in Rust while doing it.

Problem summary
- A beam enters the grid at `S` and travels downwards.
- A splitter `^` stops the beam and emits new beams to the immediate left and right (on the same row).
- Part 1: Count how many splitters are encountered when all beams finish.
- Part 2: Interpret each split as branching timelines and count how many timelines exist after all journeys complete.

Solution approach
- Part 1 (count_splits): simulate beams row-by-row as a set of active columns. When a beam hits `^` increment split count and emit left/right beams. Use `HashSet<usize>` per row so overlapping beams merge naturally.
- Part 2 (count_timelines): dynamic programming by keeping a `Vec<u128>` of counts for each column for the current row. When encountering `^` split the count to left and right columns for the next row. Use `u128` to avoid overflow from exponential splits.

Rust notes and learnings
- Read grid into `Vec<Vec<char>>` using `String::chars().collect()`.
- Use `HashSet` to deduplicate beam columns when simulating many beams in part 1. Insert/contains are cheap and clarify intent.
- Use `Vec<u128>` for numeric DP in part 2; Rust's integer types provide `saturating_add` to avoid accidental overflows if you want to be defensive.
- Small helper functions make code cleaner: `find_start` used by both parts.
- Prefer simple loops (for rows, for columns) over heavy iterator chains when the logic is stateful or imperative.

How to run
- Run the example (prints output from `main.rs`):

  cargo run --manifest-path day_seven/Cargo.toml

- Run unit/functional tests for this crate:

  cargo test -p day_seven

Notes and next steps
- Could further generalize the row-by-row traversal into a reusable iterator/visitor if future days need similar patterns.
- Add more unit tests for edge cases (splitters on edges, `S` at bottom row).
