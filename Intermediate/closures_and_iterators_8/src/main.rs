use std::{thread, time::Duration};

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    //***** Closure ****
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );

    //En genel hali ile closure tanim ornegi
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //Fonksiyonlar ve closure tanimlamalari karsilastirmasi
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 };
    // let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x| x + 1  ;

    //Eger bir closure i type i olmadan yarattiysak parametresinde,
    //o zaman ilk kullanimindan parametre type ini alir ve bundan sonra onu bekler.
    //param type i belirtilmemis closure ilk kullanimina kadar hata verir.
    //Ilk kullanimindan sonra type ini bildigi icin artik hata vermeyecektir.
    // let my_closure = |num| num + 1;
    // let first = my_closure(1);
    // let second = my_closure("Arda");

    //Closure lar gereksinime gore otomatik olarak 3 farkli sekilde calisabilir.
    //1- borrows immutable
    //2- borrows mutable
    //3- takes ownership

    //Immutable olarak alma ornegi
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // println!("After calling closure: {:?}", list);

    //Mutable olarak alma ornegi
    // let mut list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);
    // let mut borrows_mutably = || list.push(7);
   
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    //Ownership i closure in almasi icin 'move' keyword u ile onu forcelayabiliriz
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // thread::spawn(move || println!("From thread: {:?}", list))
    //     .join()
    //     .unwrap();

    //***** Iterator ****
    //Iterator lazy dir, yani tek baslarina islevsizdir
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();

    //Kullanim ornegi
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }

    //next fonksiyonu iterator daki objeyi dondurup bir ileri goturuyor structure i
    //Bunu yaparak aslinda bu method iterator u consume ediyor
    // let v1 = vec![1, 2, 3];
    // let mut v1_iter = v1.iter();
    // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);

    //Yukarida next ten aldiklarimiz bizim immutable reference larimiz
    //For a verdigimizde de aslinda ownership i aliyor ve arka tarafta mutable reference olusturuyor
    //Biz iterator in ownership i alip value lari dondurmesini istiyorsak into_iter() kullanabiliriz
    //Mutable reference ile iter etmek istiyorsak iter_mut() kullanilabilir


    //Iterator adaptorlar iterator u consume etmeyen methodlar
    //Map metodu
    // let v1: Vec<i32> = vec![1, 2, 3];
    // v1.iter().map(|x| x + 1);
    //Yukarida map bir closure aliyor. 
    //Biz closure larin lazy oldugunu biliyoruz o yuzden bu kodun aslinda bir sey yapmadigina dair uyari aliriz


    //Yukariyi asagi ile guncelleyebiliriz
    //collect() i kullanarak degerleri alabiliriz
    // let v1: Vec<i32> = vec![1, 2, 3];
    // let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    // assert_eq!(v2, vec![2, 3, 4]);
    //Yukaridaki islem sonucunda v1 vectoru herhangi bir sekilde etkilenmiyor

    //Filter baska bir iterator consume etmeyen ornek
    //filter() i kullanarak iteratordan sadece istedigimiz kriterlere uyanlari alabiliriz
    // let shoes = vec![
    //         Shoe {
    //             size: 10,
    //             style: String::from("sneaker"),
    //         },
    //         Shoe {
    //             size: 13,
    //             style: String::from("sandal"),
    //         },
    //         Shoe {
    //             size: 10,
    //             style: String::from("boot"),
    //         },
    //     ];

    //     let in_my_size = shoes_in_size(shoes, 10);

    //     assert_eq!(
    //         in_my_size,
    //         vec![
    //             Shoe {
    //                 size: 10,
    //                 style: String::from("sneaker")
    //             },
    //             Shoe {
    //                 size: 10,
    //                 style: String::from("boot")
    //             },
    //         ]
    //     );

    //     println!("{:?}", in_my_size);

    //Not: iterator ve closure lar kullandigimizda herhangi bir performans kaybi yasamayiz.
}
