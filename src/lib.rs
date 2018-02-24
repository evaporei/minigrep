#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use std::env;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_name = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, file_name, case_sensitive })
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line|
            line.to_lowercase()
                .contains(&query.to_lowercase())
        )
        .collect()
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(&query.to_lowercase()))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut file = File::open(config.file_name)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let results = if config.case_sensitive {
        search_case_sensitive(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
