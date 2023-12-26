# Advanced Features

Rust is a powerful and expressive language that offers many features to help you write fast, safe, and maintainable code. Some of these features are more advanced than others, and require a deeper understanding of the language and its guarantees. In this tutorial, we will explore some of the advanced features of Rust, such as unsafe Rust, advanced traits, advanced types, advanced functions and closures, and macros. We will also see some code samples that demonstrate how to use these features in practice.

## Unsafe Rust

Rust is a safe language by default, meaning that it prevents many common errors and bugs, such as memory leaks, data races, null pointers, buffer overflows, and undefined behavior. Rust achieves this by enforcing a set of rules and guarantees at compile time, such as ownership, borrowing, lifetimes, and mutability. However, sometimes you may need to opt out of some of these rules and guarantees, and take responsibility for manually upholding them. This is where unsafe Rust comes in.

Unsafe Rust allows you to perform some low-level operations that are not possible or not efficient in safe Rust, such as:

- Dereferencing raw pointers
- Calling unsafe functions or methods
- Accessing or modifying static mutable variables
- Implementing unsafe traits
- Interacting with foreign code (such as C or assembly)

However, unsafe Rust is not a free pass to do anything you want. You still have to follow some rules and conventions, such as:

- Using the `unsafe` keyword to mark the blocks or items that contain unsafe code
- Documenting the invariants and assumptions that your unsafe code relies on
- Isolating the unsafe code as much as possible, and minimizing its scope and exposure
- Using safe abstractions to wrap the unsafe code and provide a safe interface to the rest of the code
- Testing and reviewing the unsafe code carefully and thoroughly

Using unsafe Rust sparingly and carefully is essential, as it can introduce bugs, memory leaks, or undefined behavior if done incorrectly or carelessly. Undefined behavior is especially dangerous, as it can cause your program to behave unpredictably, crash, or even compromise the security of your system. Therefore, you should only use unsafe Rust when you have a good reason to do so, and when you are confident that you can uphold the guarantees that safe Rust expects.

Here is an example of using unsafe Rust to dereference a raw pointer:

```rust
// A raw pointer is a plain memory address that does not have any ownership or borrowing rules
// You can create a raw pointer from a reference by using the `as` keyword
let x = 5;
let raw = &x as *const i32;

// You can also create a raw pointer by using the `ptr::null` or `ptr::null_mut` functions
// These functions return a null pointer, which is a special value that indicates the absence of a valid address
// Note that creating a null pointer is safe, but dereferencing it is not
let null = std::ptr::null::<i32>();

// To dereference a raw pointer, you have to use unsafe Rust
// This means that you have to wrap the dereferencing operation in an `unsafe` block
// and ensure that the pointer is valid and aligned
unsafe {
    // Dereferencing a valid pointer will give you the value that it points to
    println!("raw points to {}", *raw);

    // Dereferencing a null pointer will cause undefined behavior
    // This may crash the program, or worse
    println!("null points to {}", *null);
}
```

Here is an example of using unsafe Rust to call an unsafe function:

```rust
// An unsafe function is a function that requires some invariants or assumptions to be upheld by the caller
// You can mark a function as unsafe by using the `unsafe` keyword before the `fn` keyword
// You should also document the invariants or assumptions that the function expects
unsafe fn dangerous(a: *mut i32) {
    // This function takes a mutable raw pointer as an argument
    // The caller must ensure that the pointer is valid, aligned, and not aliased
    // The function will modify the value that the pointer points to
    *a += 1;
}

// To call an unsafe function, you have to use unsafe Rust
// This means that you have to wrap the function call in an `unsafe` block
// and ensure that the invariants or assumptions are satisfied
let mut x = 10;
let ptr = &mut x as *mut i32;

unsafe {
    // Calling the unsafe function with a valid pointer is fine
    dangerous(ptr);
    println!("x is now {}", x);

    // Calling the unsafe function with an invalid pointer will cause undefined behavior
    dangerous(std::ptr::null_mut());
}
```

Here is an example of using unsafe Rust to access a static mutable variable:

```rust
// A static variable is a variable that has a fixed address and a `'static` lifetime
// You can create a static variable by using the `static` keyword
// Static variables are useful for storing constants or global state
static HELLO: &str = "Hello, world!";

// A static mutable variable is a static variable that can be modified
// You can create a static mutable variable by using the `static mut` keywords
// Static mutable variables are useful for implementing global mutable state, such as counters or caches
// However, they are also very unsafe, as they can cause data races or memory corruption if accessed concurrently or incorrectly
static mut COUNTER: u32 = 0;

// To access a static variable, you can use its name as an expression
// This is safe, as static variables are immutable by default
println!("{}", HELLO);

