use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    test0();
    test1();
    test2();
    test3();
    test4();
}

fn test0() {
    let expensive_closure = |num: &mut u64| {
        println!("calculating again...");
        thread::sleep(Duration::from_secs(*num));
        *num += 10;
    };
    let mut x: u64 = 1;
    expensive_closure(&mut x);
    println!("x = {}", x);

    fn test_func() { println!("func inside of a function"); }
    test_func();
}

struct Cacher<T>
    where T: Fn(u32) -> u32 
{
    func: T,
    value: Option<u32>,
}


fn test1() {
    iterator_demonstration();
}

fn iterator_demonstration() {
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();
    
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    v1.iter().map(|x| x+1);
    println!("vector is {:#?}", v1);

    let v3: Vec<_> = v1.iter().map(|x| x+1).collect();
    println!("vector3 is {:#?}", v3);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| s.size == shoe_size)
        .collect()
}

fn test2() {
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);

    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    println!("in_my_size vector is {:#?}", in_my_size);
}

struct Counter {
    count: u32
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

fn test3() {
    let mut counter = Counter::new();

    counter.next();
    counter.next();
    counter.next();
}

fn test4() {
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                                 .map(|(a,b)| a*b)
                                 .filter(|x| x%3 == 0)
                                 .sum();
    println!("sum = {}", sum);
 84 
 85     println!("in_my_size vector is {:#?}", in_my_size);
 86 }
 87 
 88 struct Counter {
 89     count: u32
 90 }
 91 
 92 impl Counter {
 93     fn new() -> Counter {
 94         Counter { count: 0 }
 95     }
 96 }
 97 
 98 impl Iterator for Counter {
 99     type Item = u32;
100 
101     fn next(&mut self) -> Option<Self::Item> {
102         self.count += 1;
103 
104         if self.count < 6 {
105             Some(self.count)
}
