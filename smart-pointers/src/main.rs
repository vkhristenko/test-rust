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
}

fn test0() {
    let b = Box::new(5);
    println!("b = {}", b);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn test1() {
    let list = List::Cons(1, 
        Box::new(List::Cons(2, 
            Box::new(List::Cons(3,
                Box::new(List::Nil))))));

    println!("list is {:?}", list);
}

fn test2() {
    let x = 5;
    let y = &x;

    println!("x = {}", x);
    println!("y = {}", *y);
}

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn test3() {
    let x = 5;
    let y = MyBox::new(5);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *(y.deref()));

    let lmbda = |name: &str| { println!("hello, {}!", name); };
    let m = MyBox::new(String::from("rust"));
    lmbda(&m);
    lmbda(&(*m)[..])
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("dropping CustomSmartPointer with data `{}`", self.data);
    }
}

fn test4() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created");
}

fn test5() {
    let x = CustomSmartPointer { data: String::from("here we go") };
    std::mem::drop(x);
//    x.drop();
}

enum ConsList {
    Cons(i32, Rc<ConsList>),
    Nil,
}

use std::rc::Rc;

fn test6() {
    let a = Rc::new(ConsList::Cons(5,
        Rc::new(ConsList::Cons(10,
            Rc::new(ConsList::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ConsList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ConsList::Cons(4, Rc::clone(&a));
        println!("coutn after creating c = {}", Rc::strong_count(&a));
    }
    println!("coutn after c goes out of scope = {}", Rc::strong_count(&a));
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T> 
    where T: Messenger {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("error: you are above your quota");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("urgent warning: you've used more than 90% of your quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("warning: you've used more than 75% of your quota");
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));

            let some = RefCell::new(vec![1,2,3,4]);
            let mut one_borrow = some.borrow_mut();
            let mut two_borrow = some.borrow_mut();
        }
    }

    #[test]
    fn test_sends_warning() {
        let mock_messenger = MockMessenger::new();
        let mut tracker = LimitTracker::new(&mock_messenger, 100);

        tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

use std::cell::RefCell;

#[derive(Debug)]
enum ListConsNew {
    Cons(Rc<RefCell<i32>>, Rc<ListConsNew>),
    Nil
}

fn test7() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(ListConsNew::Cons(Rc::clone(&value), Rc::new(ListConsNew::Nil)));

    let b = ListConsNew::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = ListConsNew::Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
