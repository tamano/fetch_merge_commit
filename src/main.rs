extern crate fetch_merge_commit;

use std::{env, process};
use fetch_merge_commit::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Error: Problem on parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = fetch_merge_commit::run(config) {
        println!("Error: Application error: {}", e);
        process::exit(1);
    }
}
