fn main() {
    // Unsafe Rust
    // Unsafe Rust allows us to access raw pointers, call unsafe functions, and implement unsafe traits
    // Unsafe Rust can be used to interact with low-level code, optimize performance, or work with foreign code
    // Unsafe Rust must be enclosed in an unsafe block or an unsafe function

    // An example of using unsafe Rust to dereference a raw pointer
    let mut num = 5;

    // Create an immutable and a mutable raw pointer from references
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Raw pointers can be immutable or mutable, and are prefixed with an asterisk (*)
    // Unlike references, raw pointers can be null, unaligned, or point to invalid memory
    // Raw pointers can be cast from any integer type

    unsafe {
        // Dereference a raw pointer within an unsafe block
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);

        // Change the value of num through r2
        *r2 = 10;
        println!("r2 is now: {}", *r2);
    }

    // An example of using unsafe Rust to call an unsafe function
    // Unsafe functions must have the unsafe keyword before them
    // Unsafe functions can be called from safe code, but only within an unsafe block or function

    // A function that takes a raw pointer and returns the value it points to
    unsafe fn deref(ptr: *const i32) -> i32 {
        // Dereference the pointer and return the value
        // This is unsafe because the pointer might be null or invalid
        *ptr
    }

    // A function that calls the unsafe function deref
    // Create a raw pointer from a reference
    let x = 42;
    let p = &x as *const i32;

    // Call the unsafe function within an unsafe block
    unsafe {
        let y = deref(p);
        println!("y is: {}", y);
    }

    // An example of using unsafe Rust to implement an unsafe trait
    // Unsafe traits are traits that have at least one unsafe method
    // Unsafe traits must have the unsafe keyword before them
    // Implementing an unsafe trait requires the unsafe keyword as well
    // Unsafe traits can be used to mark types that have invariants that must be upheld by the programmer

    // An unsafe trait that has an unsafe method
    unsafe trait Foo {
        // An unsafe method that takes a mutable reference to self
        unsafe fn bar(&mut self);
    }

    // A type that implements the unsafe trait Foo
    struct Baz {
        x: i32,
    }

    // Implementing the unsafe trait Foo for Baz
    unsafe impl Foo for Baz {
        // Implementing the unsafe method bar
        unsafe fn bar(&mut self) {
            // Change the value of x
            // This is unsafe because it might break some invariant of Baz
            self.x = 0;
        }
    }

    // A function that uses the unsafe trait Foo
    // Create an instance of Baz
    let mut baz = Baz { x: 1 };

    // Call the unsafe method bar on baz within an unsafe block
    unsafe {
        baz.bar();
        println!("baz.x is: {}", baz.x);
    }

    // Advanced Traits
    // Advanced traits are traits that use features such as associated types, default generic type parameters, and operator overloading
    // Advanced traits can be used to create more generic and expressive abstractions

    // An example of using associated types in a trait
    // Associated types are types that are associated with a trait
    // Associated types allow a trait to use a type without specifying it in the trait definition
    // Associated types can be specified by the implementor of the trait

    // A trait that has an associated type Item
    trait Iterator {
        // The associated type Item
        type Item;

        // A method that returns an option of the associated type Item
        fn next(&mut self) -> Option<Self::Item>;
    }

    // A type that implements the trait Iterator
    struct Counter {
        count: u32,
    }

    // Implementing the trait Iterator for Counter
    impl Iterator for Counter {
        // Specifying the associated type Item as u32
        type Item = u32;

        // Implementing the method next
        fn next(&mut self) -> Option<Self::Item> {
            // Increment the count and return it if it is less than 6
            // Otherwise, return None
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }

    // A function that uses the trait Iterator
    // Create an instance of Counter
    let mut counter = Counter { count: 0 };

    // Use a while let loop to iterate over the counter
    while let Some(x) = counter.next() {
        println!("x is: {}", x);
    }

    // An example of using default generic type parameters and operator overloading in a trait
    // Default generic type parameters are generic type parameters that have a default type specified
    // Default generic type parameters can be overridden by the implementor of the trait
    // Operator overloading is the ability to define the behavior of operators for custom types
    // Operator overloading can be achieved by implementing traits that correspond to operators

    // A trait that has a default generic type parameter Output and an operator overload for +
    trait Add<Rhs = Self> {
        // The default generic type parameter Output
        type Output;

        // The operator overload for +
        fn add(self, rhs: Rhs) -> Self::Output;
    }

    // A type that implements the trait Add
    struct Point {
        x: i32,
        y: i32,
    }

    // Implementing the trait Add for Point
    impl std::ops::Add for Point {
        // Using the default generic type parameter Output as Point
        type Output = Point;

        // Implementing the operator overload for +
        fn add(self, other: Point) -> Point {
            // Return a new point with the sum of the x and y coordinates
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    // Use the + operator to add the points
    let p3 = p1 + p2;
    println!("p3.x is: {}", p3.x);
    println!("p3.y is: {}", p3.y);

    // An example of using fully qualified syntax for disambiguation: calling methods with the same name
    // Fully qualified syntax is a way of specifying the exact method to call when there are multiple methods with the same name
    // Fully qualified syntax can be used to resolve ambiguity or to call methods that are hidden by other methods
    // Fully qualified syntax has the form <Type as Trait>::method(receiver, args)

    // A trait that has a method with the same name as a method of another trait
    trait Pilot {
        // A method with the same name as a method of Wizard
        fn fly(&self);
    }

    // A trait that has a method with the same name as a method of Pilot
    trait Wizard {
        // A method with the same name as a method of Pilot
        fn fly(&self);
    }

    // A type that implements both Pilot and Wizard
    struct Human;

    // Implementing Pilot for Human
    impl Pilot for Human {
        // Implementing the method fly
        fn fly(&self) {
            // Print a message
            println!("This is your captain speaking.");
        }
    }

    // Implementing Wizard for Human
    impl Wizard for Human {
        // Implementing the method fly
        fn fly(&self) {
            // Print a message
            println!("Up!");
        }
    }

    // A function that uses fully qualified syntax for disambiguation
    // Create an instance of Human
    let person = Human;

    // Call the method fly of Pilot on person using the dyn keyword for trait objects
    (<dyn Pilot>::fly)(&person);

    // Call the method fly of Wizard on person using the dyn keyword for trait objects
    (<dyn Wizard>::fly)(&person);

    // Advanced Types
    // Advanced types are types that use features such as newtype pattern, type aliases, never type, and dynamically sized types
    // Advanced types can be used to create more expressive and ergonomic types

    // An example of using newtype pattern in Rust
    // Newtype pattern is a way of creating a new type from an existing type
    // Newtype pattern can be used to implement traits on types that already have them, or to add more meaning to a type

    // A type that wraps an i32
    struct Years(i32);

    // A type that wraps an i32
    struct Days(i32);

    // A function that takes a Years and returns a Days
    fn years_to_days(years: Years) -> Days {
        // Convert the years to days by multiplying by 365
        Days(years.0 * 365)
    }

    // A function that uses the newtype patter
}
