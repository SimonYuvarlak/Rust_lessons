// Necessary imports for the file operations
use std::fs::File;
use std::io::Read;
use std::num::ParseIntError;

fn main() {
    // Defining a struct named Point with two fields: x and y
    struct Point {
        x: f64,
        y: f64,
    }

    // Creating an instance of Point with some values
    let p = Point { x: 3.0, y: 4.0 };

    // Access the fields of the struct using dot notation
    println!("The point is at ({}, {})", p.x, p.y);

    // Defining a tuple struct named Color with three fields: red, green, and blue
    struct Color(f64, f64, f64);

    // Creating an instance of Color with some values
    let c = Color(0.5, 0.8, 0.3);

    // Access the fields of the tuple struct using index notation
    println!("The color is ({}, {}, {})", c.0, c.1, c.2);

    // Defining a unit struct named Meter
    struct Meter;

    // Creating an instance of Meter
    let m = Meter;

    // Using the instance of Meter as a type annotation
    let length: Meter = m;

    // Enums and Pattern Matching

    // Define an enum named Message with four variants
    enum Message {
        Quit,
        Move(Point),
        Write(String),
        ChangeColor(Color),
    }

    // Create an instance of Message with the Move variant
    let m = Message::Move(Point { x: 10.0, y: 20.0 });

    // Match statement for handling different variants of Message
    match m {
        Message::Move(p) => println!("The message is to move to ({}, {})", p.x, p.y),
        _ => (),
    }

    // Methods and Traits for Structs

    // Define a method named distance for the Point struct
    impl Point {
        fn distance(&self, other: &Point) -> f64 {
            ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
        }
    }

    // Define a trait named Shape with a method named area
    trait Shape {
        fn area(&self) -> f64;
    }

    // Implement the Shape trait for the Point struct
    impl Shape for Point {
        fn area(&self) -> f64 {
            0.0
        }
    }

    // Creating two instances of Point
    let p1 = Point { x: 3.0, y: 4.0 };
    let p2 = Point { x: 6.0, y: 8.0 };

    // Calling the distance method on p1
    let d = p1.distance(&p2);
    println!("The distance between the points is {}", d);

    // Calling the area method on p1
    let a = p1.area();
    println!("The area of the point is {}", a);

    // Option and Result Enums

    // Define a vector of integers
    let v = vec![1, 2, 3, 4, 5];

    // Function to find an element in a vector
    fn find(v: &Vec<i32>, x: i32) -> Option<usize> {
        for (i, &n) in v.iter().enumerate() {
            if n == x {
                return Some(i);
            }
        }
        None
    }

    // Using the find function
    let r = find(&v, 3);
    match r {
        Some(i) => println!("The value is at index {}", i),
        None => println!("The value is not in the vector"),
    }

    // Parsing a string to an integer
    let s = "42";
    let r = s.parse::<i32>();
    match r {
        Ok(n) => println!("The number is {}", n),
        Err(e) => println!("The error is {}", e),
    }

    // Function to read a number from a file
    fn read_number(file_name: &str) -> Result<i32, std::io::Error> {
        let mut file = File::open(file_name)?;
        let mut s = String::new();
        file.read_to_string(&mut s)?;
        Ok(s.trim().parse().unwrap())
    }

    // Handling the result of read_number
    match read_number("number.txt") {
        Ok(n) => println!("The number is {}", n),
        Err(e) => println!("The error is {}", e),
    }
}
