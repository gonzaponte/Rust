use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Error while parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.filename);

    if let Err(e) = minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
