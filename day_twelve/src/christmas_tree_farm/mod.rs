pub fn solve(lines: &[String]) -> usize {
    inner::run(lines)
}

mod inner {
    include!("lib.rs");
}

// Re-export the public functions from the inner module for testing
#[allow(unused_imports)]
pub use inner::{can_fit_by_area, count_hashes, parse_input};
