use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod directory_handling;
mod markdown_compiling;
mod page_creation;
mod site_data;

use cli::Commands;
use directory_handling::{copy_static, init_directories, move_to_project_root, process_content};
use page_creation::create_page;

fn main() -> Result<()> {
    let args = cli::Args::parse();

    if let Some(shell) = args.completions {
        shell.generate();
        std::process::exit(0);
    }

    match args.command {
        Commands::Init => {
            init_directories().with_context(|| "Error: Could not initalize directories. ")?;
            println!("[ INFO ] directories initalized successfully!");
            Ok(())
        }
        Commands::Build => {
            move_to_project_root().with_context(|| {
                "[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init'"
            })?;
            copy_static().with_context(|| {
                "Error: Could not copy static folder, try running 'md_puppy init'."
            })?;
            process_content().with_context(|| "Error: Error processing content.")?;
            println!("[ INFO ] Building completed successfully!");
            Ok(())
        }
        Commands::New { file } => {
            move_to_project_root().with_context(|| {
                "[ ERROR ] Could not find find cargo.yaml. Try running 'md_puppy init'"
            })?;
            create_page(&file).with_context(|| "Error: Could not create new page")?;
            Ok(())
        }
    }
}
