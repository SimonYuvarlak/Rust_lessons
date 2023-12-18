# Closures and Iterators

Rust is a programming language that focuses on performance, reliability, and productivity. One of the features that Rust offers is the ability to use closures and iterators to write concise and efficient code. In this tutorial, we will learn what closures and iterators are, how to use them, and some of the benefits they provide.

## Closures

A closure is a function that can capture the environment in which it is defined. This means that a closure can access and modify variables that are outside of its scope, as long as they are in the same or a parent scope. Closures are also known as anonymous functions, lambda expressions, or function literals.

The syntax for defining a closure in Rust is similar to that of a regular function, except that the parameter types and return type are optional. For example, this is a closure that takes two integers and returns their sum:

```rust
let add = |x, y| x + y;
```

The closure is assigned to a variable called `add`, which can be used like any other function. The closure can infer the types of the parameters and the return type from the context, so we don't need to specify them explicitly. However, if we want to be more explicit, we can write the closure like this:

```rust
let add: fn(i32, i32) -> i32 = |x: i32, y: i32| -> i32 { x + y };
```

This specifies that the closure has the type `fn(i32, i32) -> i32`, which is the same as the type of a regular function that takes two integers and returns an integer. The parameter types and the return type are also annotated in the closure body.

One of the main advantages of closures is that they can capture variables from the environment. For example, suppose we have a variable called `factor` that we want to use in a closure:

```rust
let factor = 2;
let multiply_by_factor = |x| x * factor;
```

The closure `multiply_by_factor` can access and use the variable `factor`, even though it is not a parameter of the closure. This is because the closure captures the variable by reference, meaning that it borrows a reference to the variable and uses it in the closure body. This way, the closure can use the current value of the variable, even if it changes later.

However, capturing variables by reference also means that the closure can modify the variable, which can lead to unexpected behavior. For example, suppose we have a closure that increments a variable by one:

```rust
let mut count = 0;
let increment = || count += 1;
```

The closure `increment` can mutate the variable `count`, which is captured by reference. This means that every time we call the closure, the value of `count` will change. For example, if we call the closure three times, the value of `count` will be 3:

```rust
increment();
increment();
increment();
println!("{}", count); // prints 3
```

This can be useful in some cases, but it can also introduce bugs and make the code harder to reason about. To avoid this, we can use different ways of capturing variables in closures, such as by value or by mutable reference. We will see how to do this in the next section.

## Closure usage examples

In this section, we will see some examples of how to use closures in Rust. We will also see how to use different ways of capturing variables in closures, such as by value, by reference, or by mutable reference.

### Capturing by value

One way to capture variables in closures is by value, meaning that the closure takes ownership of the variable and moves it into the closure body. This way, the closure can use the variable without affecting the original variable. To capture a variable by value, we need to use the `move` keyword before the closure parameters. For example, this is a closure that captures a variable by value:

```rust
let name = String::from("Alice");
let greet = move || println!("Hello, {}", name);
```

The closure `greet` takes ownership of the variable `name` and moves it into the closure body. This means that the closure can use the variable `name` without borrowing it from the environment. However, this also means that the variable `name` is no longer available in the original scope, since it has been moved. For example, if we try to use the variable `name` after the closure definition, we will get a compile-time error:

```rust
let name = String::from("Alice");
let greet = move || println!("Hello, {}", name);
println!("{}", name); // error: use of moved value: `name`
```

The error message tells us that we are trying to use a value that has been moved, which is not allowed in Rust. This is because Rust enforces the ownership rules, which prevent us from using a value after it has been moved to another owner. This way, Rust ensures that there are no dangling references or memory leaks in the code.

Capturing by value can be useful when we want to use a variable in a closure without affecting the original variable. For example, suppose we have a vector of numbers that we want to sort in ascending order. We can use the `sort_by` method of the vector, which takes a closure as an argument. The closure defines how to compare two elements of the vector. For example, this is how we can sort the vector by using the absolute value of the elements:

