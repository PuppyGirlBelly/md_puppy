use std::env;

mod command_line;
mod directory_handling;
mod markdown_compiling;
mod template_processing;

use command_line::usage;
use directory_handling::{_copy_static, _init_directories};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        arg if arg == 2 => match String::as_str(&args[1].to_lowercase()) {
            "init" => {
                _init_directories().expect("Error: Could not initalize directories");
            }
            "build" => {
                _copy_static().expect("Error: Could not copy static folder");
                directory_handling::_process_content().expect("Error: Error processing content");
            }
            _ => {
                eprintln!("Error: Invalid Invocation");
            }
        },
        arg if arg < 2 => eprintln!("Error: Not enough arguments"),
        arg if arg > 2 => eprintln!("Error: Too many arguments"),
        _ => {
            eprintln!("Error: Invalid Invocation");
            usage();
        }
    }
}
