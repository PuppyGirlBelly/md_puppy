use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

use crate::markdown_compiling::{parse_markdown_file, Page};

pub fn file_to_html(file_path: &str, template_path: &str) -> Result<(), Box<dyn Error>> {
    let page: Page = parse_markdown_file(file_path).unwrap();
    let html_page = _process_template(page, template_path)?;
    let mut output_filename = String::from(&file_path[..file_path.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(html_page.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

    println!("[ INFO ] Parsing complete!");
    Ok(())
}

pub fn _process_template(page: Page, template_path: &str) -> Result<String, Box<dyn Error>> {
    let template: String =
        fs::read_to_string(template_path).expect("[ ERROR ] Failed to open html template!");
    let mut output: String = String::new();

    for line in template.lines() {
        output.push_str(&_replace_placeholder(line, &page)?);
        output.push('\n');
    }

    Ok(output)
}

fn _replace_placeholder(template: &str, page: &Page) -> Result<String, String> {
    if let Some(key) = _get_placeholder(template) {
        if let Some(v) = _get_value(key, page) {
            Ok(template.replace(key, &v))
        } else {
            Err(("Invalid key, ".to_owned() + key + " in template").to_string())
        }
    } else {
        Ok(template.to_string())
    }
}

fn _get_placeholder(template: &str) -> Option<&str> {
    let start_byte = template.find("{{").unwrap_or(0);
    let end_byte = template.find("}}").unwrap_or(0);

    if (start_byte == 0) && (end_byte == 0) {
        None
    } else {
        Some(&template[start_byte..(end_byte + 2)])
    }
}

fn _get_value(key: &str, page: &Page) -> Option<String> {
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
    fn template_processing_test() {
        let page: Page = parse_markdown_file("src/example_short.md").unwrap();
        let _template_path: &str = "src/boilerplate.html";
        let output = _process_template(page, _template_path);
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
  <meta property='og:url' content='https://softannalee.github.io/examples/{{title}}'>
  <meta property='og:image' content='https://softannalee.github.io/examples/{{title}}/image.jpg'>
  <link rel='stylesheet' href='css/normalize.css'>
  <link rel='stylesheet' href='css/main.css'>
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
        let page: Page = parse_markdown_file("src/example_short.md").unwrap();

        let input: &str = "This is {{title}}";
        let output = _replace_placeholder(input, &page);
        assert_eq!(output.unwrap(), "This is A Short Example");

        let input: &str = "This is title";
        let output = _replace_placeholder(input, &page);
        assert_eq!(output.unwrap(), "This is title");

        let input: &str = "This is {{tags}}";
        assert!(_replace_placeholder(input, &page).is_err());
    }

    #[test]
    fn get_value_test() {
        let page: Page = parse_markdown_file("src/example_short.md").unwrap();

        assert_eq!(_get_value("{{title}}", &page).unwrap(), "A Short Example");
        assert_eq!(
            _get_value("{{description}}", &page).unwrap(),
            "this is a short example of a markdown file"
        );
        assert_eq!(_get_value("{{category}}", &page).unwrap(), "examples");
        assert_eq!(
            _get_value("{{date}}", &page),
            Some("2022-01-17T12:34:11-0700".to_string())
        );
        assert_eq!(_get_value("{{tags}}", &page), None);
    }

    #[test]
    fn get_placeholder_test() {
        let example_text: &str = "{{title}}";
        let output: Option<&str> = _get_placeholder(example_text);

        assert_eq!(output.unwrap(), "{{title}}".to_string());

        let example_text: &str = "this is a title";
        let output: Option<&str> = _get_placeholder(example_text);

        assert_eq!(output, None);
    }
}
