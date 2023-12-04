fn main() {
    // Defining a User struct with four fields
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // Creating an instance of the User struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Creating a mutable instance of the User struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        ..user1 // copies other fields from the given instance, in this case user1 (which we created above as immutable)...
    };

    // Updating a field of the mutable User instance
    user1.email = String::from("anotheremail@example.com");

    // Function acting as a constructor for the User struct
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // Using struct update syntax to create a new User instance from an existing one
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };

    // Defining a tuple struct Color with three i32 fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    // Defining a unit-like struct that does not have fields
    struct AlwaysEqual;

    // Enums for representing IP address kinds
    enum IpAddrKind {
        V4,
        V6,
    }

    // Creating instances of the IpAddrKind enum
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Defining an enum with associated data
    enum IpAddr {
        V4(String),
        V6(String),
    }

    // Creating instances of the IpAddr enum
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    // Defining a Message enum with different variants
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // Implementing a method for the Message enum
    impl Message {
        fn call(&self) {
            // method body
        }
    }

    // Creating an instance of the Message enum and calling its method
    let m = Message::Write(String::from("hello"));
    m.call();

    // Using the Option enum to handle values that may or may not exist
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // Using match with enums to handle different cases
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    // Using if let as a concise alternative to match for certain cases
    let config_max: Option<i32> = None;
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

// Defining a struct outside of main for demonstration purposes
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementing a method for the Rectangle struct
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}
