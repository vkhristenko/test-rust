fn main() {
    let mut _s = String::from("hello");

    _s.push_str(", world");

    println!("{}", _s);

    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
}

fn test1() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

#[warn(unused_variables)]
fn test0() {
    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1;

    // can not use s1 after the move
    //println!("{}", s1);
}

fn test2() {
    let s = String::from("hello");
    takes_ownership(s);
    // can not use s after the move just above
    //println!("{}", s);

    let x = 5;
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn test3() {
    let s1 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
