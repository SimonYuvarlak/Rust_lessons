fn main() {
    // Match hatirlatmasi
    let x = Option::Some(5);
    match x {
        Some(x) => println!("{x}"),
        None => println!("None"),
    }

    // Match yerine if let de kullanabiliriz
    // Asagida if let ve else case lerinin kombinasyonunu gorebiliriz
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

    // Yukarinin match den eksigi butun olasiliklari kontrol etmemesi

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

    // Aslinda biz let kullanarak surekli pattern kullaniyorduk
    //let PATTERN = EXPRESSION;
    let x = 5; // Burada aslinda x pattern ve 5 de expression oluyor ve biz expression sonucunda ne oluyorsa onu x e bagla diyoruz
    let (x, y, z) = (1, 2, 3); // Burada use biraz daha nasil destruct edilip exression sonucunun patternlara baglandigini gorebiliriz

    // Function parameters
    fn foo(x: i32) {
        // code goes here
    }
    // Yukaridaki fonksiyonda aslinda x: i32 kismi da bir pattern

    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
    
    // Refutability
    // Function parameters, let statements, ve for loops irrefutable alabilir cunku program bu degerler match etmediginde anlamli bir sey yapamaz
    // Match de son kol haric irrefutable olmasi lazim
    // if let ve while let, refutable ve irrefutable kabul ediyor
    // let Some(x) = some_option_value;
    // if let Some(x) = some_option_value {
    //     println!("{}", x);
    // }
    // if let x = 5 {
    //     println!("{}", x);
    // };

    // Asagidaki kodda match in icindeki y yeni bir variable ve disaridakini shadowluyor
    // Ikinci arm olan Some(y) calisacak ve buradaki y aslinda x in icindeki olan 5
    // Match bitince scope u bittigi icin yeni yaratilan y yok oluyor ve print te eski y degeri yaziyor
    let x = Some(5);
        let y = 10;
    
        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {:?}", x),
        }
    
        println!("at the end: x = {:?}, y = {y}", x);

    // Match icinde or syntax i kullanabiliriz
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // March te ..= kullanimi
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

    // Value Destruction
    struct Point {
        x: i32,
        y: i32,
    }
    
    let p = Point { x: 0, y: 7 };
    
        let Point { x: a, y: b } = p; //Altta alternatif kisa yazilimi var
        // let Point { x, y } = p;
        assert_eq!(0, a);
        assert_eq!(7, b);

    // Destruction in match le kullanimi
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // Destructing Enums
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

    // Nested tructure destruction
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
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!(
            "Change color to hue {h}, saturation {s}, value {v}"
        ),
        _ => (),
    }

    // Eger cok cildirmak istersek
    struct Point2 {
        x: i32,
        y: i32,
    }

    let ((feet, inches), Point2 { x, y }) = ((3, 10), Point2 { x: 3, y: -10 });

    //Destruction da degerleri ignore etmek icin .. kullanabiliriz
    struct Point3 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3 { x: 0, y: 0, z: 0 };

    match origin {
        Point3 { x, .. } => println!("x is {}", x),
    }

    // Bu syntax i ayni zamanda tuple lar icin de kullanabiliyoruz
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // Asagidaki kod belirsiz oldugu icin hata verecektir
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

    // Yukaridaki kodun match guard ile yeniden yazimi
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);

    // Match guard expression larini | ile baglama
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
