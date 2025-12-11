use day_eleven::day_to_out::solve as solve_part_one;
use day_eleven::svr_to_out::solve as solve_part_two;

#[test]
fn example_part_one() {
    let s = include_str!("../example_p1.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = solve_part_one(&lines);

    assert_eq!(result, 5, "Expected 5 total paths from day to out");
}

#[test]
fn example_part_two() {
    let s = include_str!("../example_p2.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = solve_part_two(&lines);

    assert_eq!(
        result, 2,
        "Expected 2 paths from svr to out that visit both dac and fft"
    );
}
