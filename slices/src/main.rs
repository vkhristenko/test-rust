fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");

    let word = first_word(&s);

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{} {}", hello, world);

    let slice = &s[0..2];
    let slice = &s[..2];

    let slice = &s[0..];
    let slice = &s[..];

    let a = [1,2,4,5,6];
    let sss = &a[1..3];
    println!("{:?}", sss);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
