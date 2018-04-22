fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anothermeail@example.com");

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotehrusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotehrusername567"),
        ..user2
    };

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!(
        "The area of a new rectangle is {}",
        area_rect(&rect)
    );

    println!("rect is {:#?}", rect);

    println!(
        "The area of the other rect is {}",
        rect.area()
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height : u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn area_rect(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}
