use std::env;

mod command_line;
mod directory_handling;
mod markdown_compiling;
mod page_creation;
mod site_data;

use crate::command_line::print_short_banner;
use command_line::usage;
use directory_handling::{copy_static, init_directories, move_to_project_root, process_content};
use page_creation::create_page;

fn main() {
    let args: Vec<String> = env::args().collect();
    print_short_banner();

    match args.len() {
        1 => {
            usage();
        }
        arg if arg == 2 => match String::as_str(&args[1].to_lowercase()) {
            "init" => {
                if let Err(e) = init_directories() {
                    eprintln!("Error: Could not initalize directories. {e}")
                };
                println!("[ INFO ] directories initalized successfully!");
            }
            "build" => {
                if let Err(e) = move_to_project_root() {
                    eprintln!("[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init': {e}");
                }
                if let Err(e) = copy_static() {
                    eprintln!(
                        "Error: Could not copy static folder, try running 'md_puppy init'. {e}"
                    )
                };
                if let Err(e) = process_content() {
                    eprintln!("Error: Error processing content. {e}")
                };
                println!("[ INFO ] Building completed successfully!");
            }
            _ => {
                eprintln!("Error: Invalid Invocation");
                usage();
            }
        },
        arg if arg == 3 => match String::as_str(&args[1].to_lowercase()) {
            "new" => {
                if let Err(e) = move_to_project_root() {
                    eprintln!("[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init': {e}");
                }
                if let Err(e) = create_page(&args[2]) {
                    eprintln!("Error: Could not create new page {e}")
                };
            }
            _ => {
                eprintln!("Error: Invalid Invocation");
                usage();
            }
        },
        arg if arg < 2 => {
            eprintln!("Error: Not enough arguments");
            usage();
        }
        arg if arg > 2 => {
            eprintln!("Error: Too many arguments");
            usage();
        }
        _ => {
            eprintln!("Error: Invalid Invocation");
            usage();
        }
    }
}
