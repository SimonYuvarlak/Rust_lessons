# Smart Pointers

Smart pointers are a feature of Rust that allow you to manage memory more efficiently and safely. They are data structures that act like pointers, but also have additional metadata and capabilities, such as automatic deallocation, reference counting, or interior mutability. In this tutorial, I will explain the main types of smart pointers in Rust and how to use them in your code.

The first type of smart pointer is Box<T>, which allocates data on the heap and allows you to move values out of the stack. Boxes are useful when you have large or unknown-sized data, recursive types, or trait objects. To create a box, you can use the Box::new function, which takes a value and returns a Box<T> that points to it. For example, you can create a box that holds an i32 value like this:

```rust
let x = Box::new(5);
```

You can dereference a box to access its inner value, just like a regular pointer. You can also use the \* operator to move the value out of the box. For example, you can print the value of x and then assign it to another variable like this:

```rust
println!("{}", *x); // prints 5
let y = *x; // moves the value out of the box
```

When a box goes out of scope, it will be dropped and the memory it occupies will be freed. You can also implement the Drop trait for your own types to customize the behavior when they are dropped. The Drop trait has one method, drop, which takes a mutable reference to self and performs any cleanup logic. For example, you can create a custom type that prints a message when it is dropped like this:

```rust
struct Custom {
    name: String,
}

impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

let c = Custom {
    name: String::from("Alice"),
}; // creates a new instance of Custom

// do some work with c

// c goes out of scope and is dropped
```

The second type of smart pointer is Rc<T>, which stands for reference counting. Rc<T> enables multiple ownership of data by keeping track of the number of references to a value. When the last reference goes out of scope, the value will be dropped and the memory will be freed. Rc<T> is useful when you want to share data between multiple parts of your code without using references or cloning. To create an Rc<T>, you can use the Rc::new function, which takes a value and returns an Rc<T> that points to it. For example, you can create an Rc<T> that holds a String value like this:

```rust
let s = Rc::new(String::from("Hello"));
```

To create a new reference to the same value, you can use the Rc::clone function, which takes an Rc<T> and returns a new Rc<T> that points to the same value. Rc::clone does not perform a deep copy of the data, but only increments the reference count. For example, you can create two more references to s like this:

```rust
let s1 = Rc::clone(&s);
let s2 = Rc::clone(&s);
```

You can dereference an Rc<T> to access its inner value, just like a regular pointer. You can also use the Rc::strong_count function to get the number of references to a value. For example, you can print the value and the reference count of s like this:

```rust
println!("{}", *s); // prints Hello
println!("{}", Rc::strong_count(&s)); // prints 3
```

When an Rc<T> goes out of scope, it will decrement the reference count. When the reference count reaches zero, the value will be dropped and the memory will be freed. For example, if s1 and s2 go out of scope, the reference count of s will be 1, and if s goes out of scope, the value will be dropped.

The third type of smart pointer is RefCell<T>, which allows you to mutate data through a shared reference. RefCell<T> implements the interior mutability pattern, which means that it provides a way to change the inner value of an immutable type. RefCell<T> is useful when you want to have multiple references to a value, but also need to modify it at runtime. To create a RefCell<T>, you can use the RefCell::new function, which takes a value and returns a RefCell<T> that wraps it. For example, you can create a RefCell<T> that holds an i32 value like this:

```rust
let x = RefCell::new(5);
```

To access the inner value of a RefCell<T>, you can use the borrow and borrow_mut methods, which return a Ref<T> and a RefMut<T> respectively. These are smart pointers that point to the inner value and implement the Deref and DerefMut traits, which allow you to use them as references. For example, you can borrow the value of x and print it like this:

```rust
let x_ref = x.borrow();
println!("{}", *x_ref); // prints 5
```

You can also borrow the value of x mutably and modify it like this:

```rust
let mut x_ref_mut = x.borrow_mut();
*x_ref_mut += 1;
println!("{}", *x_ref_mut); // prints 6
```

When a Ref<T> or a RefMut<T> goes out of scope, it will release the borrow. RefCell<T> enforces the borrowing rules at runtime, which means that it will panic if you try to create multiple mutable borrows or a mutable borrow and an immutable borrow at the same time. For example, the following code will panic:

```rust
let mut x_ref_mut1 = x.borrow_mut();
let mut x_ref_mut2 = x.borrow_mut(); // panic: already borrowed
```

Here are some examples of how you can use them in functions, structs and closures:

- Functions: You can use box pointers as function parameters or return values when you have large or unknown-sized data, recursive types, or trait objects. For example, you can write a function that takes a box pointer to a trait object and calls a method on it:

```rust
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says woof!", self.name);
    }
}

fn greet(animal: Box<dyn Animal>) {
    animal.make_sound();
}

let dog = Box::new(Dog {
    name: String::from("Spot"),
});
greet(dog); // prints Spot says woof!
```

- Structs: You can use box pointers as struct fields when you have recursive types or trait objects. For example, you can write a struct that represents a binary tree node with box pointers to its left and right children:

```rust
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

let root = TreeNode {
    value: 1,
    left: Some(Box::new(TreeNode {
        value: 2,
        left: None,
        right: None,
    })),
    right: Some(Box::new(TreeNode {
        value: 3,
        left: None,
        right: None,
    })),
};
```

- Closures: You can use box pointers to store closures on the heap and pass them around as function pointers. For example, you can write a function that takes a box pointer to a closure and calls it with a given argument:

```rust
fn apply<F>(f: Box<F>, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

let add_one = Box::new(|x| x + 1);
let result = apply(add_one, 5);
println!("{}", result); // prints 6
```
