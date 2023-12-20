fn main() {
    // Closures are functions that can capture the environment in which they are defined
    // They can be defined using the syntax |parameters| -> return_type { body }
    // The parameter types and the return type are optional and can be inferred by the compiler
    // Closures can capture variables from the environment by value, by reference, or by mutable reference
    // The way of capturing variables can be indicated by the move keyword or the &mut keyword
    // Closures implement one of the three variants of the Fn trait: Fn, FnMut, or FnOnce
    // These variants correspond to the different ways of capturing variables

    // Closure usage examples

    // A closure that takes no arguments and returns a string
    let _hello = || -> String { "Hello, world!".to_string() };

    // A closure that takes a string slice as an argument and returns a string
    let _greet = |name: &str| -> String { format!("Hello, {}!", name) };

    // A closure that takes two integers as arguments and returns an integer
    let _add = |x: i32, y: i32| -> i32 { x + y };

    // A closure that captures a variable by value and returns a string
    let name = String::from("Alice");
    let _introduce = move || -> String { format!("My name is {}.", name) };

    // A closure that captures a variable by mutable reference and returns nothing
    let mut count = 0;
    let _increment = || {
        count += 1;
        println!("Count is {}", count);
    };

    // A closure that captures a variable by reference and returns a boolean
    let threshold = 10;
    let _is_above_threshold = |x: i32| -> bool { x > threshold };

    // Iterators are objects that can produce a sequence of values
    // Iterators are lazy, meaning that they do not produce the values until they are requested
    // Iterators can be created from collections using the iter method
    // Iterators can also be created using other methods, such as range, repeat, or cycle
    // Iterators implement the Iterator trait, which has one main method: next
    // The next method returns an Option value that contains either the next value or None
    // Iterators also have many other methods that provide additional functionality, such as filter, map, sum, collect, and more
    // These methods are called iterator adaptors, because they take an iterator and return another iterator that adapts the behavior
    // Iterator adaptors are generic over the type of the iterator and the type of the closure that they take as an argument
    // Iterator adaptors can be chained together to create complex and expressive computations

    // Iterators in Loops

    // One of the most common ways to use iterators is to loop over them using a for loop
    // The for loop takes an iterator as an argument and executes a block of code for each value produced by the iterator
    // The for loop consumes the iterator, meaning that it uses up all the values produced by the iterator
    // The for loop handles the termination condition automatically, stopping when the iterator runs out of values

    // A for loop that prints the elements of a vector
    let numbers = vec![1, 2, 3, 4, 5];
    for n in numbers.iter() {
        println!("{}", n);
    }

    // A for loop that prints the characters of a string
    let message = "Hello, world!";
    for c in message.chars() {
        println!("{}", c);
    }

    // A for loop that prints the numbers from 1 to 10
    let numbers = vec![1, 2, 3, 4, 5];
    let is_even = |x: &i32| *x % 2 == 0; // Remove the dereference operator *
    let square = |x: &i32| x * x;
    let sum_of_squares: i32 = numbers
        .iter()
        .filter(|&&x| is_even(&x))
        .map(|x| square(x))
        .sum();
    println!("{}", sum_of_squares);

    // An example of using a closure with an iterator to find the longest word in a string
    let sentence = "This is a sentence with some words";
    let longest_word: String = sentence
        .split_whitespace()
        .max_by_key(|s| s.len())
        .unwrap()
        .to_string();
    println!("{}", longest_word);

    // An example of using a closure with an iterator to create a new vector that contains the words that start with a vowel in a string
    let _sentence = "This is another sentence with different words";
    let _is_vowel = |c: char| -> bool {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' | 'A' | 'E' | 'I' | 'O' | 'U' => true,
            _ => false,
        }
    };
    // Add the rand crate as a dependency in your Cargo.toml file
    // This will resolve the unresolved import `rand`
    use rand::Rng; // import the rand crate to use random number generation
    let mut rng = rand::thread_rng(); // create a random number generator
    let names = ["Alice", "Bob", "Charlie", "David", "Eve"]; // create an array of names
    let mut random_name = || -> String {
        // declare the closure as mutable
        let index = rng.gen_range(0..names.len()); // generate a random index
        names[index].to_string() // return the name at the index
    };
    for name in (0..10).map(|_| random_name()) {
        // loop over the first 10 values of the closure
        println!("{}", name);
    }

    // Using a closure that takes one argument and returns a boolean
    // This type of closure can be used to filter a sequence of values, such as numbers, strings, or structs

    // An example of using a closure that takes one argument and returns a boolean to filter a vector of numbers by keeping only the prime numbers
    let _numbers = vec![2, 3, 4, 5, 6, 7, 8, 9, 10];
    let words = vec!["hello", "world", "rust"]; // Declare and initialize the `words` variable
    let _is_prime = |&x: &i32| -> bool {
        if x <= 1 {
            return false;
        }
        for i in 2..x {
            if x % i == 0 {
                return false;
            }
        }
        true
    };
    let reverse = |s: &&str| s.chars().rev().collect::<String>();
    let reversed_words: Vec<String> = words.iter().map(reverse).collect();
    // create a new vector that contains the reversed words of the original vector
    println!("{:?}", reversed_words);
}
