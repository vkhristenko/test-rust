extern crate libc;

#[link(name = "testrustc")]
extern {
    fn add_int(a: i32, b: i32) -> i32;
}

fn main() {
    println!("hello world");

    unsafe {
        let c = add_int(5, 6);
        println!("c = {}", c);
    }
}
