# Automated Tests

Rust is a programming language that emphasizes performance, reliability, and productivity. It has a built-in testing framework that allows you to write and run automated tests for your code. In this tutorial, I will show you how to use the Rust testing framework to write unit tests, documentation tests, and integration tests. I will also introduce some tools and techniques that can help you improve your testing experience and code quality.

## Unit Tests

Unit tests are the simplest and most common type of tests in Rust. They are used to verify the functionality of individual components or modules of your code. You can write unit tests in the same file as the code you are testing, or in a separate file under the `tests` directory. To write a unit test, you need to use the `#[test]` attribute to mark a function as a test, and the `assert!` or `assert_eq!` macros to check the expected behavior of your code. For example, suppose you have a function that adds two numbers:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

You can write a unit test for this function in the same file as follows:

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
```

The `#[cfg(test)]` attribute tells the compiler to only compile and run the code inside the `mod tests` block when you run `cargo test`. The `use super::*` statement imports the items from the parent module, in this case, the `add` function. The `#[test]` attribute marks the `test_add` function as a test, and the `assert_eq!` macro checks that the result of calling `add(2, 3)` is equal to `5`.

To run the unit tests, you can use the `cargo test` command in the terminal. This will compile and execute all the tests in your project, and report the results. You should see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The output shows that one test was run, and it passed. If the test fails, you will see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.68s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... FAILED

failures:

---- tests::test_add stdout ----
thread 'tests::test_add' panicked at 'assertion failed: `(left == right)`
  left: `6`,
 right: `5`', src/lib.rs:10:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    tests::test_add

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--lib'
```

The output shows that the test failed, and it gives you the line number and the message of the assertion that failed. You can also see the values of the left and right operands of the assertion, which can help you debug the problem. You can also use the `RUST_BACKTRACE=1` environment variable to get more information about the error.

## Documentation Tests

Documentation tests are a way of writing tests in the documentation comments of your code. They are useful for verifying that the examples in your documentation are correct and up-to-date. You can write documentation tests by using the `///` or `//!` syntax to create a documentation comment, and enclosing the code you want to test in a code block with the `rust` annotation. For example, you can add a documentation test for the `add` function like this:

```rust
// src/lib.rs
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// use testing_rust::add;
///
/// assert_eq!(add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

The documentation comment starts with `///`, and contains a brief description of the function, followed by a section with the `# Examples` heading. The code block starts with ````rust`, and contains the code you want to test, in this case, an import statement and an assertion. The code block ends with ````.

To run the documentation tests, you can use the same `cargo test` command as before. This will run both the unit tests and the documentation tests in your project. You should see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/lib.rs (target/debug/deps/lib-9f6b7c8a9d1f9f9c)

