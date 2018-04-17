fn main() {
    println!("Hello, world!");

    let number = 3;

    if number < 5 {
        println!("true");
    }
    else {
        println!("false");
    }
    let condition = true;
    let other = if condition {
        5
    } else {
        6
    };

    let mut number = 10;
    while number!=0 {
        println!("not zero");
        number = number - 1;
    }

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element = {}", element);
    }
}
