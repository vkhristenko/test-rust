use std::mem;
use crate::List::*;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello from main");
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
    test15();
    test16();
    test17();
    test18();
    test19();
    test20();
    test21();
    test22();
    test23();
    test24();
}

fn test0() {
    println!("hello from test0");

    let mut l1: bool = true;
    let l2: bool = false;
    let l3 = true;
    println!("{} {} {}", l1, l2, l3);

    l1 = false;
    println!("{} {} {}", l1, l2, l3);

    let i = 5i32;
    let f = 5f32;
    println!("{} {}", i, f);

    let ff = 0.000_000_1;
    let qq = 1_000_000;
    println!("{} {}", ff, qq);
}

fn test1() {
    println!("hello from test1");

    let t1 = (1, "hello", 4.5, true);
    println!("{:?}", t1);
    
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("too long tuple: {:?}", too_long_tuple);
}

fn test2() {
    let xs: [i32; 5] = [1,2,3,4,5];
    println!("{:?}", xs);

    let ys: [i32; 500] = [0; 500];

    // Arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    borrow_array_as_slice(&xs);
    borrow_array_as_slice(&xs[1..3]);
    borrow_array_as_slice(&xs[2..4]);
}

fn borrow_array_as_slice(xs: &[i32]) {
    println!("----");
    for i in 0..xs.len() {
        println!("xs[{}] = {}", i, xs[i]);
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u8
}

//struct Pair(i32, i32);

fn test3() {
    let p = Person {name: String::from("name"), age: 27};
    let name = String::from("hello");
    let age = 27;
    let p1 = Person {name, age};
    println!("{:?}", p);
    println!("{:?}", p1);

    let Person{name: n, age: a} = p1;

//    let pair = Pair(10, 10);
//    let Pair(first, second) = pair;
}

enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click {x: i32, y: i32}
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page load"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(c) => println!("key pressed {}", c),
        WebEvent::Paste(s) => println!("pasted string {}", s),
        WebEvent::Click{x, y} => println!("clicked at x={} y={}", x, y)
    }
}

fn test4() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
    
fn test5() {
    type Event = WebEvent;
    
    let pressed = Event::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = Event::Paste("my text".to_owned());
    let click   = Event::Click { x: 20, y: 80 };
    let load    = Event::PageLoad;
    let unload  = Event::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

enum List {
    Cons(u32, Box<List>),
    Nil
}

impl List {
    fn new() -> List { Nil }

    fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => format!("{}, {}", head, tail.stringify()),
            Nil => format!("{}", "Nil")
        }
    }
}

fn test6() {
    let mut list = List::new();

    for i in 1..10 { list = list.prepend(i); }

    list = list.prepend(11)
        .prepend(12)
        .prepend(13);

    println!("{}", list.stringify());
}

#[derive(Debug)]
struct Point {
    x: i32, y:i32
}

//
// TODO: how to add copy capabilities for a struct
//
impl Point {
    fn move_ownership(self) -> Self { self }

    fn doT(&self) { println!("point {} {}", self.x, self.y); }
}

fn point_modify_content(p: &mut Point) {
    p.x = 100;
}

fn take_ownership(p: Point) {
    p.doT();
}

fn test7() {
    let mut x = Point {x:10, y:20};
    x = Point {x:20, y:10};
    println!("{:?}", x);
    let mut y = x.move_ownership();
    point_modify_content(&mut y);
    println!("{:?}", y);
    y.doT();
    let xx = Point { x:20, y:100 };
    xx.doT();
    take_ownership(xx);
}

fn test8() {
    println!("hello from test8");
    let mut x = Point {x: 10, y:10};
    {
        let ref mut y = x;
        *y = Point {x:20, y:20};
    }
    println!("{:?}", x);
}

#[derive(Debug)]
struct SomeData {
    name: String,
    data: Vec<u8>
}

fn move_data(data: SomeData) -> SomeData { return data; }
fn share_data(data: &SomeData) -> () { return println!("{:?} {:p}", data, data); }
fn share_mut_data(data: &mut SomeData) -> () { return println!("{:?}", data); }

fn move_rebind_data(mut data: SomeData) -> SomeData {
    data = SomeData{name: String::from("some data"), data: vec![0xee; 500]};
    return data;
}

