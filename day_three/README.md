Day Three — Learning notes

What I solved
- Find the largest numeric value by selecting n digits out of a string (greedy stack algorithm).

Rust concepts used
- Iterators and char handling: `s.chars().filter(|c| c.is_ascii_digit())` and `collect()` to build a `Vec<char>`.
- Error handling and `Result`: `largest_n_digit` returns `Result<u64, ParseIntError>`.
- Using a `Vec` as a stack: push/pop operations to implement the greedy algorithm.
- Capacity hints: `Vec::with_capacity` to preallocate space where useful.

Approach
- Extract digit characters into a buffer, then apply a greedy stack-based algorithm that removes smaller digits to keep the largest possible number of length `keep`.

Notes / study tips
- This is a good example of combining iterator adapters with manual mutable state (the stack) to implement a linear-time greedy algorithm.
- Returning `Result` lets callers choose how to handle parse errors; unwrap only in tests or controlled places.
