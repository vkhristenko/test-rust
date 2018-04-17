fn main() {
    println!("Hello, world!");

    another_function(5);

    another_function_1(10, 20);
}

fn another_function(x: i32) {
    println!("another function print");
}

fn another_function_1(x: i32, y:i32) {
    println!("x = {}, y = {}", x, y);
}

fn five() -> i32 {
    let x = 5;
    x
}
