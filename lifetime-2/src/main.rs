fn main() {
    println!("Hello, world!");

    //test0();
    test1();
    test2();
    test3();
    test4();
}

/*
fn test0() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r = {}", r);
}*/

fn test1() {
    let s1 = String::from("abcd");

    {
        let s2 = String::from("xyz");
        let result = longest(&s1[..], &s2[..]);

        println!("result is {}", result);
    }
}

fn test2() {
    let s1 = String::from("abcd");
    let result = 5;

    {
        let s2 = String::from("xyz");
    //    result = longest(&s1[..], &s2[..]);
                                //^^ borrowed value does not live long enough 
    }

//    println!("result is {}", result);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn test3() {
    let novel = String::from("call me something. other shit ...");
    let first_sentence = novel.split(".")
        .next()
        .expect("could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence };
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention please: {}", announcement);
        self.part
    }
}

fn test4() {
    let s: &'static str = "i have a static lifetime";
    println!("static lifetime str {}", s);
}
