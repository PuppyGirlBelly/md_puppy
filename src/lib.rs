use std::error::Error;
use std::fs;
use std::fs::File;
// use std::env;
use std::path::Path;
use std::io::{BufRead, BufReader, Write};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("Not enough arguements");
        }

        let filename = args[1].clone();

        Ok(Config { 
            filename,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn usage() {
    println!("puppy_md, a personal static site generator by AnnaLee");
    println!("The Version: {}", get_version());
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));

    the_title.push_str(" (v");
    the_title.push_str(&get_version()[..]);
    the_title.push_str(") ");
    the_title.push_str(&get_description()[..]);

    return the_title;
}

fn get_version () -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

fn get_description () -> String {
    String::from(env!("CARGO_PKG_DESCRIPTION"))
}


fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}\nHomepage: {}\nUsage: puppy_md build\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

pub fn parse_markdown_file(filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let input_filename = Path::new(filename);
    let file = File::open(&input_filename)
       .expect("[ ERROR ] Filed to open file!");

    let mut ptag: bool = false; // Paragraph tag flag
    let mut htag: bool = false; // H1 tag flag
    let mut tokens: Vec<String> = Vec::new();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        match first_char.pop() {
            Some('#') => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }
                if htag {
                    ptag = false;
                    output_line.push_str("</h1>\n");
                } 
                htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            },
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }
        if htag {
            htag = false;
            output_line.push_str("</h1>\n");
        } 

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&filename[..filename.len()-3]);
    output_filename.push_str(".html");

    let mut outfile = File::create(output_filename)
        .expect("[ ERROR ] Could not create output file!");

    for line in &tokens {
        outfile.write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
}

#[cfg(test)]
mod tests{
    // use super::*;

    // #[test]
    // fn case_sensitive() {
    //     let query = "duct";
    //     let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

    //     assert_eq!(vec!["safe, fast, productive."], search(query,contents));
    // }
}