// To access or modify a static mutable variable, you have to use unsafe Rust
// This means that you have to wrap the access or modification in an `unsafe` block
// and ensure that there are no concurrent or conflicting accesses
unsafe {
    // Accessing the static mutable variable is fine
    println!("COUNTER is {}", COUNTER);

    // Modifying the static mutable variable is also fine
    COUNTER += 1;
    println!("COUNTER is now {}", COUNTER);
}
```

Here is an example of using unsafe Rust to implement an unsafe trait:

```rust
// An unsafe trait is a trait that requires some invariants or assumptions to be upheld by the implementor
// You can mark a trait as unsafe by using the `unsafe` keyword before the `trait` keyword
// You should also document the invariants or assumptions that the trait expects
unsafe trait Foo {
    // This trait has a method that takes a raw pointer as an argument
    // The implementor must ensure that the pointer is valid and aligned
    fn bar(&self, a: *const i32);
}

// To implement an unsafe trait, you have to use unsafe Rust
// This means that you have to mark the `impl` block as unsafe
// and ensure that the invariants or assumptions are satisfied
unsafe impl Foo for i32 {
    // This implementation satisfies the trait's invariant by checking the pointer for null
    fn bar(&self, a: *const i32) {
        if a.is_null() {
            println!("Got a null pointer");
        } else {
            println!("Got a valid pointer with value {}", unsafe { *a });
        }
    }
}

// To use an unsafe trait, you have to use unsafe Rust
// This means that you have to wrap the trait method call in an `unsafe` block
// and ensure that the argument is valid and aligned
let x = 42;
let ptr = &x as *const i32;

unsafe {
    // Calling the trait method with a valid pointer is fine
    x.bar(ptr);

    // Calling the trait method with a null pointer is also fine, as the implementation handles it
    x.bar(std::ptr::null());
}
```

Here is an example of using unsafe Rust to interact with foreign code:

```rust
// Foreign code is code that is written in a different language than Rust, such as C or assembly
// You can use foreign code in Rust by using the `extern` keyword to declare an external block
// An external block contains declarations of foreign functions, variables, or types
// You should also specify the application binary interface (ABI) of the foreign code, such as `"C"` or `"stdcall"`
extern "C" {
    // This declares a foreign function that takes an `i32` as an argument and returns an `i32`
    // The function is defined in a C source file called `foo.c`
    fn foo(x: i32) -> i32;

    // This declares a foreign variable that is an `i32`
    // The variable is defined in a C source file called `bar.c`
    static mut bar: i32;

    // This declares a foreign type that is a C struct
    // The type is defined in a C header file called `baz.h`
    #[repr(C)]
    struct Baz {
        a: i32,
        b: i32,
    }
}

// To use foreign code in Rust, you have to use unsafe Rust
// This means that you have to wrap the foreign code usage in an `unsafe` block
// and ensure that the foreign code is correct and compatible
unsafe {
    // Calling the foreign function is fine, as long as the argument and return types  // match the C function signature
    let x = 10;
    let y = foo(x);
    println!(“foo({}) = {}”, x, y);

    // Accessing or modifying the foreign variable is also fine, as long as the type // matches the C variable type
    println!(“bar is {}”, bar);
    bar += 1;
    println!(“bar is now {}”, bar);

    // Creating or using the foreign type is also fine, as long as the type // matches the C struct layout and alignment
    let baz = Baz { a: 1, b: 2 };
    println!(“baz has fields a = {} and b = {}”, baz.a, baz.b);
}
```

## Advanced Traits

Traits are a way of defining shared behavior across types. Rust has some advanced features for working with traits, such as associated types, default type parameters, fully qualified syntax, supertraits, and the newtype pattern. These features allow you to write more generic and expressive code, as well as implement some common design patterns. You can learn more about advanced traits in [this chapter of the official Rust book](^1^).

## Associated Types

Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation. That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

One example of a trait with an associated type is the Iterator trait that the standard library provides. The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over. The definition of the Iterator trait is as shown in the following code block.

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

The type Item is a placeholder, and the next method's definition shows that it will return values of type Option<Self::Item>. Implementors of the Iterator trait will specify the concrete type for Item, and the next method will return an Option containing a value of that concrete type.

Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle. To examine the difference between the two concepts, we'll look at an implementation of the Iterator trait on a type named Counter that specifies the Item type is u32:

```rust
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // some logic to return the next value
    }
}
```

This syntax seems comparable to that of generics. So why not just define the Iterator trait with generics, as shown in the following code block?

```rust
pub trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

The difference is that when using generics, as in the previous code block, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter. In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time. When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.

With associated types, we don't need to annotate types because we can't implement a trait on a type multiple times. In the previous code block, we can only choose what the type of Item will be once, because there can only be one impl Iterator for Counter. We don't have to specify that we want an iterator of u32 values everywhere that we call next on Counter.

The benefit of using generics to allow multiple implementations of the same trait for the same type is that generic types can have different behavior depending on what concrete types are used in their place. For example, the std::ops::Add trait can be implemented for different types, and each implementation can define how to perform the addition operation differently. The downside is that when using a generic type parameter with a trait bound, generic code can only work with the methods defined in the trait. To work with the concrete type's methods, we need to use the fully qualified syntax.

