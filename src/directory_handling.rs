use fs_extra::copy_items;
use fs_extra::dir::{get_dir_content, get_dir_content2, CopyOptions, DirOptions};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;

use crate::template_processing::file_to_html; //::{file_checker, markdown_to_html, usage, Config};

// use std::io::prelude::*;
// pub fn file_checker(filename: String) -> Result<(), Box<dyn Error>> {
//     let mut file = File::open(filename)?;
//     let mut buf: Vec<u8> = Vec::new();
//     file.read_to_end(&mut buf)?;
//     let _contents = String::from_utf8_lossy(&buf);

//     Ok(())
// }

pub fn process_content() -> Result<(), Box<dyn Error>> {
    let content_dir = get_dir_content("content/")?;

    for file in content_dir.files {
        file_to_html(&file, "template/boilerplate.html")?;
    }

    Ok(())
}

pub fn copy_static() -> Result<(), Box<dyn Error>> {
    let mut dir_options = DirOptions::new();
    dir_options.depth = 1;
    let static_dir = get_dir_content2("static/", &dir_options)?;
    let static_files: Vec<String> = static_dir.files;
    let static_chilren: Vec<String> = static_dir
        .directories
        .iter()
        .filter(|x| !x.ends_with("static/"))
        .map(|x| x.to_owned())
        .collect();

    let copy_options = CopyOptions {
        overwrite: true,
        skip_exist: false,
        buffer_size: 64000, //64kb
        copy_inside: true,
        content_only: false,
        depth: 0,
    };

    copy_items(&static_files, "site/", &copy_options)?;
    copy_items(&static_chilren, "site/", &copy_options)?;
    Ok(())
}

pub fn init_directories() -> Result<(), Box<dyn Error>> {
    _check_and_create_directory("content/")?;
    _check_and_create_directory("site/")?;
    _check_for_static_folder()?;
    _check_for_boilerplate()?;
    Ok(())
}

pub fn _check_and_create_directory(dir: &str) -> Result<(), Box<dyn Error>> {
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
fn _check_for_static_folder() -> Result<(), Box<dyn Error>> {
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

fn _check_for_boilerplate() -> Result<(), Box<dyn Error>> {
    if File::open("template/boilerplate.html").is_err() {
        _check_and_create_directory("template/")?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_static_test() {
        copy_static().expect("[ TEST ] Could not copy items from static");
        assert!(File::open("site/css/main.css").is_ok());
    }

    #[test]
    fn _check_for_boilerplate_test() {
        _check_for_boilerplate().unwrap();
        assert!(File::open("template/boilerplate.html").is_ok());
    }

    #[test]
    fn _check_for_static_folder_test() {
        _check_for_static_folder().unwrap();
        assert!(read_dir("static").is_ok());
    }

    #[test]
    fn create_directory_test() {
        assert!(_check_and_create_directory("site/").is_ok());
        assert!(_check_and_create_directory("site/examples").is_ok());
    }
}