```rust
let mut numbers = vec![-10, 5, -3, 2, 7];
let abs = |x| x.abs();
numbers.sort_by(|a, b| abs(a).cmp(&abs(b)));
println!("{:?}", numbers); // prints [2, -3, 5, 7, -10]
```

The closure `abs` takes a number and returns its absolute value. The closure `|a, b| abs(a).cmp(&abs(b))` takes two numbers and compares them by using the `cmp` method of the absolute value. The `sort_by` method uses this closure to sort the vector in ascending order of the absolute value.

However, suppose we want to sort the vector in descending order of the absolute value. We can use the `reverse` method of the vector, which reverses the order of the elements. However, this will also reverse the order of the original values, which we may not want. For example, this is what happens if we use the `reverse` method:

```rust
let mut numbers = vec![-10, 5, -3, 2, 7];
let abs = |x| x.abs();
numbers.sort_by(|a, b| abs(a).cmp(&abs(b)));
numbers.reverse();
println!("{:?}", numbers); // prints [10, -7, -5, 3, -2]
```

The vector is sorted in descending order of the absolute value, but the original values are also reversed. This may not be the desired result, since we may want to preserve the original order of the values. To avoid this, we can capture the absolute value by value and use it in the closure. For example, this is how we can do that:

```rust
let mut numbers = vec![-10, 5, -3, 2, 7];
let abs = |x| x.abs();
let abs_values: Vec<_> = numbers.iter().map(abs).collect();
numbers.sort_by(|a, b| abs_values[b].cmp(&abs_values[a]));
println!("{:?}", numbers); // prints [-10, 7, 5, -3, 2]
```

The closure `abs` takes a number and returns its absolute value. The expression `numbers.iter().map(abs).collect()` creates a new vector that contains the absolute values of the elements of the original vector. The closure `|a, b| abs_values[b].cmp(&abs_values[a])` takes two indices of the original vector and compares them by using the `cmp` method of the corresponding elements of the new vector. The `sort_by` method uses this closure to sort the original vector in descending order of the absolute value, but without reversing the original values.

By capturing the absolute value by value, we can use it in the closure without affecting the original vector. This way, we can sort the vector in the desired order without modifying the original values.

### Capturing by reference

Another way to capture variables in closures is by reference, meaning that the closure borrows a reference to the variable and uses it in the closure body. This way, the closure can use the variable without taking ownership of it. This is the default way of capturing variables in closures, so we don't need to use any keyword to indicate it. For example, this is a closure that captures a variable by reference:

```rust
let name = String::from("Alice");
let greet = || println!("Hello, {}", name);
```

The closure `greet` borrows a reference to the variable `name` and uses it in the closure body. This means that the closure can use the variable `name` without moving it from the environment. However, this also means that the variable `name` is still available in the original scope, since it has not been moved. For example, we can use the variable `name` after the closure definition, without any error:

```rust
let name = String::from("Alice");
let greet = || println!("Hello, {}", name);
println!("{}", name); // prints Alice
```

There is no error message, since we are not trying to use a value that has been moved. This is because Rust enforces the ownership rules, which prevent us from using a value after it has been moved to another owner. This way, Rust ensures that there are no dangling references or memory leaks in the code.

### Capturing by mutable reference

Another way to capture variables in closures is by mutable reference, meaning that the closure borrows a mutable reference to the variable and can modify it in the closure body. This way, the closure can use and change the variable without taking ownership of it. To capture a variable by mutable reference, we need to use the `&mut` keyword before the variable name. For example, this is a closure that captures a variable by mutable reference:

```rust
let mut count = 0;
let increment = || {
    count += 1;
    println!("Count is {}", count);
};
```

The closure `increment` borrows a mutable reference to the variable `count` and can modify it in the closure body. This means that the closure can use and change the variable `count` without moving it from the environment. However, this also means that the variable `count` is still available in the original scope, but it cannot be used or modified by any other code while the closure is using it. For example, if we try to use the variable `count` after the closure definition, we will get a compile-time error:

