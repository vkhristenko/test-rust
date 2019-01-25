use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");
    test0();
}

fn test0() {
    let expensive_closure = |num: &mut u64| {
        println!("calculating again...");
        thread::sleep(Duration::from_secs(*num));
        *num += 10;
    };
    let mut x: u64 = 5;
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
