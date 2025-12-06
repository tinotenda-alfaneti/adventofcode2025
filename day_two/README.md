Day Two — Learning notes

What I solved
- Validate and sum IDs across numeric ranges with two different rulesets.

Rust concepts used
- String parsing: splitting strings and parsing with `parse::<i64>()` (and handling parse errors).
- Control flow and defensive parsing: `match` on `parse()` results, skipping malformed ranges.
- Numeric ranges and iteration: use of `for id in lower..=upper`.
- Helper functions and unit tests: `has_repeated_pattern_twice` and `has_more_than_two_repeated_patterns` with focused unit tests.

Approach
- Parse CSV-like input into range bounds and iterate each integer in a range.
- For each id, run a validation predicate and accumulate results using `saturating_add` to avoid overflow surprises.

Notes / study tips
- Prefer returning `Result` from parsing utilities for clearer error propagation; current approach is fine for robustness in puzzle-solving.
- Small helper functions with unit tests make reasoning about edge cases simpler.
