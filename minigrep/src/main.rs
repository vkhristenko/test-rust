use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    // get the cli arguments
    let args: Vec<String> = env::args().collect();

    let (query, filename) = parse_config(&args);

    /*
    println!("{:?}", args);

    // save them to vars
    let query = &args[1];
    let filename = &args[2];

    println!("searching for {}", query);
    println!("in file {}", filename);

    // open up the file 
    let mut f = File::open(filename).expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    
    println!("test:\n************************\n {}", contents);
    */
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];

    (query, filename)
}
