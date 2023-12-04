pub mod first_module {
    use std::fmt::Display;

    pub fn print_pro<T: Display>(t:T){
        println!("print_pro() calisiyor");
        println!("{}",t);
    }

    struct deneme {
        value: i32,
    }
}