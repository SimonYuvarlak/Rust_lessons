# Lifetime Task

## Task Description

Your task is to write a Rust program that demonstrates the use of lifetimes. Lifetimes are an important concept in Rust that help ensure memory safety and prevent dangling references.

## Requirements

1. Create a struct called `Person` with a single field `name` of type `&str`.
2. Implement a method `greet` for the `Person` struct that takes a reference to `self` and prints a greeting message including the person's name.
3. Create a function called `main` that creates an instance of `Person` and calls the `greet` method.

## Constraints

- The `greet` method should not take ownership of the `Person` instance.
- The `name` field of the `Person` struct should have a lifetime specifier to ensure it doesn't outlive the struct.
