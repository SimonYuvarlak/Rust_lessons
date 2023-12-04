mod functions;
use crate::functions::first_module;
fn main() {
    // Generics ***********
    // Asagida klasik bir vector ornegi var
    // let mut vector_integer: Vec<i32> = vec![20,30];
    // vector_integer.push(40);
    // println!("{:?}",vector_integer);

    // Asagidaki kod hata verecek cunku vector ayni type ta elemanlar tutabilir
    // vector_integer.push("hello"); 
    // error[E0308]: mismatched types
    // println!("{:?}",vector_integer);

    // Simdi yukaridaki ornegi generic le bircok tip tutabilir hale getirelim
    // Oncelikle kullanmak icin yeni bir struct belirleyecegiz
    // struct Data<T> {
    //     value:T,
    // }

    // Simdi vektor tanimlamamizi yapacagiz
    // generic type of i32
    // let t:Data<i32> = Data{value:350};
    // println!("value is :{} ",t.value);
    //generic type of String
    // let t2:Data<String> = Data{value:"Tom".to_string()};
    // println!("value is :{} ",t2.value);
    // Trait *********
    // Trait leri object oriented programmingdeki abstract class lar gibi dusunebiliriz
    /*
        Trait ler, somut methodlar veya soyut methodlar içerebilir. 
        Method tanımı Trait i uygulayan tüm yapılar tarafından paylaşılacaksa somut bir yöntem kullanın.
        Ancak, bir yapı, özellik tarafından tanımlanan bir işlevi geçersiz kılmayı seçebilir.
    */
    //create an instance of the structure
    let b1 = Book {
        id:1001,
        name:"Rust in Action"
    };
    b1.print();
    // b1.default_print();
    // Traitler belli siniflarla varolabildikleri icin kendi baslarina struct impl lari gibi cagirilamazlar
    // Printable::sahipsiz_print();

    // Generic functions **********
    first_module::print_pro(10 as u8);
    first_module::print_pro(20 as u16);
    first_module::print_pro("Bir de string verelim");

}
//declare a structure
struct Book {
    name:&'static str,
    id:u32
 }
 //declare a trait
 trait Printable {
    fn print(&self);
    fn default_print(&self) {
        println!("Bu default belirledigimiz print function i");
    }
    fn sahipsiz_print() {
        println!("Bu sahipsiz print");
    }
 }
 //implement the trait
 impl Printable for Book {
    fn print(&self){
       println!("Printing book with id:{} and name {}",self.id,self.name)
    }
    fn default_print(&self) {
        println!("Bu book icin olan default print");
    }
 }

 // Generic function
 // prinln! macro su ile calismasi icin verilen type in Display trait ini implement etmis olmasi gerekli