The benefit of using associated types is that the code that uses the trait knows what concrete type it's working with, and can use methods specific to that type. The downside is that there can be only one implementation of the trait for each type, rather than many implementations with different generic parameters.

## Default Generic Type Parameters and Operator Overloading

When we use generic type parameters, we can specify a default concrete type for the generic type. This eliminates the need for implementors of the trait to specify a concrete type if the default type works. The syntax for specifying a default type for a generic type is <PlaceholderType=ConcreteType> when declaring the generic type.

A great example of a situation where this technique is useful is with operator overloading. Operator overloading is customizing the behavior of an operator (such as +) in particular situations.

Rust doesn't allow you to create your own operators or overload arbitrary operators, but the operations and corresponding traits listed in std::ops can be overloaded by implementing the traits associated with the operator. For example, in Listing 19-14 we overload the + operator to add two Point instances together. We use the Add trait to overload the + operator.

Filename: src/main.rs

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}
```

Listing 19-14: Implementing the Add trait to overload the + operator for Point instances

The default generic type in this code is within the Add trait. Here is its definition:

```rust
trait Add<RHS=Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
```

This code should look generally familiar: a trait with one method and an associated type. The new part is RHS=Self. This syntax is called default type parameters. The RHS generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method. If we don’t specify a concrete type for RHS when we implement the Add trait, the type of RHS will default to Self, which will be the type we’re implementing Add on.

When we implemented Add for Point, we used the default for RHS because we wanted to add two Point instances. Let’s look at an example of implementing the Add trait where we want to customize the RHS type rather than using the default.

We have two structs, Millimeters and Meters, holding values in different units. We want to add values in millimeters to values in meters and have the implementation of Add do the conversion correctly. We can implement Add for Millimeters with Meters as the RHS, as shown in Listing 19-15.

Filename: src/main.rs

```rust
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```

Listing 19-15: Implementing the Add trait on Millimeters to add Millimeters to Meters

In this case, we specify Meters as the RHS. Because we’ve specified a type for RHS, we don’t use the default type parameter of Self for RHS.

## Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type. It’s also possible to implement a method directly on the type with the same name as methods from traits.

When calling methods with the same name, you’ll need to tell Rust which one you want to use. Consider the code in Listing 19-16 where we’ve defined two traits, Pilot and Wizard, that both have a method called fly. We then implement both traits on a type Human that already has a method named fly implemented on it. Each fly method does something different.

Filename: src/main.rs

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

Listing 19-16: Two traits are defined to have a fly method and are implemented on the Human type, and a fly method is implemented on Human directly

When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type, as shown in Listing 19-17.

Filename: src/main.rs

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}


```

Calling the method fly on a Human instance defaults to the method implemented directly on the type:

```rust
let person = Human;
person.fly(); // Prints: *waving arms furiously*

```

To specify which trait's method to call, we use the fully qualified syntax:

```rust
Pilot::fly(&person); // Prints: This is your captain speaking.
Wizard::fly(&person); // Prints: Up!
```

### Operator Overloading

Rust allows operator overloading via traits. This can be used to customize the behavior of operators for custom types.

#### Example

Implementing the Add trait to overload the + operator:

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(Point { x: 1, y: 0 } + Point { x: 2, y: 3 }, Point { x: 3, y: 3 });
}

```

In this example, we've defined how the + operator should work for our Point struct.

## Conclusion

This tutorial has provided a brief overview of some advanced traits in Rust, including associated types, operator overloading, and fully qualified syntax for disambiguation. These features enhance the power and flexibility of Rust's type system, allowing for more expressive and efficient code. For a deeper understanding and more complex scenarios, always refer to the official Rust documentation and resources.

## Advanced Types

Advanced types in Rust are types that have some special features or properties that are not common in other types. Some examples of advanced types are:

- Newtypes: these are types that wrap around another type and have a different name. They are useful for adding more meaning or functionality to existing types, such as implementing traits or enforcing invariants. For example, you can define a newtype called `Meters` that wraps around a `u32` and represents a distance in meters. You can then implement methods or traits for `Meters` that are specific to distances, such as converting to other units or adding or subtracting distances. To create a newtype, you use a tuple struct with one field that contains the underlying type. For example:

```rust
struct Meters(u32);

impl Meters {
    // methods for Meters
}

