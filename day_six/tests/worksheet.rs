use day_six as lib;
#[test]
fn grouping_and_parse_right_to_left() {
    let s = include_str!("../example.txt");
    let mut lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let ws = lib::worksheet::Worksheet::from_mut(&mut lines);
    let groups = ws.column_groups();
    assert_eq!(groups.len(), 4);

    let mut results = Vec::new();
    for g in groups {
        let p = g
            .parse_problem(&lib::worksheet::RightToLeft)
            .expect("parse failed");
        results.push(p.result());
    }

    assert_eq!(results, vec![8544, 625, 3253600, 1058]);
    assert_eq!(results.iter().sum::<i64>(), 3263827);
}