```rust
let mut count = 0;
let increment = || {
    count += 1;
    println!("Count is {}", count);
};
println!("{}", count); // error: cannot use `count` because it was mutably borrowed
```

The error message tells us that we cannot use `count` because it was mutably borrowed by the closure, which means that the closure has exclusive access to the variable. This is because Rust enforces the borrowing rules, which prevent us from having multiple mutable references or a mutable and an immutable reference to the same value at the same time. This way, Rust ensures that there are no data races or memory corruption in the code.

Capturing by mutable reference can be useful when we want to use and change a variable in a closure without taking ownership of it. For example, suppose we have a vector of numbers that we want to filter by some condition. We can use the `retain` method of the vector, which takes a closure as an argument. The closure defines the condition that the elements of the vector must satisfy to be kept. For example, this is how we can filter the vector by keeping only the even numbers:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6];
let is_even = |x| x % 2 == 0;
numbers.retain(is_even);
println!("{:?}", numbers); // prints [2, 4, 6]
```

The closure `is_even` takes a number and returns true if it is even and false otherwise. The `retain` method uses this closure to filter the vector by keeping only the elements that return true.

However, suppose we want to filter the vector by keeping only the numbers that are divisible by a certain factor. We can use a variable to store the factor and use it in the closure. For example, this is how we can filter the vector by keeping only the numbers that are divisible by 3:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6];
let factor = 3;
let is_divisible_by_factor = |x| x % factor == 0;
numbers.retain(is_divisible_by_factor);
println!("{:?}", numbers); // prints [3, 6]
```

The variable `factor` stores the value 3. The closure `is_divisible_by_factor` takes a number and returns true if it is divisible by the factor and false otherwise. The `retain` method uses this closure to filter the vector by keeping only the elements that return true.

However, suppose we want to change the factor dynamically and filter the vector by different factors. We can capture the factor by mutable reference and modify it in the closure. For example, this is how we can filter the vector by keeping only the numbers that are divisible by 2, then by 3, then by 4:

```rust
let mut numbers = vec![1, 2, 3, 4, 5, 6];
let mut factor = 2;
let is_divisible_by_factor = || {
    let result = numbers.iter().filter(|&x| x % factor == 0).collect::<Vec<_>>();
    factor += 1;
    result
};
println!("{:?}", is_divisible_by_factor()); // prints [2, 4, 6]
println!("{:?}", is_divisible_by_factor()); // prints [3, 6]
println!("{:?}", is_divisible_by_factor()); // prints [4]
```

The variable `factor` is captured by mutable reference and initialized to 2. The closure `is_divisible_by_factor` returns a vector that contains the elements of the original vector that are divisible by the factor. The closure also increments the factor by 1 after each call. This way, the closure can filter the vector by different factors dynamically.

By capturing the factor by mutable reference, we can use and change it in the closure without taking ownership of it. This way, we can filter the vector by different conditions without modifying the original vector.

## Iterators

An iterator is an object that can produce a sequence of values. Iterators are useful for processing collections of data, such as arrays, vectors, or hashes. Iterators can also be used to generate infinite sequences of values, such as numbers, characters, or random values.

The syntax for creating an iterator in Rust is to use the `iter` method of a collection, which returns an iterator over the elements of the collection. For example, this is how we can create an iterator over a vector of numbers:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let iter = numbers.iter();
```

The variable `iter` is an iterator that can produce the values 1, 2, 3, 4, and 5. The iterator does not consume the original vector, but only borrows a reference to it. This means that the vector is still available in the original scope, and we can use it or create another iterator from it.

One of the main advantages of iterators is that they are lazy, meaning that they do not produce the values until they are requested. This allows us to chain multiple operations on an iterator, such as filtering, mapping, or reducing, without creating intermediate collections. For example, this is how we can use an iterator to compute the sum of the squares of the even numbers in a vector:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let sum_of_squares = numbers.iter()
    .filter(|&x| x % 2 == 0) // keep only the even numbers
    .map(|&x| x * x) // square each number
    .sum(); // add up the numbers
println!("{}", sum_of_squares); // prints 20
```

