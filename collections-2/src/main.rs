fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
    test5();
}

fn test0() {
    let v: Vec<i32> = Vec::new();
    let v1 = vec![1,2,3];
}

fn test1() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
}

fn test2() {
    let v = vec![1,2,3,4,5,6];

    let third: &i32 = &v[2];
    println!("the third element is {}", third);

    match v.get(2) {
        Some(third) => println!("the third element is {}", third),
        None => println!("there is no third element"),
    }
}

fn test3() {
    let v = vec![1,2,3,4,5];
    //let does_not_exist = &v[100];
    //let does_not_exist = v.get(100);
}

fn test4() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
        }
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn test5() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
