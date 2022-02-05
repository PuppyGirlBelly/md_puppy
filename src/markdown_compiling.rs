use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use yaml_rust::YamlLoader;

use crate::directory_handling::_check_and_create_directory;

#[derive(Deserialize)]
pub struct Page {
    pub title: String,
    pub description: String,
    pub category: String,
    pub date: String,
    pub site_name: String,
    pub content: String,
}

impl Page {
    pub fn new() -> Page {
        Page {
            title: String::from("default_title"),
            description: String::from("default_description"),
            category: String::from("default_cat"),
            date: String::from("default_date"),
            site_name: String::from("default_site_name"),
            content: String::from("<h1>default_content</h1>"),
        }
    }
}

pub fn parse_markdown_file(filename: &str) -> Result<Page, Box<dyn Error>> {
    println!("[ INFO ] Trying to parse {}...", filename);

    let path: &Path = Path::new(filename);
    let input: String = fs::read_to_string(path).expect("[ ERROR ] Failed to open file!");

    let mut page: Page = Page::new();

    let output: Vec<&str> = input.splitn(3, "---").filter(|&x| !x.is_empty()).collect();
    parse_frontmatter(output[0], &mut page)?;
    page.content = content_to_html(output[1]);
    println!("[ INFO ] Parsing {:?} complete!", path);

    Ok(page)
}

fn parse_frontmatter<'a>(frontmatter: &'a str, page: &'a mut Page) -> Result<&'a Page, String> {
    let yaml = YamlLoader::load_from_str(frontmatter);

    match yaml {
        Err(_) => Err("[ ERROR ] Frontmatter is missing".to_string()),
        Ok(y) => {
            let fm = &y[0];

            page.title = fm["title"].as_str().unwrap_or("Default Title").to_string();
            page.description = fm["description"]
                .as_str()
                .unwrap_or("Site generated with puppy_md")
                .to_string();
            page.date = fm["date"]
                .as_str()
                .unwrap_or("1970-01-01T00:00:00-0000")
                .to_string();
            page.category = fm["category"].as_str().unwrap_or("").to_string();

            Ok(page)
        }
    }
}