The expression `numbers.iter()` creates an iterator over the vector. The method `filter` takes a closure as an argument and returns another iterator that produces only the values that satisfy the closure. The method `map` takes a closure as an argument and returns another iterator that produces the values that are obtained by applying the closure to each value. The method `sum` consumes the iterator and returns the sum of the values. The result is assigned to the variable `sum_of_squares`.

The iterator does not perform any computation until the `sum` method is called, which requests the final value. This means that the iterator does not create any intermediate vectors, but only produces the values on demand. This makes the code more efficient and concise.

## Iterators in Loops

One of the most common ways to use iterators is to loop over them using a `for` loop. The `for` loop takes an iterator as an argument and executes a block of code for each value produced by the iterator. For example, this is how we can use a `for` loop to print the elements of a vector:

```rust
let numbers = vec![1, 2, 3, 4, 5];
for n in numbers.iter() {
    println!("{}", n);
}
```

The `for` loop takes the iterator `numbers.iter()` as an argument and assigns each value to the variable `n`. The block of code `println!("{}", n)` is executed for each value of `n`. The output is:

```
1
2
3
4
5
```

The `for` loop consumes the iterator, meaning that it uses up all the values produced by the iterator. This means that the iterator is no longer available after the loop, and we cannot use it again. However, the original vector is still available, since the iterator only borrowed a reference to it.

One of the benefits of using a `for` loop with an iterator is that it handles the termination condition automatically. The `for` loop stops when the iterator runs out of values, so we don't need to check for the end of the iterator manually. For example, this is how we can use a `for` loop to print the elements of an array:

```rust
let letters = ['a', 'b', 'c', 'd', 'e'];
for c in letters.iter() {
    println!("{}", c);
}
```

The `for` loop takes the iterator `letters.iter()` as an argument and assigns each value to the variable `c`. The block of code `println!("{}", c)` is executed for each value of `c`. The output is:

```
a
b
c
d
e
```

The `for` loop stops when the iterator runs out of values, which is after the last element of the array. This way, we don’t need to use an index variable or a length variable to iterate over the array. The for loop handles the iteration logic for us.

## Using closures with iterators

One of the powerful features of Rust is that we can use closures with iterators to create complex and expressive computations. Closures and iterators work well together because they both use the concept of traits, which are a way of defining common behavior for different types. Traits allow us to write generic code that can work with different types of values, as long as they implement the required behavior.

In Rust, there are two main traits that are related to closures and iterators: the `Fn` trait and the `Iterator` trait. The `Fn` trait defines the behavior of a function-like object, such as a closure or a function pointer. The `Iterator` trait defines the behavior of an object that can produce a sequence of values, such as an iterator or a generator.

The `Fn` trait has three variants: `Fn`, `FnMut`, and `FnOnce`. These variants correspond to the different ways of capturing variables in closures, as we saw in the previous section. The `Fn` trait is implemented by closures that capture variables by reference, meaning that they can be called multiple times without consuming the variables. The `FnMut` trait is implemented by closures that capture variables by mutable reference, meaning that they can be called multiple times and can modify the variables. The `FnOnce` trait is implemented by closures that capture variables by value, meaning that they can be called only once and consume the variables.

The `Iterator` trait has one main method: `next`. The `next` method returns an `Option` value that contains either the next value produced by the iterator, or `None` if the iterator has no more values. The `Iterator` trait also has many other methods that provide additional functionality, such as `filter`, `map`, `sum`, `collect`, and more. These methods are called iterator adaptors, because they take an iterator as an argument and return another iterator that adapts the behavior of the original iterator.

The iterator adaptors are generic over the type of the iterator and the type of the closure that they take as an argument. This means that they can work with any iterator and any closure, as long as they implement the required traits. For example, the `filter` method takes an iterator and a closure that implements the `Fn` trait, and returns another iterator that produces only the values that satisfy the closure. The `map` method takes an iterator and a closure that implements the `Fn` trait, and returns another iterator that produces the values that are obtained by applying the closure to each value. The `sum` method takes an iterator and consumes it, returning the sum of the values. The `collect` method takes an iterator and consumes it, returning a collection that contains the values.

