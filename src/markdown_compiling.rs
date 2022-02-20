use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use pulldown_cmark::{html, Options, Parser};
use serde::Deserialize;
use yaml_rust::YamlLoader;

use crate::directory_handling::check_and_create_directory;
use crate::site_data::{convert_datetime, Site};

#[derive(Clone, Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Page {
    pub date: String,
    pub title: String,
    pub category: String,
    pub description: String,
    pub content: String,
    pub filepath: String,
    pub filename: String,
    pub output_path: String,
}

impl Page {
    fn new() -> Page {
        Page {
            title: String::from("default_title"),
            description: String::from("default_description"),
            category: String::from("default_cat"),
            date: String::from("default_date"),
            content: String::from("<h1>default_content</h1>"),
            filepath: String::from("content/index.md"),
            filename: String::from("index"),
            output_path: String::from("/"),
        }
    }

    pub fn from_file(filename: &str) -> Result<Page, Box<dyn Error>> {
        let mut page: Page = Page::new();
        let path: &Path = Path::new(filename);
        let input: Vec<String> = fs::read_to_string(path)
            .expect("[ ERROR ] Failed to open file!")
            .splitn(3, "---")
            .filter(|&x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();

        page.filepath = filename.to_string();
        page.filename = get_filename_from_path(filename);
        page.parse_frontmatter(&input[0])?;
        page.output_path = get_output_dir(&page.category);
        page.content = input[1].to_string();
        page.content_to_html("template/boilerplate.html")?;

        Ok(page)
    }

    pub fn parse_frontmatter(&mut self, frontmatter: &str) -> Result<(), String> {
        let yaml = YamlLoader::load_from_str(frontmatter);

        match yaml {
            Err(_) => Err("[ ERROR ] Frontmatter is missing".to_string()),
            Ok(y) => {
                let fm = &y[0];

                self.title = fm["title"].as_str().unwrap_or("Default Title").to_string();
                self.description = fm["description"]
                    .as_str()
                    .unwrap_or("Site generated with puppy_md")
                    .to_string();
                self.date = fm["date"]
                    .as_str()
                    .unwrap_or("1970-01-01T00:00:00-0000")
                    .to_string();
                self.category = fm["category"].as_str().unwrap_or("").to_string();

                Ok(())
            }
        }
    }

    pub fn content_to_html(&mut self, template_path: &str) -> Result<(), Box<dyn Error>> {
        self.content = markdown_to_html(&self.content);

        let template: String =
            fs::read_to_string(template_path).expect("[ ERROR ] Failed to open html template!");
        let mut output: String = String::new();

        for line in template.lines() {
            output.push_str(&replace_placeholder(line, self)?);
            output.push('\n');
        }

        self.content = output;

        Ok(())
    }

    pub fn write_to_file(&mut self) -> Result<(), Box<dyn Error>> {
        let output_directory: String = format!("site{}", self.output_path);

        check_and_create_directory(&output_directory)?;

        let output_filename: String = format!("{}/{}.html", output_directory, self.filename);

        let mut outfile =
            File::create(output_filename).expect("[ ERROR ] Could not create output file!");

        outfile
            .write_all(self.content.as_bytes())
            .expect("[ ERROR ] Could not write to output file!");

        Ok(())
    }

    pub fn replace_navbar(&mut self, navbar: &str) {
        self.content = self.content.replace("<div id=topnav>", navbar);
    }

    pub fn replace_index(&mut self, site: &mut Site) {
        let categories = site.categories.to_owned();
        for cat in categories {
            let placeholder = format!("<div id='index' class='{cat}'>");
            if self.content.contains(&placeholder) {
                let index = site.create_category_index(&cat);
                self.content = self.content.replace(&placeholder, &index);
            }
        }
    }

    pub fn replace_site_name(&mut self, site_name: &str) {
        self.content = self.content.replace("<div id=site_name>", site_name);
    }

    pub fn replace_base_url(&mut self, base_url: &str) {
        self.content = self.content.replace("<base href=base_url>", base_url);
    }
}

pub fn markdown_to_html(input: &str) -> String {
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

pub fn get_output_dir(category: &str) -> String {
    match category.to_lowercase().as_str() {
        "home" | "index" | "" => String::from("/"),
        cat => format!("/{}", cat),
    }
}

pub fn get_filename_from_path(path: &str) -> String {
    path[(path.rfind('/').unwrap() + 1)..path.len() - 3].to_string()
}

fn replace_placeholder(input_text: &str, page: &Page) -> Result<String, String> {
    if let Some(key) = get_placeholder(input_text) {
        if let Some(value) = get_value(key, page) {
            // Recursive to get multiple placeholders in line
            replace_placeholder(&input_text.replace(key, &value), page)
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
        "{{date}}" => Some(convert_datetime(&page.date.to_string())),
        "{{content}}" => Some(page.content.to_string()),
        "{{filename}}" => Some(page.filename.to_string()),
        "{{output_path}}" => Some(page.output_path.to_string()),
        "{{base_url}}" => Some(String::from("<base href=base_url>")),
        "{{site_name}}" => Some(String::from("<div id=site_name>")),
        "{{topnav}}" => Some(String::from("<div id=topnav>")),
        youtube_key if youtube_key.contains("youtube") => Some(embed_youtube(youtube_key)),
        index if index.contains("index") => {
            let category = index
                .strip_prefix("{{ index ")
                .unwrap()
                .strip_suffix(" }}")
                .unwrap();
            Some(format!("<div id='index' class='{category}'>"))
        }
        _ => None,
    }
}

fn embed_youtube(youtube_key: &str) -> String {
    let embed_template: &str = "<div id='youtube-div'><iframe id='youtube-iframe' src='https://www.youtube-nocookie.com/embed/{{video_id}}' title='YouTube video player' frameborder='0' allow='accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture' allowfullscreen></iframe></div>'";
    let video_id = youtube_key
        .strip_prefix("{{ youtube ")
        .unwrap()
        .strip_suffix(" }}")
        .unwrap();

    embed_template.replace("{{video_id}}", video_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn markdown_parse_test() {
    //     let output: Page = parse_markdown_file("content/example/example_short.md").unwrap();
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
    // <p><a href=\"/examples/example.html\">A bigger test</a></p>
    // <p><a href=\"/index.html\">Go home</a></p>
    // ";
    //     assert_eq!(output.content, answer);
    // }

    #[test]
    fn frontmatter_parsing_test() {
        let mut page: Page = Page::new();
        let frontmatter: &str = "---
title: example_title
description: example_description
category: example_category
date: example_date
";
        page.parse_frontmatter(frontmatter)
            .expect("[ ERROR ] Failed to parse frontatter!");

        assert_eq!(page.title, String::from("example_title"));
        assert_eq!(page.description, String::from("example_description"));
        assert_eq!(page.category, String::from("example_category"));
        assert_eq!(page.date, String::from("example_date"));
    }

    // #[test]
    // fn index_test() {
    //     file_to_html("content/index.md").expect("[ TEST ] Could not make index");
    //     assert!(File::open("site/index.html").is_ok());
    // }
    // #[test]
    // fn file_to_html_test() {
    //     crate::directory_handling::check_and_create_directory("site/examples")
    //         .expect("[ TEST ERR ] This directory could not be created.");
    //     file_to_html("content/example/example_short.md")
    //         .expect("[ TEST ERR ] This file could not be processed.");
    //     assert!(File::open("site/examples/example_short.html").is_ok());
    // }
    #[test]
    fn get_output_dir_test() {
        let output: String = get_output_dir("home");
        assert_eq!(output, String::from("/"));
        let output: String = get_output_dir("index");
        assert_eq!(output, String::from("/"));
        let output: String = get_output_dir("");
        assert_eq!(output, String::from("/"));
        let output: String = get_output_dir("test");
        assert_eq!(output, String::from("/test"));
    }

    #[test]
    fn get_filename_from_path_test() {
        let output: String = get_filename_from_path("site/example/index.md");
        assert_eq!(output, String::from("index"));
        let output: String = get_filename_from_path("site/index.md");
        assert_eq!(output, String::from("index"));
        let output: String = get_filename_from_path("site/testo.md");
        assert_eq!(output, String::from("testo"));
        let output: String = get_filename_from_path("site/testo.html");
        assert_ne!(output, String::from("testo"));
    }

    // #[test]
    // fn template_processing_test() {
    //     let page: Page = parse_markdown_file("content/example/example_short.md").unwrap();
    //     let template_path: &str = "template/boilerplate.html";
    //     let output = process_template(&page, template_path);
    //     let answer = "\
    // <!doctype html>
    // <html class='no-js' lang='en'>
    // <head>
    // <meta charset='utf-8'>
    // <title>A Short Example</title>
    // <meta name='description' content='static-generated site made with md_puppy'>
    // <meta name='viewport' content='width=device-width, initial-scale=1'>
    // <meta property='og:site_name' content='default_site_name'>
    // <meta property='og:title' content='A Short Example'>
    // <meta property='og:type' content='website'>
    // <meta property='og:url' content='https://softannalee.github.io/examples/A Short Example'>
    // <meta property='og:image' content='https://softannalee.github.io/examples/A Short Example/image.jpg'>
    // <link rel='stylesheet' href='/css/normalize.css'>
    // <link rel='stylesheet' href='/css/main.css'>
    // <base href='{{base_url}}'>
    // </head>
    // <body>
    // <ul>
    // <li><a href='/index.html'>Home</a></li>
    // <li><a href='/blog/index.html'>blog</a></li>
    // <li><a href='/examples/index.html'>examples</a></li>
    // </ul>

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
    // <p><a href=\"/examples/example.html\">A bigger test</a></p>
    // <p><a href=\"/index.html\">Go home</a></p>

    // </body>
    // </html>
    // ";
    //     assert_eq!(output.unwrap(), answer);
    // }

    // #[test]
    // fn template_replacement_test() {
    //     let page: Page = parse_markdown_file("content/example/example_short.md").unwrap();

    //     let input: &str = "This is {{title}}";
    //     let output = replace_placeholder(input, &page);
    //     assert_eq!(output.unwrap(), "This is A Short Example");

    //     let input: &str = "{{category}}: This is {{title}}";
    //     let output = replace_placeholder(input, &page);
    //     assert_eq!(output.unwrap(), "examples: This is A Short Example");

    //     let input: &str = "This is title";
    //     let output = replace_placeholder(input, &page);
    //     assert_eq!(output.unwrap(), "This is title");

    //     let input: &str = "This is {{tags}}";
    //     assert!(replace_placeholder(input, &page).is_err());
    // }

    // #[test]
    // fn get_value_test() {
    //     let page: Page = parse_markdown_file("content/example/example_short.md").unwrap();

    //     assert_eq!(get_value("{{title}}", &page).unwrap(), "A Short Example");
    //     assert_eq!(
    //         get_value("{{description}}", &page).unwrap(),
    //         "this is a short example of a markdown file"
    //     );
    //     assert_eq!(get_value("{{category}}", &page).unwrap(), "examples");
    //     assert_eq!(
    //         get_value("{{date}}", &page),
    //         Some("2022-01-17T12:34:11-0700".to_string())
    //     );
    //     assert_eq!(get_value("{{tags}}", &page), None);
    // }

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
