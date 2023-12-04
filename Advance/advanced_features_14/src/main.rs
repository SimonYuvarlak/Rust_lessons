fn add_one(x: i32) -> i32 {
    x + 1
}

// Fonksiyonlar, baska fonksiyonlari parametre olarak alabilir
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    // Advanced Fonksiyonlar -----

    // Asagidaki do_twice fonksiyonu kendi icinde add_one fonksiyonunu cagiriyor
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // Map e closure verme yontemi
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = 
        list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Map e closure yerine fonksiyon verme
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();

    // Asagidaki enumda Value(32) aslinda bir fonksiyon 
    // Biz de bundan asagidaki satirdaki kullanimla yararlanabiliriz
    enum Status {
        Value(u32),
        Stop,
    }
    // Burada 0 dan 20 ye kadar Status::Value degerleri olusturup bunlari vector olarak variable a dondurecek
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // Bunun compile time da boyutu belli olmadigi icin hata verecek
    //
    // fn returns_closure() -> dyn Fn(i32) -> i32 {
    //     |x| x + 1
    // }

    // Yukaridaki sorun asagidaki gibi trait object ile cozulebilir
    // Box in boyutu belli oldugu icin compile time size belirsizligi sorunu ortadan kalkiyor
    fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    
}
