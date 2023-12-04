# **Control Flow, Ownership, and References in Rust**

#### **Control Flow**

Control flow in Rust allows you to execute different parts of your code based on certain conditions or loops.

##### **`if` Statements**

The `if` statement is used for conditional execution. Rust doesn’t require parentheses around the condition but braces are mandatory.

```rust
fn main() {
    let number = 6;

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }
}
```

##### **Using `if` in a `let` Statement**

In Rust, `if` can also be used on the right side of a `let` statement to assign values conditionally.

```rust
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

##### **Repetition with Loops**

Rust provides several loops for repetition: `loop`, `while`, and `for`.

- **`loop`**: Repeatedly executes a block of code until you explicitly tell it to stop.

```rust
fn main() {
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }
}
```

- **`while`**: Continues to execute as long as a condition is true.

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}
```

- **`for` Loops**: Iterates over elements of a collection, such as an array.

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }
}
```

#### **Ownership**

Ownership is a set of rules that governs how Rust manages memory. It’s a unique feature of Rust and crucial for ensuring memory safety without a garbage collector.

##### **Rules of Ownership**

1. Each value in Rust has an owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

##### **String Type and Ownership**

The `String` type in Rust is mutable and can grow or shrink in size. When a variable goes out of scope, Rust automatically calls the `drop` function and cleans up the heap memory.

```rust
fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}
```

#### **References and Borrowing**

References allow you to refer to some value without taking ownership. The concept of borrowing is when you have a reference to an object.

##### **References**

You can create references by using the `&` symbol. References are immutable by default.

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

##### **Mutable References**

Mutable references allow you to change the data they point to. However, you can only have one mutable reference to a particular piece of data in a particular scope.

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

#### **Conclusion**

This section of our Rust tutorial covered the essentials of control flow, ownership, and references. These concepts are fundamental to understanding how Rust manages memory and executes code based on conditions. By mastering these, you'll have a solid foundation for writing efficient and safe Rust programs.
