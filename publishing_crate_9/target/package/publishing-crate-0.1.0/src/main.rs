use publishing_crate::PrimaryColor;
use publishing_crate::mix;
//lib.rs dosyasinda yukaridakileri pub use ile mod disinda export ettigimiz icin top level da kullanabildik.

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
    println!("When we mix red and yellow, we get: {:?}", orange);
}
