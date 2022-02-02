use std::env;
use std::process;

mod command_line;
mod directory_handling;
mod markdown_compiling;
mod template_processing;

use command_line::{file_checker, usage, Input};
use directory_handling::_init_directories;
use template_processing::file_to_html;

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
        3 => {
            file_to_html(&args[1], "template/template.html").expect("Error: Could not parse file");
        }
        _ => {
            eprintln!("Error: Invalid Invocation");
            usage();
        }
    }
}
