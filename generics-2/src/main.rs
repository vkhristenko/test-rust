fn main() {
    println!("Hello, world!");

    test0();
}

fn test0() {
    let list = vec![1,2,3,4,5,6];
    let result = largest(&list[..]);
    println!("largest is {}", result);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
