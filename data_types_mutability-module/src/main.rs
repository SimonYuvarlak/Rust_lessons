// • Download Rust
// 	○ https://www.rust-lang.org/tools/install
// • Download Cargo
// 	○ https://doc.rust-lang.org/cargo/getting-started/installation.html
// • Rustlings
//     https://github.com/rust-lang/rustlings/

fn main() {
    // *************************** Mutability ***************************
    // Variable lar JS de oldugu gibi let kelimesi ile declera ediliyor
    // let x = 5;
    // println!("{}", x);

    // x = 26;
    // println!("{}", x);

    // let mut x = 5;
    // println!("x: {}", x);

    // x = 32;
    // println!("x: {}", x);

    // *************************** Data Types ***************************
    // Integers
    // i8, i32, i64, i128, i256 -> aynilari unsigned icin de gecerli
    // let x: u32 = 1;
    // let y: i32 = -1;
    // println!("x is {} and y is {}", x, y);

    // Asagidaki kod hata verecek
    // Unsigned integer lar  2^n kadar degere sahip olabilir
    // Alabilecegi maksimum deger 2^n-1 dir cunku 1 tane elemanini 0 icin kullaniyor.
    // Bu mantikla asagidaki kod hata verecektir cunku 2^8 = 256 olmasina ragmen 255 e kadar deger alabilir, 0 a yer actigi icin.
    // let z: u8 = 256;

    // Signed integer larin unsigned integerlardan farki pozitif deger alabilmeleri.
    // O yuzden signed integer lar unsigned integer larin yarisi kadar deger alabilirler.
    // Bu mantikla asagidaki kod hata verecektir cunku 2^n-1 / 2 kadar negatif deger alabilmesine ragmen negatif signed integer
    // Pozitif signed integer 2^n-1 / 2 - 1 kadar deger alabilir pozitife yer actigi icin.
    // let z: i8 = 128;

    // Asagidaki kod ise hata vermeyecektir cunku negatif degerlerde 0 dan dolayi oluan fark yok
    // let z: i8 = -128;

    // Overflow sorunu icin checked_<function_name> fonksiyonunu kullanabiliriz
    // let a: i8 = 6;
    // let b: i8 = 122; //Bunu 123 yapip dene
    // let c = a.checked_add(b); //Burada option donuyor elimize kesin bir deger donmuyor cunku overflow olabilir
    // println!("{:?}", c);

    // Floats
    // Iki tur float var, f32 veya f64
    // default f64
    // let x = 2.0; // Bu default olarak belirtilmedigi icin f64 oldugu varsayiliyor
    // let y: f32 = 11.0; // Belirtirsek f32 olarak da kullanabiliriz

    // Boolean
    // Rust ta boolean in diger dillerden ozel bir farki yok
    // Asil kullanim amaci genelde conditional lar ve control flow lar
    // let a = true;
    // let b: bool = false;
    // println!("a -> {} && b -> {}", a, b);

    // Character
    // Character diger yazilim dillerinde oldugu gibi
    // " " yerine ' ' kullaniyoruz
    // let first_letter = 'a';
    // println!("Alfabenin ilk harfi {}", first_letter);

    // Tuple
    // Birkac dilde olan multiple degerler icin kullaniliyor tuple lar.
    // Tuple lar fixed size o yuzden bir kere yaratildi mi modifiye edemiyoruz
    // let user: (&str, u8, char) = ("poseidon", 32, 'm');
    // let user_name = user.0;
    // let user_age = user.1;
    // let user_gender = user.2;
    // let (name, age, gender) = user;
    // println!("user {} {} yasinda bir {}", name, age, gender);

    // Array
    // Rust ta array ler fixed size
    // Diger dillerde oldugu gibi index 0 dan basliyor
    // Datayi heap yerine stack te tutmak istedigimizde faydali (heap ve stack i ilerde konusacagiz)
    // Array e benzer bir yapinin adi da vector. Array den farkli olarak onun size i degistirilebilir (ilerde konusacagiz)
    // let gunler: [&str; 7] = [
    //     "Pazartesi",
    //     "Sali",
    //     "Carsamba",
    //     "Persembe",
    //     "Cuma",
    //     "Cumartesi",
    //     "Pazar",
    // ];
    // println!("Haftanin ilk gunu {}", gunler[0]);

    // Ayni zamanda bir array i icerigini doldurarak da olusturabiliriz
    // array.len() fonksiyonu ile arrayin boyunu dondurebiliriz
    // let arr = [1; 4];
    // println!(
    //     "Arrayin ilk elemani {} arrayin son elemani {}",
    //     arr[0],
    //     arr[arr.len() - 1]
    // );

    // Asagidaki kod hata verecek cunku arrayin son elemanina ulasmaya calisiyoruz ama vermemiz gerekenden 1 fazla veriyoruz indexi
    // println!("{}", arr[arr.len()]);
    // *************************** Functions ***************************
    // no_parameter_no_return();
    // take_parameter_no_return(4);
    // let sonuc = take_parameter_return_value(300, 15);
    // println!("Your total cost is {}", sonuc);

    // Expression example
    // let eleven = {
    //     let a = 5;
    //     let b = 6;
    //     a + b
    // };
    // println!("Bu bir expression sonucu {}", eleven);

    // *************************** Control Flow ***************************
    // if
    // if in diger dillerden bir farki yok. Sadece Rustta parantezleri kullanmiyoruz.
    // let x = 15;
    // if x < 18 {
    //     println!("18 yasinin altindakilere satilmaz");
    // }

    // let is_full = true;
    // if is_full {
    //     println!("Araba dolu");
    // } else {
    //     println!("Buyrun");
    // }

    //Bazi dillerde gecerli olan ama Rust ta olmayan bir check
    // let num = 5;
    // if num {
    //     println!("Numara {}", num);
    // }

    // Bircok durumu ayni anda test edebiliyoruz
    // && -> ve demek
    // || -> veya demek
    // let x = 0;
    // if x > 0 && 5 >= x {
    //     println!("X 1 ile 5 arasinda");
    // } else if x > 5 {
    //     println!("X 5 ten buyuk");
    // } else {
    //     println!("X negatif veya 0");
    // }

    // If i ayni zamanda variable a atamada da kullanabiliriz.
    // let gender = 'm';
    // let user_gender = if gender == 'f' { "female" } else { "male" };
    // println!("Kullanicinin cinsiyeti {}", user_gender);

    // Loops
    // loop -> Rust ta diger dillerden farkli olarak infinite loop var direk kullanabilecegimiz
    // let mut index = 0;
    // loop {
    //     println!("Index {}", index);
    //     index += 1;
    //     if index > 5 {
    //         break;
    //     }
    // }

    // loop tan deger dondurup variable a atayabiliriz
    /////////////////////////////////////////////
    // let mut index = 0;
    // let is_ten = loop {
    //     if index == 10 {
    //         break index;
    //     } else {
    //         index += 1;
    //         continue;
    //     }
    // };
    // println!("{}", is_ten);

    /////////////////////////////////////////////
    // Asagidaki kod yukari ile ayni isi yapiyor
    // let mut i = 0;
    // let is_five: bool;
    // loop {
    //     if i == 5 {
    //         is_five = true;
    //         break;
    //     } else {
    //         i += 1;
    //         continue;
    //     }
    // }

    // println!("{}", is_five);

    // while loop
    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // for loop
    // let group_c = ["Argentina", "Poland", "Mexico", "Saudi Arabia"];
    // for team in group_c {
    //     println!("Takim adi : {}", team);
    // }

    // Eger bir numara araligi vermek istersek for a asagidaki gibi kullanabiliriz.
    // Burada 1 degeri dahil ama 4 degil
    // Sondaki .rev() fonksiyonu 1 den 4 e olan sirayi ters cevirerek 4 ten 1 e gidecek hale getiriyor .rev() -> reverse
    // for number in (1..4).rev() {
    //     println!("{number}!");
    // }
    // println!("LIFTOFF!!!");

    // match
    // Cok guclu bir ozellik bunu daha detayli incelyecegiz ama simdilik bunun diger dillerdeki switch in karsiligi oldugunu bilmek yeterli
    // let x = 1;
    // match x {
    //     0 => println!("x in degeri 0"),
    //     1 => println!("x in degeri 1"),
    //     2 => println!("x in degeri 2"),
    //     _ => println!("x in degeri 0, 1 veya 2 degil"),
    // }

    // *************************** Ownership ***************************
    // Iki farkli storage type imiz var; heap ve stack.
    /*
        3 kuralimiz var burada:
        Rust ta her degerin bir owner i vardir.
        Ayni anda sadece 1 tane owner olabilir.
        Sahibi scope disina cikinca deger birakilir (dropped).
    */

    // Degerler belirlendikleri scope larda kullanilabilirler
    // {
    //     // burada degeri tanimlamadik o yuzden burda kullanilamaz
    //     let deger = "tanimlandi"; // Burada deger tanimlandi
    //                               // Bu aralikta deger tanimli ve kullanilabilir.
    // } // Artik scope un disina cikiyoruz ve deger kullanilamaz.
    // Burada anlamamiz gereken, degerler tanimli olduklari araliklarda (scope) gecerliler ve bunun disina cikinca gecerliligini yitiriyorlar.
    // Bunu polis gibi dusunebilirsiniz, yetkisi oldugu araliklarda islevsel ama kendi isinin tanimli oldugu araliktan cikarsa mesela
    // baska bir ulke o zaman yetkisi gecersiz oluyor ve rutbesi kullanilmaz oluyor.

    // String
    // String ler heap te depolaniyor.
    /*
        Su ana kadar biz string literal lar gormustuk &str. Bunlar stack te depolaniyorlar ve immutable lar.
        String ise heap te depolaniyor ve mutable.
        String literal ve string farkini array ve vector olarak dusunebilirsiniz &str -> array, String vector.
        String lerin uzunlugu compile time da bilinmeyebilir &str aksine.
    */

    // String yaratimi ornegi
    // let my_string = String::from("bu bir string ve bunun uzunlugu degisken olabilir.");

    // &str ve String mutability karsilastirmasi
    // let mut string_literal = "string literal";
    // string_literal = "ilk deger";
    // println!("{}", string_literal);
    // Burada aslinda biz depolanan degeri degistirmiyoruz onun yerine bizim o variable imiz yeni bir lokasyondaki literali isaret ediyor.
    // Boylelikle aslinda yeni bir deger verildiginde olan deger degistirilmis olmuyor yeni bir deger yaratilmis oluyor.
    // let mut s = String::from("hello"); // String tanimladik

    // s.push_str(", world!"); // push_str() metodu elimizdeki hello degerine , world! kismini ekliyor yeni bir deger yaratmak yerine
    // olani guncelliyor.

    // println!("{}", s); // Bu fonksiyon hello, world! yazdiracak.

    // &str compile time da bilindigi icin hizli ve efficient bu da immutable olmasindan geliyor.
    // String de ise biz run time da memory istiyoruz bilgisayardan
    // let s = String::from("Burada from kullanrak memory istemis olduk aslinda");

    // Ayni zamanda isimiz bittiginde aldigimiz yeri geri vermemiz gerekiyor ama neyse ki rust bu kismi bizim icin yapiyor.
    // {
    //     let s = String::from("String yaratildi");
    // } // Buradan sonra scope dan cikacagi icin bu hafiza otomatik olarak deallocate edildi (bosaltildi)
    // Biz aslinda gormuyoruz ama burda } isaretini gorunce rust ozel drop adinda bir fonksiyon cagiriyor ve bize hafiza aciyor tekrardan

    // Data move
    // let x = 5;
    // let y = 3;
    // Yukaridaki kod bekledigimiz gibi calisiyor.
    // Once x icin yer acip ona 5 degerini atiyor sonra da y icin yer acip y ye x in degerini atiyor/
    // Suan bizim burada iki degerimiz var; x ve y.
    // Bunlarin bu sekilde calismasinin sebebi integer in rust icin basit bir tur olmasi ve bu degerlerin stack te depolanmasi.
    // Stack teki degerlere erisim kolay ve ucuz oldugu icin bu islem herhangi bir sorun yaratmiyor rust icin.

    // Simdi yukaridaki ornegin String versiyonuna bakalim
    // let birinci = String::from("Nevizade geceleri");
    // let ikinci = birinci;
    // Bu yukaridaki kod da ayni sekilde gozukse de aslinda ustteki kodla ayni isi yapmiyor.
    // Bunu daha iyi anlamak icin perdenin arkasinda ne oldugunu anlamak onemli.
    // Burda bizim birinci string degerimiz uc kisimdan olusuyor; ptr, len, capacity.
    // Burda ptr heap deki lokasyonu gosteriyor.
    // Len stringin suanki uzunlugunu, kapasite de olacabilecek maksimum kapasiteyi gosteriyor.
    // Biz burada bu uc degeri kopyaliyoruz ikinci degere; ptr, len ve capacity yi.
    // O ptr in gosterdigi lokasyondaki degeri degil.
    // Bu yuzden bu iki deger aslinda heap te ayni yeri gosteriyor oluyor ve ayni degere sahip 2 variable olmak yerine
    // ayni variable a erisimin iki farkli yolu oluyorlar.
    // Mesela bilgisayarda bir uygulamaya hem masaustunden ulasabilirsiniz hem de uygulamalar altindaki logodan.
    // Burada farkli yollar kullanilsa da aslinda ayni uygulamaya erisiyoruz birbirinin kopyasi iki uygulamaya degil.
    // Rust heap teki datanin degisebilecegi biliyor. Ve boyutunu bilemedigi icin zaten zahmetli ve masrafli olan heap koyasini
    // riske edip icerigi ile kopyalamak istemiyor. Bu sayede daha az hafiza tuketerek daha hizli ve guvenli bir sekilde calisabiliyor.

    // Hatirliyorsaniz biz daha once bir deger scope tan cikinca artik bu degerin gecerli olmadigini gormustuk.
    // Buradaki sorun su ki, burada bizim birinci ve ikinci scope tan cikinca rust iki kere bosaltmayi deneyecek.
    // Rust da bunu engellemek icin let ikinci = birinci; dedigimizde artik birinci degeri kullanilabilir olarak birakmiyor.
    // Yani bu atamayi yaptiktan sonra biz birinci degerinin tekrar kullanmamiyoruz, siliniyor.
    // Simdi tekrardan yazalim ornegi
    // let s1 = String::from("some value");
    // let s2 = s1;
    // println!("{}", s1);

    // Diger dillerde duyduysaniz bu shallow copy konseptine benziyor. Rust da bu degere moved diyoruz. Sanki s1 i s2 ye kaydirmisiz gibi.
    // Rust bu hareketi ile 2 kere ayni yeri bosaltma sorununu cozuyor.
    // Ayni zamanda bu bilerek alinmis bir karar ve boylelikle kodun hizli ve az hafizayla calismasi garanti altina aliniyor.

    // Clone
    // Dedigimiz gibi rust bizim icin direk bu ucuza mal olan kopyayi yaratiyor ama biz daha derin (deep) copy istersek bunu clone ile yapiyoruz
    // let s1 = String::from("S1");
    // let _s2 = s1.clone();
    // println!("s1 degerini hala kullanabiliyoruz {}", s1);

    // Ne zaman clone() fonksiyonunun cagrildigini gorurseniz bilin ki burada pahali bir islem yapiliyor ve ayni zamanda farkli bir sey oldugnu
    // gormek icin de iyi bir indikator

    //Stack-Only Data: Copy
    // let x = 5;
    // let y = x;

    // println!("x = {}, y = {}", x, y);
    // Yukaridaki kod da calisiyor ama bizim konustuklarimizlacelisiyor.
    // Bunun sebebi yukaridaki degerlerin integer olmasi ve stack te depolanmasi.
    // Stack teki islemler ucuz oldugu icin heap teki gibi bir yola gitmemize gerek yok.
    // Bu tarz degerler copy trait ini implement ettigi icin bu sekilde kopyalanabiliyor.
    // Bu nedir daha sonra ogrenecegiz ama bir bilgi olarak bulunsun simdilik.
    // Drop fonksiyonunu kullanmayan ve copy trait i implement eden type lar bu yolla kopyalanabiliyor
    // Integerlar, boolean, float, char, tuple lar ama sadece belirli turleri iceriyorlarsa (i8, i8) olur ama (i8, String) olmaz

    // Ownership -> functions
    // let s = String::from("merhaba");

    // takes_ownership(s); // s in ownership i su anda fonksiyona ait
    // ... burdan sonra s i kullanamayiz

    // println!("{}", s);

    // let x = 5;
    // makes_copy(x); // x in degeri fonksiyona tasindi
    // x burda i32 oldugu icin stack te depolanan copy trait ini kullanan ve drop fonksiyonu implement edilmemis bir deger, sorun yasamiyoruz
    // x sonra kullanilabilir
    // println!("x hala gecerli! x -> {}", x);

    // fn takes_ownership(some_string: String) {
    //     // some_string degeri scope a geldi
    //     // println!("{}", some_string);
    // } // Burda some_string degeri scope tan ciktigi icin drop fonksiyonunu cagirdi rust ve o yeri bosaltti

    // fn makes_copy(some_integer: i32) {
    //     // some_integer deger scope a geldi
    //     println!("{}", some_integer);
    // } // Burda da some_integer degeri scope tan cikiyor ama ozel bir durum olmuyor turu i32 oldugu icin

    // Buradaki cozumlerden biri aldigimiz degerleri dondurebiliriz boylelikle scope tan cikinca silinmezler cunku o degeri return ediyor oluruz
    // Ama tabi ki her fonksiyon icin bunu yapmak hic efficient degil ve rust bunun icin bir yontem gelistirmis

    // References and Ownership
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);
    // println!("The length of '{}' is {}.", s1, len);

    // fn calculate_length(s: &String) -> usize {
    //     s.len()
    // }

    // Mutable References
    // let mut s = String::from("hello");
    // change(&mut s);
    // println!("{}", s);
    // fn change(some_string: &mut String) {
    //     some_string.push_str(", world");
    // }

    // Eger bizim bir degere mutable bir referansimiz varsa, o degere baska bir referans olamaz
    // Sebebi race condition engellemek
    // Bu mantikla asagidaki kod hata verecektir
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // Farkli bir scope da tanimli olduklari surece birden cok mutable referans olabilir cunku scope u bitmis oluyor diger referansin
    // let mut s = String::from("hello");

    // {
    //     let r1 = &mut s;
    // } // r1 burda scope tan ciktigi icin yeni mutable referans olusturabildik
    // let r2 = &mut s;

    // Bir objeden hem mutable hem immutable referans veremeyiz
    // Bu mantikli cunku immutable olan bir referans, referans oldugu degerin bir anda degismesini beklemez
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

    // Asagidaki kodsa calisacaktir yukaridan farkli olarak cunku yaratilan r1 ve r2 tekrar kullanilmiyor
    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // Dangling References
    // fn dangle() -> &String {
    //     let s = String::from("hello");

    //     return &s;
    // }
    // Burda s scope tan cikti ve drop ile bosaltildi ama referansi hala dangling
    // Cozum olarak &s yerine s dondurebiliriz

    /*
    Herhangi bir zamanda, bir değiştirilebilir referansınız veya herhangi bir sayıda değişmez referansınız olabilir.
    Referanslar her zaman geçerli olmalıdır.
    */

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
    // let slice = &a[1..3];
}
// *************************** Functions ***************************
// Zorunlu olmasa da fonksiyonlarda genellikle snake case kullaniliyor.
// Fonksiyonlar fn kelimesi ile tanimlaniyor.
// Burda fonksiyonu mainin disinda tanimladik. Once ya da sonra tanimlamamiz fark etmez.
#[allow(dead_code)] //Bu satir asagidaki fonksiyon kullanilmadiginda compiler in warning vermesini onluyor. Kullanimi zorunlu degil.
fn no_parameter_no_return() {
    println!("Bu fonksiyon parametre almiyor ve deger dondurmuyor");
}

// Rust ta fonksiyona verilen parametreler argument olarak da adlandiriliyor, bahsi bu sekilde de gecebilir.
#[allow(dead_code)]
fn take_parameter_no_return(lucky_number: u128) {
    println!("Your lucky number is {}", lucky_number);
}

// Return kelimesi kullanilarak da deger dondurulebilir ya da sonuna ; konmadan deger return kelimesi kullanillmadan da yazilanilir asagidaki ornekte oldugu gibi
// Return degeri olmayan fonksiyonlar statement olarak adlandirilirken deger donduren fonksiyonlar expression olarak adlandiriliyor.
#[allow(dead_code)]
fn take_parameter_return_value(cost: u128, tip: u64) -> u128 {
    let total_cost = cost + tip as u128;
    total_cost
}
