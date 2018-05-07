use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename).expect("file not found");
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;
    println!("contents\n {}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "not enough args")]
    fn test_config_should_panic() {
        let args = [String::from("./myprogram")];
//        let config = Config::new(&args)

        if let Err(e) = Config::new(&args) {
            panic!("{}", e);
        }
    }

    #[test]
    fn test_config_simple() {
        let args = [String::from("./myprogram"), String::from("test"), 
            String::from("poem.txt")];

        if let Ok(config) = Config::new(&args) {
            assert_eq!(config.query, String::from("test"));
            assert_eq!(config.filename , String::from("poem.txt"));
        }
    }
}
