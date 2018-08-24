extern crate libc;

#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

#[link(name = "testrustc")]
extern {
    fn add_int(a: i32, b: i32) -> i32;

    fn show_point(p: Point);
    fn show_point_byref(p: *mut Point);
}

fn main() {
    println!("hello world");

    unsafe {
        let c = add_int(5, 6);
        println!("c = {}", c);
    }

    unsafe {
        let p1 = Point {x: 10, y: 20};
        show_point(p1);

        let mut p2 = Point {x: 10, y:20};
        show_point_byref(&mut p2);
    }
}
