fn main() {
    println!("Hello, world!");

    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("the lenght of '{}' is {}.", s1, len);

    test0();
}

fn test0() {
    let mut s1 = String::from("hello");

    change(&mut s1);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
    // ^^^^^^^^^^^ `some_string` is a `&` reference, so the data it refers to cannot be borrowed as mutable
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
