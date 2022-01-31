use reqwest;
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use tempfile;
use zip;

use std::io::Write;

pub fn create_folders() -> Result<(), Box<dyn Error>> {
    check_and_create_directory("content/")?;
    check_and_create_directory("site/")?;
    check_for_static_folder()?;
    check_for_boilerplate()?;
    Ok(())
}

pub fn check_and_create_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    if read_dir(dir).is_err() {
        create_dir_all(dir)?;
        Ok(())
    } else {
        Ok(())
    }
}

/* The code that was used to figure out how to download and unzip a file was taken from this stack
* overflow answer;
* https://stackoverflow.com/a/50471953 */
fn check_for_static_folder() -> Result<(), Box<dyn Error>> {
    if read_dir("static/").is_err() {
        let url = "https://github.com/SoftAnnaLee/md_puppy/releases/download/static/static.zip";
        let mut tmpfile = tempfile::tempfile()?;
        reqwest::blocking::get(url).unwrap().copy_to(&mut tmpfile)?;
        let mut zip = zip::ZipArchive::new(tmpfile)?;
        zip.extract("static/")?;
        Ok(())
    } else {
        Ok(())
    }
}

fn check_for_boilerplate() -> Result<(), Box<dyn Error>> {
    if File::open("template/boilerplate.html").is_err() {
        check_and_create_directory("template/")?;
        let url =
            "https://raw.githubusercontent.com/SoftAnnaLee/md_puppy/main/template/boilerplate.html";
        let download = reqwest::blocking::get(url).unwrap().bytes()?;
        let mut file = File::create("template/boilerplate.html")?;
        file.write_all(&download)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _check_for_boilerplate_test() {
        check_for_boilerplate().unwrap();
        assert!(File::open("template/boilerplate.html").is_ok());
    }

    #[test]
    fn _check_for_static_folder_test() {
        check_for_static_folder().unwrap();
        assert!(read_dir("static").is_ok());
    }

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
        assert!(check_and_create_directory("site/").is_ok());
        assert!(check_and_create_directory("site/example").is_ok());
    }
}
