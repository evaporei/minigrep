use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_name);

    let mut file = File::open(config.file_name)
        .expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    file_name: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let file_name = args[2].clone();

        Config { query, file_name }
    }
}
