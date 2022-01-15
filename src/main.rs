use std::env;
use std::process;

mod command_line;
mod markdown_compiling;

use command_line::{file_checker, usage, Input};
use markdown_compiling::file_to_html;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = Input::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguements: {}", err);
        process::exit(1);
    });

    if let Err(e) = file_checker(input) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };

    match args.len() {
        1 => {
            file_to_html(&args[1]).expect("Error: Could not parse file");
        }
        _ => {
            eprintln!("Error: Invalid Invocation");
            usage();
        }
    }
}
