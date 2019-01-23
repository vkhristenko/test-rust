use std::fs::File;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    //test0();
    //test1();
    //test2();
    //test3();
    test4();
    test5();
}

fn test1() {
    let v = vec![1,2,3];
    v[99];
}

fn test0() {
    panic!("crash and burn");
}

fn test2() {
    let f = File::open("hello.txt");
    let f = match f {
        Result::Ok(file) => file,
        Result::Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Result::Ok(fc) => { 
                    println!("created a file: {:#?}", fc);
                    fc
                },
                Result::Err(e) => panic!("tried to create a problem but there was a problem: {:#?}", e),
            },
            other_error => panic!("there was a problem opening the file {:#?}", error),
        },
    };
}

fn test3() {
    let f = File::open("hello1.txt").expect("failed to open hello1.txt");
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let f = File::open("hello2.txt");

    let mut f = match f {
        Result::Ok(file) => file,
        Result::Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, std::io::Error> {
    let mut f = File::open("hello3.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s);
    Ok(s)
}

fn test4() {
    match read_username_from_file() {
        Ok(s) => println!("username is {}", s),
        Err(e) => println!("there was a error reading the username: {:#?}", e),
    };
}

fn test5() {
    match read_username_from_file_2() {
        Ok(s) => println!("username is {}", s),
        Err(e) => println!("there was an error reading the username: {:#?}", e),
    };
}
