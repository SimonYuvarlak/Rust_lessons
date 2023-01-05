use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    // Threads -----
    // Thread ler programlarin ayni anda bigisayarin farkli islemci core larini kullanabilmesine yariyor.
    // Boylelikle kodunuz tek tek is yapmak yerine, elindeki bircok isi paralel bir sekilde yapabiliyor.
    // Thread kullanimi Race conditions ve deadlock gibi sorunlar yaratabilir.

    // Thread ler thread::spawn() ile yaratiliyor ve spawn icine bir closure aliyor.
    // O closure in icindeki kod yeni bir thread de calismis oluyor.
    // thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("thread number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     println!("main number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Isletim sisteminiz core allocation ina gore her seferinde verdigi sonuc farkli olacaktir yukaridaki ornegin.

    // Yukaridaki ornekte bizim thread imizin isi yarida kesildi cunku main once bitti.
    // Main isini bitirince butun calisan thread ler de durduruluyor.
    // Bununla ilgili cozum olarak asagidaki yol izlenebilir.
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         // println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // for i in 1..5 {
    //     // println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // handle.join().unwrap(); // Bu satir bizim thread imizi bitiriyor.
    // Eger biz yukaridaki satirin yerini main deki for dan once olacak hale getirirsek once thread butun isini bitirir sonra main calisir.
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap();

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // Move Closure ve Thread
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();
    // Yukaridaki kod hata verecektir cunku v closure a referans olarak veriliyor ama thread kendi is yaptigi surece
    // o aldigi referansin valid olup olmadigini bilmedigi surece islem yapmak istemez.
    // Bu durum icin biz move kelimesi ile v nin ownership ini verebiliriz thread e.
    // let v = vec![1, 2, 3];

    // let handle = thread::spawn(move || {
    //     println!("Here's a vector: {:?}", v);
    // });

    // handle.join().unwrap();

    // Threadler ve Mesajlar -----
    // Kanal acma
    // let (tx, rx) = mpsc::channel();
    // Tx mesaji yollayan kisim, rx ise alan kisim.
    // channel() fojnksiyonundan donen degerleri tuple seklinde yakaladik.

    // -----

    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    // });
    // Yukarida kanal acip, thread imizin icinde kanalimizdan val degerini gonderdik.
    // Bunun icin send metodunu kullandik.
    // Send metodu bir Result donduruyor.

    // Gonderdigimiz degerleri asagidaki sekilde receive edebiliriz.
    // Asagidaki receiver thread, main thread.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap(); // Nehire biraktin
    // });

    // let received = rx.recv().unwrap(); // Nehirde aldin
    // println!("Got: {}", received);

    // Asagidaki kod hata verecek.
    // Cunku biz thread imizin icindeki val degerini kanala yolladiktan sonra yazdirmaya calisiyoruz.
    // Rust buna izin vermiyor. Cunku val degeri yollandigi kanalda droplanabilir veya bir sorun yasayabiliriz.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let val = String::from("hi");
    //     tx.send(val).unwrap();
    //     println!("val is {}", val);
    // });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);
    

    // Asagidaki ornekte thread degerleri tek tek send ediyor.
    // Main thread te biz rx i iterator gibi kullandik.
    // Main degerler geldikce yazdirdi.
    // let (tx, rx) = mpsc::channel();

    // thread::spawn(move || {
    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("thread"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // for received in rx {
    //     println!("Got: {}", received);
    // }

    // // Ayni anda bircok send yapmak isteseydik bircok thread den:
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
