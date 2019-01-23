use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}

fn test0() {
    let mut map = HashMap::new();

    map.insert(String::from("Blue"), 10);
    map.insert(String::from("Red"), 50);

    println!("map is {:#?}", map);
}

fn test1() {
    let teams = vec![String::from("blue"), String::from("red")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("map is {:#?}", scores);
}

fn test2() {
    let field_name = String::from("some color");
    let field_value = String::from("some value");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    println!("map is {:#?}", map);

//    println!("field_name is {}", field_name);
//    we moved "field_name" upon inserting into the hash map
}

fn test3() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name);
    match score {
        Some(num) => println!("score is {}", num),
        None => println!("no key found"),
    }

    for (key, value) in &scores {
        println!("key = {} value = {}", key, value);
    }
}

fn test4() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn test5() {
    let mut map = HashMap::new();

    map.insert(String::from("blue"), 20);
    map.entry(String::from("yellow")).or_insert(50);
    map.entry(String::from("blue")).or_insert(100);
    println!("map is {:#?}", map);
}

fn test6() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("map is {:#?}", map);
}