running 1 test
test src/lib.rs - add (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The output shows that both the unit test and the documentation test passed. If the documentation test fails, you will see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.69s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/lib.rs (target/debug/deps/lib-9f6b7c8a9d1f9f9c)

running 1 test
test src/lib.rs - add (line 3) ... FAILED

failures:

---- src/lib.rs - add (line 3) stdout ----
Test executable failed (exit code 101).

stderr:
thread 'main' panicked at 'assertion failed: `(left == right)`
  left: `6`,
 right: `5`', src/lib.rs:10:1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    src/lib.rs - add (line 3)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '--doc'
```

The output shows that the documentation test failed, and it gives you the line number and the message of the assertion that failed. You can also see the values of the left and right operands of the assertion, which can help you debug the problem. You can also use the `RUST_BACKTRACE=1` environment variable to get more information about the error.

## Integration Tests

Integration tests are used to test how different parts of your code work together, or how your code interacts with external systems or libraries. You can write integration tests in separate files under the `tests` directory in your project root. Unlike unit tests, you don't need to use the `#[cfg(test)]` attribute or the `mod tests` block to write integration tests. You also don't need to use the `use super::*` statement to import the items from your library, instead, you can use the `use` statement with the name of your crate. For example, suppose you have a function that greets a user:

```rust
// src/lib.rs
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}
```

You can write an integration test for this function in a separate file under the `tests` directory as follows:

```rust
// tests/greet.rs
use testing_rust::greet;

#[test]
fn test_greet() {
    assert_eq!(greet("Alice"), "Hello, Alice!");
}
```

The file name `greet.rs` is arbitrary, you can name it anything you want. The `use testing_rust::greet` statement imports the `greet` function from your library, which is named `testing_rust` in this example. The `#[test]` attribute marks the `test_greet` function as a test, and the `assert_eq!` macro checks that the result of calling `greet("Alice")` is equal to `"Hello, Alice!"`.

To run the integration tests, you can use the same `cargo test` command as before. This will run all the tests in your project, including the unit tests, the documentation tests, and the integration tests. You should see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.70s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/lib.rs (target/debug/deps/lib-9f6b7c8a9d1f9f9c)

running 1 test
test src/lib.rs - add (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/greet-9f6b7c8a9d1f9f9c

running 1 test
test test_greet ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

The output shows that all the tests passed. If the integration test fails, you will see something like this:

```text
$ cargo test
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.70s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/lib.rs (target/debug/deps/lib-9f6b7c8a9d1f9f9c)

running 1 test
test src/lib.rs - add (line 3) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/greet-9f6b7c8a9d1f9f9c

running 1 test
test test_greet ... FAILED

failures:

---- test_greet stdout ----
thread 'test_greet' panicked at 'assertion failed: `(left == right)`
  left: `"Hello, Alice!"`,
 right: `"Hi, Alice!"`', tests/greet.rs:6:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    test_greet

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass '-p testing-rust --test greet'
```

The output shows that the integration test failed, and it gives you the line number and the message of the assertion that failed. You can also see the values of the left and right operands of the assertion, which can help you debug the problem. You can also use the `RUST_BACKTRACE=1` environment variable to get more information about the error.

## Tools and Techniques

In this section, I will introduce some tools and techniques that can help you improve your testing experience and code quality in Rust. These are not mandatory, but they can be useful in some situations.

### Test Organization

You can organize your tests into submodules, groups, or categories by using the `mod` keyword and the `#[cfg(test)]` attribute. For example, you can create a submodule for unit tests and another one for integration tests in the same file as follows:

```rust
// src/lib.rs
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

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
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
    }
}
```

This way, you can separate your tests by their type or purpose, and make your code more organized and readable. You can also use the `mod` keyword to create submodules within submodules, and nest them as much as you want.

You can also use the `#[test]` attribute to create custom groups or categories for your tests, by using the `#[cfg(feature = "name")]` syntax. For example, you can create a feature called `slow` for tests that take a long time to run, and mark them with the `#[cfg(feature = "slow")]` attribute. Then, you can use the `--features` flag to enable or disable the feature when running the tests. For example, you can write a slow test like this:

```rust
// src/lib.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "slow")]
    fn test_slow() {
        // some slow test
    }
}
```

Then, you can run the test with the `--features slow` flag, or skip it with the `--no-default-features` flag. For example:

```text
$ cargo test --features slow
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 2 tests
test tests::test_add ... ok
test tests::test_slow ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

$ cargo test --no-default-features
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

This way, you can create custom features for your tests, and enable or disable them as you wish. You can also use the `--all-features` flag to run all the tests with all the features enabled.

### Test Filtering

You can filter your tests by name or pattern by using the `--test` or `--tests` flags. For example, you can run only the tests that contain the word `add` in their name by using the `--test add` flag. For example:

```text
$ cargo test --test add
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 1 test
test tests::test_add ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

You can also use the `--tests` flag to run multiple tests that match a pattern. For example, you can run all the tests that start with `test_` by using the `--tests test_*` flag. For example:

```text
$ cargo test --tests test_*
   Compiling testing-rust v0.1.0 (/home/user/testing-rust)
    Finished test [unoptimized + debuginfo] target(s) in 0.71s
     Running unittests (target/debug/deps/testing_rust-9f6b7c8a9d1f9f9c)

running 2 tests
test tests::test_add ... ok
test tests::test_slow ... ok

test result: ok. 2 passed; 0 failed;