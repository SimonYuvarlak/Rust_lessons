# Lifetimes

## Part 1

Lifetimes are a feature of Rust that help the compiler prevent dangling references and memory leaks. They are a way of describing how long a reference is valid for, and how it relates to other references in the same scope. In this tutorial, I will explain the basics of lifetimes and how to use them in Rust.

The first thing to understand is that every reference in Rust has a lifetime, which is the scope for which that reference is valid. For example, consider this code:

```rust
fn main() {
    let x = 5; // x has a lifetime of the entire main function
    let r = &x; // r has a lifetime of the inner scope
    println!("r: {}", r); // r is valid here
} // x and r go out of scope and are dropped
```

Here, we have two variables: x, which is an integer, and r, which is a reference to x. The variable x has a lifetime of the entire main function, because it is declared at the top level. The variable r has a lifetime of the inner scope, because it is declared inside the curly braces. The reference r is valid as long as it ends before the variable x is dropped, which happens at the end of the main function. This code compiles and runs without any errors.

However, if we try to extend the lifetime of r beyond the lifetime of x, we will get a compile-time error. For example, consider this code:

```rust
fn main() {
    let r; // r has no initial value
    {
        let x = 5; // x has a lifetime of the inner scope
        r = &x; // r has a lifetime of the inner scope
    } // x goes out of scope and is dropped
    println!("r: {}", r); // r is not valid here
} // r goes out of scope and is dropped
```

Here, we have the same variables as before, but we declare r at the top level, without giving it an initial value. Then, we assign r to a reference to x, which is declared in the inner scope. The problem is that x goes out of scope and is dropped at the end of the inner scope, but r tries to use it after that. This is a dangling reference, which means that r is pointing to a memory location that is no longer valid. This code will not compile, and we will get an error message like this:

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x; // r has a lifetime of the inner scope
  |             ^^ borrowed value does not live long enough
7 |     } // x goes out of scope and is dropped
  |     - `x` dropped here while still borrowed
8 |     println!("r: {}", r); // r is not valid here
  |                        - borrow later used here
```

The error message tells us that x does not live long enough, and that it is dropped while still borrowed by r. It also tells us where the borrow occurs and where it is used later. This is how the Rust compiler, or more specifically, its borrow checker, ensures that all references are valid and do not cause memory errors.

To fix this error, we need to make sure that the lifetime of r is shorter than or equal to the lifetime of x. One way to do that is to move the declaration of r inside the inner scope, like this:

```rust
fn main() {
    {
        let x = 5; // x has a lifetime of the inner scope
        let r = &x; // r has a lifetime of the inner scope
        println!("r: {}", r); // r is valid here
    } // x and r go out of scope and are dropped
} // nothing to drop here
```

Now, both x and r have the same lifetime, and r is valid as long as x is valid. This code compiles and runs without any errors.

Another way to fix the error is to use a return value instead of a reference, like this:

```rust
fn main() {
    let r = foo(); // r has a lifetime of the main function
    println!("r: {}", r); // r is valid here
} // r goes out of scope and is dropped

