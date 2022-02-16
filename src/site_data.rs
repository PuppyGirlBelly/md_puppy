use std::fs;
use std::path::Path;
use std::{collections::HashSet, error::Error};

use chrono::DateTime;
use yaml_rust::YamlLoader;

use crate::markdown_compiling::Page;

pub struct Site {
    pub pages: Vec<Page>,
    pub directory: Vec<String>,
    pub categories: HashSet<String>,
    pub site_name: String,
    pub static_url: String,
    pub template_url: String,
    pub template_path: String,
    pub dithering: bool,
    pub base_url: String,
}

impl Site {
    pub fn new() -> Result<Site, Box<dyn Error>> {
        let mut site = Site {
            pages: Vec::new(),
            directory: Vec::new(),
            categories: HashSet::new(),
            site_name: String::from("md_puppy site"),
            static_url: String::from(
                "https://github.com/SoftAnnaLee/md_puppy/releases/download/static/static.zip",
            ),
            template_url: String::from("https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/template/boilerplate.html"),
            template_path: String::from("template/boilerplate.html"),
            dithering: false,
            base_url: String::from("https://www.example.com"),
        };

        site.parse_config()?;

        Ok(site)
    }

    fn parse_config(&mut self) -> Result<(), String> {
        let path: &Path = Path::new("config.yaml");
        let file: String = fs::read_to_string(path).expect("[ ERROR ] Failed to open file!");
        let yaml = YamlLoader::load_from_str(&file);

        match yaml {
            Err(_) => Err("[ ERROR ] Config file is missing or corrupt".to_string()),
            Ok(y) => {
                let fm = &y[0];

                self.site_name = fm["title"].as_str().unwrap_or("md_puppy site").to_string();
                self.static_url = fm["static_url"]
                    .as_str()
                    .unwrap_or("https://github.com/SoftAnnaLee/md_puppy/releases/download/static/static.zip")
                    .to_string();
                self.template_url = fm["template_url"]
                    .as_str()
                    .unwrap_or("https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/template/boilerplate.html")
                    .to_string();
                self.dithering = fm["dithering"].as_bool().unwrap_or(false);
                self.base_url = fm["base_url"]
                    .as_str()
                    .unwrap_or("https://www.example.com")
                    .to_string();

                Ok(())
            }
        }
    }

    pub fn add_page(&mut self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let page: Page = Page::from_file(filepath)?;
        let cat: &str = &page.category.to_lowercase();
        let path = format!("{}{}.html", page.output_path, page.filename);

        let ignored_categories = ["home", "index", "draft", ""];
        if !ignored_categories.contains(&cat) {
            self.categories.insert(cat.to_string());
        }

        if cat != "draft" {
            self.pages.push(page);
            self.directory.push(path);
        }

        Ok(())
    }

    pub fn create_category_links(&mut self) -> String {
        let mut output: String =
            String::from("<nav>\n<ul>\n<li><a href='/index.html'>Home</a></li>\n");

        let mut categories: Vec<String> = self.categories.iter().map(String::from).collect();
        categories.sort();

        for cat in categories {
            output.push_str(&format!("<li><a href='/{cat}/index.html'>{cat}</a></li>\n"));
        }

        output.push_str("</ul>\n</nav>\n");
        output
    }

    pub fn create_category_index(&mut self, category: &str) -> String {
        let mut output: String = String::from("<ul>\n");

        self.pages.sort_by(|a, b| b.cmp(a));

        for page in &self.pages {
            if page.category == category && !page.filepath.ends_with("index.md") {
                let path = format!("{}{}.html", &page.output_path, &page.filename);
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
        let input = "2022-02-08T15:16:19-07:00";
        let output = "February  8, 2022 |  3:16 pm".to_string();
        assert_eq!(convert_datetime(input), output);
    }
}
