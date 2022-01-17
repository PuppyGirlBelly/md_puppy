use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
// use std::env;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use yaml_rust::YamlLoader;

use super::command_line::print_short_banner; //::{file_checker, markdown_to_html, usage, Config};

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub description: String,
    // TODO: Swap tags to single category, there will only be one category
    // pub category: String,
    pub tags: Vec<String>,
    pub date: String,
    pub content: String,
}

impl Page {
    pub fn new() -> Page {
        Page {
            title: String::from("default_title"),
            description: String::from("default_description"),
            tags: vec![String::from("default_cat")],
            date: String::from("default_date"),
            content: String::from("<h1>default_content</h1>"),
        }
    }
}

pub fn parse_markdown_file(filename: &str) -> Result<Page, Box<dyn Error>> {
    print_short_banner();
    println!("[ INFO ] Trying to parse {}...", filename);

    let path: &Path = Path::new(filename);
    let input: String = fs::read_to_string(path).expect("[ ERROR ] Failed to open file!");

    let mut page: Page = Page::new();

    let output: Vec<&str> = input.split("---").filter(|&x| !x.is_empty()).collect();
    parse_frontmatter(output[0], &mut page)?;
    page.content = content_to_html(output[1])?;
    println!("[ INFO ] Parsing {:?} complete!", path);
    println!("output: {:?}", output);

    Ok(page)
}

fn parse_frontmatter<'a>(
    frontmatter: &'a str,
    page: &'a mut Page,
) -> Result<&'a Page, Box<dyn Error>> {
    let yaml = YamlLoader::load_from_str(frontmatter).unwrap();
    let fm = &yaml[0];

    page.title = fm["title"].as_str().unwrap().to_string();
    page.description = fm["description"].as_str().unwrap().to_string();
    page.date = fm["date"].as_str().unwrap().to_string();

    let fm_tags = fm["tags"].as_vec().unwrap();
    let mut tags: Vec<String> = Vec::new();
    for tag in fm_tags {
        tags.push(tag.as_str().unwrap().to_string());
    }
    page.tags = tags;

    Ok(page)
}

pub fn content_to_html(input: &str) -> Result<String, Box<dyn Error>> {
    // Setup options and commonmark parser
    let mut parser_options = pulldown_cmark::Options::empty();
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(input, parser_options);

    // Write to String buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Ok(html_output)
}

pub fn file_to_html(filename: &str) -> Result<(), Box<dyn Error>> {
    let page: Page = parse_markdown_file(filename)?;
    let content: String = page.content;
    let mut output_filename = String::from(&filename[..filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(content.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

    println!("[ INFO ] Parsing complete!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parse_test() {
        let output: Page = parse_markdown_file("src/example_short.md").unwrap();
        let answer = "\
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
        assert_eq!(output.content, answer);
    }

    #[test]
    fn frontmatter_parsing_test() {
        let mut page: Page = Page::new();
        let frontmatter: &str = "---
title: example_title
description: example_description
tags: 
- example_tag
- testo
- pineapple
date: example_date
";
        parse_frontmatter(frontmatter, &mut page).expect("[ ERROR ] Failed to parse frontatter!");

        assert_eq!(page.title, String::from("example_title"));
        assert_eq!(page.description, String::from("example_description"));
        assert_eq!(page.tags[0], String::from("example_tag"));
        assert_eq!(page.tags[1], String::from("testo"));
        assert_eq!(page.tags[2], String::from("pineapple"));
        assert_eq!(page.date, String::from("example_date"));
    }
}
