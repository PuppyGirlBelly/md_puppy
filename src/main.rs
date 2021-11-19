use std::env;
use std::process;

use md_puppy::{Config, parse_markdown_file, usage};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    if let Err(e) = md_puppy::run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            eprintln!("Error: Invalid Invocation");
            usage();
        }
    }
}
