# Advent of Code (Rust)

Welcome to my Advent of Code playground — a tiny, slightly nerdy, learning-focused Rust project.

This repo is where I'm practicing Rust by solving Advent of Code puzzles. I built it as a hands-on way to learn the language, rustacean-style (with safety, speed, and a little whimsy). The code here is intentionally simple and incremental: experiments, notes, and small wins live side-by-side.

## Why this project (aka: learning goals)
- Get comfy with Rust's ownership, borrowing, and lifetimes.
- Practice reading files and parsing text idiomatically.
- Organize code into small, testable modules.
- Learn how to use `cargo run`, `cargo test`, and `cargo fmt`.
- Grow an intuition for iterator combinators and pattern matching.

## A tiny, whimsical license
Do whatever you like with this code. If it helps you learn, it's already done its job.

---

Happy hacking, and may your borrow checker be generous!

- Me (learning Rust, one puzzle at a time)
 
## Repo layout and how to run

This repository contains one small crate per puzzle (folders named `day_one`, `day_two`, etc.) plus a shared crate `aoc_common` used for helpers. Each day is a tiny, focused crate with its own `Cargo.toml` and small test harnesses.

This layout is perfectly fine and common for personal learning projects. The added per-day README files document the approach used and the Rust concepts in each solution so they serve as a study notebook.

How to run a specific day (options):

- From the repo root (if you're using a Cargo workspace with members), you can run a specific package with:

	cargo run -p day_one

- Or change into the day's folder and run directly:

	cd day_one; cargo run

Running tests for a day:

	cd day_one; cargo test
