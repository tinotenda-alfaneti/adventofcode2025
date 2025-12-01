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