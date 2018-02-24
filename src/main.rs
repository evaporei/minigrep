use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_name) = parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", file_name);

    let mut file = File::open(file_name)
        .expect("file not found");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_name = &args[2];

    (query, file_name)
}
