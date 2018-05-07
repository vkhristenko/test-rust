extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // get the cli arguments
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("problem parsing cli args: {}", err);
        process::exit(1);
    });
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    println!("******************");

    if let Err(e) = minigrep::run(config) {
        eprintln!("application error: {}", e);

        process::exit(1);
    }

   // let mut f = File::open(config.filename).expect("file not found");



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

fn parse_config(args: &[String]) -> Result<Config, &'static str> {
    if args.len()<3 {
        return Err("not enough args");
//        panic!("not enough args");
    }

    let query = args[1].clone();
    let filename = args[2].clone();

    let case_sensitive = true;
    Ok(Config { query, filename, case_sensitive })
}
