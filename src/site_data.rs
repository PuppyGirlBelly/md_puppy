use chrono::DateTime;
use std::{collections::HashSet, error::Error};

use crate::markdown_compiling::{get_filename_from_path, get_output_dir, Page};

pub struct Site {
    pub pages: Vec<Page>,
    pub categories: HashSet<String>,
    pub static_url: String,
    pub template_path: String,
}

impl Site {
    pub fn new() -> Site {
        Site {
            pages: Vec::new(),
            categories: HashSet::new(),
            static_url: String::from(
                "https://github.com/SoftAnnaLee/md_puppy/releases/download/static/static.zip",
            ),
            template_path: String::from("template/boilerplate.html"),
        }
    }

    pub fn add_page(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let page: Page = Page::from_file(filepath)?;
        let cat: &str = &page.category;
        let ignored_categories = ["home", "index", "draft", ""];

        if !ignored_categories.contains(&cat) {
            self.categories.insert(page.category.to_owned());
        }

        self.pages.push(page);

        Ok(())
    }

    pub fn create_category_links(&mut self) -> String {
        let mut output: String = String::from("<ul>\n<li><a href='/index.html'>Home</a></li>\n");

        let mut categories: Vec<String> = self.categories.iter().map(String::from).collect();
        categories.sort();

        for cat in categories {
            output.push_str(&format!("<li><a href='/{cat}/index.html'>{cat}</a></li>\n"));
        }

        output.push_str("</ul>\n");
        output
    }

    pub fn create_category_index(&mut self, category: &str) -> String {
        let mut output: String = String::from("<ul>\n");

        self.pages.sort_by(|a, b| b.cmp(a));

        for page in &self.pages {
            if page.category == category {
                let filename = get_filename_from_path(&page.filepath);
                let output_directory = get_output_dir(&page.category)
                    .strip_prefix("site")
                    .unwrap()
                    .to_string();
                let path = format!("{}{}.html", output_directory, &filename);
                let date = convert_datetime(&page.date);
                let title = &page.title;
                output.push_str(&format!("<li><a href='{path}'>{date} - {title}</a></li>\n",));
            }
        }

        output.push_str("</ul>\n");
        output
    }
}

fn convert_datetime(timestamp: &str) -> String {
    let datetime = DateTime::parse_from_rfc3339(timestamp).unwrap();
    datetime.format("%B %e, %Y | %l:%M %P").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_datetime_test() {
        let input = "2022-02-08T15:16:19-07:00".to_string();
        let output = "February  8, 2022 |  3:16 pm".to_string();
        assert_eq!(convert_datetime(input), output);
    }
}
