# Pattern Matching in Rust

Pattern matching is a powerful feature of Rust that allows you to compare a value against a series of patterns and execute code based on which pattern matches. Pattern matching can be used to simplify code, avoid repetition, and handle complex scenarios.

In this task, you will learn how to use pattern matching in Rust, a programming language that supports various kinds of patterns, such as literals, variables, enums, structs, tuples, arrays, slices, references, and ranges.

The task consists of the following steps:

1. Read the tutorial on pattern matching in Rust. This tutorial will explain the basic concepts of pattern matching, such as match expressions, if let and while let statements, and the @ operator. You will also see some examples of how to use pattern matching to implement algorithms based on types and data.
2. Write a program that uses pattern matching to calculate the area of different shapes. The shapes are defined as an enum with three variants: Circle, Rectangle, and Triangle. Each variant has a tuple as its associated data, containing the relevant dimensions of the shape. For example, Circle(f64) has a single f64 value as its radius, Rectangle(f64, f64) has two f64 values as its width and height, and Triangle(f64, f64, f64) has three f64 values as its sides. The area of each shape can be calculated using the following formulas:

   - Circle: $$\pi r^2$$
   - Rectangle: $$w \times h$$
   - Triangle: $$\sqrt{s(s-a)(s-b)(s-c)}$$ where $$s = \frac{a+b+c}{2}$$

   Use a match expression to evaluate the area based on the shape variant and its associated data. Use the std::f64::consts::PI constant for the value of $$\pi$$.

3. Test your program with different inputs and verify that it produces the correct output. For example, if the shape is Circle(2.0), the area should be 12.566370614359172. If the shape is Rectangle(3.0, 4.0), the area should be 12.0. If the shape is Triangle(3.0, 4.0, 5.0), the area should be 6.0.
