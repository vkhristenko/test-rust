use std::env;
use std::error::Error;
use std::process;
use std::fs::File;
use std::io::prelude::*;

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    println!("contents\n {}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}
