# Error Handling

Error handling is an essential skill for any programmer, as it allows us to deal with unexpected situations and prevent our programs from crashing or behaving incorrectly. Rust has a powerful and expressive error handling system that uses two main types of errors: recoverable and unrecoverable. In this tutorial, I will explain what these types of errors are, how to handle them, and how to create and propagate custom errors.

## Recoverable Errors

Recoverable errors are errors that can be handled gracefully and allow the program to continue running. For example, trying to open a file that does not exist is a recoverable error, as we can choose to create the file, use a default file, or report the error to the user.

Rust uses the `Result` enum to represent the outcome of an operation that may fail. The `Result` enum has two variants: `Ok` and `Err`. The `Ok` variant contains the value of the successful operation, and the `Err` variant contains the error of the failed operation. For example, the `std::fs::File::open` function returns a `Result<File, Error>`, where `File` is the type of the file handle, and `Error` is the type of the error.

To handle a `Result` value, we can use several methods and operators, such as:

- `match`: a control flow construct that allows us to match the `Result` value against different patterns, and execute different blocks of code depending on the variant. For example, we can use the `match` statement to handle the result of opening a file:

```rust
use std::fs::File;

// Try to open a file and assign the result to a variable
let result = File::open("hello.txt");

// Use the match statement to handle the result
match result {
    // If the result is Ok, bind the file handle to a variable named file
    Ok(file) => {
        // Do something with the file
        println!("The file is {:?}", file);
    }
    // If the result is Err, bind the error to a variable named error
    Err(error) => {
        // Do something with the error
        println!("The error is {:?}", error);
    }
}
```

- `unwrap`: a method that returns the value inside the `Ok` variant if the result is `Ok`, or panics if the result is `Err`. A panic is a type of unrecoverable error that causes the program to terminate and print a message to the standard error. The `unwrap` method is useful when we are confident that the operation will not fail, or when we want the program to crash if it does. For example, we can use the `unwrap` method to get the file handle or panic:

```rust
use std::fs::File;

// Try to open a file and unwrap the result
let file = File::open("hello.txt").unwrap();

// Do something with the file
println!("The file is {:?}", file);
```

- `expect`: a method that is similar to `unwrap`, but allows us to provide a custom message that will be printed if the result is `Err`. The `expect` method is useful when we want to provide more information about the error and the context of the operation. For example, we can use the `expect` method to get the file handle or panic with a message:

```rust
use std::fs::File;

// Try to open a file and expect the result
let file = File::open("hello.txt").expect("Failed to open hello.txt");

// Do something with the file
println!("The file is {:?}", file);
```

- `?`: an operator that returns the value inside the `Ok` variant if the result is `Ok`, or returns the error inside the `Err` variant to the calling function if the result is `Err`. The `?` operator is useful when we want to propagate the error to the caller, and let them decide how to handle it. For example, we can use the `?` operator to read a file and return the result:

```rust
use std::fs::File;
use std::io::Read;
use std::io::Error;

// Define a function that takes a file name and returns a Result<String, Error>
fn read_file(file_name: &str) -> Result<String, Error> {
    // Try to open the file and use the ? operator to propagate the error
    let mut file = File::open(file_name)?;
    // Create an empty string
    let mut s = String::new();
    // Try to read the file contents into the string and use the ? operator to propagate the error
    file.read_to_string(&mut s)?;
    // Return the string as Ok
    Ok(s)
}

// Call the read_file function and handle the result
match read_file("hello.txt") {
    // If the result is Ok, print the contents
    Ok(contents) => println!("The file contents are {}", contents),
    // If the result is Err, print the error
    Err(error) => println!("The error is {:?}", error),
}
```

## Unrecoverable Errors

Unrecoverable errors are errors that cannot be handled gracefully and require the program to terminate. For example, trying to access an invalid memory location is an unrecoverable error, as there is no way to recover from it.

Rust uses the `panic!` macro to trigger a panic, which is a type of unrecoverable error that causes the program to terminate and print a message to the standard error. The `panic!` macro can take a custom message as an argument, or use a default message. For example, we can use the `panic!` macro to cause a panic with a message:

```rust
// Cause a panic with a message
panic!("Something went wrong");
```

We can also cause a panic by using the `unwrap` or `expect` methods on a `Result` value that is `Err`, as we saw in the previous section. For example, we can cause a panic by trying to open a file that does not exist and using the `unwrap` method:

```rust
use std::fs::File;

// Try to open a file that does not exist and unwrap the result
let file = File::open("hello.txt").unwrap(); // this will panic
```

When a panic occurs, Rust will try to clean up the resources that the program was using, such as closing files and freeing memory. This process is called unwinding, and it can be expensive and complex. Alternatively, we can choose to abort the program immediately, without unwinding. This can make the program faster and smaller, but it can also leave some resources in an inconsistent state. To choose the behavior of the program when a panic occurs, we can use the `panic` key in the `Cargo.toml` file, and set it to `unwind` or `abort`. For example, we can set the `panic` key to `abort` to make the program abort on panic:

```toml
# Cargo.toml
[profile.release]
panic = 'abort'
```

## Custom Errors

Sometimes, we may want to create and use our own types of errors, instead of relying on the standard library or other crates. This can give us more control and flexibility over the error handling process, and allow us to express the semantics and context of our errors more clearly.

To create a custom error type, we need to implement the `std::error::Error` trait, which is the standard trait for all types of errors in Rust. The `Error` trait has two required methods: `source` and `fmt`, and one optional method: `description`. The `source` method returns the underlying cause of the error, if any. The `fmt` method formats the error for display, using the `std::fmt::Display` and `std::fmt::Debug` traits. The `description` method returns a short description of the error, but it is deprecated and should not be used.

To implement the `Error` trait, we can use the `derive` attribute to automatically implement the `Display` and `Debug` traits, and then define the `source` method manually. For example, we can create a custom error type named `MathError` that represents an error in a mathematical operation:

```rust
use std::error::Error;
use std::fmt;

// Define a custom error type named MathError
#[derive(Debug)]
struct MathError {
    // Add a field named message that contains a string
    message: String,
}

// Implement the Display trait for MathError
impl fmt::Display for MathError {
    // Define the fmt method that takes a formatter and returns a Result
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write the message to the formatter
        write!(f, "{}", self.message)
    }
}

// Implement the Error trait for MathError
impl Error for MathError {
    // Define the source method that returns the underlying cause of the error
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        // Return None, as there is no underlying cause
        None
    }
}
```

To use a custom error type, we can create and return an instance of it from a function that may fail, using the `Err` variant of the `Result` enum. For example, we can create a function named `div` that takes two integers and returns their quotient, or a `MathError` if the second integer is zero:

```rust
// Define a function named div that takes two integers and returns a Result<i32, MathError>
fn div(x: i32, y: i32) -> Result<i32, MathError> {
    // Check if the second integer is zero
    if y == 0 {