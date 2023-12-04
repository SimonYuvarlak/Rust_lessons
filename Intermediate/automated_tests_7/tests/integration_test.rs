//Eger package imiz binary ise calismayacaktir integration test, sadece lib crate icin var
//Ayni zamanda binary crate te asagidaki gibi use kullanimi yapamayiz
use automated_tests_7;
//Yukardaki gibi crate imizi scope a getirmemiz lazim cunku burdaki her test dosyasi kendisi crate gibi calisiyor
mod common;

#[test]
fn integration_test_function() {
    let _x = automated_tests_7::add(1, 2);
    common::setup();
    println!("Integration test calisiyor");
}