impl std::ops::Add for Meters {
    // implement the + operator for Meters
}
```

- Type aliases: these are types that have the same name as another type, but are declared with the `type` keyword. They are useful for simplifying complex or long type names, such as generic types or function types. For example, you can define a type alias called `Result<T>` that is equivalent to `std::result::Result<T, Box<dyn std::error::Error>>`, which is a common type for returning errors in Rust. You can then use `Result<T>` instead of the full type name whenever you need to. To create a type alias, you use the `type` keyword followed by the new name and the existing type. For example:

```rust
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
```

- The never type: this is a type that represents the absence of a value, and is written as `!`. It is useful for indicating that a function or expression will never return normally, such as when it always panics, loops forever, or exits the program. For example, you can use the never type as the return type of a function that always panics, such as `fn fail() -> !`. The never type is also compatible with any other type, which means that you can use it in places where the compiler expects a certain type, such as in match arms or if-else expressions. For example, you can write something like this:

```rust
let x = match some_value {
    Ok(n) => n,
    Err(e) => panic!("Error: {}", e),
};
```

Here, the compiler infers that the type of `x` is the same as the type of `n`, even though the `Err` branch returns `!`. This is because `!` can be converted to any type, so it doesn't affect the type inference.

- Dynamically sized types: these are types that have an unknown size at compile time, and are written as `?Sized`. They are useful for working with types that can have different sizes depending on the runtime conditions, such as slices, strings, or trait objects. For example, you can use a dynamically sized type as a generic parameter of a struct or a function, such as `struct MySlice<T: ?Sized> { data: *const T, len: usize }` or `fn print_slice<T: ?Sized + std::fmt::Display>(slice: &T)`. Dynamically sized types can only be used behind pointers, such as references, boxes, or raw pointers, because they need a way to store their size information at runtime. For example, you can write something like this:

```rust
let s: &str = "hello"; // &str is a dynamically sized type
let b: Box<str> = Box::from("world"); // Box<str> is also a dynamically sized type
let r: &dyn std::fmt::Display = &s; // &dyn std::fmt::Display is a trait object, which is a dynamically sized type
```

Here, the references, boxes, and trait objects store the size of the dynamically sized types they point to, so they can be used normally.

If you want to learn more about advanced types in Rust, you can check out the following resources:

- The Rust book, Chapter 19.4: Advanced Types

## Advanced Functions and Closures

Rust is a powerful and expressive programming language that supports both functions and closures. Functions are named blocks of code that can take parameters and return values. Closures are anonymous functions that can capture variables from their surrounding environment. Both functions and closures can be passed as arguments to other functions, or returned from functions, using the fn type and the Fn traits.

In this tutorial, I will explain the differences between functions and closures (you can think this as a quick recap), how to use them effectively, and how to work with the advanced features of Rust related to functions and closures. I will also provide some coding examples to illustrate the concepts.

## Functions

Functions are defined using the fn keyword, followed by the function name, a list of parameters in parentheses, an optional return type, and a block of code. For example, the following function takes two integers as parameters and returns their sum:

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

Functions can be called by using their name and passing the arguments in parentheses. For example, the following code calls the add function and prints the result:

```rust
let result = add(2, 3);
println!("The result is {}", result);
```

Functions can also be assigned to variables or passed as arguments to other functions, using the fn type. The fn type is a function pointer, which means it points to the location of the function in memory. The syntax for specifying a function pointer is similar to that of a regular function, except that the function name is replaced by the fn keyword. For example, the following code declares a variable f that can hold a function pointer to any function that takes two i32 parameters and returns an i32:

```rust
let f: fn(i32, i32) -> i32;
```

The variable f can then be assigned to any function that matches the signature, such as the add function:

```rust
f = add;
```

The function pointer can then be used to call the function, just like a regular function:

```rust
let result = f(2, 3);
println!("The result is {}", result);
```

Function pointers can also be passed as arguments to other functions, using the same syntax. For example, the following function takes a function pointer as a parameter and calls it twice with the same argument, then returns the sum of the two results:

```rust
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
```

The do_twice function can then be called with any function that matches the signature, such as the following function that doubles its parameter:

```rust
fn double(x: i32) -> i32 {
    x * 2
}

