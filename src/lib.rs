use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

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
    let mut f = File::open(config.filename)?;

    let mut text = String::new();
    f.read_to_string(&mut text)?;

    println!("{}", text);

    Ok(())
}
