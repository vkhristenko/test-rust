fn main() {
    println!("Hello, world!");

    {
        let s = "hello";
    
        println!("{}", s);
    }

    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let x = 5;
    let y = x;

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);

    let ss = String::from("hello");

    takes_ownership(ss);

    let xx = 5;
    makes_copy(xx);

    let ss1 = gives_ownership();
    let ss2 = String::from("hello");
    let ss3 = takes_and_gives_back(ss2);

    let (sss, len) = calculate_length(String::from("something"));
    println!("sss = {} len = {}", sss, len);

    let mut sss1 = String::from("hello");
    let len = calculate_length_ref(&sss1);

    change(&mut sss1);
    println!("sss1 = {}", sss1);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", something else");
}

/*
fn dangle() -> &String {
    let s = String::from("hello");

    &s
}*/

fn no_dangle() -> String {
        let s = String::from("hello");

            s
}
