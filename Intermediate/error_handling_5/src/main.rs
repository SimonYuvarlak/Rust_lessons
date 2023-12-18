// Import the standard library modules
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

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

// Define a function named div that takes two integers and returns a Result<i32, MathError>
fn div(x: i32, y: i32) -> Result<i32, MathError> {
    // Check if the second integer is zero
    if y == 0 {
        // Return a MathError with a message
        Err(MathError {
            message: "Cannot divide by zero".to_string(),
        })
    } else {
        // Return the quotient as Ok
        Ok(x / y)
    }
}

// Define a collection named PrimeSet that implements a set of prime numbers
#[derive(Debug)]
pub struct PrimeSet {
    // Add a field named data that contains a HashSet<i32>
    data: HashSet<i32>,
}

// Implement the new method for the PrimeSet collection that returns an empty set of prime numbers
impl PrimeSet {
    fn new() -> PrimeSet {
        // Create an empty HashSet and assign it to a variable
        let data = HashSet::new();
        // Return a PrimeSet with the data field
        PrimeSet { data }
    }
}

// Implement the insert method for the PrimeSet collection that takes an integer and inserts it into the set if it is a prime number, and returns a boolean indicating whether the insertion was successful or not
impl PrimeSet {
    fn insert(&mut self, x: i32) -> bool {
        // Check if the integer is a prime number
        if is_prime(x) {
            // Insert the integer into the data field and return true
            self.data.insert(x);
            true
        } else {
            // Return false
            false
        }
    }
}

// Implement the contains method for the PrimeSet collection that takes an integer and returns a boolean indicating whether the set contains the given number or not
impl PrimeSet {
    fn contains(&self, x: i32) -> bool {
        // Check if the data field contains the integer and return the result
        self.data.contains(&x)
    }
}

// Define a module named utils that contains some utility functions for working with numbers
mod utils {
    // Define a function named is_prime that takes an integer and returns a boolean indicating whether it is a prime number or not
    pub fn is_prime(x: i32) -> bool {
        // Check if the integer is less than or equal to one
        if x <= 1 {
            // Return false
            return false;
        }
        // Iterate from two to the square root of the integer
        for i in 2..=((x as f64).sqrt() as i32) {
            // Check if the integer is divisible by the iterator
            if x % i == 0 {
                // Return false
                return false;
            }
        }
        // Return true
        true
    }

    // Define a function named is_even that takes an integer and returns a boolean indicating whether it is an even number or not
    pub fn is_even(x: i32) -> bool {
        // Check if the integer is divisible by two and return the result
        x % 2 == 0
    }
}

// Define a module named math that re-exports the utils module and the PrimeSet collection
pub mod math {
    // Bring the utils module and the PrimeSet collection into scope
    use crate::utils;
    use crate::PrimeSet;
    // Re-export the utils module
    pub use utils::*;
}

// Use the use keyword to bring the math module into scope
use math::*;

fn main() {
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

    // Try to open a file that does not exist and unwrap the result
    let file = File::open("hello.txt").unwrap(); // this will panic

    // Try to open a file that does not exist and expect the result
    let file = File::open("hello.txt").expect("Failed to open hello.txt"); // this will panic with a message

    // Try to divide two integers and handle the result
    match div(10, 2) {
        // If the result is Ok, print the quotient
        Ok(q) => println!("The quotient is {}", q),
        // If the result is Err, print the error
        Err(e) => println!("The error is {}", e),
    }

    // Try to divide two integers and unwrap the result
    let q = div(10, 0).unwrap(); // this will panic

    // Try to divide two integers and expect the result
    let q = div(10, 0).expect("Failed to divide"); // this will panic with a message

    // Create a PrimeSet and assign it to a variable
    let mut p = PrimeSet::new();

    // Insert some numbers into the PrimeSet and check if they are prime or even
    for n in 1..=10 {
        println!("Is {} prime? {}", n, p.insert(n));
        println!("Is {} even? {}", n, is_even(n));
    }

    // Check if the PrimeSet contains some numbers
    for n in 1..=10 {
        println!("Does the set contain {}? {}", n, p.contains(n));
    }
}

