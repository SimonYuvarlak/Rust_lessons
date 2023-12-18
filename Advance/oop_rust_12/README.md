# OOP in Rust

Object-oriented programming (OOP) is a paradigm that organizes data and behavior into reusable units called objects. Rust is not a pure OOP language, but it supports some OOP features such as encapsulation, polymorphism, and inheritance. In this tutorial, I will show you how to use these features in Rust to create your own objects and implement OOP design patterns.

## Defining Objects in Rust

In Rust, objects are usually defined using structs and enums, which are data structures that can hold fields of different types. Structs are used to represent objects that have a fixed set of fields, while enums are used to represent objects that can have different variants. For example, we can define a struct to represent a point in a two-dimensional space:

```rust
struct Point {
    x: f64,
    y: f64,
}
```

We can also define an enum to represent a shape that can be either a circle, a rectangle, or a triangle:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}
```

To create an instance of a struct or an enum, we can use the syntax `StructName { field: value, .. }` or `EnumName::Variant { field: value, .. }`. For example, we can create a point and a circle like this:

```rust
let p = Point { x: 3.0, y: 4.0 };
let c = Shape::Circle { radius: 5.0 };
```

## Implementing Methods and Traits

To define the behavior of our objects, we can use methods and traits. Methods are functions that are associated with a specific type and can be invoked using the dot notation. Traits are collections of methods that can be implemented by different types to share common behavior. For example, we can define a method to calculate the distance between two points, and a trait to calculate the area of a shape:

```rust
// Method for Point
impl Point {
    fn distance(&self, other: &Point) -> f64 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
}

// Trait for Shape
trait Area {
    fn area(&self) -> f64;
}

// Implement Area for Circle
impl Area for Shape::Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powi(2)
    }
}

// Implement Area for Rectangle
impl Area for Shape::Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// Implement Area for Triangle
impl Area for Shape::Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}
```

To call a method or a trait method, we can use the syntax `object.method(args)`. For example, we can calculate the distance between two points and the area of a circle like this:

```rust
let p1 = Point { x: 0.0, y: 0.0 };
let p2 = Point { x: 3.0, y: 4.0 };
let c = Shape::Circle { radius: 5.0 };

println!("The distance between p1 and p2 is {}", p1.distance(&p2));
println!("The area of c is {}", c.area());
```

## Using Encapsulation and Information Hiding

Encapsulation is the principle of grouping data and behavior into a single unit, and hiding the internal details from the outside world. Information hiding is the principle of restricting the access to the data and behavior of an object, and exposing only the necessary interface to the users. In Rust, we can use the `pub` keyword to mark the fields and methods that we want to make public, and leave the rest private by default. For example, we can define a struct to represent a bank account, and make the balance field private, and the deposit and withdraw methods public:

```rust
struct BankAccount {
    // Private field
    balance: f64,
}

impl BankAccount {
    // Public method
    pub fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    // Public method
    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount > self.balance {
            Err("Insufficient funds".to_string())
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }

    // Private method
    fn get_balance(&self) -> f64 {
        self.balance
    }
}
```

To create a new bank account, we can use the `new` method, which is a common convention for constructors in Rust:

```rust
impl BankAccount {
    // Constructor
    pub fn new(initial_balance: f64) -> BankAccount {
        BankAccount {
            balance: initial_balance,
        }
    }
}
```

To use the bank account, we can only access the public methods, and not the private field or method:

```rust
let mut account = BankAccount::new(1000.0);
account.deposit(500.0);
println!("The balance after deposit is {}", account.get_balance()); // Error: get_balance is private
println!("The balance after withdraw is {}", account.withdraw(200.0).unwrap());
println!("The balance is {}", account.balance); // Error: balance is private
```

## Using Polymorphism and Inheritance

Polymorphism is the ability of an object to take different forms depending on the context. Inheritance is the mechanism of deriving new classes from existing ones, and reusing their data and behavior. Rust does not support inheritance in the traditional OOP sense, but it supports polymorphism through the use of trait objects and generics. Trait objects are pointers to any type that implements a specific trait, and they allow us to use dynamic dispatch, which means the method to call is determined at runtime. Generics are parameters that can be replaced with any type that satisfies certain constraints, and they allow us to use static dispatch, which means the method to call is determined at compile time. For example, we can define a trait to represent an animal, and implement it for different types of animals:

```rust
// Trait for Animal
trait Animal {
    fn make_sound(&self) -> &str;
}

// Implement Animal for Dog
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> &str {
        "Woof"
    }
}

// Implement Animal for Cat
struct Cat {
    name: String,
}

impl Animal for Cat {
    fn make_sound(&self) -> &str {
        "Meow"
    }
}
```

To use trait objects, we can use the syntax `&dyn Trait` or `Box<dyn Trait>` to create a reference or a pointer to any type that implements the trait. For example, we can create a vector of trait objects to store different animals, and iterate over them to make them sound:

```rust
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog { name: "Fido".to_string() }),
    Box::new(Cat { name: "Mimi".to_string() }),
];

for animal in animals {
    println!("{}", animal.make_sound());
}
```

To use generics, we can use the syntax `<T: Trait>` or `<T>` to declare a type parameter that can be replaced with any type that implements the trait or satisfies the constraints. For example, we can define a function that takes a generic animal and makes it sound:

```rust
fn make_animal_sound<T: Animal>(animal: &T) {
    println!("{}", animal.make_sound());
}

let dog = Dog { name: "Fido".to_string() };
let cat = Cat { name: "Mimi".to_string() };

make_animal_sound(&dog);
make_animal_sound(&cat);
```

## Conclusion

In this tutorial, you learned how to use OOP features in Rust, such as defining objects, implementing methods and traits, using encapsulation and information hiding, and using polymorphism and inheritance. Rust is not a pure OOP language, but it offers some powerful and flexible ways to organize your data and behavior.
