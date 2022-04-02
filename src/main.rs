use std::{env, process};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: Problem on parsing arguments: {}", err);
        process::exit(1);
    });

    let mut f = File::open(config.filename)
        .expect("File not found");

    let mut text = String::new();
    f.read_to_string(&mut text)
        .expect("File unreadable");

    println!("{}", text)
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
