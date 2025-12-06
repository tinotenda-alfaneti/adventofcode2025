#[test]
fn simple_counts() {
    let lines = vec!["@@@".to_string(), "...".to_string(), "..@".to_string()];
    let (p1, p2) = day_four::solve(lines.clone());
    assert_eq!(p1, 4);
    // p2 should remove the three top '@' and the bottom-right separately
    assert_eq!(p2, 4);
}

#[test]
fn removal_rounds() {
    // single row of three '@'s removed in one round
    let lines = vec!["@@@".to_string(), "...".to_string()];
    let (p1, p2) = day_four::solve(lines.clone());
    assert_eq!(p1, 3);
    assert_eq!(p2, 3);
}