fn test9() {
    let data = SomeData{name: String::from("some data"), data: vec![0xff; 500]};
    println!("{:?}", data);
    let data = move_data(data);
    share_data(&data);
    let mut mut_data = SomeData{name: String::from("some data"), data: vec![0xff; 500]};
    share_mut_data(&mut mut_data);
    
    let mut_rebind_data = SomeData{name: String::from("some data"), data: vec![0xff; 500]};
    println!("{:?}", mut_rebind_data);
    let somedata = move_rebind_data(mut_rebind_data);
    println!("{:?}", somedata);
}

fn move_data_mut(mut data: SomeData) -> SomeData { 
    for i in 0..data.data.len() {
        data.data[i] += 1;
    }
    data = SomeData{name: String::from("move data mut"), data: data.data};
    return data; 
}

fn move_data_mut_1(mut data: SomeData) -> SomeData {
    data = SomeData{name: String::from("move data mut"), data: data.data};
    return data; 
}

fn test10() {
    println!("--- test10 ---");
    let data = SomeData{name: String::from("some data"), data: vec![0x10; 500]};
    println!("{:p}", &data);
    let data = move_data(data);
    println!("{:p}", &data);
    let data = move_data_mut(data);
    println!("{:p}", &data);
    let data = move_data_mut_1(data);
    println!("{:p}", &data);
}

impl SomeData {
    fn func_share(&self) -> () {
    }
    
    fn func_modify(&mut self) -> () {
    }
}

fn share(data: &SomeData) -> () { println!("{:p}", data); }
fn share_mod(data: &mut SomeData) -> () { println!("{:p}", data); }
fn share_mod1(data: &mut SomeData) -> () {
    println!("{:p}", data);
    *data = SomeData{name: String::from("some data"), data: vec![0x20; 500]};
    println!("{:p}", data);
}

fn share_mod2(data: &mut SomeData) -> () {
    println!("{:p}", data);
    *data = SomeData{name: String::from("some data"), data: vec![0x20; 500]};
    println!("{:p}", data);
}

fn test11() {
    println!("--- test11 ---");
    let mut data = SomeData{name: String::from("some data"), data: vec![0x10; 500]};
    share(&data);
    println!("{:p}", &data);
    share_mod(&mut data);
    println!("{:p}", &data);
    share_mod1(&mut data);
    println!("{:p}", &data);
}

fn test12() {
    let mut v = vec![1,2,3,4,5,6,7,8,9,10];
    for val in v.split(|&value| value == 5) {
        println!("{:?}", val);
    }
}

trait MyTrait {
    fn do_something(&self) -> i32;
}

#[derive(Debug, Clone)]
struct TestData {
    data: Vec<i32>
}

fn test13() {
    println!("--- test13 ---");
    let data = TestData { data: vec![1,2,3,4,5] };
    let data1 = TestData { data: vec![1,2,3,4,5] };
    let mut datamut = TestData { data: vec![1,2,3,4,5] };
    let mut datamut1 = TestData { data: vec![1,2,3,4,5] };

    println!("{:p}", &data);
    move_tdata(data);
    move_tdata(datamut);

    share_tdata(&data1);
    share_tdata(&datamut1);

    println!("{:p}", &datamut1);
    share_tdata_mut(&mut datamut1);
    // share_tdata_mut(&mut data1); // can not share as mutable something is not mutalble
    
    
    let mut datamut3 = TestData { data: vec![1,2,3,4,5] };
    let datamut4 = TestData { data: vec![1,2,3,4,5] };
    share_tdata_mut_2(&mut datamut3, datamut4);
    
    let mut datamut5 = TestData { data: vec![1,2,3,4,5] };
    let mut datamut6 = TestData { data: vec![1,2,3,4,5] };
    println!("{:p}", &datamut5);
    share_tdata_mut_3(&mut datamut5, &mut datamut6);
}

fn move_tdata(data: TestData) {
    println!("{:p}", &data);
}

fn move_tdata_mut(mut data: TestData) {}

fn share_tdata(data: &TestData) {}

fn share_tdata_mut(data: &mut TestData) {
    println!("{:p}", data);
    println!("{:p}", &data);
    *data = TestData { data: vec![1,2,3,4,5] };
    println!("{:p}", data);
    println!("{:p}", &data);
}

