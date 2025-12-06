Day One — Learning notes

What I solved
- Implemented two parts that simulate dial movements and count when the dial hits zero.

Rust concepts used
- Parsing and ownership: input is read as a single string, converted to a `Vec<String>` with `input.lines().map(|s| s.to_string()).collect()` in `lib.rs`.
- Borrowing/slices: the core functions take `&[String]` so they operate on borrowed data.
- Pattern matching & control flow: `if let Ok((letter, number)) = ...` and `match` on a string to branch left/right.
- Numeric wrapping: use of `rem_euclid` to keep values in range.
- Error handling: simple error reporting with `eprintln!` on parse failures.
- Tests: small smoke tests under `#[cfg(test)]`.

Approach
- Parse each line into a (direction, steps) pair (reusing helpers from `aoc_common`).
- Use a dial position variable and update it per instruction, counting occurrences when it becomes zero.

Notes / study tips
- `rem_euclid` is handy for modular arithmetic with positive results.
- Passing `&[String]` avoids cloning for each call and demonstrates borrowing collections.
- `Cargo.toml` — project manifest. Run the project with Cargo.
- `src/main.rs` — the program entry point. Contains orchestration and I/O handling.
- `src/solutions.rs` — solution logic split out for clarity and testability.
- `src/utils.rs` — small helper functions (parsing, trimming, etc.).
- `example.txt` — small example input from the puzzle description.
- `input.txt` — my real puzzle input (keeps the leaderboard mystery alive).

## How to run (quick)
Open a terminal at the `day_one` folder and run:

```powershell
cargo run --release
```

If you want the faster edit/test cycle during development, use:

```powershell
cargo run
```

To run any tests (if present):

```powershell
cargo test
```


## Quick usage notes
- The program reads `input.txt` by default. Replace or edit it for new inputs.
- `example.txt` is useful for stepping through the logic with a tiny dataset.
- I prefer keeping I/O and parsing in small helpers in `utils.rs` so `solutions.rs` stays focused on problem logic.


## Next steps (mini roadmap)
- Add unit tests for tricky parsing edge cases, try TDD for next challenge.
- Add command-line flags to choose `example` vs `input` or to run specific parts.
- Continue with Day Two, keeping the same structure.