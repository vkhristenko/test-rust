use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let vv = vec![1,2,3,4,5];

    println!("{:?}", v);
    println!("{:?}", vv);

    {
        let vvv = vec![1,2,3,4,5];
        // vvv goes out of scope -> gets destroyed
    }

    {
        let v = vec![1,2,3,4,5];
        let third: &i32 = &v[2];
        let third: i32 = v[2];
        let third: Option<&i32> = v.get(2);
    }

    {
        let mut s = String::new();

        let data = "initial contents";
        let s = data.to_string();
        let s = "initial contents".to_string();
    }

    {
        let mut s = String::from("foo");
        s.push_str("gsagf");
        println!("str = {}", s);
        let s2 = String::from("some");
        s.push_str(&s2);
        println!("str = {}", s);
    }

    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world");
        let s3 = s1 + &s2;
        println!("{}", s3);
    }

    {
        let s1 = String::from("Hello");
        let s2 = String::from("Здравствуйте");
        println!("{} {}", s1.len(), s2.len());
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("blue"), 10);
        scores.insert(String::from("yellow"), 50);

        let teams  = vec![String::from("Blue"), String::from("Yellow")];
        let initial_scores = vec![10, 50];

        let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
        println!("{:?}", scores);

        let name = String::from("Blue");
        match scores.get(&name) {
            Some(score) => println!("score = {}", score),
            None => println!("nothing to print"),
        }

        for (key, value) in &scores {
            println!("{}: {}", key, value);
        }
    }

    {
        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
    }

    {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
    }
}
