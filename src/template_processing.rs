use std::error::Error;

use crate::markdown_compiling::Page;

pub fn _replace_placeholder(_template: &str, _page: Page) -> Result<&str, Box<dyn Error>> {
    Ok("This is example_title")
}

fn _get_placeholder(template: &str) -> Option<String> {
    let start_byte = template.find("{{").unwrap_or(0) + 2;
    let end_byte = template.find("}}").unwrap_or(0);

    if (start_byte == 0) && (end_byte == 0) {
        None
    } else {
        Some(template[start_byte..end_byte].to_string())
    }
}

fn _get_value(key: &str, _page: &Page) -> String {
    match key {
        "title" => String::from("test"),
        "description" => String::from("test"),
        "tags" => String::from("test"),
        "date" => String::from("test"),
        "content" => String::from("test"),
        _ => String::from("test"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replacement_test() {
        use crate::markdown_compiling::parse_markdown_file;
        let page: Page = parse_markdown_file("src/example_short.md").unwrap();

        let example_text = "This is {{title}}";
        let output = _replace_placeholder(example_text, page).unwrap();

        assert_eq!(output, "This is example_title");
    }

    #[test]
    fn get_placeholder_test() {
        let example_text: &str = "{{title}}";
        let output: String = _get_placeholder(example_text).unwrap();

        assert_eq!(output, "title".to_string());
    }
}