fn share_tdata_mut_2(data1: & mut TestData, data2: TestData) {
    println!("{:p}", data1);
    println!("{:p}", &data1);
    *data1 = data2;
    println!("{:p}", data1);
    println!("{:p}", &data1);
}

fn share_tdata_mut_3<'life>(mut data1: &'life mut TestData, mut data2: &'life mut TestData) {
    println!("share_tdata_mut_3");
    println!("{:p}", data1);
    println!("{:p}", &data1);
    println!("{:p}", &data2);
    println!("{:p}", data2);
    //data1 = data2;
    let mut tmp = &mut data2;
    println!("{:p}", data1);
    println!("{:p}", &data2);
    println!("{:p}", &data1);
}

fn test14() {
    println!("--- test14 ---");
    let mut x: i32 = 5;
    println!("x = {}", x);
    {
        let mut y: &mut i32 = &mut x;
        *y += 10;
        println!("y = {}", y);
        println!("{:p}", y);
        test_and_check(y);
        let mut z: &mut &mut i32 = &mut y;
        **z += 15;
        println!("z = {}", z);
        let mut q: &mut &mut &mut i32 = &mut z;
        ***q += 20;
        println!("q = {}", q);
        let m: &mut &mut &mut &mut i32 = &mut q;
        ****m += 30;
        println!("m = {}", m);
    }
    println!("x = {}", x);
}

fn test15() {
    let mut x = TestData { data: vec![1,2,3,4,5] };
    {
        let mut y: &mut TestData = &mut x;
        println!("{:p}", y);
        test_and_check_1(y);
        println!("{:p}", y);
        println!("{:?}", *y);
        let mut z: &mut &mut TestData = &mut y;
        test_and_check_2(z);
        let mut a: &mut &mut &mut TestData = &mut z;
    }
}

fn test_and_check_1(x: &mut TestData) {
    *x = TestData { data: vec![1,2,3,4] };
    println!("{:p}", x);
}

fn test_and_check_2(x: &mut &mut TestData) {
}

fn test_and_check(x: & i32) {
    println!("{:p}", x);
    //(*x) +=1;
}

fn test16() {
    let mut x = 10; 
    println!("x = {}", x);
    let mut y = &mut x;
    *y = 20;
    println!("x = {}", y);
    let mut z = &mut y;
    **z = 30;
    println!("x = {}", z);
}

struct Pair<T, U> {
    first: T,
    second: U
}

trait Revertable<T, U> {
    fn revert(&self) -> Pair<U, T>;
}

impl<T, U> Pair<T, U> {
    fn first(&self) -> &T { return &self.first; }
    fn second(&self) -> &U { return &self.second; }
}

impl<T: MyTrait+Clone, U: MyTrait+Clone> Revertable<T, U> for Pair<T, U> {
    fn revert(&self) -> Pair<U, T> {
        Pair { first: self.second.clone(), second: self.first.clone() }
    }
}

/*
impl Revertable<i32, String> for Pair<i32, String> {
    fn revert(&self) -> Pair<String, i32> {
        Pair { first: self.second.clone(), second: self.first }
    }
}
*/

impl Revertable<TestData, TestData> for Pair<TestData, TestData> {
    fn revert(&self) -> Pair<TestData, TestData> {
        Pair { first: self.second.clone(), second: self.first.clone() }
    }
}

fn test17() {
/*    let pair = Pair { first: 10, second: String::from("hello") };
    //let pair = Pair { first: 10, second: 10.5 };
    println!("{}", pair.first());
    println!("{}", pair.second());*/

    let p = Pair { first: TestData {data: vec![1,2,3,4]}, second: TestData { data: vec![1,2,3,4,5,6,7,8]}};
    println!("{:?}", p.first());
    println!("{:?}", p.second());
    let mut p1 = p.revert();

    let Pair{ first: v1, second: v2} = &mut p1;
    println!("v1 = {:?}", v1);
    *v1 = TestData { data: vec![1,2,3] };
    println!("v1 = {:?}", v1);
    println!("v1 = {:?}", p1.first());
}

struct ToDrop;

impl Drop for ToDrop {
    fn drop(&mut self) {
        println!("ToDrop is being dropped");
    }
}

fn test18() {
    let x = ToDrop;
    println!("Made a ToDrop!");
}

fn myfunc(_: i32) -> i32 { 5 }

