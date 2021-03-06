use std::fmt::{Display, Debug};

fn main() {
    println!("Hello, world!");

    test0();
    test1();
    test2();
    test3();
    test4();
}


fn test0() {
    let list = vec![1,2,3,4,5,6];
    let result = largest(&list[..]);
    println!("largest is {}", result);
}

fn largest<T: std::cmp::PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

enum option<T> {
    some(T),
    none,
}

fn test1() {
    let p1 = Point {
        x: 5,
        y: 10
    };
    let p2 = Point { x: 10.0, y: 15.0 };
    println!("x = {}", p2.x());
    println!("y = {}", p2.y());
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.y
    }
}

struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn mixup<V, W>(self, other: Pair<V, W>) -> Pair<T, W> {
        Pair {
            first: self.first,
            second: other.second,
        }
    }
}

fn test2() {
    let p1 = Pair {
        first: 15,
        second: 10.0,
    };
    let p2 = Pair {
        first: String::from("hello"),
        second: 'a',
    };
    let p3 = p1.mixup(p2);

    println!("p3.first is {} and p3.second is {}", p3.first, p3.second);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String, 
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}", self.username, self.content)
    }
}

fn test3() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("some text"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
    notify(&tweet);
    notify_trait_bounds(&tweet);
}

impl<T> Summary for Vec<T> {
    fn summarize(&self) -> String {
        format!("size is {}", self.len())
    }
}

fn test4() {
    let v = vec![1,2,3,4,5,6];
    println!("summar: {}", v.summarize());
}

pub fn notify(item: & impl Summary) {
    println!("breaking news: {}", item.summarize());
}

pub trait SomeTrait {
    fn some_func() {}
}

pub fn notify_trait_bounds<T: Summary + SomeTrait>(item: &T) {
    println!("breaking news: {}", item.summarize());
}

impl SomeTrait for Tweet {}

fn some_function_1<T: Display + Clone, U: Clone + Debug>(t: T, u: U) -> i32 {
    5
}

fn some_function_2<T, U>(t: T, u: U) -> i32 
    where T: Display + Clone,
          U: Clone + Debug
{
    5
}

struct MyPair<T> {
    first: T,
    second: T,
}

impl<T> MyPair<T> {
    fn new(first: T, second: T) -> MyPair<T> {
        MyPair {
            first,
            second,
        }
    }
}

impl<T: Display + PartialOrd> MyPair<T> {
    fn cmd_display(&self) {
        if self.first > self.second {
            println!("the largest is {}", self.first);
        } else {
            println!("the largest is {}", self.second);
        }
    }
}
