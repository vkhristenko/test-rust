mod sound2;

mod sound {
    pub mod instrument {
        pub fn clarinet() {
            //
            super::breathe_in();
        }
    }

    fn breathe_in() {
    }
}

mod performance_group {
    pub use crate::sound::instrument;

    pub fn clarinet_trio() {
        instrument::clarinet();
        instrument::clarinet();
    }
}

mod plant {
    pub struct Vegetable {
        pub name: String,
        id: i32,
    }

    impl Vegetable {
        pub fn new(name: &str) -> Vegetable {
            Vegetable {
                name: String::from(name),
                id: 1,
            }
        }
    }
}

mod menu {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

fn test1() {
    let order1 = menu::Appetizer::Soup;
    let order2 = menu::Appetizer::Salad;
}

fn test0() {
    let mut v = plant::Vegetable::new("squash");

    v.name = String::from("butternut squash");
    println!("{} are delicious", v.name);

    // won't compile
    //println!("the id is {}", v.id);
}

use std::fmt::Result;
use std::io::Result as IoResult;
use std::{cmp::Ordering, io};

fn main() {
    println!("Hello, world!");

    // absolute path
    crate::sound::instrument::clarinet();

    // relative path
    sound::instrument::clarinet();

    performance_group::instrument::clarinet();

    sound2::instrument::clarinet();
}
