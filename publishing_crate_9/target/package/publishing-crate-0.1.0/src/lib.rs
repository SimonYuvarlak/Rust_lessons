//Iki farkli cargo build versiyonu var. Biri dev biri release.
//Dev development icin daha iyi konfigusrasyonlarla gelirken release end product compilation i icin daha iyi konfigurasyonlarla geliyor
//$ cargo build --release
//Bu iki farkli build versiyonunu yukaridaki sekilde kullanabiliriz (default/dev -> $ cargo build)
//Cargo bu iki versiyon icin default konfigurasyonlara sahip olsa da biz bunlari degistirebiliriz.

//Nasil biz crate.io dan kod kullanabiliyorsak, oraya kod da yukleyebiliriz.
//Bunun icin kodumuzun oncelikle public consumption a hazir olmasi lazim.
//Documentation commentler bunun icin kullaniliyor.
// Yerine /// ile yapiliyor.


//Crate ile ilgili aciklama yapmak icin asagidaki sekilde bir kullanim yapabiliriz.
//Bu tarz dokumentasyon crate e ayri yer aciyor olusan html de
//! # publishing_crate
//! `publishing_crate` is a module explaning how to publish your own crate on crate.io

/// Adds one to the given number
/// 
/// # Examples
/// 
/// ```
/// let num = 5;
/// let res = publishing_crate::add_one(num);
/// 
/// assert_eq!(6, res);
/// ```
pub fn add_one(num: i32) -> i32 {
    num + 1
}

//Dokumentasyonlari asagidaki commentle html formunda yaratabiliriz. Eger direk acmak istersek sonuna --open ekleyebiliriz
//Yukardaki Examples section i gibi bircok section var docs tan bakilabilecek.

//Yazdiginiz example lari test edebilirsiniz.
//$ cargo test

pub use self::kinds::PrimaryColor;
pub use self::kinds::SecondaryColor;
pub use self::utils::mix;

pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,

    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        //Mix primary colors and get the secondary color
        SecondaryColor::Orange
    }
}

//Api token : cio444SwPnOsxv9SKygnD9IJFgKuY5YDMrb