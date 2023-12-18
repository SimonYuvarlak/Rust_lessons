# **Structs and Enums in Rust**

Structs and enums are two of the most common data structures in Rust. They allow us to define custom types that can store different kinds of data and have different behaviors. In this tutorial, I will explain how to create and use structs and enums, how to implement methods and traits for them, and how to use the match statement to handle different variants of enums.

## Structs

A struct is a data structure that groups together some values that are related to each other. For example, we can define a struct that represents a point in a two-dimensional space:

```rust
// Define a struct named Point with two fields: x and y
struct Point {
    x: f64,
    y: f64,
}

// Create an instance of Point with some values
let p = Point { x: 3.0, y: 4.0 };

// Access the fields of the struct using dot notation
println!("The point is at ({}, {})", p.x, p.y);
```

There are three types of structs in Rust: named structs, tuple structs, and unit structs. Named structs have named fields, as shown in the previous example. Tuple structs have unnamed fields, and they are similar to tuples. Unit structs have no fields, and they are useful for creating new types that are distinct from other types. For example, we can define a tuple struct that represents a color:

```rust
// Define a tuple struct named Color with three fields: red, green, and blue
struct Color(f64, f64, f64);

// Create an instance of Color with some values
let c = Color(0.5, 0.8, 0.3);

// Access the fields of the tuple struct using dot notation and index notation
println!("The color is ({}, {}, {})", c.0, c.1, c.2);
```

We can also define a unit struct that represents a unit of measurement:

```rust
// Define a unit struct named Meter
struct Meter;

// Create an instance of Meter
let m = Meter;

// Use the instance of Meter as a type annotation
let length: Meter = m;
```

## Enums

An enum is a data structure that defines a set of possible variants, each of which can store some data. For example, we can define an enum that represents a message:

```rust
// Define an enum named Message with four variants: Quit, Move, Write, and ChangeColor
enum Message {
    // Quit has no data associated with it
    Quit,
    // Move has a tuple struct associated with it, which contains the x and y coordinates
    Move(Point),
    // Write has a String associated with it, which contains the text of the message
    Write(String),
    // ChangeColor has a tuple struct associated with it, which contains the red, green, and blue values
    ChangeColor(Color),
}

// Create an instance of Message with the Move variant and some values
let m = Message::Move(Point { x: 10.0, y: 20.0 });

// Access the variant and the data of the enum using dot notation and pattern matching
match m {
    // If the variant is Move, bind the data to a variable named p
    Message::Move(p) => println!("The message is to move to ({}, {})", p.x, p.y),
    // If the variant is any other, do nothing
    _ => (),
}
```

The match statement is a powerful control flow construct that allows us to handle different variants of an enum in a concise and exhaustive way. The match statement takes an expression and compares it to a series of patterns, each of which has a corresponding block of code. The patterns can be literals, variables, wildcards, or other enums. The code associated with the first pattern that matches the expression is executed. If no pattern matches, the program will panic, unless there is a default case using the wildcard pattern `_`.

## Methods and Traits

We can define methods and traits for structs and enums, which allow us to add functionality and behavior to our custom types. A method is a function that is associated with a type and can be called on an instance of that type. A trait is a collection of methods that can be implemented for different types. For example, we can define a method that calculates the distance between two points, and a trait that defines a method that returns the area of a shape:

```rust
// Define a method named distance for the Point struct
impl Point {
    // The first parameter is always self, which refers to the instance of the struct
    // The second parameter is another instance of Point
    fn distance(&self, other: &Point) -> f64 {
        // Use the Pythagorean theorem to calculate the distance
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Define a trait named Shape that has a method named area
trait Shape {
    // The method has no parameters and returns a f64
    fn area(&self) -> f64;
}

// Implement the Shape trait for the Point struct
impl Shape for Point {
    // Define the area method
    fn area(&self) -> f64 {
        // A point has zero area
        0.0
    }
}

// Create two instances of Point
let p1 = Point { x: 3.0, y: 4.0 };
let p2 = Point { x: 6.0, y: 8.0 };

// Call the distance method on p1 and pass p2 as an argument
let d = p1.distance(&p2);
println!("The distance between the points is {}", d);

// Call the area method on p1
let a = p1.area();
println!("The area of the point is {}", a);
```

## Option and Result Enums

Rust has two built-in enums that are widely used in the standard library and in many other crates: Option and Result. These enums are useful for handling situations where a value may be present or absent, or where an operation may succeed or fail.

The Option enum has two variants: Some and None. The Some variant contains a value of some type, and the None variant indicates the absence of a value. For example, we can use the Option enum to represent the result of searching for a value in a vector:

```rust
// Define a vector of integers
let v = vec![1, 2, 3, 4, 5];

// Define a function that takes a vector and a value and returns an Option<usize>
fn find(v: &Vec<i32>, x: i32) -> Option<usize> {
    // Iterate over the vector and its indices
    for (i, &n) in v.iter().enumerate() {
        // If the value matches the element, return Some with the index
        if n == x {
            return Some(i);
        }
    }
    // If the value is not found, return None
    None
}

// Call the find function with the vector and a value
let r = find(&v, 3);

// Use the match statement to handle the result
match r {
    // If the result is Some, print the index
    Some(i) => println!("The value is at index {}", i),
    // If the result is None, print a message
    None => println!("The value is not in the vector"),
}
```

The Result enum has two variants: Ok and Err. The Ok variant contains a value of some type, and the Err variant contains an error of some type. For example, we can use the Result enum to represent the result of parsing a string into a number:

```rust
// Define a string that contains a number
let s = "42";

// Use the parse method to convert the string into a number
// The parse method returns a Result<i32, ParseIntError>
let r = s.parse::<i32>();

// Use the match statement to handle the result
match r {
    // If the result is Ok, print the number
    Ok(n) => println!("The number is {}", n),
    // If the result is Err, print the error
    Err(e) => println!("The error is {}", e),
}
```

We can also use the `?` operator to propagate errors from a function that returns a Result. The `?` operator will return the value inside the Ok variant if the result is Ok, or return the error inside the Err variant if the result is Err. For example, we can define a function that reads a file and parses its contents into a number:

```rust
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

// Define a function that takes a file name and returns a Result<i32, ParseIntError>
fn read_number(file_name: &str) -> Result<i32, ParseIntError> {
    // Open the file and handle the possible error using the ? operator
    let mut file = File::open(file_name)?;
    // Create an empty string
    let mut s = String::new();
    // Read the file contents into the string and handle the possible error using the ? operator
    file.read_to_string(&mut s)?;
    // Parse the string into a number and return the result
    s.trim().parse::<i32>()
}

// Call the read_number function with a file name
let r = read_number("number.txt");

// Use the match statement to handle the result
match r {
    // If the result is Ok, print the number
    Ok(n) => println!("The number is {}", n),
    // If the result is Err, print the error
    Err(e) => println!("The error is {}", e),
}
```

## Summary

In this tutorial, we learned how to create and use structs and enums, how to implement methods and traits for them, and how to use the match statement to handle different variants of enums. We also learned about the Option and Result enums, which are useful for handling situations where a value may be present or absent, or where an operation may succeed or fail.
