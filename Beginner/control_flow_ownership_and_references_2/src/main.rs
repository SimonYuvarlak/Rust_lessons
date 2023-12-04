fn main() {
    // Control Flow: if Statements
    // Checking if a number is even or odd using if-else
    let number = 6;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // Using if in a let Statement
    // Assigning a variable based on a condition using if-else
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Repetition with Loops
    // Using a loop to repeat actions until a condition is met
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break; // Exits the loop when count reaches 3
        }
    }

    // Using a while loop for conditional repetition
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Iterating over an array with a for loop
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Ownership in Rust
    // Demonstrating how Rust automatically manages memory using the String type
    let mut s = String::from("hello");
    s.push_str(", world!"); // Modifying the string
    println!("{}", s); // s is valid here as it's within its scope

    // References and Borrowing
    // Creating a reference to a String and borrowing it
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // &s1 creates a reference to s1
    println!("The length of '{}' is {}.", s1, len);

    // Mutable References
    // Modifying a String through a mutable reference
    let mut s = String::from("hello");
    change(&mut s); // Passing a mutable reference to s
    println!("{}", s);
}

// Function to calculate the length of a string
// It takes a string reference and returns the length
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function to change a string
// It takes a mutable string reference and appends ", world"
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
