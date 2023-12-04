use std::fs::File;
use std::io::{self, Read, ErrorKind};
fn main() {
    // panic! macrosu direk bizim tarafimizdan cagirilabilen bir makro
    // Execution i durdurur ve stack i bosaltir
    // panic!("crash and burn");

    // Asagidaki ornekte array de olmayan bir elemana erismeye calisiyoruz.
    // Eger bu C de olsaydi o lokasyonda ne varsa onu allirdik -> buffer overread
    // Ama rust bu durum olusmasin diye panikliyor ve execution i durduruyor.
    // let v = vec![1, 2, 3];
    // v[99];

    // Result enum i
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     },
    // };

    // Yukaridaki konseptin daha kisa yazimi 
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // }); 
    // Unwrap yerine expect kullanirsak custom mesaj verebiliriz panic! e
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    // let res = read_username_from_file();
    // println!("{:?}", res);

    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

}

// Asagidaki method ilk satirinda ? isaretine geldiginde eger hata aliyorsa direk onu donduruyor ilerlemiyor.
fn read_username_from_file() -> Result<String, io::Error> {
    println!("Method started");
    let mut username_file = File::open("hello.txt")?;
    println!("passed ?");
    let mut username = String::new();
    println!("Did not return after ?");
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Yukaridaki fonksiyonu asagidaki gibi daha kisa sekilde de yazabiliriz
fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

