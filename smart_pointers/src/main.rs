use List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    // BOX -----
    // Box lar bizim heap te memory allocate etmemize yariyor.
    // Heap e istenilen deger kaydoluyor stack e ise o degerin referansi (o degerin bulundugu memory adresini isaret eden pointer) kaydoluyor.
    // Box i kullanabileceginiz durumlara birkac ornek:
    // Compile time da size i belli olmayan variable lar icin
    // Buyuk boyutlardaki datalari transfer etmek istediginizde ama kopyalamak istemediginizde
    // Trait object durumunda -> ileride konusulacak
    let my_box = Box::new(1);
    println!("benim kutum {}", my_box);

    // -----
    // 1, 2, 3 -> iceren liste enuminin yaratimi.
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // DeRef Trait -----
    // * sembolu pointerdan degeri almak icin kullaniliyor.
    let x = 5;
    let y = &x; // Burada y, x e esit degil, x i isaret eden bir pointera esit

    assert_eq!(5, x); // 5 = x
    assert_eq!(5, *y); // Burada biz *y diyerek y nin isaret ettigi adres lokasyonundaki degeri aliyoruz, burada 5
    // assert_eq!(5, y); // 5 y ye esit degil, y 5 in depolandigi lokasyonu isaret eden pointera esit.

    let y = MyBox::new(5); 
    // Bizim yukarida new() fonksiyonunun icine verdigimiz deger 5 degil 5 in kopyasi. 5 stack te depolandigi icin.
    // Yukaridaki ornekten farki bu.
    assert_eq!(5, *y);

    // -----
    // Implicit Deref Coercion
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    // Gecerli oldugu durumlar
    // From &T to &U when T: Deref<Target=U>
    // From &mut T to &mut U when T: DerefMut<Target=U>
    // From &mut T to &U when T: Deref<Target=U>

    // Drop Trait ----
    // Yaratilan smart pointerlar scope larinin sonuna geldiklerinde cagiriliyor bu metod.
    // Burada kaynaklari bosaltabilir veya istediginiz farkli bir islemi yapabilirsiniz.
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    // Biz bir degeri scope undan once birakmak istersek hata aliriz.
    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    println!("CustomSmartPointer dropped before the end of main.");
    // Yukaridaki kullanim yerine bizim icin standard kutuphanede yaratilmis bir fonksiyon kullanabiliriz.

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    std::mem::drop(c);
    println!("CustomSmartPointer dropped before the end of main.");

}

// Asagida bir tane recursive enum ornegimiz var.
// Bu deger recursive oldugu icin boyutu compile time da bilinmiyor.
// Enumlar size hesabi icin her bir degeri tek tek hesapliyor ve degeri en buyuk olana gore yer ayriiyor.
// Bu recursive durumda ise o field in size inin hesaplanamamasi sorun yaratiyor, o yuzden Box kullaniyoruz.
// Box smart pointer i fixed size oldugu icin degeri bilinebiliyor.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Smart pointer yaratimi
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
