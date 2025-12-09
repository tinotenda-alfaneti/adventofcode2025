use day_nine as lib;

#[test]
fn example_rectangle_area() {
    let s = include_str!("../example.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let (res, res2) = lib::solve(&lines);
    assert_eq!(res, 50);
    assert_eq!(res2, 24);
}
