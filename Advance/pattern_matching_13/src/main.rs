fn main() {
    // Reminder about 'match'
    let x = Option::Some(5);
    match x {
        Some(x) => println!("{x}"),
        None => println!("None"),
    }

    // We can also use 'if let' instead of 'match'
    // Below is a combination of 'if let' and 'else' cases
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // The drawback of the above method compared to 'match' is not checking all possibilities

    // While let
    fn main() {
        let mut stack = Vec::new();

        stack.push(1);
        stack.push(2);
        stack.push(3);

        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // For loop
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // Actually, we were constantly using pattern with let
    //let PATTERN = EXPRESSION;
    let x = 5; // Here, x is the pattern and 5 is the expression, and we bind the result of expression to x
    let (x, y, z) = (1, 2, 3); // Here we can see how the result of an expression is destructed and bound to patterns

    // Function parameters
    fn foo(x: i32) {
        // code goes here
    }
    // In the above function, the part 'x: i32' is also a pattern

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);

    // Refutability
    // Function parameters, let statements, and for loops accept irrefutable patterns because the program cannot do anything meaningful when these values do not match
    // Match arm except the last one must be irrefutable
    // if let and while let accept both refutable and irrefutable patterns
    // let Some(x) = some_option_value;
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    // if let x = 5 {
    //     println!("{}", x);
    // };

    // In the code below, 'y' inside the match is a new variable and shadows the outside one
    // The second arm 'Some(y)' will execute, and here 'y' is actually the 5 inside 'x'
    // When the match ends, the scope of the newly created 'y' ends, and the old value of 'y' is printed
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // We can use 'or' syntax in match
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // Using ..= in match
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // Destructuring Values
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p; // Below is the shorter alternative
                                  // let Point { x, y } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Destructuring in match
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructuring Enums
    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // let msg = Message::ChangeColor(0, 160, 255);

    // match msg {
    //     Message::Quit => {
    //         println!("The Quit variant has no data to destructure.");
    //     }
    //     Message::Move { x, y } => {
    //         println!(
    //             "Move in the x direction {x} and in the y direction {y}"
    //         );
    //     }
    //     Message::Write(text) => {
    //         println!("Text message: {text}");
    //     }
    //     Message::ChangeColor(r, g, b) => println!(
    //         "Change the color to red {r}, green {g}, and blue {b}",
    //     ),
    // }

    // Nested Structure Destructuring
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}")
        }
        _ => (),
    }

    // If we want to go crazy
    struct Point2 {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point2 { x, y }) = ((3, 10), Point2 { x: 3, y: -10 });

    // To ignore values in destructuring, use ..
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    // This syntax can also be used for tuples
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // The following code will throw an error because it is ambiguous
    // let numbers = (2, 4, 8, 16, 32);

    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }

    // Match Guard
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // Rewriting the above code with match guard
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Combining match guard expressions with |
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // @ Binding
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}
