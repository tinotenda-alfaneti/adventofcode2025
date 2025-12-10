use day_ten::indicator_presses;
use day_ten::joltage_presses;

#[test]
fn example_indicator_lights() {
    let s = include_str!("../example.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = indicator_presses::solve(&lines);
    // For the 3 example machines:
    // Machine 1: [.##.] with 6 buttons -> optimal solution
    // Machine 2: [...#.] with 5 buttons -> optimal solution
    // Machine 3: [.###.#] with 4 buttons -> optimal solution
    // Total minimum presses: 7
    assert_eq!(
        result, 7,
        "Expected 7 total presses for indicator lights in example"
    );
}

#[test]
fn example_joltage_counters() {
    let s = include_str!("../example.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = joltage_presses::solve(&lines);
    // For the 3 example machines with joltage targets:
    // Machine 1: targets {3,5,4,7} -> ILP finds minimum presses
    // Machine 2: targets {7,5,12,7,2} -> ILP finds minimum presses
    // Machine 3: targets {10,11,11,5,10,5} -> ILP finds minimum presses
    // Total minimum presses: 33
    assert_eq!(
        result, 33,
        "Expected 33 total presses for joltage counters in example"
    );
}

#[test]
fn input_indicator_lights() {
    let s = include_str!("../input.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = indicator_presses::solve(&lines);
    assert_eq!(
        result, 571,
        "Expected 571 total presses for indicator lights"
    );
}

#[test]
fn input_joltage_counters() {
    let s = include_str!("../input.txt");
    let lines: Vec<String> = s.lines().map(|l| l.to_string()).collect();

    let result = joltage_presses::solve(&lines);
    assert_eq!(
        result, 20869,
        "Expected 20869 total presses for joltage counters"
    );
}