fn foo() -> i32 {
    let x = 5; // x has a lifetime of the foo function
    x // x is returned by value, not by reference
} // x goes out of scope and is dropped
```

Here, we have a function foo that returns an integer by value, not by reference. The variable x has a lifetime of the foo function, and it is dropped at the end of the function. However, the return value is copied to the variable r, which has a lifetime of the main function. The variable r is valid as long as the main function is running. This code compiles and runs without any errors.

So far, we have seen how lifetimes work implicitly, without any annotations. Most of the time, lifetimes are inferred by the compiler, and we don't need to write them explicitly. However, sometimes we need to annotate lifetimes when the compiler cannot infer them automatically, or when we want to express some constraints on the lifetimes of references. This is especially true when we have functions that take references as parameters or return references as values.

To annotate lifetimes, we use generic lifetime parameters, which are similar to generic type parameters. We write them inside angle brackets, after the function name, and before the parameter list. For example, consider this function signature:

```rust
fn bar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    // some code here
}
```

Here, we have a function bar that takes two references to integers as parameters, and returns a reference to an integer as a value. We also have a generic lifetime parameter 'a, which is written inside angle brackets, after the function name. The lifetime parameter 'a is used to annotate the lifetimes of the references, by prefixing them with an apostrophe and the parameter name. This means that all the references in the function signature have the same lifetime 'a, and that the return value has the same lifetime as the parameters.

The lifetime parameter 'a is a way of describing the relationship between the lifetimes of the references in the function signature. It does not specify the exact duration of the lifetime, but rather how it relates to other lifetimes. For example, the lifetime parameter 'a could mean any of the following:

- The lifetime of the references is the same as the lifetime of the function call.
- The lifetime of the references is the same as the lifetime of the smallest scope that contains the references.
- The lifetime of the references is the same as the lifetime of some other reference that is passed to or returned from the function.

The exact meaning of the lifetime parameter 'a depends on the context where the function is called and how the references are used. The compiler will check that the lifetime parameter 'a is valid for all possible scenarios, and that it does not cause any dangling references or memory errors.

For example, consider this code:

```rust
fn main() {
    let x = 5; // x has a lifetime of the main function
    let y = 6; // y has a lifetime of the main function
    let z = bar(&x, &y); // z has a lifetime of the main function
    println!("z: {}", z); // z is valid here
} // x, y, and z go out of scope and are dropped

fn bar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x // return a reference to x
    } else {
        y // return a reference to y
    }
}
```

Here, we have the same function bar as before, and we call it from the main function with two references to integers, x and y. The function bar returns a reference to the larger of the two integers, either x or y. The return value is assigned to the variable z, which is also a reference to an integer.

In this case, the lifetime parameter 'a means the lifetime of the main function, because that is the smallest scope that contains all the references. The references x, y, and z all have the same lifetime, which is the lifetime of the main function. The function bar returns a reference that has the same lifetime as the parameters, which is also the lifetime of the main function. This code compiles and runs without any errors, because all the references are valid for the entire duration of the main function.

However, if we try to change the lifetimes of the references, we will get a compile-time error. For example, consider this code:

```rust
fn main() {
    let z; // z has no initial value
    {
        let x = 5; // x has a lifetime of the inner scope
        let y = 6; // y has a lifetime of the inner scope
        z = bar(&x, &y); // z has a lifetime of the inner scope
    } // x and y go out of scope and are dropped
    println!("z: {}", z); // z is not valid here
} // z goes out of scope and is dropped

