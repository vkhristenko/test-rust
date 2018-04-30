use std::fs::File;
use std::io::Read;
use std::io;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    {
        let v = vec![1,2,3];
        //v[99];
    }

    {
        let f = File::open("hello.txt");

        let f = match f {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => {
                        panic!("tried to create a file but there was a problem {:?}",
                            e)
                    },
                }
            },
            Err(error) => {
                panic!("there was a problem opening the file: {:?}",
                    error)
            },
        };
    }

    {
        match read_username_from_file() {
            Ok(s) => println!("s = {}", s),
            Err(e) => panic!("error reading a username"),
        };
    }

}

fn read_username_from_file() -> Result<String, io::Error> {

        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e)
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
