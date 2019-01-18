fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
}

fn test1() {
    let user1 = User {
        email: String::from("some"),
        username: String::from("other"),
        active: true,
        sign_in_count: 5,
    };

    let user2 = User {
        email: String::from("test"),
        username: String::from("other"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        ..user2
    };
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32, 
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn make_square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn test2() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let sq = Rectangle::make_square(10);

    println!("rect1 is {:#?}", rect1);
    println!("area of rect is {}", rect1.area());
    println!("area of rect is {}", (&rect1).area());
}

fn test3() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test0() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user2.email = String::from("anotheremail@example.com");

    let email = String::from("some_string");
    let username = String::from("some_other_string");
    let user3 = User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    };
}
