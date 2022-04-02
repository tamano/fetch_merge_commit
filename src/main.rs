use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let mut f = File::open(filename)
        .expect("File not found");

    let mut text = String::new();
    f.read_to_string(&mut text)
        .expect("File unreadable");

    println!("{}", text)
}