fn test19() {
    let f = myfunc; 
    println!("f() = {}", f(10));

    let clos = || {
        println!("hello from a closure");
    };
    clos();

    let fx_1 = |x: &mut i32| { *x+=1; };
    let mut x = 10;
    fx_1(&mut x);
    println!("fx_1() = {}", x);
    let z = 11;
    let check = |x: &i32| { *x==z };
    println!("check() = {}", check(&x));

    let v1 = vec![1,2,3,4,5];
    let v2 = vec![1,2,3,4,5];
    let clos = |x: &Vec<i32>| { *x == v1 };
    println!("clos() = {}", clos(&v2));
    println!("v1 = {:?}", v1);
    
    let mut v3 = vec![1,2,3,4,5];
    let v4 = vec![1,2,3,4,5];
    let mut clos1 = |x: &Vec<i32>| { v3.push(1); *x == v3 };
    let clos1_res = clos1(&v4);
    println!("clos1_res = {}", clos1_res);
    
    {
        let mut v3 = vec![1,2,3,4,5];
        let v4 = vec![1,2,3,4,5];
        let mut clos1 = |x: &Vec<i32>| { v3.push(1); *x == v3 };
        let clos1_res = clos1(&v4);
        println!("clos1_res = {}", clos1_res);
        println!("v3 = {:?}", v3);
    }

    {
        let i = 5;
        let clos = |&i| i ;
        let ii = clos(&i);
    }
}

fn create_fn() -> impl Fn() {
    let text = "Fn".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fn_1() -> impl Fn(i32) -> i32 {
    |x| x
}

fn create_fnmut() -> impl FnMut() {
    let text = "FnMut".to_owned();

    move || println!("This is a: {}", text)
}

fn create_fnonce() -> impl FnOnce() {
    let text = "FnOnce".to_owned();

    move || println!("This is a: {}", text)
}

fn test20() {
    let fn_plain = create_fn();
    let mut fn_mut = create_fnmut();
    let fn_once = create_fnonce();
    let fn_plain_1 = create_fn_1();

    fn_plain();
    fn_mut();
    fn_once();
    println!("fn_plain_1() = {}", fn_plain_1(5));
}

fn test21() {
    //create_write_file();
    //read_file();
}

fn create_write_file() {
    let mut file = File::create("test.txt").unwrap();
    let s = String::from("Hello Linux Journal!\n");
    println!("len s = {}", s.len());
    println!("len s.as_bytes().len() = {}", s.as_bytes().len());
    file.write_all(s.as_bytes()).unwrap();
}

fn read_file() {
    //let path = String::from("test.txt");
    let path = String::from("/Users/vk/software/signavio/coding-challenge-viktor-khristenko/samples/Activity_Log_2004_to_2014.csv");
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("contents = {}", contents);
}

mod test_mod1;
mod test_mod2;

fn test22() {
    test_mod1::test0();
    test_mod2::test0();
}



fn test23() {
    let mut v = vec![1,3,2,5,4];
    println!("{:?}", v);
    v.sort_by(|a, b| b.cmp(&a));
    println!("{:?}", v);

    println!("----");

    let mut x = vec![1,2,3,4];
    some_func_10(&mut x);
    println!("{}", x.len());
}

fn some_func_10(x: &mut Vec<i32>) {}

#[derive(Debug)]
struct MyPoint {
    x: i32,
    y: i32
}

use std::ops;

impl ops::Add<MyPoint> for MyPoint {
    type Output = MyPoint;

    fn add(self, rhs: MyPoint) -> MyPoint {
        println!("> MyPoint.add(MyPoint) was called");

        MyPoint { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<'a> ops::Add<&'a MyPoint> for &'a MyPoint {
    type Output = MyPoint;

    fn add(self: &'a MyPoint, rhs: &'a MyPoint) -> MyPoint {
        println!("> MyPoint.add(MyPoint) was called");

        MyPoint { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Drop for MyPoint {
    fn drop(&mut self) {
        println!("> Dropping {:?}", self);
    }
}

fn test24() {
    let p1 = MyPoint { x: 5, y: 5 };
    let p2 = MyPoint { x: 10, y: 10 };
    println!("{:?} {:?}", p1, p2);

    //let p = p1 + &p2;
    let p = (&p1).add(&p2);
    println!("{:?}", p);
    println!("{:?}", p1);
}
