use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
// use std::env;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};

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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
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

fn print_short_banner() {
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

pub fn parse_markdown_file(filename: &str) -> Result<String, Box<dyn Error>> {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let path: &Path = Path::new(filename);
    let input: String = fs::read_to_string(path).expect("[ ERROR ] Failed to open file!");

    // Setup options and commonmark parser
    let mut parser_options = pulldown_cmark::Options::empty();
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(&input, parser_options);

    // Write to String buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("[ INFO ] Parsing {:?} complete!", path);
    Ok(html_output)
}

fn _markdown_to_html(filename: &str) -> Result<(), Box<dyn Error>> {
    let input: String = parse_markdown_file(filename)?;
    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    for line in input.lines() {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");
    }

    println!("[ INFO ] Parsing complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parse_test() {
        let input: String = parse_markdown_file("src/example_short.md").unwrap();
        let output = "\
<h1>An h1 header</h1>
<p>============</p>
<p>Paragraphs are separated by a blank line.</p>
<p>2nd paragraph. <em>Italic</em>, <strong>bold</strong>, and <code>monospace</code>. Itemized lists
look like:</p>
<ul>
<li>this one</li>
<li>that one</li>
<li>the other one</li>
</ul>
";
        assert_eq!(input, output);
    }

    #[test]
    fn markdown_to_html_test() {
        assert!(_markdown_to_html("src/example_short.md").is_ok());
        let output: String = fs::read_to_string("src/example_short.html").unwrap();
        let answer = "\
<h1>An h1 header</h1>\
<p>============</p>\
<p>Paragraphs are separated by a blank line.</p>\
<p>2nd paragraph. <em>Italic</em>, <strong>bold</strong>, and <code>monospace</code>. Itemized lists\
look like:</p>\
<ul>\
<li>this one</li>\
<li>that one</li>\
<li>the other one</li>\
</ul>";
        assert_eq!(output, answer);
    }
}
