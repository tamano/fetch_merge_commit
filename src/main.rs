use std::{env, process};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: Problem on parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Error: Application error: {}", e);
        process::exit(1);
    }
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() != 2 {
            return Err("Illegal number of  Argument");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;

    let mut text = String::new();
    f.read_to_string(&mut text)?;

    println!("{}", text);

    Ok(())
}