fn main() {
    println!("Hello, world!");

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let test: Option<i32> = None;
    let test: Option<i32> = Some(54);

    value_in_cents(Coin::Quarter(State::Iowa));
}

enum IpAddrKind {
    V4(String),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }
}

#[derive(Debug)]
enum State {
    Alabama,
    Iowa,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(State),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("state quarter {:?}", state);
            25
        },
    }
}