fn bar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x // return a reference to x
    } else {
        y // return a reference to y
    }
}
```

## Part 2

In the previous part, we learned how to use generic lifetime parameters to annotate the lifetimes of references in a function signature. We also saw how the compiler checks that the lifetimes are valid and do not cause any dangling references. In this part, we will learn about some advanced features and rules of lifetimes, such as:

- Lifetime elision: how the compiler can infer some lifetimes automatically, without explicit annotations.
- The 'static lifetime: a special lifetime that lasts for the entire duration of the program.
- The 'a and 'b conventions: how to name multiple lifetimes in a function signature, and what they mean.

Let's start with lifetime elision. Lifetime elision is a set of rules that the compiler uses to infer the lifetimes of references in a function signature, when they are not explicitly annotated. This makes the code more concise and readable, without losing the safety guarantees of lifetimes. The lifetime elision rules are based on the following principles:

- Each parameter that is a reference gets its own lifetime parameter. For example, a function that takes two references as parameters will have two lifetime parameters, one for each reference.
- If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters. For example, a function that takes one reference as a parameter and returns a reference as a value will have the same lifetime for both the input and the output.
- If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. For example, a method that takes a reference to self and another reference as parameters and returns a reference as a value will have the same lifetime as self for both the input and the output.

These rules cover most of the common cases where lifetimes can be inferred. However, there are some cases where lifetimes cannot be inferred, and explicit annotations are required. For example, consider this function signature:

```rust
fn baz(x: &i32, y: &i32) -> &i32 {
    // some code here
}
```

This function takes two references as parameters and returns a reference as a value. However, the compiler cannot infer the lifetimes of the references, because there are multiple input lifetime parameters, and none of them is self. Therefore, this function signature is invalid, and we need to annotate the lifetimes explicitly, like this:

```rust
fn baz<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    // some code here
}
```

Here, we have a generic lifetime parameter 'a, which is used to annotate the lifetimes of all the references in the function signature. This means that the input and the output references have the same lifetime 'a, and that the function returns a reference that has the same lifetime as the parameters.

However, this is not the only way to annotate the lifetimes. We could also use different lifetime parameters for different references, like this:

```rust
fn baz<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    // some code here
}
```

Here, we have two generic lifetime parameters 'a and 'b, which are used to annotate the lifetimes of the input references. The output reference has the lifetime 'a, which means that it has the same lifetime as the first parameter x. The second parameter y has a different lifetime 'b, which means that it is independent of the first parameter and the output.

The choice of the lifetime parameters depends on the logic of the function and the intended behavior of the references. For example, the function baz could be implemented as follows:

```rust
fn baz<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        x // return a reference to x
    } else {
        y // return a reference to y
    }
}
```

This function returns a reference to the larger of the two input integers, either x or y. However, this function is invalid, because it violates the lifetime annotation. The function signature says that the output reference has the lifetime 'a, which is the same as the first parameter x. However, the function body may return a reference to y, which has a different lifetime 'b. This could cause a dangling reference, if y goes out of scope before x. Therefore, the compiler will reject this function, and we will get an error message like this:

```text
error[E0491]: return type references lifetime `'a`, but the anonymous lifetime #2 defined on the function body only lasts for the lifetime `'b`
 --> src/main.rs:1:38
  |
1 | fn baz<'a, 'b>(x: &'a i32, y: &'b i32) -> &'a i32 {
  |                                      ^^^^^^^^^^^ return type references lifetime `'a`
