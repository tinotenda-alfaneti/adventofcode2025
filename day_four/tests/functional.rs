use day_four as lib;
#[test]
fn simple_counts() {
    let lines = vec!["@@@".to_string(), "...".to_string(), "..@".to_string()];
    let (p1, p2) = lib::solve(lines.clone());
    assert_eq!(p1, 4);
    assert_eq!(p2, 4);
}

#[test]
fn removal_rounds() {
    let lines = vec!["@@@".to_string(), "...".to_string()];
    let (p1, p2) = lib::solve(lines.clone());
    assert_eq!(p1, 3);
    assert_eq!(p2, 3);
}
