# **Introduction to Rust Programming**

#### **Setting Up Your Environment**

Before beginning, ensure Rust and Cargo are installed on your system. Visit [Rust Installation Page](https://www.rust-lang.org/tools/install) and follow the instructions to install Rust and Cargo. After installation, you can further enhance your learning with exercises from [Rustlings](https://github.com/rust-lang/rustlings/).

#### **Diving Deep into Rust Fundamentals**

##### **1. Mutability in Rust**

In Rust, variables are immutable by default, which means their value cannot be changed once set. This immutability is a design choice that promotes safer and more predictable code. However, you can make variables mutable by using the `mut` keyword.

```rust
fn main() {
    let x = 5;
    println!("Immutable x: {}", x); // This will print 5

    let mut y = 5;
    println!("Mutable y before change: {}", y); // 5

    y = 32;
    println!("Mutable y after change: {}", y); // 32
}
```

##### **2. Understanding Data Types**

###### **Integers**

Integers in Rust are of two types: signed and unsigned.

- **Signed Integers (`i8`, `i16`, `i32`, `i64`, `i128`, `isize`)**: These can store both negative and positive numbers. The number following the 'i' represents the number of bits used to store the integer. For example, `i32` is a 32-bit signed integer. The range of values it can store is from -2^(n-1) to 2^(n-1)-1, where 'n' is the number of bits. So for `i32`, it's from -2,147,483,648 to 2,147,483,647.

- **Unsigned Integers (`u8`, `u16`, `u32`, `u64`, `u128`, `usize`)**: These can only store positive numbers and zero. The 'u' signifies 'unsigned'. A `u32` is a 32-bit unsigned integer, capable of storing values from 0 to 2^n-1, which for `u32` is 0 to 4,294,967,295.

```rust
fn main() {
    let positive_num: u32 = 100; // Unsigned 32-bit integer
    let negative_num: i32 = -100; // Signed 32-bit integer
    println!("Positive: {}, Negative: {}", positive_num, negative_num);
}
```

###### **Floats**

Rust supports two floating-point types: `f32` and `f64`. The `f64` type is a double-precision floating-point and is the default due to its precision.

```rust
fn main() {
    let x = 2.0; // f64 by default
    let y: f32 = 3.0; // f32, specified explicitly
}
```

###### **Boolean**

The boolean type in Rust is declared as `bool`. It represents truth with `true` and `false` values. Booleans are commonly used in conditional statements like `if`.

```rust
fn main() {
    let is_active: bool = true;
    let is_inactive = false; // type inferred as bool
}
```

###### **Character**

Rust's `char` type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Character literals are specified with single quotes.

```rust
fn main() {
    let letter: char = 'a'; // Regular ASCII character
    let emoji: char = '😊'; // Emoji (Unicode scalar value)
}
```

###### **Tuple**

A tuple is a compound type that can group together values with different types. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```rust
fn main() {
    let mixed: (i32, f64, char) = (500, 6.4, 'z');
    let (x, y, z) = mixed; // Destructuring a tuple
}
```

###### **Array**

An array is a collection of elements of the same type, stored in contiguous memory locations. The length of an array is fixed when it is declared and cannot be changed.

```rust
fn main() {
    let days = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    println!("First day of the week: {}", days[0]);
}
```

##### **Functions in Rust**

Functions are declared using the `fn` keyword and have a set of parameters

and an optional return type. Rust supports function overloading, meaning you can have multiple functions with the same name but different parameters.

```rust
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn add(x: i32, y: i32) -> i32 {
    x + y // Expression; return keyword is optional
}

fn main() {
    greet("Alice");
    let sum = add(5, 6);
    println!("Sum is: {}", sum);
}
```

In the next section, we'll explore control flow, ownership, references, and more, further deepening our understanding of Rust.

---

This approach to the tutorial ensures a deeper understanding of Rust's syntax and features. Each topic is explained with clarity, making it suitable for beginners and a comprehensive resource for learners.
