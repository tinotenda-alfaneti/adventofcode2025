Day Six — Learning notes

What I solved
- A worksheet-style parser where problems are parsed from groups of lines and solved left-to-right and right-to-left.

Rust concepts used
- Module organization: `mod`, `pub mod`, and splitting functionality across `part_one.rs`, `part_two.rs`, and `worksheet.rs`.
- Passing mutable references: `Worksheet::from_mut(lines)` takes `&mut Vec<String>` so parsing can mutate or reuse buffers.
- Error handling with `anyhow::Result` and propagating errors from parsing.
- Small use of traits / strategy (LeftToRight / RightToLeft) in parsing to alter behavior.

Approach
- Encapsulate worksheet parsing into its own module and reuse the parsing across both parts with a strategy parameter (direction).

Notes / study tips
- Splitting related logic into modules helps keep `part_one` and `part_two` files small and focused.
- Passing `&mut Vec<String>` can be more efficient when you want to reuse or mutate input lines.
