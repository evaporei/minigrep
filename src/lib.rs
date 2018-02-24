#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        Ok(Config { query, file_name })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    println!("With text:\n{}", contents);

    Ok(())
}
