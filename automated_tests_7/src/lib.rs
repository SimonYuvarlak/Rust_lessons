pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub fn add(first: i32, second: i32) -> i32 {
    first + second
}

fn private_function() {
    println!("This function is private");
}

#[cfg(test)]
mod tests {
    use super::*;

    //assert!
    // #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(result.contains("Car"));
    // }

    //assert_eq!
    // #[test]
    // fn add_works() {
    //     let result = add(1, 5);
    //     assert_eq!(6, result);
    // }

    //assert_ne!
    // #[test]
    // fn add_not_working() {
    //     let result = add(2, 2);
    //     assert_ne!(6, result);
    // }

    //custom mesaj ekleyebiliriz hatalarimiza
    // #[test]
    // fn add_with_custom_message() {
    //     let result = add(2, 3);
    //     assert_eq!(4, result, "result 4 olmaliydi ama {} degeri bulundu", result);
    // }

    //should_panic
    // #[test]
    // #[should_panic]
    // fn add_should_panic() {
    //     let result = add(2, 3);
    //     assert_eq!(4, result);
    // }

    //Result kullanimi | #[should_panic] -> Result donduren fonksiyonlarda kullanilamaz
    // #[test]
    // fn add_result() -> Result<(), i32> {
    //     let result: i32 = add(17, 3);
    //     if result == 20 {
    //         Ok(())
    //     } else {
    //         Err(0)
    //     }
    // }

    //Normalde rust testte bircok test oldugunda, testler paralel calisiyorlar.
    //Ayni dosyaya yaziyorlarsa ornegin, ayni anda birkac testin ayni dosyaya yazmaya calismasi ve o
    //dosyalari okumaya calismasi hata verebilir. Bunun icin ya ayni dosyanin kullanilmadigindan 
    //emin olmamiz gerekiyor ya da 
    //$ cargo test -- --test-threads=1
    //komutu ile tek thread de calismasini saglayabiliriz

    //Biz testler fail ladiginda sadece output yazdiriyorlarsa onlari goruyoruz.
    // #[test]
    // fn this_will_fail() {
    //     println!("Bu fail olan testin ouputu");
    //     assert!(false);
    // }

    //Eger gecen testlerde outputu gormek istiyorsak asagidaki komutu kullanabiliriz
    //$ cargo test -- --show-output
    // #[test]
    // fn this_will_pass() {
    //     print!("Bu gecen testin outputu");
    //     assert!(true);
    // }

    //$ cargo test komutu ile elimizdeki butun testler calisiyor
    //Ama biz cargo test <test_fonksiyonunun_adi> seklinde yazarsak tek test calisir

    //#[ignore] ile belli testleri ignore edebiliriz
    //Mesela asagidaki fonksiyon cargo test komutunda ignore edilecek
    //Sadece ignore lu olan testleri runlamak icin asagidaki commandimizi kullanabiliriz
    //$ cargo test -- --ignored
    //Eger ki ignore lu ya da degil butun testleri calistirmak istiyorsak:
    //$ cargo test -- --include-ignored
    // #[test]
    // #[ignore]
    // fn this_will_be_ignored() {
    //     println!("This test will be ignored");
    // }

    //Rust ta private functionlari da test edebiliriz
    // #[test]
    // fn private_function_test() {
    //     private_function();
    // }

    
}