fn main() {
    //Structs

    //Struct ornegi
    // struct User {
    //     active: bool,
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    // }

    //////////////

    //Struct instance
    // let user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    //////////////

    //Mutable ornek
    // let mut user1 = User {
    //     email: String::from("someone@example.com"),
    //     username: String::from("someusername123"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    //////////////

    //Field guncelleme
    // user1.email = String::from("anotheremail@example.com");
    //////////////

    //Struct yaratma fonksiyonu
    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         email: email,
    //         username: username,
    //         active: true,
    //         sign_in_count: 1,
    //     }
    // }

    //////////////

    //Kisayol ornegi
    // fn build_user2(email: String, username: String) -> User {
    //     User {
    //         email,
    //         username,
    //         active: true,
    //         sign_in_count: 1,
    //     }
    // }

    //////////////

    //////////////

    //Kisayol syntax ornegi
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     ..user1
    // };

    //////////////

    //Tuple struct ornegi
    // struct Color(i32, i32, i32);
    // struct Point(i32, i32, i32);

    //////////////

    //Unit like structs
    // struct AlwaysEqual;

    //////////////

    //Ownership of struct

    //////////////

    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!("Benim dikdortgenim {:?}", rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );

    //////////////

    //Enums
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;

    // enum IpAddr {
    //     V4(String),
    //     V6(String),
    // }

    // let home = IpAddr::V4(String::from("127.0.0.1"));
    // let loopback = IpAddr::V6(String::from("::1"));

    //////////////

    // enum IpAddr {
    //     V4(u8, u8, u8, u8),
    //     V6(String),
    // }

    // let home = IpAddr::V4(127, 0, 0, 1);
    // let loopback = IpAddr::V6(String::from("::1"));

    //////////////

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         println!("Mesaj enumdaki fonksiyon cagrildi");
    //     }
    // }

    // let m = Message::Write(String::from("hello"));
    // m.call();

    //////////////

    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    //////////////

    // let some_number = Some(5);
    // let some_char = Some('e');

    // let absent_number: Option<i32> = None;

    //////////////

    //Asagidaki kod hata verecek
    // let x: i8 = 5;
    // let y: Option<i8> = Some(5);

    // let sum = x + y;

    //////////////

    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter,
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter => 25,
    //     }
    // }

    //////////////

    // #[derive(Debug)] // so we can inspect the state in a minute
    // enum UsState {
    //     Alabama,
    //     Alaska,
    //     California,
    //     // boyle devam ediyor
    // }

    // enum Coin {
    //     Penny,
    //     Nickel,
    //     Dime,
    //     Quarter(UsState),
    // }

    // fn value_in_cents(coin: Coin) -> u8 {
    //     match coin {
    //         Coin::Penny => 1,
    //         Coin::Nickel => 5,
    //         Coin::Dime => 10,
    //         Coin::Quarter(state) => {
    //             println!("State quarter from {:?}!", state);
    //             25
    //         }
    //     }
    // }

    // let para = Coin::Quarter(UsState::California);

    // let sonuc = value_in_cents(para);
    // println!("{}", sonuc);

    //////////////

    // fn plus_one(x: Option<i32>) -> Option<i32> {
    //     match x {
    //         None => None,
    //         Some(i) => Some(i + 1),
    //     }
    // }

    // let five = Some(5);
    // let six = plus_one(five); // => Some(6)
    // let none = plus_one(None);

    ////////////
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     other => move_player(other),
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn move_player(num_spaces: u8) {}

    ////////////

    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => reroll(),
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}
    // fn reroll() {
    //     //Bos fonksiyon
    // }

    ////////////
    //Tuple type da _ da alinacak degerler icin kodun o kismi pas gecmesi acisindan kullanilabilir
    // let dice_roll = 9;
    // match dice_roll {
    //     3 => add_fancy_hat(),
    //     7 => remove_fancy_hat(),
    //     _ => (),
    // }

    // fn add_fancy_hat() {}
    // fn remove_fancy_hat() {}

    ////////////
    // let config_max = Some(3u8);
    // match config_max {
    //     Some(max) => println!("The maximum is configured to be {}", max),
    //     _ => (),
    // }
    // Bu yukardaki kodun esiti asagida if let ile
    // let config_max: Option<i32> = None;
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }

    /////////
    //if let li ifadeler else ile de baglanabilir
    // let mut count = 0;
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     count += 1;
    // }

    ////////
    // String Slices
    // let s = String::from("hello world");

    // let hello: &str = &s[0..5]; // slice
    // let world: &str = &s[6..11]; // slice
    // let s2: &String = &s; // slice degil

    // let s = String::from("hello");
    // let slice = &s[0..2];
    // let slice = &s[..2];

    // let len = s.len();
    // let slice = &s[3..len];
    // let slice = &s[3..];

    // let slice = &s[0..len];
    // let slice = &s[..];

    // // Slice lar sadece string slice olmak zorunda degil
    // let a = [1, 2, 3, 4, 5];
    // let slice = &a[1..3]; // [2, 3, 4]
}

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
