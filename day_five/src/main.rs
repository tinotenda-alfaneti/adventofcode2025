use aoc_common as common;
fn main() {
    let file_path = "input.txt";
    let lines = match common::read_file_to_vec(file_path) {
        Ok(lines) => lines,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            return;
        }
    };
    let (mut ranges, id_lines) = get_ranges_and_ids(lines);

    let merged = merge_ranges(&mut ranges);

    let fresh_count = get_fresh_count(&merged, &id_lines);

    let total_fresh_ids: u64 = get_total_fresh(&merged);

    println!("Part 1 - Total fresh count: {}", fresh_count);
    println!("Part 2 - Total fresh ingredient IDs: {}", total_fresh_ids);



}

fn get_fresh_count(merged: &[(u64, u64)], id_lines: &[String]) -> usize {
    let mut fresh_count = 0;
    for line in id_lines {
        let id: u64 = line.parse().unwrap();
        if is_fresh(&merged, id) {
            fresh_count += 1;
        }
    }
    fresh_count
}

fn get_ranges_and_ids(lines: Vec<String>) -> (Vec<(u64, u64)>, Vec<String>) {
    let blank_index = lines.iter().position(|l| l.trim().is_empty()).unwrap();

    let range_lines = &lines[..blank_index];

    let id_lines = &lines[blank_index + 1..];

    let ranges: Vec<(u64, u64)> = range_lines
    .iter()
    .map(|line| {
        let (start, end) = line.split_once('-').unwrap();
        (start.parse().unwrap(), end.parse().unwrap())
    })
    .collect();

    (ranges, id_lines.to_vec())
}

fn get_total_fresh(merged: &[(u64, u64)]) -> u64 {
    merged.iter().map(|&(s, e)| e - s + 1).sum()
}

// binary search to see if id is in any of the merged ranges
fn is_fresh(merged: &[(u64, u64)], id: u64) -> bool {
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


fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    if ranges.is_empty() {
        return vec![];
    }

    ranges.sort_by_key(|r| r.0);

    let mut merged = vec![ranges[0]];
    for &(start, end) in &ranges[1..] {
        let last = merged.last_mut().unwrap();
        if start <= last.1 {
            last.1 = last.1.max(end);
        } else {
            merged.push((start, end));
        }
    }

    merged

}
