mod functions;
use crate::functions::hashmap;
// use std::collections::HashMap;

fn main() {
    // Vector ********************

    // Vector yaratma
    // let v: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3];

    // Vector e yani elemanlari push fonksiyonu ile ekleyebiliriz
    // v.push(4);
    // println!("{}", v.len());

    // Vectorden eleman okuma
    // let ilk_eleman = &v[0];
    // println!("ilk eleman -> {}", ilk_eleman);

    // Get metodu ile eleman alirsak o zaman Option doner
    // let son_eleman = v.get(v.len() - 1);
    // match son_eleman {
    //     Some(eleman) => {
    //         println!("Son eleman -> {}", eleman);
    //     },
    //     None => {
    //         println!("Erisimde sorun yasandi.");
    //     }
    // }

    // Eger mutable bir vectorun elemanini mutable olmayan bir variable a esitlersek sonra o vektor u kullanirken sikinti yasariz
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // println!("The first element is: {first}");
    // v.push(6);


    // Vector un elemanlarini yazdirma=
    // for i in &v {
    //     println!("{}", i);
    // }

    // for i in &mut v {
    //     *i -= 1;
    //     println!("{}", i);
    // }

    // Vectorde ayni tipte elemanlari tutabiliriz ama farkli tiptekileri tutmak istersek de enum kullanabiliriz

    // String ********************
    // to_string() metodu
    // let orijinal_string = String::from("orijinal string");
    // let kopya_string = "Ilk string".to_string();
    // println!("Orijinal string -> {}", orijinal_string);
    // println!("Kopya string -> {}", kopya_string);

    // bos string yaratma
    // let mut bos_string = String::new();

    // string upadateleme
    // bos_string.push_str("dolu artik bu string");
    // println!("Dolu string -> {}", bos_string);

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // s1 move edildi ve artik kullanilamaz

    // println!("s1 -> {}", s1);
    // println!("s1 -> {}", s2);
    // println!("s1 -> {}", s3);
    
    // chars() fonsksiyonu
    // Bircok dil stringlerde my_string[index] desteklese de rust desteklemiyor.
    // Bu islem icin chars() fonksiyonunu kullanabiliriz
    // let my_string = String::from("deneme");
    // for i in my_string.chars() {
    //     println!("{}", i);
    // }

    // HashMaps ********************
    // Hash map yaratimi
    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue@gmail.com"), 101265);
    // scores.insert(String::from("Yellow"), 50);
    // println!("{:?}", scores);

    // let team_name = String::from("Blue");
    // let score = scores.get(&String::from("Blue@gmail.com")).copied().unwrap_or(0);
    // println!("score -> {}", score);

    // Iterate
    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // Hashmap ve ownership
    // String heap te doplandigi icin burada insert ettigimizde, move ediliyor stack gibi kopyalanmiyor
    // O yuzden insert isleminden sonra string degerleri tekrar kullanmamiyoruz
    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");
    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // let name = field_name;

    // Overwrite a value
    // Burada once Blue ya 20, sonra o 20 yerine 25 yazilir ve son degeri 25 burda
    // scores.insert(String::from("Blue"), 20);
    // scores.insert(String::from("Blue"), 25);
    // println!("{:?}", scores);

    // entry fonksiyonu eger o deger yoksa degeri yaratir varsa ellemez hash map i
    // scores.entry(String::from("Black")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);
    // println!("{:?}", scores);

    // Asagidaki fonksiyon verilen cumlede kelimelerin kac kere gectigini sayiyor
    let my_map = hashmap::count_words("Deneme bir iki, deneme bir ki uc");
    println!("{:?}", my_map);
}
