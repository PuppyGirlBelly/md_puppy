use clap::{ArgEnum, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, shells::*};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Commands>,

    /// Generate a SHELL completion script and print to stdout
    #[clap(long, short, arg_enum, value_name = "SHELL")]
    pub completions: Option<Shell>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Download and initalize directories needed for website
    Init,
    /// Process all files in the 'content/' folder and parse into a website
    Build,
    /// Create a new file within the 'content/' folder with default frontmatter
    New { file: String },
}

#[derive(Parser, Copy, Clone, ArgEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    PowerShell,
    Elvish,
}

impl Shell {
    pub fn generate(&self) {
        let mut app = Args::command();
        let mut fd = std::io::stdout();
        match self {
            Shell::Bash => generate(Bash, &mut app, "md_puppy", &mut fd),
            Shell::Zsh => generate(Zsh, &mut app, "md_puppy", &mut fd),
            Shell::Fish => generate(Fish, &mut app, "md_puppy", &mut fd),
            Shell::PowerShell => generate(PowerShell, &mut app, "md_puppy", &mut fd),
            Shell::Elvish => generate(Elvish, &mut app, "md_puppy", &mut fd),
            _ => generate(Bash, &mut app, "md_puppy", &mut fd),
        }
    }
}