2 |     if x > y {
3 |         x // return a reference to x
4 |     } else {
5 |         y // return a reference to y
  |         - this returned value is of type `&'b i32`, which does not outlive `'a`
```

The error message tells us that the return type references lifetime 'a, but the function body may return a value of type &'b i32, which does not outlive 'a. It also tells us where the lifetimes are defined and where they are used. This is how the compiler ensures that the lifetimes are consistent and do not cause any memory errors.

To fix this error, we need to change the lifetime annotation to match the logic of the function. One way to do that is to use the same lifetime for all the references, like this:

```rust
fn baz<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x // return a reference to x
    } else {
        y // return a reference to y
    }
}
```

Now, the function signature says that the input and the output references have the same lifetime 'a, and that the function returns a reference that has the same lifetime as the parameters. This means that the function can return either x or y, as long as they have the same lifetime. This function is valid, and it will compile and run without any errors.

Another way to fix the error is to use different lifetimes for the input and the output references, and add a constraint that the output lifetime is shorter than or equal to the input lifetimes, like this:

```rust
fn baz<'a, 'b, 'c>(x: &'a i32, y: &'b i32) -> &'c i32
where
    'a: 'c,
    'b: 'c,
{
    if x > y {
        x // return a reference to x
    } else {
        y // return a reference to y
    }
}
```

Here, we have three generic lifetime parameters 'a, 'b, and 'c, which are used to annotate the lifetimes of the input and the output references. The output reference has the lifetime 'c, which is different from the input lifetimes 'a and 'b. However, we also have a where clause, which adds some constraints on the lifetimes. The where clause says that 'a: 'c and 'b: 'c, which means that the lifetime 'c is shorter than or equal to the lifetimes 'a and 'b. This ensures that the output reference will not outlive the input references, and that the function can return either x or y, as long as they are valid for the lifetime 'c. This function is also valid, and it will compile and run without any errors.

The choice of the lifetimes and the constraints depends on the logic of the function and the intended behavior of the references. There is no single correct way to annotate the lifetimes, but there are some conventions that are commonly used. One of them is to use the names 'a and 'b for the input lifetimes, and 'c for the output lifetime, as we did in the last example. Another convention is to use the name 'r for a generic lifetime parameter, when there is only one input lifetime parameter, or when the lifetimes of all the references are the same. For example, consider this function signature:

```rust
fn qux(x: &i32) -> &i32 {
    // some code here
}
```

This function takes one reference as a parameter and returns a reference as a value. The compiler can infer the lifetimes of the references using the lifetime elision rules, but we can also annotate them explicitly, like this:

```rust
fn qux<'r>(x: &'r i32) -> &'r i32 {
    // some code here
}
```

Here, we have a generic lifetime parameter 'r, which is used to annotate the lifetimes of both the input and the output references. This means that the input and the output references have the same lifetime 'r, and that the function returns a reference that has the same lifetime as the parameter. The name 'r is a convention for a generic lifetime parameter, when there is only one input lifetime parameter, or when the lifetimes of all the references are the same.

## Part 3

In the previous part, we learned about some advanced features and rules of lifetimes, such as lifetime elision, the 'static lifetime, and the 'a and 'b conventions. In this part, we will learn about some common lifetime-related errors and how to fix them.

One of the most common lifetime-related errors is the **borrowed value does not live long enough** error. This error occurs when a reference is created in a scope that is shorter than the scope where it is used. For example, consider this code:

```rust
fn main() {
    let r; // r has no initial value
    {
        let x = 5; // x has a lifetime of the inner scope
        r = &x; // r has a lifetime of the inner scope
    } // x goes out of scope and is dropped
    println!("r: {}", r); // r is not valid here
} // r goes out of scope and is dropped
```

Here, we have the same code as before, where we try to extend the lifetime of r beyond the lifetime of x. This is a dangling reference, which means that r is pointing to a memory location that is no longer valid. This code will not compile, and we will get an error message like this:

```text
error[E0597]: `x` does not live long enough
 --> src/main.rs:6:13
  |
6 |         r = &x; // r has a lifetime of the inner scope
  |             ^^ borrowed value does not live long enough
7 |     } // x goes out of scope and is dropped
  |     - `x` dropped here while still borrowed
8 |     println!("r: {}", r); // r is not valid here
  |                        - borrow later used here
```

The error message tells us that x does not live long enough, and that it is dropped while still borrowed by r. It also tells us where the borrow occurs and where it is used later. This is how the Rust compiler, or more specifically, its borrow checker, ensures that all references are valid and do not cause memory errors.

To fix this error, we need to make sure that the lifetime of r is shorter than or equal to the lifetime of x. One way to do that is to move the declaration of r inside the inner scope, like this:

```rust
fn main() {
    {
        let x = 5; // x has a lifetime of the inner scope
        let r = &x; // r has a lifetime of the inner scope
        println!("r: {}", r); // r is valid here
    } // x and r go out of scope and are dropped
} // nothing to drop here
```

Now, both x and r have the same lifetime, and r is valid as long as x is valid. This code compiles and runs without any errors.

Another way to fix the error is to use a return value instead of a reference, like this:

```rust
fn main() {
    let r = foo(); // r has a lifetime of the main function
    println!("r: {}", r); // r is valid here
} // r goes out of scope and is dropped

fn foo() -> i32 {
    let x = 5; // x has a lifetime of the foo function
    x // x is returned by value, not by reference
} // x goes out of scope and is dropped
```

Here, we have a function foo that returns an integer by value, not by reference. The variable x has a lifetime of the foo function, and it is dropped at the end of the function. However, the return value is copied to the variable r, which has a lifetime of the main function. The variable r is valid as long as the main function is running. This code compiles and runs without any errors.

Another common lifetime-related error is the **mismatched types** error. This error occurs when the lifetimes of the references do not match the expected lifetimes. For example, consider this code:

```rust
fn main() {
    let x = 5; // x has a lifetime of the main function
    let y = &x; // y has a lifetime of the main function
    let z = bar(y); // z has a lifetime of the main function
    println!("z: {}", z); // z is valid here
} // x, y, and z go out of scope and are dropped

