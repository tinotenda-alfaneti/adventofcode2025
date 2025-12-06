use day_three::{largest_n_digit, largest_two_digit};

#[test]
fn test_largest_two_digit_basic() {
    assert_eq!(largest_two_digit("3052"), 52);
    assert_eq!(largest_two_digit("ab1c2"), 12);
    assert_eq!(largest_two_digit("9"), 0);
}

#[test]
fn test_largest_n_digit_basic() {
    assert_eq!(largest_n_digit("123456", 3).unwrap(), 456);
    assert_eq!(largest_n_digit("7654321", 4).unwrap(), 7654);
    assert_eq!(largest_n_digit("42", 5).unwrap(), 42);
}

#[test]
fn test_largest_n_digit_errors() {
    assert!(largest_n_digit("1234", 0).is_err());
}
