fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
}

fn test0() {
    let mut s = String::new();

    let data = "some string slice";
    let s = data.to_string();
    let s = "some othter string slice".to_string();
    let s = String::from("another string slice");

    let mut ss = String::new();
    ss.push_str("foo");
    let s2 = "bar";
    ss.push_str(s2);
    println!("ss is {}", ss);
    ss.push('x');
    println!("ss is {}", ss);
}

fn test1() {
    let s1 = String::from("hello ");
    let s2 = String::from("world");
    let s3 = s1 + &s2; // s1 has been moved
    //let s3 = &s1 + &s2; // s1 has been moved
    println!("s3 is {}", s3);
}

fn test2() {
    let len = String::from("Hola").len();
    println!("len = {}", len);
    let len = String::from("Здравствуйте").len();
    println!("len = {}", len);
}

fn test3() {
    let s = String::from("Здравствуйте");
    let sli = &s[0..4];
//    let sli2 = &s[0..1];
}

fn test4() {
    for c in "Здравствуйте".chars() {
        println!("char is {}", c);
    }

    for b in "Здравствуйте".bytes() {
        println!("byte is {}", b);
    }
}
