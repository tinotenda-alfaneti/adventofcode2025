#[test]
fn merge_and_solve_smoke() {
    let input = vec![
        "1-3".to_string(),
        "5-5".to_string(),
        "".to_string(),
        "2".to_string(),
        "5".to_string(),
        "6".to_string(),
    ];

    let (fresh_count, total) = day_five::solve(input).expect("solve failed");
    assert_eq!(total, 4);
    assert_eq!(fresh_count, 2);
}

#[test]
fn merged_properties() {
    let ranges = vec![(1, 3), (2, 6), (8, 9)];
    let merged = day_five::merge_ranges(ranges);
    assert_eq!(merged, vec![(1, 6), (8, 9)]);
}
