# Collections and Modules

Collections and modules are two important features of Rust that help us organize and manipulate data in a safe and efficient way. In this tutorial, I will explain what collections and modules are, how to create and use them, and what are some of the common types of collections and modules in Rust.

## Collections

A collection is a data structure that can store multiple values of the same type. Rust provides several built-in collections in the standard library, such as:

- `Vec`: a growable, heap-allocated array that can store any type of value.
- `String`: a growable, heap-allocated string that can store UTF-8 encoded text.
- `HashMap`: a hash map that can store key-value pairs of any type, and provide fast lookup by key.
- `HashSet`: a hash set that can store unique values of any type, and provide fast membership checking.
- `BTreeMap`: a binary tree map that can store key-value pairs of any type, and provide sorted iteration by key.
- `BTreeSet`: a binary tree set that can store unique values of any type, and provide sorted iteration.

To use these collections, we need to import them from the `std::collections` module using the `use` keyword:

```rust
// Import the collections from the standard library
use std::collections::{Vec, String, HashMap, HashSet, BTreeMap, BTreeSet};
```

To create a collection, we can use the `new` method, which returns an empty collection, or the `from` method, which converts an existing value into a collection. For example, we can create a `Vec` from a slice of integers, or a `String` from a string literal:

```rust
// Create a Vec from a slice of integers
let v = Vec::from([1, 2, 3, 4, 5]);

// Create a String from a string literal
let s = String::from("hello");
```

To add elements to a collection, we can use methods such as `push`, `insert`, or `append`, depending on the type of the collection. For example, we can push an element to the end of a `Vec`, insert a key-value pair into a `HashMap`, or append another `String` to a `String`:

```rust
// Push an element to the end of a Vec
v.push(6);

// Insert a key-value pair into a HashMap
let mut h = HashMap::new();
h.insert("name", "Alice");

// Append another String to a String
s.push_str(", world");
```

To access elements from a collection, we can use methods such as `get`, `index`, or `iter`, depending on the type of the collection and the operation we want to perform. For example, we can get an element from a `Vec` by index, index a `HashMap` by key, or iterate over a `HashSet` using a `for` loop:

```rust
// Get an element from a Vec by index
let x = v.get(0); // returns Some(1)

// Index a HashMap by key
let y = h["name"]; // returns "Alice"

// Iterate over a HashSet using a for loop
let z = HashSet::from([1, 2, 3]);
for n in &z {
    println!("{}", n);
}
```

To modify elements in a collection, we can use methods such as `set`, `update`, or `remove`, depending on the type of the collection and the operation we want to perform. For example, we can set an element in a `Vec` by index, update a value in a `HashMap` by key, or remove an element from a `HashSet` by value:

```rust
// Set an element in a Vec by index
v[0] = 10;

// Update a value in a HashMap by key
h.entry("name").and_modify(|v| *v = "Bob");

// Remove an element from a HashSet by value
z.remove(&1);
```

To check the size of a collection, we can use the `len` method, which returns the number of elements in the collection. For example, we can check the length of a `Vec`, a `String`, or a `BTreeMap`:

```rust
// Check the length of a Vec
let l1 = v.len(); // returns 6

// Check the length of a String
let l2 = s.len(); // returns 12

// Check the length of a BTreeMap
let l3 = BTreeMap::new().len(); // returns 0
```

To clear a collection, we can use the `clear` method, which removes all the elements from the collection. For example, we can clear a `Vec`, a `String`, or a `HashMap`:

```rust
// Clear a Vec
v.clear();

// Clear a String
s.clear();

// Clear a HashMap
h.clear();
```

## Modules

A module is a way of organizing code into logical units that can be nested and have different levels of visibility. Rust provides a powerful module system that allows us to split code into multiple files, control the scope and privacy of items, and reuse code across projects.

To define a module, we use the `mod` keyword, followed by the name of the module and a block of code. For example, we can define a module named `math` that contains some functions for arithmetic operations:

```rust
// Define a module named math
mod math {
    // Define a function named add that takes two integers and returns their sum
    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Define a function named sub that takes two integers and returns their difference
    fn sub(x: i32, y: i32) -> i32 {
        x - y
    }

    // Define a function named mul that takes two integers and returns their product
    fn mul(x: i32, y: i32) -> i32 {
        x * y
    }

    // Define a function named div that takes two integers and returns their quotient
    fn div(x: i32, y: i32) -> i32 {
        x / y
    }
}
```

