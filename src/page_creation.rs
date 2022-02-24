use anyhow::Result;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::directory_handling::check_and_create_directory;

pub fn create_page(input: &str) -> Result<()> {
    let mut filename = input;
    if let Some(s) = input.strip_suffix(".md") {
        filename = s;
    }
    if let Some(parent) = Path::new(filename).parent() {
        let dir = format!("content/{}", parent.display());
        check_and_create_directory(&dir).expect("[ERROR] Could not create parent directory");
    }
    let time: DateTime<Local> = Local::now();
    let timestamp: String = time.to_rfc3339();
    let output_filename: String = format!("content/{filename}.md");
    let content: String = format!(
        "\
---
title: Default Title
description: Default Description
category: draft
date: {timestamp}
---
# {{{{title}}}}

{{{{date}}}}

---
"
    );

    let mut outfile =
        File::create(&output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(content.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

    println!("[ INFO ] Page {output_filename} has been created!");

    Ok(())
}

pub fn create_index_page(filename: &str, category: &str) -> Result<()> {
    let time: DateTime<Local> = Local::now();
    let timestamp: String = time.to_rfc3339();
    let output_directory: String = format!("content/{category}/");
    let output_filename: String = format!("{output_directory}/{filename}.md");
    let content: String = format!(
        "\
---
title: {category} Index
description: Index for {category}
category: {category}
date: {timestamp}
---

{{{{ index {category} }}}}
"
    );

    check_and_create_directory(&output_directory)
        .expect("[ ERROR ] Could not create directory for index file!");

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create index file!");

    outfile
        .write_all(content.as_bytes())
        .expect("[ ERROR ] Could not write to index file!");

    Ok(())
}
