mod summary;
mod pair;

use summary::Summarizable;
use summary::Tweet;
use pair::Pair;

use std::fmt::Display;

fn main() {
    println!("Hello, world!");

    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for &number in number_list.iter() {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);

    {
        let rresult = largest_i32(&number_list);
        println!("the largest number is {}", rresult);

        let number_list = vec![102, 34, 6000, 89, 54, 2,43, 8];
        let rresult = largest_i32(&number_list);
        println!("the largest number is {}", rresult);
    }

    {
        let integer = Point{ x: 5, y:10};
        let float = Point {x: 1.0, y: 4.0};

        let p = Point { x: 10, y: 20};
        println!("p.x = {}", p.x());
    }

    {
        let tweet = summary::Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false
        };
        
        println!("1 new tweet: {}", tweet.summary());
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

fn largest_i32(list: &[i32]) -> i32 {
    let mut max = list[0];

    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }

    max
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }

    fn test<U>(&self, other: Point<U>) -> Point<U> {
        Point { x: other.x, y: other.y }
    }
}
