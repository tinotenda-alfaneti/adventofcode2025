use anyhow::Result;
use crate::worksheet::{Worksheet, LeftToRight};

pub fn solve(lines: &mut Vec<String>) -> Result<i64> {
    let ws = Worksheet::from_mut(lines);
    let groups = ws.column_groups();

    let mut grand_total = 0_i64;

    for group in groups {
        let problem = group.parse_problem(&LeftToRight)?;
        grand_total += problem.result();
    }

    Ok(grand_total)
}