pub fn content_to_html(input: &str) -> String {
    // Setup options and commonmark parser
    let mut parser_options = pulldown_cmark::Options::empty();
    parser_options.insert(Options::ENABLE_TABLES);
    parser_options.insert(Options::ENABLE_FOOTNOTES);
    parser_options.insert(Options::ENABLE_STRIKETHROUGH);
    parser_options.insert(Options::ENABLE_TASKLISTS);
    parser_options.insert(Options::ENABLE_SMART_PUNCTUATION);
    let parser = Parser::new_ext(input, parser_options);

    // Write to String buffer
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub fn file_to_html(file_path: &str, template_path: &str) -> Result<(), Box<dyn Error>> {
    let page: Page = parse_markdown_file(file_path).unwrap();
    let html_page: String = process_template(&page, template_path)?;
    let filename: String =
        String::from(&file_path[file_path.rfind('/').unwrap()..file_path.len() - 3]);
    let output_directory: String = get_output_dir(&page.category);

    _check_and_create_directory(&output_directory)?;

    let output_filename: String = format!("{}/{}.html", output_directory, &filename);

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(html_page.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

    Ok(())
}

fn get_output_dir(category: &str) -> String {
    match category.to_lowercase().as_str() {
        "home" | "index" | "" => String::from("site/"),
        cat => format!("site/{}/", cat),
    }
}

pub fn process_template(page: &Page, template_path: &str) -> Result<String, Box<dyn Error>> {
    let template: String =
        fs::read_to_string(template_path).expect("[ ERROR ] Failed to open html template!");
    let mut output: String = String::new();

    for line in template.lines() {
        output.push_str(&replace_placeholder(line, page)?);
        output.push('\n');
    }

    Ok(output)
}

fn replace_placeholder(input_text: &str, page: &Page) -> Result<String, String> {
    if let Some(key) = get_placeholder(input_text) {
        if let Some(v) = get_value(key, page) {
            replace_placeholder(&input_text.replace(key, &v), page)
        } else {
            Err("Invalid key, ".to_owned() + key + " in template")
        }
    } else {
        Ok(input_text.to_string())
    }
}

fn get_placeholder(template: &str) -> Option<&str> {
    let start_byte = template.find("{{").unwrap_or(0);
    let end_byte = template.find("}}").unwrap_or(0);

    if (start_byte == 0) && (end_byte == 0) {
        None
    } else {
        Some(&template[start_byte..(end_byte + 2)])
    }
}

fn get_value(key: &str, page: &Page) -> Option<String> {
    match key {
        "{{title}}" => Some(page.title.to_string()),
        "{{description}}" => Some(page.description.to_string()),
        "{{category}}" => Some(page.category.to_string()),
        "{{date}}" => Some(page.date.to_string()),
        "{{site_name}}" => Some(page.site_name.to_string()),
        "{{content}}" => Some(page.content.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn markdown_parse_test() {
        let output: Page = parse_markdown_file("content/example_short.md").unwrap();
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
category: example_category
date: example_date
";
        parse_frontmatter(frontmatter, &mut page).expect("[ ERROR ] Failed to parse frontatter!");

        assert_eq!(page.title, String::from("example_title"));
        assert_eq!(page.description, String::from("example_description"));
        assert_eq!(page.category, String::from("example_category"));
        assert_eq!(page.date, String::from("example_date"));
    }

    #[test]
    fn index_test() {
        file_to_html("content/index.md", "template/boilerplate.html")
            .expect("[ TEST ] Could not make index");
        assert!(File::open("site/index.html").is_ok());
    }

    #[test]
    fn file_to_html_test() {
        crate::directory_handling::_check_and_create_directory("site/examples")
            .expect("[ TEST ERR ] This directory could not be created.");
        file_to_html("content/example_short.md", "template/boilerplate.html")
            .expect("[ TEST ERR ] This file could not be processed.");
        assert!(File::open("site/examples/example_short.html").is_ok());
    }

    #[test]
    fn get_output_dir_test() {
        let output: String = get_output_dir("home");
        assert_eq!(output, String::from("site/"));
        let output: String = get_output_dir("index");
        assert_eq!(output, String::from("site/"));
        let output: String = get_output_dir("");
        assert_eq!(output, String::from("site/"));
        let output: String = get_output_dir("test");
        assert_eq!(output, String::from("site/test/"));
    }

    #[test]
    fn template_processing_test() {
        let page: Page = parse_markdown_file("content/example_short.md").unwrap();
        let template_path: &str = "template/boilerplate.html";
        let output = process_template(&page, template_path);
        let answer = "\
<!doctype html>
<html class='no-js' lang='en'>
<head>
  <meta charset='utf-8'>
  <title>A Short Example</title>
  <meta name='description' content='static-generated site made with md_puppy'>
  <meta name='viewport' content='width=device-width, initial-scale=1'>
  <meta property='og:site_name' content='default_site_name'>
  <meta property='og:title' content='A Short Example'>
  <meta property='og:type' content='website'>
  <meta property='og:url' content='https://softannalee.github.io/examples/A Short Example'>
  <meta property='og:image' content='https://softannalee.github.io/examples/A Short Example/image.jpg'>
  <link rel='stylesheet' href='/css/normalize.css'>
  <link rel='stylesheet' href='/css/main.css'>
</head>
<body>
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

</body>
</html>
";

        assert_eq!(output.unwrap(), answer);
    }

    #[test]
    fn template_replacement_test() {
        let page: Page = parse_markdown_file("content/example_short.md").unwrap();

        let input: &str = "This is {{title}}";
        let output = replace_placeholder(input, &page);
        assert_eq!(output.unwrap(), "This is A Short Example");

        let input: &str = "{{category}}: This is {{title}}";
        let output = replace_placeholder(input, &page);
        assert_eq!(output.unwrap(), "examples: This is A Short Example");

        let input: &str = "This is title";
        let output = replace_placeholder(input, &page);
        assert_eq!(output.unwrap(), "This is title");

        let input: &str = "This is {{tags}}";
        assert!(replace_placeholder(input, &page).is_err());
    }

    #[test]
    fn get_value_test() {
        let page: Page = parse_markdown_file("content/example_short.md").unwrap();

        assert_eq!(get_value("{{title}}", &page).unwrap(), "A Short Example");
        assert_eq!(
            get_value("{{description}}", &page).unwrap(),
            "this is a short example of a markdown file"
        );
        assert_eq!(get_value("{{category}}", &page).unwrap(), "examples");
        assert_eq!(
            get_value("{{date}}", &page),
            Some("2022-01-17T12:34:11-0700".to_string())
        );
        assert_eq!(get_value("{{tags}}", &page), None);
    }

    #[test]
    fn get_placeholder_test() {
        let example_text: &str = "{{title}}";
        let output: Option<&str> = get_placeholder(example_text);

        assert_eq!(output.unwrap(), "{{title}}".to_string());

        let example_text: &str = "this is a title";
        let output: Option<&str> = get_placeholder(example_text);

        assert_eq!(output, None);
    }
}
