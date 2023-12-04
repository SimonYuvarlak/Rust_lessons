# **Structs and Enums in Rust**

#### **Understanding Structs**

Structs, or structures, are a way of organizing related data in Rust. They are similar to classes in other languages, providing a way to group related data fields together.

##### **Defining and Instantiating Structs**

- **Defining a Struct**

  ```rust
  // A User struct with several fields
  struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```

- **Creating an Instance of a Struct**

  ```rust
  // Creating an instance of User
  let user1 = User {
      email: String::from("someone@example.com"),
      username: String::from("someusername123"),
      active: true,
      sign_in_count: 1,
  };
  ```

- **Mutability of Structs**
  Rust's struct instances are immutable by default. However, you can make the entire instance mutable.

  ```rust
  // Making a User instance mutable
  let mut user1 = User {
      email: String::from("someone@example.com"),
      // other fields...
  };

  // Updating a field in a mutable struct
  user1.email = String::from("anotheremail@example.com");
  ```

- **Constructors and Field Init Shorthand**
  Rust does not have constructors in the traditional sense. However, you can define functions that create instances.

  ```rust
  // Function that acts as a constructor
  fn build_user(email: String, username: String) -> User {
      User {
          email: email,
          username: username,
          active: true,
          sign_in_count: 1,
      }
  }

  // Shorthand for field initialization
  fn build_user2(email: String, username: String) -> User {
      User {
          email,
          username,
          active: true,
          sign_in_count: 1,
      }
  }
  ```

- **Struct Update Syntax**
  This syntax allows you to create a new struct instance using values from an existing instance.
  ```rust
  // Creating a new User instance from an existing one
  let user2 = User {
      email: String::from("another@example.com"),
      ..user1
  };
  ```

##### **Tuple Structs and Unit-Like Structs**

- **Tuple Structs**
  These are similar to tuples, where fields have types but do not have names.

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  ```

- **Unit-Like Structs**
  These do not have any fields and are useful for traits.
  ```rust
  struct AlwaysEqual;
  ```

#### **Exploring Enums**

Enums in Rust allow you to define a type by enumerating its possible variants.

##### **Basic Enums**

- **Defining an Enum**

  ```rust
  enum IpAddrKind {
      V4,
      V6,
  }

  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  ```

- **Enums with Data**
  Enums can also store different types of data.

  ```rust
  enum IpAddr {
      V4(String),
      V6(String),
  }

  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));
  ```

- **Method Definitions in Enums**
  Enums can have methods defined on them.

  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }

  impl Message {
      fn call(&self) {
          // method body
      }
  }
  ```

- **The `Option` Enum**
  Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent.

  ```rust
  enum Option<T> {
      None,
      Some(T),
  }

  let some_number = Some(5);
  let some_char = Some('e');
  let absent_number: Option<i32> = None;
  ```

##### **Match Control Flow Operator**

Rust’s `match` allows you to compare a value against a series of patterns and then execute code based on the matching pattern.

- **Basic Match Usage**

  ```rust
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
          Coin::Quarter => 25
  ```

,
}
}

````

- **Patterns and Destructuring**
```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
````

##### **Using `if let`**

`if let` is a syntax sugar for a `match` that runs code when the value matches one pattern and ignores other patterns.

- **`if let` Example**
  ```rust
  let config_max: Option<i32> = None;
  if let Some(max) = config_max {
      println!("The maximum is configured to be {}", max);
  }
  ```

#### **Conclusion**

In this section, we explored Rust's structs and enums, which are fundamental to structuring and handling data in Rust. We covered how to define and use structs, including tuple structs and unit-like structs, and how to define and match enums, including the powerful `Option` enum. These tools provide a robust framework for handling complex data structures and various cases in Rust, significantly enhancing the language's expressiveness and safety.
