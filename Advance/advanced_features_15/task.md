# Task: Implement a Custom Iterator with Macros, Advanced Functions and Closures

In this task, you will implement a custom iterator that can iterate over a range of numbers and apply a function to each element. This will demonstrate the use of macros, generics, traits, and closures, which are some of the advanced features of Rust.

## Requirements

- Define a struct named `MapRange` that has two fields: `start` and `end`, which are both `i32` values. This struct will represent a range of numbers from `start` to `end` (exclusive).
- Implement the `Iterator` trait for `MapRange`. The `Iterator` trait requires you to define an associated type named `Item` and a method named `next` that returns an `Option<Self::Item>`. The `Item` type should be a generic type parameter that can be any type that implements the `From<i32>` trait. The `next` method should return the next element in the range, converted to the `Item` type, or `None` if the range is exhausted.
- Implement a custom macro named `map_range!` that can create a `MapRange` struct from a range expression, such as `map_range!(1..10)`. The macro should accept any expression that can be converted to a `Range<i32>` type, such as `1..10`, `start..end`, or `start..=end-1`. The macro should expand to a `MapRange` struct with the corresponding `start` and `end` values.
- Implement a method named `map` for `MapRange` that can take a function as an argument and apply it to each element in the range. The function should be a generic type parameter that can be any type that implements the `Fn(Item) -> Item` trait, where `Item` is the same type as the `Iterator` associated type. The `map` method should return a new `MapRange` struct that has the same `start` and `end` values, but a different `Item` type that is the result of applying the function. The `map` method should also implement the `Iterator` trait, so that it can be chained with other iterators.

## Examples

Here are some examples of using the `MapRange` struct and the `map_range!` macro:

```rust
// Create a MapRange struct that iterates over the numbers from 1 to 10 as i32
let mr1 = MapRange { start: 1, end: 10 };

// Use the map_range! macro to create the same struct with a shorter syntax
let mr2 = map_range!(1..10);

// Use the map_range! macro with a different range expression
let start = 5;
let end = 15;
let mr3 = map_range!(start..end);

// Use the map_range! macro with an inclusive range expression
let mr4 = map_range!(1..=9);

// Iterate over the MapRange struct and print each element
for x in mr1 {
    println!("{}", x);
}

// Convert the MapRange struct to a different type using the From trait
let mr5: MapRange<f32> = mr1.into();

// Use the map method to apply a function to each element in the range
let mr6 = mr5.map(|x| x * 2.0);

// Chain the map method with another iterator method
let sum: f32 = mr6.sum();
println!("The sum is {}", sum);
```
