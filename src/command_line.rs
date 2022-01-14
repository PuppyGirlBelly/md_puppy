use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguements");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn file_checker(config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn usage() {
    print_long_banner();
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));

    the_title.push_str(" (v");
    the_title.push_str(&get_version()[..]);
    the_title.push_str(") ");
    the_title.push_str(&get_description()[..]);

    the_title
}

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

fn get_description() -> String {
    String::from(env!("CARGO_PKG_DESCRIPTION"))
}

pub fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: puppy_md build\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}
