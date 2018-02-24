use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    if let Err(err) = run(config) {
        println!("Application error: {}", err);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}
