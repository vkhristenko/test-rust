fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
}

fn test4() {
    let a = [1,2,3,4,5,6];
    let slice = &a[1..3];
}

fn test3() {
    let my_string = String::from("hello world");

    let word = first_word_slices_slices(&my_string[..]);

    let my_string_literal = "hello world";
    let word = first_word_slices_slices(&my_string_literal[..]);

    let word = first_word_slices_slices(my_string_literal);
}

fn test2() {
    let mut x = String::from("hello");
    let test = &x;
}

fn test1() {
    let mut s = String::from("hello world");
    let word = first_word_slices(&s);
    let test = &s[..];
//    s.clear(); // error!
    //^^^^^^^^^ mutable borrow occurs here    
    println!("the first word is: {}", word);
                                    //---- immutable borrow later used here 
}

fn test0() {
    let s = String::from("hello world");

    let hello = &s[0..=4];
    let world = &s[6..=10];
}

fn first_word_slices(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slices_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
