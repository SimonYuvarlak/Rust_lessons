fn main() {
    let x = 5; // x has a lifetime of the entire main function
    let r = &x; // r has a lifetime of the inner scope
    println!("r: {}", r); // r is valid here
                          // x and r go out of scope and are dropped
}

// fn main_with_error() {
//     let r; // r has no initial value
//     {
//         let x = 5; // x has a lifetime of the inner scope
//         r = &x; // r has a lifetime of the inner scope
//     } // x goes out of scope and is dropped
//     println!("r: {}", r); // r is not valid here
// }

fn main_with_inner_scope() {
    {
        let x = 5; // x has a lifetime of the inner scope
        let r = &x; // r has a lifetime of the inner scope
        println!("r: {}", r); // r is valid here
    } // x and r go out of scope and are dropped
}

fn main_with_return_value() {
    let r = foo(); // r has a lifetime of the main function
    println!("r: {}", r); // r is valid here
}

fn foo() -> i32 {
    let x = 5; // x has a lifetime of the foo function
    x // x is returned by value, not by reference
}

fn bar<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn main_with_bar() {
    let x = 5; // x has a lifetime of the main function
    let y = 6; // y has a lifetime of the main function
    let z = bar(&x, &y); // z has a lifetime of the main function
    println!("z: {}", z); // z is valid here
}

// fn main_with_error_in_bar() {
//     let z; // z has no initial value
//     {
//         let x = 5; // x has a lifetime of the inner scope
//         let y = 6; // y has a lifetime of the inner scope
//         z = bar(&x, &y); // z has a lifetime of the inner scope
//     } // x and y go out of scope and are dropped
//     println!("z: {}", z); // z is not valid here
// }

fn baz<'a, 'b: 'a>(x: &'a i32, y: &'b i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn baz_with_constraints<'a, 'b, 'c>(x: &'a i32, y: &'b i32) -> &'c i32
where
    'a: 'c,
    'b: 'c,
{
    if x > y {
        x
    } else {
        y
    }
}

fn qux<'r>(x: &'r i32) -> &'r i32 {
    x
}

fn main_with_different_lifetime_in_bar() {
    let x = 5; // x has a lifetime of the main function
    let y = &x; // y has a lifetime of the main function
    let z = bar_with_different_lifetime(y); // z has a lifetime of the bar function
    println!("z: {}", z); // z is valid here
}

fn bar_with_different_lifetime<'a, 'b>(x: &'a i32) -> i32
where
    'b: 'a,
{
    let y = 6; // y has a lifetime of the bar function
    y // return y by value, not by reference
}

fn main_with_return_value_in_bar() {
    let x = 5; // x has a lifetime of the main function
    let y = &x; // y has a lifetime of the main function
    let z = bar_with_return_value(y); // z has a lifetime of the main function
    println!("z: {}", z); // z is valid here
}

fn bar_with_return_value<'a>(x: &'a i32) -> i32 {
    let y = 6; // y has a lifetime of the bar function
    y // return y by value, not by reference
}
