// TODO: Add anyhow library and use that for better error handling.
// TODO: Write up a README.md
// TODO: Get a license
// TODO: Publish to crates.io

use clap::Parser;

mod cli;
mod directory_handling;
mod markdown_compiling;
mod page_creation;
mod site_data;

use cli::Commands;
use directory_handling::{copy_static, init_directories, move_to_project_root, process_content};
use page_creation::create_page;

fn main() {
    let args = cli::Args::parse();

    if let Some(shell) = args.completions {
        shell.generate();
        std::process::exit(0);
    }

    match args.command {
        Some(Commands::Init) => {
            if let Err(e) = init_directories() {
                eprintln!("Error: Could not initalize directories. {e}");
                std::process::exit(0);
            };
            println!("[ INFO ] directories initalized successfully!");
        }
        Some(Commands::Build) => {
            if let Err(e) = move_to_project_root() {
                eprintln!(
                    "[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init': {e}"
                );
                std::process::exit(0);
            }
            if let Err(e) = copy_static() {
                eprintln!("Error: Could not copy static folder, try running 'md_puppy init'. {e}");
                std::process::exit(0);
            };
            if let Err(e) = process_content() {
                eprintln!("Error: Error processing content. {e}");
                std::process::exit(0);
            };
            println!("[ INFO ] Building completed successfully!");
        }
        Some(Commands::New { file }) => {
            if let Err(e) = move_to_project_root() {
                eprintln!(
                    "[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init': {e}"
                );
                std::process::exit(0);
            }
            if let Err(e) = create_page(&file) {
                eprintln!("Error: Could not create new page {e}");
                std::process::exit(0);
            };
        }
        _ => {
            eprintln!("Error: Invalid Invocation");
        }
    }
}
