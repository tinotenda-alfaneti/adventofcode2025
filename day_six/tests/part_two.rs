#[test]
fn example_part_two() {
    // load example.txt bundled with the crate
    let s = include_str!("../example.txt");
    let mut lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let res = day_six::part_two::solve(&mut lines).expect("part two failed");
    assert_eq!(res, 3263827);
}
