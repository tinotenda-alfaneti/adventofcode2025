Day Five — Learning notes

What I solved
- Parse ranges and ids from a separated input, merge overlapping ranges, and compute counts and totals.

Rust concepts used
- Error handling with `anyhow`: `Result<T>` + `.with_context()` to add helpful error messages.
- Parsing with `split_once` and `parse::<u64>()`.
- Sorting and merging: `ranges.sort_by_key(|r| r.0)` and merging adjacent/overlapping intervals.
- Efficient searches: use of `partition_point` on slices to quickly locate range positions.
- Iterator adapters and `iter().map(...).sum()` for aggregation.

Approach
- Parse inputs into typed ranges and ids, sort and merge ranges, then answer queries by binary-search-like slice operations and iterators.

Notes / study tips
- `anyhow` is great for quick CLI/puzzle tools because it simplifies error propagation while letting you add context.
- `partition_point` is a handy slice primitive for binary-search-style queries on sorted data.