let result = do_twice(double, 5);
println!("The result is {}", result);
```

This code prints The result is 20.

## Closures

Closures are anonymous functions that can capture variables from their surrounding environment. Closures are defined using vertical pipes (|) to enclose the list of parameters, followed by a block of code. For example, the following closure takes one i32 parameter and returns its square:

```rust
|x| x * x
```

Closures can be assigned to variables or passed as arguments to other functions, using the Fn, FnMut, or FnOnce traits. These traits are implemented by the compiler for any type that can be called as a function. The Fn trait means that the closure can be called multiple times and only borrows the captured variables immutably. The FnMut trait means that the closure can be called multiple times and mutates the captured variables. The FnOnce trait means that the closure can be called only once and consumes the captured variables.

The syntax for specifying a closure trait is similar to that of a function pointer, except that the fn keyword is replaced by the trait name. For example, the following code declares a variable f that can hold any closure that takes one i32 parameter and returns an i32, and implements the Fn trait:

```rust
let f: Fn(i32) -> i32;
```

The variable f can then be assigned to any closure that matches the signature and the trait, such as the following closure that adds one to its parameter:

```rust
f = |x| x + 1;
```

The closure can then be used to call the function, just like a regular function:

```rust
let result = f(2);
println!("The result is {}", result);
```

Closures can also be passed as arguments to other functions, using the same syntax. For example, the following function takes a closure as a parameter and calls it twice with the same argument, then returns the sum of the two results:

```rust
fn do_twice<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}
```

The do_twice function uses a generic type parameter F to accept any type that implements the Fn trait. The where clause specifies the trait bound for F. The do_twice function can then be called with any closure that matches the signature and the trait, such as the following closure that doubles its parameter:

```rust
let result = do_twice(|x| x * 2, 5);
println!("The result is {}", result);
```

This code prints The result is 20.

One of the main advantages of closures over functions is that closures can capture variables from their surrounding environment. This allows closures to access data that is not passed as parameters. For example, the following code defines a closure that takes one i32 parameter and adds it to another i32 variable that is defined outside the closure:

```rust
let y = 10;
let add_y = |x| x + y;
```

The closure add_y captures the variable y by reference, which means it borrows y immutably. The closure can then be used to call the function with any i32 argument, and it will add y to it:

```rust
let result = add_y(2);
println!("The result is {}", result);
```

This code prints The result is 12.

Closures can also capture variables by value, which means they take ownership of the variables. This is done by using the move keyword before the closure parameters. For example, the following code defines a closure that takes one i32 parameter and adds it to another i32 variable that is defined outside the closure, but captures it by value:

```rust
let y = 10;
let add_y = move |x| x + y;
```

The closure add_y captures the variable y by value, which means it moves y into the closure. The closure can then be used to call the function with any i32 argument, and it will add y to it:

```rust
let result = add_y(2);
println!("The result is {}", result);
```

This code prints The result is 12.

However, after the closure is defined, the variable y is no longer accessible in the outer scope, because it has been moved into the closure. For example, the following code will not compile, because it tries to use y after it has been moved:

```rust
let y = 10;
let add_y = move |x| x + y;
let result = add_y(2);
println!("The result is {}", result);
println!("y is {}", y); // error: use of moved value: `y`
```

Capturing variables by value is useful when the closure needs to outlive the scope where the variables are defined, such as when returning a closure from a function or passing a closure to another thread.

### Higher-Order Functions

Higher-order functions are functions that take one or more functions as arguments, or return a function as a result. Higher-order functions are useful for creating abstractions and composing behaviors. Rust supports higher-order functions, both with regular functions and closures.

We have already seen some examples of higher-order functions, such as the do_twice function that takes a function or a closure as an argument and calls it twice. Another example of a higher-order function is the map method provided by the Iterator trait in the standard library. The map method takes a closure as an argument and applies it to each element of the iterator, creating a new iterator with the transformed elements. For example, the following code uses the map method to turn a vector of numbers into a vector of strings:

```rust
let list_of_numbers = vec![1, 2, 3];
let list_of_strings: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string())
    .collect();
```

The collect method is another higher-order function that takes an iterator and converts it into a collection, such as a vector. The type of the collection must be specified, because the collect method is generic and can create different types of collections.

### Returning Closures

Functions can also return closures, which can be useful for creating custom iterators or function generators. However, returning closures is a bit tricky, because closures are represented by traits, and traits are not sized types, meaning they don't have a fixed size at compile time. Therefore, we cannot return a closure directly, but we have to use a pointer, such as a Box, a reference, or an Rc.

For example, the following code defines a function that returns a closure that adds a given value to its parameter:

```rust
fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |y| x + y)
}
```

### Dynamically Dispatched Functions

Dynamically dispatched functions are functions that are resolved at runtime, based on the actual type of the value that implements a trait. Dynamically dispatched functions are useful when you want to use multiple types that share a common trait, without knowing the exact type at compile time.

To use dynamically dispatched functions in Rust, you need to use trait objects. Trait objects are pointers to values that implement a specific trait, such as `&dyn Trait` or `Box<dyn Trait>`. The `dyn` keyword indicates that the trait is used for dynamic dispatch.

Trait objects can be passed as arguments to other functions, or returned from functions, using the trait name as the type. For example, the following function takes a trait object as a parameter and calls its `speak` method:

```rust
trait Speak {
    fn speak(&self) -> &str;
}

fn say_hello(speaker: &dyn Speak) {
    println!("Hello, {}", speaker.speak());
}
```

The `say_hello` function can accept any type that implements the `Speak` trait, such as the following structs:

```rust
struct Person {
    name: String,
}

impl Speak for Person {
    fn speak(&self) -> &str {
        &self.name
    }
}

struct Dog {
    name: String,
}

impl Speak for Dog {
    fn speak(&self) -> &str {
        "woof"
    }
}
```

To create a trait object, you need to use a reference or a pointer to the value that implements the trait, and cast it to the trait type. For example, the following code creates two trait objects and passes them to the `say_hello` function:

```rust
let person = Person {
    name: "Alice".to_string(),
};

