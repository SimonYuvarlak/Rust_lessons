// Define a trait for animals
trait Animal {
    fn make_sound(&self);
}

// Define a struct for dogs
struct Dog {
    name: String,
}

// Implement the Animal trait for dogs
impl Animal for Dog {
    fn make_sound(&self) {
        println!("{} says woof!", self.name);
    }
}

// Define a function that takes a box pointer to an animal and calls its method
fn greet(animal: Box<dyn Animal>) {
    animal.make_sound();
}

// Define a struct for tree nodes
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

// Define a function that takes a box pointer to a closure and calls it with an argument
fn apply<F>(f: Box<F>, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(x)
}

fn main() {
    // Create a box pointer to a dog
    let dog = Box::new(Dog {
        name: String::from("Spot"),
    });
    // Call the greet function with the dog
    greet(dog); // prints Spot says woof!

    // Create a box pointer to a tree node
    let root = Box::new(TreeNode {
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
    });
    // Print the value of the root node
    println!("{}", root.value); // prints 1

    // Create a box pointer to a closure
    let add_one = Box::new(|x| x + 1);
    // Call the apply function with the closure and an argument
    let result = apply(add_one, 5);
    // Print the result
    println!("{}", result); // prints 6
}
