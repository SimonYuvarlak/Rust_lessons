# Pattern Matching

Pattern matching is a powerful feature in Rust that allows you to compare a value against a series of patterns and then execute code based on which pattern matches. Pattern matching can be used in various contexts, such as match expressions, if let expressions, while let expressions, for loops, and function parameters.

Here is a basic tutorial on how to use pattern matching in Rust:

- To create a match expression, use the keyword `match` followed by a value and a set of arms. Each arm consists of a pattern and an expression to run if the pattern matches the value. The arms are separated by commas and enclosed in curly braces. For example:

```rust
let x = 1;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

This code prints "one" because the value of x is 1. The underscore (\_) is a wildcard pattern that matches any value.

- You can also use match expressions to bind variables to values or parts of values. For example, if you have an enum that represents a coin, you can use a match expression to assign different values to each variant:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

This function takes a Coin enum as a parameter and returns the corresponding value in cents as a u8. Note that there is no need for a wildcard pattern here because we have covered all the possible variants of the enum.

- You can also use match expressions to destructure complex values, such as tuples, structs, or enums with data. For example, if you have an enum that represents a message, you can use a match expression to access the data inside each variant:

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn print_message(msg: Message) {
    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!("Move in the x direction {} and in the y direction {}", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change the color to red {}, green {}, and blue {}", r, g, b),
    }
}
```

This function takes a Message enum as a parameter and prints a different message depending on the variant and the data inside it. Note that we can use curly braces to destructure structs and named fields, and parentheses to destructure tuples and unnamed fields.

- You can also use conditional expressions, called guards, to add extra logic to your match arms. A guard is an expression that follows the pattern and is preceded by the keyword `if`. The guard must evaluate to a boolean value. If the guard is true, the arm is chosen. If the guard is false, the arm is skipped. For example, you can use a guard to check if a number is divisible by another number:

```rust
let num = 4;

match num {
    x if x % 2 == 0 => println!("{} is even", x),
    x if x % 3 == 0 => println!("{} is divisible by 3", x),
    x => println!("{} is neither even nor divisible by 3", x),
}
```

This code prints "4 is even" because the first arm matches and the guard is true. The second arm is skipped because the guard is false. The third arm is also skipped because the pattern does not match.

- Sometimes, you may want to run some code for a pattern, but also match other patterns with the same code. You can use the vertical bar (`|`) to specify multiple patterns in the same arm. For example, you can use this syntax to check if a number is within a range:

```rust
let num = 5;

match num {
    1 | 2 | 3 => println!("{} is a low number", num),
    4 | 5 | 6 => println!("{} is a medium number", num),
    7 | 8 | 9 => println!("{} is a high number", num),
    _ => println!("{} is out of range", num),
}
```

This code prints "5 is a medium number" because the second arm matches one of the patterns.

- If you want to match a pattern but also keep the original value, you can use the `@` symbol to bind the value to a variable. For example, you can use this syntax to print the name of a color and its RGB values:

```rust
enum Color {
    Red,
    Green,
    Blue,
    Rgb(i32, i32, i32),
}

let color = Color::Rgb(255, 0, 0);

match color {
    Color::Red => println!("The color is red"),
    Color::Green => println!("The color is green"),
    Color::Blue => println!("The color is blue"),
    c @ Color::Rgb(r, g, b) => println!("The color is {:?} with RGB values ({}, {}, {})", c, r, g, b),
}
```

This code prints "The color is Rgb(255, 0, 0) with RGB values (255, 0, 0)" because the last arm matches and binds the value of color to the variable c.

- Sometimes, you may want to match a value against a single pattern without using a match expression. You can use the `if let` syntax to do this. The `if let` syntax takes a pattern and an expression, and runs the expression if the pattern matches. You can also add an `else` block to run some code if the pattern does not match. For example, you can use the `if let` syntax to check if an optional value is Some or None:

```rust
let some_option = Some(3);

if let Some(x) = some_option {
    println!("The value is {}", x);
} else {
    println!("The value is None");
}
```

This code prints "The value is 3" because the pattern matches and binds the value of some_option to the variable x. If some_option was None, the code would print "The value is None" instead.

- Similarly, you can use the `while let` syntax to run a loop while a pattern matches. The `while let` syntax takes a pattern and an expression, and runs the expression as long as the pattern matches. For example, you can use the `while let` syntax to pop values from a vector until it is empty:

```rust
let mut vec = vec![1, 2, 3];

while let Some(x) = vec.pop() {
    println!("{}", x);
}
```

This code prints "3", "2", and "1" because the pattern matches and pops the last element of the vector until it is empty.

- You can also use patterns in for loops to iterate over elements of an iterator. For example, you can use a pattern to destructure a tuple in a vector:

```rust
let vec = vec![(1, 2), (3, 4), (5, 6)];

for (a, b) in vec {
    println!("{} and {}", a, b);
}
```

This code prints "1 and 2", "3 and 4", and "5 and 6" because the pattern matches and destructures each tuple in the vector.

- Finally, you can also use patterns in function parameters to destructure the arguments. For example, you can use a pattern to destructure a struct in a function:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn print_point(Point { x, y }: Point) {
    println!("The point is at ({}, {})", x, y);
}
```

This function takes a Point struct as a parameter and prints its fields by using a pattern to destructure the struct.
