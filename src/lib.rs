use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Illegal number of  Argument");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let f = File::open(config.filename)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let result = get_merge_info(line?);
        println!("{}", result);
    }

    Ok(())
}

// TODO: not implemented
fn get_merge_info(pr_url: String) -> &'static str {
    fetch_github(pr_url)
}

// TODO: not implemented
fn fetch_github(pr_url: String) -> &'static str {

    println!("target url: {}", pr_url);

    r#"
    {
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#
}