// Mutability in Rust
fn main() {
    // Immutable variable
    let x = 5;
    println!("Immutable x: {}", x);

    // Mutable variable
    let mut y = 5;
    println!("Mutable y before change: {}", y);
    y = 32;
    println!("Mutable y after change: {}", y);

    // Integer Types
    let positive_num: u32 = 100; // Unsigned 32-bit integer
    let negative_num: i32 = -100; // Signed 32-bit integer
    println!("Positive: {}, Negative: {}", positive_num, negative_num);

    // Floating-Point Types
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32, specified explicitly

    // Boolean Type
    let is_active: bool = true;
    let is_inactive = false; // type inferred as bool

    // Character Type
    let letter: char = 'a'; // Regular ASCII character
    let emoji: char = '😊'; // Emoji (Unicode scalar value)

    // Tuple Type
    let mixed: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = mixed; // Destructuring a tuple

    // Array Type
    let days = [
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
        "Sunday",
    ];
    println!("First day of the week: {}", days[0]);

    //Function calls
    greet("Alice");
    let sum = add(5, 6);
    println!("Sum is: {}", sum);
}

// Functions in Rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // Expression; return keyword is optional
}
