# **Student Task: Exploring Structs and Enums in Rust**

#### **Task Overview**

This exercise is designed to apply the concepts learned in the "Structs and Enums in Rust" section of the tutorial. You will create a Rust program to demonstrate your understanding of struct and enum usage, including methods, pattern matching, and more.

#### **Task Description**

1. **Develop a Rust Program:**

   - Set up a new Rust project using Cargo.

2. **Implement Key Features:**

   - **Structs Implementation:**

     - Define a struct named `Book` with fields like `title`, `author`, and `year`.
     - Create a function that returns an instance of `Book`.
     - Instantiate a `Book` in `main` and print its details.

   - **Enums and Pattern Matching:**

     - Define an enum `Genre` with variants like `Fiction`, `NonFiction`, `Fantasy`, and `Biography`.
     - Add a `genre` field of type `Genre` to your `Book` struct.
     - Use a `match` statement in `main` to print a custom message based on the book's genre.

   - **Method Definitions in Structs:**
     - Implement a method for the `Book` struct that returns a formatted string containing the book's details.

3. **Advanced Enum Usage:**

   - Create an enum `LibraryItem` with variants `BookItem(Book)` and `MediaItem(String)`.
   - Demonstrate how to use pattern matching with `if let` to check if a `LibraryItem` is a `BookItem` and then print its details.

4. **Expected Output**
   - Your program should compile and run, printing the details of the book, messages based on its genre, and handling different types of library items.

This task will help reinforce your understanding of Rust's structs and enums, crucial for creating well-structured and robust programs. Good luck!
