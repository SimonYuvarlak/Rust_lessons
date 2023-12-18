/// Adds two numbers.
///
/// # Examples
///
/// use automated_tests_7::add;
///
/// assert_eq!(add(2, 3), 5);
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

/// Greets a user.
///
/// # Examples
///
/// use automated_test_7::greet;
///
/// assert_eq!(greet("Alice"), "Hello, Alice!");
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Bob"), "Hello, Bob!");
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    #[cfg(feature = "slow")]
    fn test_slow() {
        // some slow test
    }
}