use automated_tests_7::greet;

#[test]
fn test_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}