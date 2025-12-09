use day_nine as lib;

#[test]
fn input_answers() {
    let s = include_str!("../input.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let (part1, part2) = lib::solve(&lines);
    assert_eq!(part1, 4743645488);
    assert_eq!(part2, 1529011204);
}
