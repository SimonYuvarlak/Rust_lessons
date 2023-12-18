## Task: Handle and create errors in Rust

In this task, you will practice handling and creating errors in Rust. You will also learn how to use the `Result` enum, the `panic!` macro, and the `Error` trait.

### Step 1: Define a function named `read_number` that takes a file name and returns a `Result<i32, Error>`

- Use the `fn` keyword to define a function named `read_number` that takes a string slice as a parameter and returns a `Result<i32, Error>`.
- Use the `use` keyword to import the `File`, `Read`, and `Error` types from the `std::fs` and `std::io` modules.
- Use the `File::open` function to try to open the file with the given name and assign the result to a variable.
- Use the `?` operator to propagate the error if the result is `Err`.
- Create an empty string and assign it to a variable.
- Use the `read_to_string` method to try to read the file contents into the string and assign the result to a variable.
- Use the `?` operator to propagate the error if the result is `Err`.
- Use the `trim` method to remove any whitespace from the string.
- Use the `parse` method to try to convert the string into a number and return the result.

### Step 2: Define a custom error type named `MathError` that represents an error in a mathematical operation

- Use the `struct` keyword to define a custom error type named `MathError` that has a field named `message` of type `String`.
- Use the `derive` attribute to automatically implement the `Debug` trait for `MathError`.
- Use the `impl` keyword to implement the `Display` trait for `MathError`.
- Define the `fmt` method that takes a formatter and returns a `Result`.
- Use the `write!` macro to write the message to the formatter.
- Use the `impl` keyword to implement the `Error` trait for `MathError`.
- Define the `source` method that returns the underlying cause of the error, which is `None` in this case.

### Step 3: Define a function named `div` that takes two integers and returns a `Result<i32, MathError>`

- Use the `fn` keyword to define a function named `div` that takes two integers as parameters and returns a `Result<i32, MathError>`.
- Check if the second integer is zero.
- If it is, return a `MathError` with a message that says "Cannot divide by zero".
- If it is not, return the quotient of the two integers as `Ok`.

### Step 4: Test your code by calling the `read_number` and `div` functions and handling their results

- Use the `let` keyword to create some variables and assign them the results of calling the `read_number` and `div` functions with some arguments. For example, you can write `let x = read_number("number.txt");`.
- Use the `match` statement to handle the results of the functions. For example, you can write `match x { Ok(n) => println!("The number is {}", n), Err(e) => println!("The error is {}", e) };`.
- Use the `unwrap` or `expect` methods to get the values or panic if the results are `Err`. For example, you can write `let y = div(10, 0).unwrap();`.
- Use the `?` operator to propagate the errors to the main function. For example, you can write `let z = div(10, 2)?;`.

### Expected output

Your code should compile and run without errors, and produce an output similar to this:

```rust
The number is 42
The error is Cannot divide by zero
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: MathError { message: "Cannot divide by zero" }', src\main.rs:52:29
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
The quotient is 5
```