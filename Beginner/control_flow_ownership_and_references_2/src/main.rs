fn main() {
    // Control Flow

    // Using `if` Statements for Conditional Execution
    let number = 6;
    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // Using `if` in a `let` Statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    // Repetition with Loops

    // Using `loop` for Indefinite Looping
    let mut count = 0;
    loop {
        count += 1;
        if count == 3 {
            break;
        }
    }

    // Using `while` for Conditional Looping
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // Using `for` Loops for Iteration over a Collection
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // Ownership

    // Creating and Moving a String
    let s1 = String::from("hello");
    let s2 = s1; // Ownership of the string is moved to s2
    // println!("{}", s1); // This line would cause an error because s1 is no longer valid

    // Cloning a String to Keep the Original Data
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 is still valid after cloning: {}", s1);

    // Passing a String to a Function and Transferring Ownership
    let s = String::from("hello");
    take_ownership(s);
    // println!("{}", s); // This line would cause an error because ownership of s has been moved

    // References and Borrowing

    // Borrowing a String with Immutable Reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // Borrowing a String with Mutable Reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("String after change: {}", s);
}

// Function to Take Ownership of a String
fn take_ownership(some_string: String) {
    println!("Function that takes ownership: {}", some_string);
}

// Function to Calculate Length of a String using Reference
fn calculate_length(s: &String) -> usize {
    s.len()
}

// Function to Change a String using Mutable Reference
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
