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

# **Ownership**

Ownership is a key concept in Rust that ensures memory safety and prevents common errors such as dangling pointers and data races. In this tutorial, I will explain the basics of ownership and how it works in Rust.

## What is Ownership?

Ownership is a set of rules that the Rust compiler enforces to manage the memory usage of a program. The rules are:

- Each value in Rust has a variable that is called its owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

These rules ensure that each value has a well-defined lifetime and that no two values can modify the same memory location at the same time.

## The Stack and the Heap

To understand ownership, we need to know how Rust stores data in memory. Rust uses two parts of memory: the stack and the heap.

The stack is a fast and efficient way of storing data that has a fixed size and is known at compile time. The stack works like a pile of plates: you can only add or remove data from the top of the stack. This is called pushing and popping. The stack is also a last-in, first-out (LIFO) structure: the last value pushed onto the stack is the first one popped off.

The heap is a more flexible way of storing data that has a variable or unknown size at compile time. The heap works like a large pool of memory: you can allocate any amount of space you need and get a pointer to that location. This pointer is a fixed-size value that can be stored on the stack. However, accessing the data on the heap is slower than accessing the data on the stack, because you have to follow the pointer. The heap is also less organized: you have to keep track of the allocated and freed space, and avoid memory leaks and fragmentation.

## Ownership Examples

Let's see some examples of how ownership works in Rust. We will use the `String` type, which is a growable, UTF-8 encoded text type that is stored on the heap.

### Creating a String

We can create a new `String` from a string literal using the `from` function:

```rust
let s = String::from("hello"); // s is the owner of the string
```

This allocates memory on the heap for the string data and returns a `String` instance that owns that data. When `s` goes out of scope, Rust will automatically call the `drop` function to free the memory.

### Moving a String

We can assign a `String` to another variable, but this does not create a copy of the data. Instead, it moves the ownership from one variable to another:

```rust
let s1 = String::from("hello"); // s1 is the owner of the string
let s2 = s1; // s2 is the new owner of the string, s1 is no longer valid
```

This means that `s1` is no longer usable after the assignment, and Rust will prevent us from using it:

```rust
println!("{}", s1); // error: use of moved value: `s1`
```

This behavior prevents us from accidentally creating two pointers to the same memory location, which could cause data corruption or memory leaks.

### Cloning a String

If we want to create a copy of the data on the heap, we can use the `clone` method:

```rust
let s1 = String::from("hello"); // s1 is the owner of the string
let s2 = s1.clone(); // s2 is the owner of a new copy of the string, s1 is still valid
```

This will allocate a new memory region on the heap and copy the data from `s1` to `s2`. Both `s1` and `s2` are valid and independent, and they will be dropped when they go out of scope.

### Passing a String to a Function

We can also transfer ownership by passing a `String` to a function:

```rust
fn main() {
    let s = String::from("hello"); // s is the owner of the string
    take_ownership(s); // s is moved to the function parameter, s is no longer valid
    // println!("{}", s); // error: use of moved value: `s`
}

fn take_ownership(some_string: String) { // some_string is the new owner of the string
    println!("{}", some_string);
} // some_string goes out of scope and the string is dropped
```

This means that the function is responsible for freeing the memory of the string when it finishes. If we want to use the string again after the function call, we need to return it from the function and assign it back to a variable:

```rust
fn main() {
    let s = String::from("hello"); // s is the owner of the string
    let s = give_ownership(s); // s is moved to the function parameter and returned, s is the new owner of the string
    println!("{}", s); // ok: s is valid
}

fn give_ownership(some_string: String) -> String { // some_string is the owner of the string
    some_string // some_string is returned and moved to the caller
}
```

## References and Borrowing

Moving ownership can be cumbersome and inefficient, especially when we only want to use a value temporarily or read-only. Rust has a feature called references that allows us to pass a value to a function without transferring ownership. This is called borrowing, because we are temporarily borrowing the value from the owner.

To create a reference, we use the `&` operator:

```rust
let s = String::from("hello"); // s is the owner of the string
let r = &s; // r is a reference to s, s is still the owner of the string
```

A reference is a pointer to a value, but it does not own the value. This means that the value will not be dropped when the reference goes out of scope. We can have multiple references to the same value, as long as they are immutable:

```rust
let s = String::from("hello"); // s is the owner of the string
let r1 = &s; // r1 is an immutable reference to s
let r2 = &s; // r2 is another immutable reference to s
println!("{} and {}", r1, r2); // ok: we can use both references
```

However, we cannot have a mutable reference while we have one or more immutable references:

```rust
let s = String::from("hello"); // s is the owner of the string
let r1 = &s; // r1 is an immutable reference to s
let r2 = &mut s; // r2 is a mutable reference to s
println!("{} and {}", r1, r2); // error: cannot borrow `s` as mutable because it is also borrowed as immutable
```

This rule prevents us from creating data races, which can cause undefined behavior and hard-to-find bugs.

To pass a reference to a function, we use the same syntax as passing a value:

```rust
fn main() {
    let s = String::from("hello"); // s is the owner of the string
    borrow_ownership(&s); // s is borrowed by the function parameter, s is still the owner of the string
    println!("{}", s); // ok: s is valid
}

fn borrow_ownership(some_string: &String) { // some_string is a reference to a string
    println!("{}", some_string);
} // some_string goes out of scope, but the string is not dropped
```

The function parameter is a reference, so it does not take ownership of the value. This means that the value will not be dropped when the function finishes, and we can use it again in the main function.

If we want to modify a value that we borrow, we need to use a mutable reference:

```rust
fn main() {
    let mut s = String::from("hello"); // s is the owner of the string
    change(&mut s); // s is mutably borrowed by the function parameter
    println!("{}", s); // ok: s is valid
}

fn change(some_string: &mut String) { // some_string is a mutable reference to a string
    some_string.push_str(", world"); // we can modify the string through the reference
} // some_string goes out of scope, but the string is not dropped
```

Note that we need to declare the variable as mutable in order to create a mutable reference. We can only have one mutable reference to a value at a time, to prevent data races:

```rust
let mut s = String::from("hello"); // s is the owner of the string
let r1 = &mut s; // r1 is a mutable reference to s
let r2 = &mut s; // r2 is another mutable reference to s
println!("{} and {}", r1, r2); // error: cannot borrow `s` as mutable more than once at a time
```

## The Slice Type

Another data type that does not have ownership is the slice type. A slice is a reference to a contiguous sequence of elements in a collection, such as a string or an array. Slices let us work with a subset of the collection without copying it.

To create a slice, we use the `[start..end]` syntax, where `start` is the index of the first element and `end` is the index of one past the last element. For example, to get a slice of the first three elements of an array, we write:

```rust
let a = [1, 2, 3, 4, 5]; // a is an array of five integers
let s = &a[0..3]; //
```

#### **Conclusion**

This section of our Rust tutorial covered the essentials of control flow, ownership, and references. These concepts are fundamental to understanding how Rust manages memory and executes code based on conditions. By mastering these, you'll have a solid foundation for writing efficient and safe Rust programs.