By default, all items in a module are private, which means they can only be accessed from within the module. To make an item public, we use the `pub` keyword before its definition. For example, we can make the `math` module and its functions public:

```rust
// Define a public module named math
pub mod math {
    // Define a public function named add that takes two integers and returns their sum
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    // Define a public function named sub that takes two integers and returns their difference
    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }

    // Define a public function named mul that takes two integers and returns their product
    pub fn mul(x: i32, y: i32) -> i32 {
        x * y
    }

    // Define a public function named div that takes two integers and returns their quotient
    pub fn div(x: i32, y: i32) -> i32 {
        x / y
    }
}
```

To access an item from a module, we use the `::` operator, which is called the path separator. A path is a sequence of names that leads to an item. For example, we can access the `add` function from the `math` module using the path `math::add`:

```rust
// Access the add function from the math module
let z = math::add(2, 3); // returns 5
```

To avoid repeating the module name every time we want to access an item, we can use the `use` keyword, which brings a path into scope. For example, we can use the `math::add` function without the module name by using the `use` keyword:

```rust
// Bring the math::add function into scope
use math::add;

// Use the add function without the module name
let z = add(2, 3); // returns 5
```

We can also use the `use` keyword to bring multiple items from the same module into scope, using the `{}` syntax. For example, we can use the `math::sub` and `math::mul` functions without the module name by using the `use` keyword:

```rust
// Bring the math::sub and math::mul functions into scope
use math::{sub, mul};

// Use the sub and mul functions without the module name
let z1 = sub(5, 2); // returns 3
let z2 = mul(3, 4); // returns 12
```

We can also use the `*` syntax to bring all the items from a module into scope, but this is usually not recommended, as it can cause name conflicts and make the code less clear. For example, we can use all the functions from the `math` module without the module name by using the `use` keyword:

```rust
// Bring all the items from the math module into scope
use math::*;

// Use the add, sub, mul, and div functions without the module name
let z1 = add(2, 3); // returns 5
let z2 = sub(5, 2); // returns 3
let z3 = mul(3, 4); // returns 12
let z4 = div(12, 3); // returns 4
```

To split a module into multiple files in Rust, we need to follow a few steps:

- Create a new file with the same name as the module we want to split, and add the `.rs` extension. For example, if we want to split the `matrix` module, we create a file named `matrix.rs`.
- Move the code that belongs to the module from the parent file to the new file. For example, we move the code inside the `mod matrix` block to the `matrix.rs` file.
- Remove the `mod matrix` block from the parent file, and replace it with a `mod matrix;` declaration. This tells the compiler to look for the module definition in the `matrix.rs` file.
- Repeat these steps for any sub-modules that we want to split as well.

For example, if we have a file named `math.rs` that contains the following code:

```rust
pub mod matrix {
    pub struct Matrix {
        // some fields
    }

    impl Matrix {
        // some methods
    }

    pub mod ops {
        // some functions for matrix operations
    }
}

pub mod vector {
    // some code for vector module
}
```

We can split the `matrix` module into a separate file by creating a file named `matrix.rs` and moving the code inside the `mod matrix` block to it:

```rust
// matrix.rs
pub struct Matrix {
    // some fields
}

impl Matrix {
    // some methods
}

pub mod ops {
    // some functions for matrix operations
}
```

Then we remove the `mod matrix` block from the `math.rs` file and replace it with a `mod matrix;` declaration:

```rust
// math.rs
mod matrix;

pub mod vector {
    // some code for vector module
}
```

If we want to split the `ops` sub-module as well, we can create a file named `ops.rs` inside a directory named `matrix`, and move the code inside the `mod ops` block to it:

```rust
// matrix/ops.rs
// some functions for matrix operations
```

Then we remove the `mod ops` block from the `matrix.rs` file and replace it with a `mod ops;` declaration:

```rust
// matrix.rs
pub struct Matrix {
    // some fields
}

impl Matrix {
    // some methods
}

mod ops;
```

By splitting modules into different files, we can make our code more organized and maintainable. However, we should also be careful not to create too many files or directories, as that can make the code harder to navigate and understand. A good rule of thumb is to group related items together in a module, and split a module into multiple files only when it becomes too large or complex.

