fn main() {
    println!("Hello, world!");

    test0();
    test1();
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
