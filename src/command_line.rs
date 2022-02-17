pub fn usage() {
    print_long_banner();
    println!(
        "Usage:
    init - Download and initalize directories needed for website
    build - Process all files in the content/ folder and parse into a website
    new <filepath> - Create a new file within the content/ folder with default frontmatter"
    );
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(&String::from(env!("CARGO_PKG_VERSION"))[..]);
    the_title.push_str(") ");
    the_title.push_str(&String::from(env!("CARGO_PKG_DESCRIPTION"))[..]);
    the_title
}

pub fn print_short_banner() {
    println!("{}", get_title());
}

fn print_long_banner() {
    println!(
        "Written by: {}\nHomepage: {}\nUsage: puppy_md <command>\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn input_handler_test() {}
// }