let dog = Dog {
    name: "Bob".to_string(),
};

say_hello(&person as &dyn Speak); // prints "Hello, Alice"
say_hello(&dog as &dyn Speak); // prints "Hello, woof"
```

Alternatively, you can use the `Box` type to create a trait object on the heap, and avoid the explicit casting. For example:

```rust
let person = Box::new(Person {
    name: "Alice".to_string(),
});

let dog = Box::new(Dog {
    name: "Bob".to_string(),
});

say_hello(&*person); // prints "Hello, Alice"
say_hello(&*dog); // prints "Hello, woof"
```

The `Box` type implements the `Deref` trait, which allows you to use the `*` operator to dereference the pointer and get a reference to the value. The reference is then automatically coerced to a trait object.

You can also use other pointer types, such as `Rc` or `Arc`, to create trait objects, as long as they implement the `Deref` trait.

Trait objects can also be returned from functions, using the `Box` type or another pointer type. For example, the following function returns a trait object that implements the `Speak` trait:

```rust
fn make_speaker(kind: &str) -> Box<dyn Speak> {
    match kind {
        "person" => Box::new(Person {
            name: "Alice".to_string(),
        }),
        "dog" => Box::new(Dog {
            name: "Bob".to_string(),
        }),
        _ => panic!("Unknown kind"),
    }
}
```

The `make_speaker` function takes a string as a parameter and returns a `Box<dyn Speak>` depending on the value of the string. The `Box<dyn Speak>` type is a trait object that can hold any type that implements the `Speak` trait.

You can then use the trait object to call the `speak` method, or pass it to another function that accepts a trait object. For example:

```rust
let speaker = make_speaker("person");
println!("{}", speaker.speak()); // prints "Alice"
say_hello(&*speaker); // prints "Hello, Alice"
```

Dynamically dispatched functions have some advantages and disadvantages compared to statically dispatched functions, which are resolved at compile time. Dynamically dispatched functions are more flexible and can reduce code duplication, but they also have some runtime overhead and require the trait to be object-safe.

A trait is object-safe if it meets two conditions:

- The trait does not require `Self` to be `Sized`
- All of its methods are object-safe

A method is object-safe if it meets one of the following conditions:

- It does not have any generic type parameters
- It has a `where Self: Sized` bound

If a trait is not object-safe, you cannot use it for dynamic dispatch. For example, the following trait is not object-safe, because it has a generic type parameter:

```rust
trait Add<T> {
    fn add(&self, other: T) -> Self;
}
```

You cannot create a trait object of type `&dyn Add<T>`, because the compiler does not know the size of `T` at runtime. You can make the trait object-safe by adding a `where Self: Sized` bound to the method:

```rust
trait Add<T> {
    fn add(&self, other: T) -> Self
    where
        Self: Sized;
}
```

However, this means that you cannot call the `add` method on a trait object, because the trait object is not `Sized`. You can only call the `add` method on a concrete type that implements the trait.

## Macros

Macros are a powerful feature of Rust that allow you to write code that writes other code, which is called metaprogramming. Macros can help you reduce the amount of code you have to write and maintain, and also enable you to do things that are not possible with functions.

There are two types of macros in Rust: declarative macros and procedural macros. Declarative macros are defined with the macro_rules! keyword and use a match-like syntax to generate code based on the input. Procedural macros are functions that take a TokenStream as input and output another TokenStream, where a TokenStream is a sequence of tokens that represent Rust code. Procedural macros can be further divided into three kinds: custom derive macros, attribute-like macros, and function-like macros.

In this tutorial, I will show you how to create and use both declarative and procedural macros in Rust with some examples. I will also explain the benefits and limitations of each type of macro, and provide some resources for further learning.

Let's start with declarative macros. ¹

Declarative macros are the most common and easy to use form of macros in Rust. They are also sometimes called "macros by example" or "macro_rules! macros". You can think of them as a way of writing a match expression that operates on Rust code instead of values. For example, the println! macro is a declarative macro that takes a variable number of arguments and generates code to print them to the standard output.

To define a declarative macro, you use the macro_rules! keyword followed by the name of the macro and a block of rules. Each rule consists of a pattern and a block of code. The pattern specifies the structure and syntax of the macro invocation, and the code specifies the code that will replace the macro invocation. You can use $ to indicate a placeholder for a part of the macro input, and specify its type with a designator, such as expr for an expression, ident for an identifier, or tt for any token tree. You can also use + or \* to indicate repetition of a part of the input, and use () or [] or {} to group parts of the input. You can separate multiple rules with a comma, and use => to separate the pattern and the code.

Here is an example of a simple declarative macro that takes two expressions and adds them together:

```rust
// use macro_rules! <name of macro> {<Body>}
macro_rules! add {
    // match like arm for macro
    ($a:expr, $b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };
}

