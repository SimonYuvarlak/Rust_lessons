## Task: Create and use your own collections and modules

In this task, you will practice creating and using your own collections and modules in Rust. You will also learn how to import and access items from the standard library and other crates.

### Step 1: Define a module named utils that contains some utility functions for working with numbers

- Use the `mod` keyword to define a module named `utils`.
- Add two functions to the module: `is_prime` and `is_even`, both of which take an integer as a parameter and return a boolean as the output.
- The `is_prime` function should check if the given number is a prime number, and the `is_even` function should check if the given number is an even number.
- Make the module and the functions public by using the `pub` keyword before their definitions.

### Step 2: Define a collection named PrimeSet that implements a set of prime numbers

- Use the `struct` keyword to define a collection named `PrimeSet` that has a field named `data` of type `HashSet<i32>`.
- Use the `use` keyword to import the `HashSet` type from the `std::collections` module.
- Implement the `new` method for the `PrimeSet` collection that returns an empty set of prime numbers.
- Implement the `insert` method for the `PrimeSet` collection that takes an integer as a parameter and inserts it into the set if it is a prime number, and returns a boolean indicating whether the insertion was successful or not.
- Implement the `contains` method for the `PrimeSet` collection that takes an integer as a parameter and returns a boolean indicating whether the set contains the given number or not.
- Make the collection and the methods public by using the `pub` keyword before their definitions.

### Step 3: Define a module named math that re-exports the utils module and the PrimeSet collection

- Use the `mod` keyword to define a module named `math`.
- Use the `use` keyword to bring the `utils` module and the `PrimeSet` collection into scope.
- Use the `pub` keyword and the `use` keyword to re-export the `utils` module and the `PrimeSet` collection from the `math` module, so that they can be accessed by other modules.

### Step 4: Test your code by creating some instances of collections and modules and calling their functions

- Use the `let` keyword to create some instances of collections and modules, and assign them to variables. For example, you can create a `PrimeSet` and assign it to a variable named `p`.
- Use the dot notation to call the functions of the collections and modules, and print the results using the `println!` macro. For example, you can insert some numbers into the `PrimeSet` and check if they are prime or even by writing `println!("Is {} prime? {}", n, p.insert(n));` and `println!("Is {} even? {}", n, utils::is_even(n));`.

### Expected output

Your code should compile and run without errors, and produce an output similar to this:

```rust
Is 2 prime? true
Is 2 even? true
Is 3 prime? true
Is 3 even? false
Is 4 prime? false
Is 4 even? true
Is 5 prime? true
Is 5 even? false
Is 6 prime? false
Is 6 even? true
```