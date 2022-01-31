use std::error::Error;
use std::fs::{create_dir_all, read_dir};

pub fn _check_and_create_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    if read_dir(dir).is_err() {
        create_dir_all(dir)?;
        Ok(())
    } else {
        Ok(())
    }
}

pub fn get_output_dir(filename: String, category: &str) -> String {
    match category.to_lowercase().as_str() {
        "home" | "index" | "" => format!("site/{}.html", filename),
        cat => format!("site/{}/{}.html", cat, filename),
    }
}

fn _check_for_static_folder() -> Result<(), Box<dyn Error>> {
    if read_dir("static/").is_err() {
        Ok(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_output_dir_test() {
        let output: String = get_output_dir("test".to_string(), "home");
        assert_eq!(output, String::from("site/test.html"));
        let output: String = get_output_dir("test".to_string(), "index");
        assert_eq!(output, String::from("site/test.html"));
        let output: String = get_output_dir("test".to_string(), "");
        assert_eq!(output, String::from("site/test.html"));
        let output: String = get_output_dir("test".to_string(), "example");
        assert_eq!(output, String::from("site/example/test.html"));
    }

    #[test]
    fn create_directory_test() {
        assert!(_check_and_create_directory("site/").is_ok());
        assert!(_check_and_create_directory("site/example").is_ok());
    }
}