fn main() {
    // call to macro, $a=1 and $b=2
    let result = add!(1, 2);
    println!("The result is {}", result); // prints "The result is 3"
}
```

As you can see, the macro definition starts with macro_rules! add, followed by a block of rules. There is only one rule in this case, which matches any two expressions separated by a comma, and generates code that evaluates and adds them together. The macro invocation looks like a function call, but with an exclamation mark after the name. The macro input is passed as arguments to the macro, and the macro output replaces the macro call in the code. The macro output is wrapped in a block to avoid conflicts with other variables or expressions.

You can use this macro anywhere in your code, as long as you define it or import it before you use it. You can also pass any valid expressions to the macro, such as literals, variables, or function calls. For example, you can write:

```rust
let x = 10;
let y = 20;
let z = add!(x, y); // z = 30
let w = add!(add!(x, y), z); // w = 60
let v = add!(x, add!(y, z)); // v = 60
let u = add!(x, add!(y, add!(z, w))); // u = 150
```

You can also make the macro more flexible by allowing it to take a variable number of arguments, and adding them all together. To do this, you can use the + designator to indicate that the macro can take one or more expressions, and use recursion to iterate over them. For example, you can write:

```rust
macro_rules! add {
    // base case: one expression
    ($a:expr) => ($a);
    // recursive case: two or more expressions
    ($a:expr, $($b:expr),+) => {
        // add the first expression to the result of calling the macro with the rest of the expressions
        $a + add!($($b),+)
    };
}

fn main() {
    let result = add!(1, 2, 3, 4, 5); // result = 15
    println!("The result is {}", result);
}
```

In this case, the macro has two rules: one for the base case of one expression, and one for the recursive case of two or more expressions. The recursive rule uses the syntax $($b:expr),+ to indicate that the macro can take one or more expressions separated by commas, and assigns them to the variable $b. The syntax $($b),+ then expands to the list of expressions, separated by commas, and passes them to the macro call. The macro then adds the first expression to the result of the recursive call, until there is only one expression left.

This is a simple example of how to use declarative macros in Rust, but you can create more complex and powerful macros with this syntax. For example, you can create macros that generate structs, enums, traits, impls, tests, or any other Rust code. You can also use different designators, such as item, block, stmt, pat, ty, or path, to match different kinds of Rust syntax. You can also use conditional logic, such as if, else, or match, to generate different code based on the input. You can also use hygiene and scoping rules to avoid name conflicts and ensure that the macro output is valid Rust code.

However, declarative macros also have some limitations. For example, you cannot use them to implement traits, because traits require a type name, and you cannot match a type name with a designator. You also cannot use them to create custom attributes, because attributes are not part of the macro syntax. You also cannot use them to manipulate the abstract syntax tree (AST) of the Rust code, because they only operate on the token level. For these cases, you need to use procedural macros.

Procedural macros are a more advanced and powerful form of macros in Rust. They are also sometimes called "proc macros" or "compiler plugins". They are functions that take a TokenStream as input and output another TokenStream, where a TokenStream is a sequence of tokens that represent Rust code. Procedural macros can access and modify the AST of the Rust code, and generate any kind of Rust code. Procedural macros can be further divided into three kinds: custom derive macros, attribute-like macros, and function-like macros.

Custom derive macros allow you to automatically implement a trait for a struct or an enum by using the derive attribute. For example, you can use the #[derive(Debug)] attribute to implement the Debug trait for your type, which allows you to print it with the {:?} format specifier. Custom derive macros are defined in a separate crate with the proc-macro attribute, and use the syn and quote crates to parse and generate Rust code. For example, you can write:

```rust
// create a proc-macro crate
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// use the proc-macro attribute to indicate that this is a custom derive macro
#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // parse the input as a DeriveInput, which represents a struct or an enum
    let input = parse_macro_input!(input as DeriveInput);

    // get the name of the type
    let name = input.ident;

    // generate code that implements the HelloMacro trait for the type
    let expanded = quote! {
        // use the impl keyword to implement a trait
        impl HelloMacro for #name {
            // define the hello_macro method
            fn hello_macro() {
                // use the stringify! macro to convert the type name to a string
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    // convert the generated code to a TokenStream and return it
    TokenStream::from(expanded)
}
```

In this case, the macro definition starts with the #[proc_macro_derive(HelloMacro)] attribute, which indicates that this is a custom derive macro named HelloMacro. The macro function takes a TokenStream as input and returns a TokenStream as output. The input is parsed as a DeriveInput, which represents a struct or an enum. The input.ident field gives the name of the type. The quote! macro is used to generate Rust code as a TokenStream, using # to interpolate variables. The generated code implements the HelloMacro trait for the type, which defines a hello_macro method that prints the type name. The output is converted to a TokenStream and returned.

To use this macro, you need to create another crate that depends on the proc-macro crate, and use the derive attribute on your type. For example, you can write:

```rust
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}", stringify!(#name));
            }
        }
    };

    TokenStream::from(expanded)
}
```

## Attribute-like Macros

Attribute-like macros are procedural macros that define custom attributes that can be used on any item, such as functions, structs, enums, modules, etc. Attribute-like macros can modify or generate code based on the item they are applied to. For example, you can use an attribute-like macro to implement a custom logging system, or to generate getters and setters for a struct.

To define an attribute-like macro, you use the #[proc_macro_attribute] attribute on a function that takes two TokenStream parameters and returns a TokenStream. The first parameter is the attribute input, which contains the arguments passed to the attribute. The second parameter is the item input, which contains the item the attribute is applied to. The output is the modified or generated code that replaces the original item.

Here is an example of a simple attribute-like macro that adds a prefix to the name of a function:

```rust
// use the proc_macro_attribute attribute to indicate that this is an attribute-like macro
#[proc_macro_attribute]
pub fn add_prefix(attr: TokenStream, item: TokenStream) -> TokenStream {
    // parse the attribute input as a single expression, which is the prefix
    let prefix = parse_macro_input!(attr as Expr);

    // parse the item input as a function
    let mut function = parse_macro_input!(item as ItemFn);

    // get the original function name
    let original_name = function.sig.ident.clone();

    // create a new function name by concatenating the prefix and the original name
    let new_name = format!("{}_{}", prefix.to_token_stream(), original_name);

    // replace the function name with the new name
    function.sig.ident = Ident::new(&new_name, original_name.span());

    // convert the modified function to a TokenStream and return it
    TokenStream::from(quote! {
        #function
    })
}
```

In this case, the macro definition starts with the #[proc_macro_attribute] attribute, which indicates that this is an attribute-like macro named add_prefix. The macro function takes two TokenStream parameters: attr and item. The attr parameter is parsed as an expression, which is the prefix to be added to the function name. The item parameter is parsed as a function, which is the item the attribute is applied to. The function name is modified by concatenating the prefix and the original name, and the modified function is converted to a TokenStream and returned.

To use this macro, you need to create another crate that depends on the proc-macro crate, and use the attribute on your function. For example, you can write:

```rust
// create another crate that depends on the proc-macro crate
use add_prefix::add_prefix;

