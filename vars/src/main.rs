fn main() {
    println!("Hello, world!");

    let x = 5;
    println!("x = {}", x);

    let y = 10;

    let y = 20;

    // interger singed and unsigned types
    let x0: u8 = 1;
    let x1: u16 = 1;
    let x2: u32 = 1;
    let x3: u64 = 1;
    let x4: usize = 1;

    let y0: i8 = 1;
    let y1: i16 = 1;
    let y2: i32 = 1;
    let y3: i64 = 1;
    let y4: isize = 1;

    // integer literals
    let xx0 = 98_222;
    let xx1 = 0xff;
    let xx2 = 0o77;
    let xx3 = 0b1111_0000;

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple = {:?}", tup);

    // arrays
    let a = [1,2,3,4];

    println!("a[5] = {}", a[5]);

}
