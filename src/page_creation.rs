use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;

use crate::directory_handling::check_and_create_directory;

pub fn create_page(filename: &str) -> Result<(), String> {
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
"
    );

    let mut outfile =
        File::create(output_filename).expect("[ ERROR ] Could not create output file!");

    outfile
        .write_all(content.as_bytes())
        .expect("[ ERROR ] Could not write to output file!");

    Ok(())
}

pub fn create_index_page(filename: &str, category: &str) -> Result<(), String> {
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
