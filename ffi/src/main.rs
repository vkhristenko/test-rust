extern crate libc;

use std::ffi::CString;
use std::os::raw::c_char;

#[repr(C)]
struct Point {
    x: i32,
    y: i32
}

#[repr(C)]
struct FileContext {
    file: *mut libc::c_void
}

impl Copy for FileContext {}
impl Clone for FileContext {
    fn clone(&self) -> FileContext {
        *self
    }
}

#[link(name = "testrustc")]
extern {
    fn add_int(a: i32, b: i32) -> i32;

    fn show_point(p: Point);
    fn show_point_byref(p: *mut Point);

    fn open_context_towrite() -> FileContext;
    fn write_buffer(ctx: FileContext, buf: *const c_char, size: i32);
    fn close_context(ctx: FileContext);
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

    unsafe {
        let ctx = open_context_towrite();
        //let buf: *mut i8 = vec![0x00, 0x01, 0x02, 0x03].as_mut_ptr(); 
        let s = "Hello World!";
        let buf = CString::new(s).unwrap();
        write_buffer(ctx, buf.as_ptr(), s.len() as i32);
        close_context(ctx);
    }
}
