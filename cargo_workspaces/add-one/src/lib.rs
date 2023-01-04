use rand;

pub fn add_one(x: i32) -> i32 {
    x + 1
}

//Testleri root folder da calistirmak asagidaki olan butun testleri de calistirir.
//Eger tek biri ile yapmak isterseniz cargo test -p <package-name> seklinde yapilabilir.
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one(2));
    }
}
