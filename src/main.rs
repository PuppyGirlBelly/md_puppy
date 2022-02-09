use std::env;

mod command_line;
mod directory_handling;
mod markdown_compiling;
mod page_creation;
mod site_data;

use crate::command_line::print_short_banner;
use command_line::usage;
use directory_handling::{copy_static, init_directories, process_content};
use page_creation::create_page;

fn main() {
    let args: Vec<String> = env::args().collect();
    print_short_banner();

    match args.len() {
        arg if arg == 2 => match String::as_str(&args[1].to_lowercase()) {
            "init" => {
                init_directories().expect("Error: Could not initalize directories.");
                println!("[ INFO ] directories initalized successfully!");
            }
            "build" => {
                copy_static()
                    .expect("Error: Could not copy static folder, try running 'md_puppy init'.");
                process_content().expect("Error: Error processing content.");
                println!("[ INFO ] Building completed successfully!");
            }
            "new" => {}
            _ => {
                eprintln!("Error: Invalid Invocation");
            }
        },
        arg if arg == 3 => match String::as_str(&args[1].to_lowercase()) {
            "new" => create_page(&args[2]).expect("Error: Could not create new page"),
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
