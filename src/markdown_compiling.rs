use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
// use std::env;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
// use toml;

use super::command_line::print_short_banner; //::{file_checker, markdown_to_html, usage, Config};

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub description: String,
    pub categories: Vec<String>,
    pub date: String,
    pub content: String,
}

impl Page {
    pub fn new(filename: String) -> Result<Page, Box<dyn Error>> {
        let result = Page {
            title: String::from("test_title"),
            description: String::from("test_description"),
            categories: vec![String::from("test_cat")],
            date: String::from("test_date"),
            content: parse_markdown_file(&filename).unwrap(),
        };

        Ok(result)
    }
}

fn parse_markdown_file(filename: &str) -> Result<String, Box<dyn Error>> {
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

pub fn content_to_html(filename: &str) -> Result<(), Box<dyn Error>> {
    let input: String = parse_markdown_file(filename)?;
    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(input.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

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

    // #[test]
    // fn markdown_to_html_test() {
    //     assert!(content_to_html("src/example_short.md").is_ok());
    //     let output: String = fs::read_to_string("src/example_short.html").unwrap();
    //     let answer = "\
    // <h1>An h1 header</h1>
    // <p>============</p>
    // <p>Paragraphs are separated by a blank line.</p>
    // <p>2nd paragraph. <em>Italic</em>, <strong>bold</strong>, and <code>monospace</code>. Itemized lists
    // look like:</p>
    // <ul>
    // <li>this one</li>
    // <li>that one</li>
    // <li>the other one</li>
    // </ul>
    // ";
    //     assert_eq!(output, answer);
    // }
}
