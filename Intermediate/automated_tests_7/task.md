## Task: Write and run automated tests for a custom type in Rust

In this task, you will practice writing and running automated tests for a custom type in Rust. You will also learn how to use the `#[test]` attribute, the `assert!` and `assert_eq!` macros, and the `cargo test` command.

### Step 1: Define a custom type named `Rectangle` that has two fields: `width` and `height`

- Use the `struct` keyword to define a custom type named `Rectangle` that has two fields: `width` and `height`.
- Use the curly braces to enclose the fields of the `Rectangle` type and separate them by commas.
- Use the colon to specify the types of the fields. The types of the fields should be numeric, such as `u32` or `f64`.

### Step 2: Define a method named `area` for the `Rectangle` type that returns the area of the rectangle as the same type as the fields

- Use the `impl` keyword to define a method for the `Rectangle` type. You need to use the `Self` keyword to refer to the type that the method belongs to.
- Use the curly braces to enclose the method definition.
- Use the `fn` keyword to define a method named `area` that takes an immutable reference to `self` as a parameter and returns the same type as the fields. The `self` parameter refers to the `Rectangle` instance that calls the method.
- Use the asterisk operator to multiply the `width` and `height` fields of the `self` parameter and return the result.

### Step 3: Define a test module named `tests` that contains some test functions for the `Rectangle` type

- Use the `mod` keyword to define a test module named `tests` that contains some test functions for the `Rectangle` type.
- Use the `#[cfg(test)]` attribute to indicate that the test module should only be compiled and run when testing the code, not when building or running it.
- Use the curly braces to enclose the test functions of the test module.
- Use the `#[test]` attribute to mark each test function as a test. A test function should have no parameters and return nothing.
- Use the `assert!` and `assert_eq!` macros to check the expected and actual values in the test functions. The `assert!` macro takes a boolean expression and panics if it evaluates to false. The `assert_eq!` macro takes two values and panics if they are not equal.
- Use the `let` keyword to create some `Rectangle` instances and assign them some values in the test functions. For example, you can write `let rect1 = Rectangle { width: 10, height: 20 };`.
- Use the dot operator to call the `area` method on the `Rectangle` instances and compare the results with the expected values in the test functions. For example, you can write `assert_eq!(rect1.area(), 200);`.

### Step 4: Run the tests using the `cargo test` command and check the output

- Use the `cargo test` command to run the tests in the terminal. The command will compile and run the tests and print the results.
- Check the output of the command and see if all the tests passed or if any of them failed. If a test fails, the output will show the name of the test function, the line number where the failure occurred, and the expected and actual values.
- Fix any errors in the code or the tests and run the tests again until all of them pass.

### Expected output

Your code should compile and run without errors, and produce an output similar to this:

```rust
   Compiling rectangle v0.1.0 (/home/user/rectangle)
    Finished test [unoptimized + debuginfo] target(s) in 0.42s
     Running target/debug/deps/rectangle-8e1b9f4f8f7a4c4c

running 3 tests
test tests::test_area ... ok
test tests::test_zero_area ... ok
test tests::test_negative_area ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```