By using closures with iterators, we can create complex and expressive computations that are efficient and concise. For example, this is how we can use closures with iterators to find the average of the odd numbers in a vector:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let is_odd = |x| x % 2 == 1;
let odd_numbers: Vec<_> = numbers.iter().filter(is_odd).collect();
let sum_of_odd_numbers: i32 = odd_numbers.iter().sum();
let average_of_odd_numbers: f64 = sum_of_odd_numbers as f64 / odd_numbers.len() as f64;
println!("{}", average_of_odd_numbers); // prints 3
```

The closure `is_odd` takes a number and returns true if it is odd and false otherwise. The expression `numbers.iter().filter(is_odd).collect()` creates a new vector that contains the odd numbers of the original vector. The expression `odd_numbers.iter().sum()` computes the sum of the odd numbers. The expression `sum_of_odd_numbers as f64 / odd_numbers.len() as f64` computes the average of the odd numbers by casting the integers to floating-point numbers. The result is assigned to the variable `average_of_odd_numbers`.

The code uses closures with iterators to create a complex computation that is efficient and concise. The code does not create any intermediate vectors, but only produces the values on demand. The code also uses generic code that can work with different types of values, as long as they implement the required traits.

## Different Closures That Can Be Used In Iterators

In this section, we will see some examples of different closures that can be used in iterators. We will also see how to use different ways of capturing variables in closures, such as by value, by reference, or by mutable reference.

### Using a closure that takes no arguments

One of the simplest closures that can be used in iterators is a closure that takes no arguments. This type of closure can be used to generate a sequence of values, such as numbers, characters, or random values. For example, this is how we can use a closure that takes no arguments to generate an infinite sequence of Fibonacci numbers:

```rust
let mut a = 0;
let mut b = 1;
let fibonacci = || {
    let c = a + b;
    a = b;
    b = c;
    c
};
for n in fibonacci.take(10) {
    println!("{}", n);
}
```

The closure `fibonacci` takes no arguments and returns the next Fibonacci number. The closure also updates the variables `a` and `b`, which are captured by mutable reference. The expression `fibonacci.take(10)` creates an iterator that produces the first 10 values of the closure. The `for` loop prints the values of the iterator. The output is:

```
1
2
3
5
8
13
21
34
55
89
```

The code uses a closure that takes no arguments to generate an infinite sequence of Fibonacci numbers. The code also uses a mutable reference to capture the variables `a` and `b`, which are used to compute the Fibonacci numbers.

### Using a closure that takes one argument

Another common closure that can be used in iterators is a closure that takes one argument. This type of closure can be used to transform or filter a sequence of values, such as numbers, strings, or structs. For example, this is how we can use a closure that takes one argument to capitalize the first letter of each word in a string:

```rust
let sentence = "hello world";
let capitalize = |s: &str| {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
};
let capitalized_words: Vec<_> = sentence.split_whitespace().map(capitalize).collect();
println!("{:?}", capitalized_words); // prints ["Hello", "World"]
```

The closure `capitalize` takes a string slice as an argument and returns a new string that has the first letter capitalized. The closure uses the `chars` method to get an iterator over the characters of the string. The closure uses the `next` method to get the first character of the iterator, and the `as_str` method to get the rest of the string. The closure uses the `to_uppercase` method to convert the first character to uppercase, and the `collect` method to create a new string from the iterator. The closure uses the `match` expression to handle the case where the string is empty.

The expression `sentence.split_whitespace().map(capitalize).collect()` creates a new vector that contains the words of the sentence with the first letter capitalized. The method `split_whitespace` returns an iterator over the words of the sentence. The method `map` takes the closure `capitalize` as an argument and returns another iterator that produces the capitalized words. The method `collect` consumes the iterator and returns a vector that contains the words.

The code uses a closure that takes one argument to transform a sequence of strings. The code also uses a reference to capture the string slice, which is the argument of the closure.