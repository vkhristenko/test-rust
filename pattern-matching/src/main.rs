fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
    test7();
    test8();
    test9();
    test10();
    test11();
    test12();
    test13();
    test14();
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

fn test3() {
    let y: Option<_> = Some(5);
//    let Some(x) = y;

    if let Some(x) = y {
        println!("x = {}", x);
    }
}

fn foo(x: i32) {

}

fn test4() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        _ => println!("anything"),
    }
}

fn test5() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("got 50"),
        Some(y) => println!("matched, y = {:?}", y),
        _ => println!("default case, x = {:?}", x),
    }

    println!("at the end, {:?}, y = {:?}", x , y);
}

fn test6() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1 ... 5 => println!("one through five"),
        _ => println!("soemthing else"),
    }
}

fn test7() {
    let x = 'c';

    match x {
        'a' ... 'j' => println!("early ascii letters"),
        'k' ... 'z' => println!("late ascii letters"),
        _ => println!("something else"),
    }
}

struct Point {
    x: i32,
    y: i32,
}

fn test8() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}

fn test9() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    
    match p {
        Point { x, y: 0 } => println!("on the x axis at {}", x),
        Point { x: 0, y } => println!("on the y axis at {}", y),
        Point { x, y } => println!("on neither axis: ({}, {})", x, y),
    }
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn test10() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("the quit variant has no data to destructure");
        },
        Message::Move { x, y } => {
            println!(
                "move in the x direction {} and in the y direction {}",
                x,
                y);
        },
        Message::Write(text) => println!("text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("change the color to red {}, gree {}, blue {}",
                     r,
                     g,
                     b);
        }
    }
}

fn test11() {
    let points = vec![
        Point { x: 0, y: 0 },
        Point { x: 1, y: 5 },
        Point { x: 10, y: -3 },
    ];

    let sum_of_squares: i32 = points
        .iter()
        .map(|&Point {x, y}| x*x + y*y)
        .sum();
}

fn test12() {
    let num = Some(10);

    match num {
        Some(x) if x < 20 => println!("less than twenty {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }
}

fn test13() {
    enum Message {
        Hello { id: i32 },
    };

    let msg = Message::Hello{ id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3...7 } => {
            println!("found an id in range: {}", id_variable)
        },
        Message::Hello { id: 10...12 } => {
            println!("found an id in another range")
        },
        Message::Hello { id } => {
            println!("found some other id: {}", id)
        },
    }
}

fn test14() {
    let robot_name = &Some(String::from("Bors"));

    match robot_name {
        Some(name) => println!("found a name: {}", name),
        None => (),
    }

    match robot_name {
        &Some(ref name) => println!("found a name: {}", name),
        None => ()r
    }

    println!("robot_name is {:?}", robot_name);
}
