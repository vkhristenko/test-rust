use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    {
        let x = 5;
        let r = &x;

        println!("r: {}", r);

        {
            let x = 5;
 //           r = &x;
        }

//        println!("r: {}", r);
    }

    let string0 = "qqqq";
    {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("the longest string is {}", result);
    }

    {
        let novel = String::from("could not find a '.'");
        let first_sentence = novel.split('.')
            .next()
            .expect("could not find a '.'");
        let i = ImportantExcerpt { part: first_sentence };
    }
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

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("attention pelse: {}", announcement);
        self.part
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str 
where T: Display {
    println!("announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
