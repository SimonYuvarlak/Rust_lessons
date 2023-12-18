mod functions; // We are importing the example function from this module

fn main() {
    // Collections

    // Vec: a growable, heap-allocated array
    let mut v = Vec::from([1, 2, 3, 4, 5]);
    v.push(6); // Pushing an element to the end of the Vec
    let x = v.get(0); // Accessing an element by index
    println!("First element in Vec: {:?}", x);

    // String: a growable, heap-allocated string
    let mut s = String::from("hello");
    s.push_str(", world"); // Appending another String
    println!("String: {}", s);

    // HashMap: a hash map storing key-value pairs
    let mut h = std::collections::HashMap::new();
    h.insert("name", "Alice");
    let y = h.get("name"); // Accessing a value by key
    println!("Value in HashMap: {:?}", y);

    // HashSet: a hash set storing unique values
    let mut z = std::collections::HashSet::from([1, 2, 3]);
    z.insert(4); // Inserting a new element
    for n in &z {
        println!("Element in HashSet: {}", n);
    }

    // BTreeMap: a binary tree map
    let mut btm = std::collections::BTreeMap::new();
    btm.insert(1, "One");
    btm.insert(2, "Two");
    let l3 = btm.len(); // Checking the length
    println!("Number of elements in BTreeMap: {}", l3);

    // BTreeSet: a binary tree set
    let mut bts = std::collections::BTreeSet::new();
    bts.insert("Apple");
    bts.insert("Banana");
    for item in &bts {
        println!("Element in BTreeSet: {}", item);
    }

    // Modules

    // Accessing functions from a module
    // Assuming math module is defined and public in the context of this program
    let z = math::add(2, 3); // Using a function from the math module
    println!("Sum from math module: {}", z);

    // Using `use` keyword to simplify function calls
    use math::sub;
    let z1 = sub(5, 2); // Using sub function directly
    println!("Difference using sub function: {}", z1);

    // Now, we will use example_function from the functions module
    // First we need to import the module
    use functions::example_function;
    example_function::example(); // Calling the example_function
}

// Assuming the math module is defined somewhere in the program
pub mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    pub fn sub(x: i32, y: i32) -> i32 {
        x - y
    }
}
