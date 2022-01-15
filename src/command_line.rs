use std::error::Error;
use std::fs;

pub struct Input {
    pub filename: String,
    pub site_name: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() < 2 {
            return Err("Not enough arguements");
        }

        let filename = args[1].clone();
        let site_name = String::from("md_puppy");

        Ok(Input {
            filename,
            site_name,
        })
    }
}

pub fn file_checker(config: Input) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn usage() {
    print_long_banner();
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));

    the_title.push_str(" (v");
    the_title.push_str(&String::from(env!("CARGO_PKG_VERSION"))[..]);
    the_title.push_str(") ");
    the_title.push_str(&String::from(env!("CARGO_PKG_DESCRIPTION"))[..]);

    the_title
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
