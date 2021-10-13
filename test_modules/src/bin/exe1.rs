use test_modules::test1;
use test_modules::{mod1, mod2};

fn main() {
    println!("hello from exe1");
    test1();
    mod1::test1();
    mod2::test1();
    mod2::test2();
    println!("---");
    mod2::mod1::test1();
}
