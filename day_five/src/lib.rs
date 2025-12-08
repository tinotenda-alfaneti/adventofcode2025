use anyhow::{Context, Result, anyhow};

type Parsed = (Vec<(u64, u64)>, Vec<u64>);

/// Parse the input lines into a list of ranges and a list of ids.
pub fn parse_input(lines: Vec<String>) -> Result<Parsed> {
    let blank_index = lines
        .iter()
        .position(|l| l.trim().is_empty())
        .ok_or_else(|| anyhow!("input missing blank separator line"))?;

    let range_lines = &lines[..blank_index];
    let id_lines = &lines[blank_index + 1..];

    let mut ranges = Vec::with_capacity(range_lines.len());
    for line in range_lines {
        let (start_s, end_s) = line
            .split_once('-')
            .with_context(|| format!("invalid range line: '{}'", line))?;
        let start = start_s
            .trim()
            .parse::<u64>()
            .with_context(|| format!("invalid start in line: '{}'", line))?;
        let end = end_s
            .trim()
            .parse::<u64>()
            .with_context(|| format!("invalid end in line: '{}'", line))?;
        ranges.push((start, end));
    }

    let mut ids = Vec::with_capacity(id_lines.len());
    for s in id_lines {
        let id = s
            .trim()
            .parse::<u64>()
            .with_context(|| format!("invalid id line: '{}'", s))?;
        ids.push(id);
    }

    Ok((ranges, ids))
}

pub fn merge_ranges(mut ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged: Vec<(u64, u64)> = Vec::new();

    for (s, e) in ranges.into_iter() {
        if let Some(last) = merged.last_mut()
            && s <= last.1
        {
            last.1 = last.1.max(e);
            continue;
        }
        merged.push((s, e));
    }

    merged
}

pub fn is_fresh(merged: &[(u64, u64)], id: u64) -> bool {
    if merged.is_empty() {
        return false;
    }

    let i = merged.partition_point(|&(s, _)| s <= id);

    if i == 0 {
        return false;
    }

    let (_, e) = merged[i - 1];
    id <= e
}

pub fn get_fresh_count(merged: &[(u64, u64)], ids: &[u64]) -> usize {
    ids.iter().filter(|&&id| is_fresh(merged, id)).count()
}

pub fn get_total_fresh(merged: &[(u64, u64)]) -> u64 {
    merged.iter().map(|&(s, e)| e - s + 1).sum()
}

pub fn solve(lines: Vec<String>) -> Result<(usize, u64)> {
    let (ranges, ids) = parse_input(lines)?;
    let merged = merge_ranges(ranges);
    let fresh_count = get_fresh_count(&merged, &ids);
    let total = get_total_fresh(&merged);
    Ok((fresh_count, total))
}