fn bar<'a>(x: &'a i32) -> &'a i32 {
    let y = 6; // y has a lifetime of the bar function
    &y // return a reference to y
} // y goes out of scope and is dropped
```

Here, we have a function bar that takes a reference to an integer as a parameter and returns a reference to an integer as a value. We also have a generic lifetime parameter 'a, which is used to annotate the lifetimes of the input and the output references. The function bar creates a new variable y, which is an integer, and returns a reference to it. The function bar is called from the main function with a reference to x, which is also an integer. The return value is assigned to the variable z, which is also a reference to an integer.

However, this code is invalid, because the lifetimes of the references do not match. The function bar returns a reference that has the lifetime 'a, which is the same as the input lifetime. However, the input lifetime is the lifetime of the main function, because that is the scope where the reference y is created. The reference y points to the variable y, which has a lifetime of the bar function, because that is the scope where the variable y is created. The variable y goes out of scope and is dropped at the end of the bar function, but the reference y tries to use it after that. This is a dangling reference, which means that y is pointing to a memory location that is no longer valid. This code will not compile, and we will get an error message like this:

```text
error[E0106]: missing lifetime specifier
 --> src/main.rs:10:5
  |
10 |     &y // return a reference to y
  |     ^^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x`
```

The error message tells us that the return type contains a borrowed value, but the signature does not say whether it is borrowed from x. It also tells us that we need to specify a named lifetime parameter for the return type. This is how the compiler detects that the lifetimes of the references do not match the expected lifetimes.

To fix this error, we need to change the lifetime annotation to match the logic of the function. One way to do that is to use a different lifetime for the output reference, and add a constraint that the output lifetime is shorter than or equal to the input lifetime, like this:

```rust
fn main() {
    let x = 5; // x has a lifetime of the main function
    let y = &x; // y has a lifetime of the main function
    let z = bar(y); // z has a lifetime of the bar function
    println!("z: {}", z); // z is valid here
} // x, y, and z go out of scope and are dropped

fn bar<'a, 'b>(x: &'a i32) -> &'b i32
where
    'b: 'a,
{
    let y = 6; // y has a lifetime of the bar function
    &y // return a reference to y
} // y goes out of scope and is dropped
```

Here, we have two generic lifetime parameters 'a and 'b, which are used to annotate the lifetimes of the input and the output references. The input reference has the lifetime 'a, which is the same as the lifetime of the main function. The output reference has the lifetime 'b, which is different from the input lifetime. However, we also have a where clause, which adds a constraint on the lifetimes. The where clause says that 'b: 'a, which means that the lifetime 'b is shorter than or equal to the lifetime 'a. This ensures that the output reference will not outlive the input reference, and that the function can return a reference to y, as long as it is valid for the lifetime 'b. This code is valid, and it will compile and run without any errors.

Another way to fix the error is to use a return value instead of a reference, like this:

```rust
fn main() {
    let x = 5; // x has a lifetime of the main function
    let y = &x; // y has a lifetime of the main function
    let z = bar(y); // z has a lifetime of the main function
    println!("z: {}", z); // z is valid here
} // x, y, and z go out of scope and are dropped

fn bar<'a>(x: &'a i32) -> i32 {
    let y = 6; // y has a lifetime of the bar function
    y // return y by value, not by reference
} // y goes out of scope and is dropped
```

Here, we have a function bar that takes a reference to an integer as a parameter and returns an integer as a value. We also have a generic lifetime parameter 'a, which is used to annotate the lifetime of the input reference. The function bar creates a new variable y, which is an integer, and returns it by value, not by reference.
