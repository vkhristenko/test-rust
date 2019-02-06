fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
}

fn test0() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("using your favorite color {}", color);
    } else if is_tuesday {
        println!("tuesday is a green day");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("using purple as the bkg color");
        } else {
            println!("using orange");
        }
    } else {
        println!("using blue");
    }
}

fn test1() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    while let Some(top) = stack.pop() {
        println!("top is {}", top);
    }
}

fn test2() {
    let v = vec!['a', 'b', 'c', 'd'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let (x, y, z) = (1,2,3);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("current location ({}, {})", x, y);
}

fn foo(x: i32) {

}
