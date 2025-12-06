use day_two as lib;

#[test]
fn patterns_and_sum() {
    assert!(lib::has_repeated_pattern_twice("1212"));
    assert!(!lib::has_repeated_pattern_twice("123"));

    assert!(lib::has_more_than_two_repeated_patterns("1111"));
    assert!(lib::has_more_than_two_repeated_patterns("1212"));

    let input = "10-12,1212-1212";
    assert_eq!(lib::sum_invalid_ids(input, 2, false), 1223);
}
