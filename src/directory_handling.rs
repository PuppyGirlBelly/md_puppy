use fs_extra::copy_items;
use fs_extra::dir::{get_dir_content, get_dir_content2, CopyOptions, DirOptions};
use std::error::Error;
use std::fs::{create_dir_all, read_dir, File};
use std::io::Write;

use crate::page_creation::create_index_page;
use crate::site_data::Site;

pub fn process_content() -> Result<(), Box<dyn Error>> {
    let content_dir = get_dir_content("content/")?;
    let mut site: Site = Site::new();

    // for file in content_dir.files {}

    for file in content_dir.files {
        println!("[ INFO ] Processing {file}");
        site.add_page(&file)?;
    }

    let nav_links = site.create_category_links();
    let categories = site.categories.to_owned();

    for cat in categories {
        let cat_index: String = format!("/{cat}/index.html");
        if !site.directory.contains(&cat_index) {
            println!("[ INFO ] Creating index for {cat}");
            create_index_page("index", &cat)?;
            let new_page = format!("content/{cat}/index.md");
            site.add_page(&new_page)?;
        }
    }

    let pages = site.pages.to_vec();

    for mut page in pages {
        println!("[ INFO ] Writing {}", &page.filepath);
        let category = &page.category.to_owned();
        let index = site.create_category_index(category);
        page.replace_index(category, &index);
        page.replace_navbar(&nav_links);
        page.write_to_file()?;
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

    check_and_create_directory("site/").expect("[ ERROR ] Could not create site directory");
    copy_items(&static_files, "site/", &copy_options)?;
    copy_items(&static_chilren, "site/", &copy_options)?;
    Ok(())
}

pub fn init_directories() -> Result<(), Box<dyn Error>> {
    check_and_create_directory("content/")?;
    check_and_create_directory("site/")?;
    _check_for_static_folder()?;
    _check_for_boilerplate()?;
    Ok(())
}

pub fn check_and_create_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    if read_dir(dir).is_err() {
        create_dir_all(dir).expect("[ ERROR ] Could not create directory {dir}");
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
        assert!(check_and_create_directory("site/").is_ok());
        assert!(check_and_create_directory("site/examples").is_ok());
    }
}
