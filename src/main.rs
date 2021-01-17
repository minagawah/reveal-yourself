use std::{env, process};

use reveal_yourself::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing: {}", err);
        process::exit(1);
    });

    if let Err(e) = reveal_yourself::run(config) {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}