// use the attribute on a function, passing the prefix as an argument
#[add_prefix(hello)]
fn world() {
    println!("Hello, world!");
}

fn main() {
    // call the modified function, which has the prefix added to its name
    hello_world();
}
```

As you can see, the attribute is used on the function with the syntax #[add_prefix(hello)], where hello is the prefix to be added to the function name. The attribute modifies the function name to hello_world, and the function can be called with the new name.

This is a simple example of how to use attribute-like macros in Rust, but you can create more complex and powerful macros with this syntax. For example, you can create macros that generate code based on the type or attributes of the item, or that perform some analysis or validation on the item.

## Function-like Macros

Function-like macros are procedural macros that look like function calls but operate on the tokens specified as their argument. Function-like macros are similar to declarative macros, but they have more flexibility and power, as they can manipulate the AST of the Rust code and generate any kind of Rust code.

To define a function-like macro, you use the #[proc_macro] attribute on a function that takes a TokenStream parameter and returns a TokenStream. The parameter is the macro input, which contains the tokens passed to the macro. The output is the macro output, which contains the tokens that replace the macro call.

Here is an example of a simple function-like macro that reverses the order of the tokens passed to it:

```rust
// use the proc_macro attribute to indicate that this is a function-like macro
#[proc_macro]
pub fn reverse(input: TokenStream) -> TokenStream {
    // convert the input to a vector of tokens
    let mut tokens = input.into_iter().collect::<Vec<_>>();

    // reverse the order of the tokens
    tokens.reverse();

    // convert the vector of tokens to a TokenStream and return it
    TokenStream::from_iter(tokens)
}
```

In this case, the macro definition starts with the #[proc_macro] attribute, which indicates that this is a function-like macro named reverse. The macro function takes a TokenStream parameter, which is the macro input. The input is converted to a vector of tokens, which is then reversed. The reversed vector of tokens is converted back to a TokenStream and returned.

To use this macro, you need to create another crate that depends on the proc-macro crate, and use the macro as an expression. For example, you can write:

```rust
// create another crate that depends on the proc-macro crate
use reverse::reverse;

fn main() {
    // use the macro as an expression, passing some tokens to it
    let x = reverse!(1 + 2 * 3);

    // print the result of the expression, which is the reverse of the tokens
    println!("x = {}", x); // prints "x = 3 * 2 + 1"
}
```

As you can see, the macro is used as an expression with the syntax reverse!(1 + 2 _ 3), where 1 + 2 _ 3 are the tokens passed to the macro. The macro reverses the order of the tokens and returns the expression 3 \* 2 + 1, which is then assigned to x and printed.

This is a simple example of how to use function-like macros in Rust, but you can create more complex and powerful macros with this syntax. For example, you can create macros that parse and generate custom syntax, such as SQL queries or HTML templates.
