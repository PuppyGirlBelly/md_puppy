use std::error::Error;
use std::fs;

pub struct Input {
    pub filename: String,
    pub site_name: String,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        match args.len() {
            arg if arg < 2 => Err("Not enough arguments"),
            arg if arg > 2 => Err("Too many arguments"),
            _ => {
                let filename = args[0].clone();
                let site_name = String::from("md_puppy");
                Ok(Input {
                    filename,
                    site_name,
                })
            }
        }
    }
}

pub fn file_checker(config: Input) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn usage() {
    print_long_banner();
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
    print_short_banner();
    println!(
        "Written by: {}\nHomepage: {}\nUsage: puppy_md build\n",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_test() {
        let input = [String::from("test"), String::from("src/boilerplate.html")];
        let test_config: Input = Input::new(&input).unwrap();

        assert_eq!(test_config.filename, "test");
    }

    #[test]
    // #[should_panic]
    fn config_fail_test() {
        let input = [];
        assert!(Input::new(&input).is_err());
        let input = [
            String::from("test"),
            String::from("test"),
            String::from("src/boilerplate.html"),
        ];
        assert!(Input::new(&input).is_err());
    }

    #[test]
    fn file_checker_test() {
        let input = [
            String::from("src/example_short.md"),
            String::from("src/boilerplate.html"),
        ];
        let test_config: Input = Input::new(&input).unwrap();
        assert!(file_checker(test_config).is_ok());

        let input = [
            String::from("src/not_an_example.md"),
            String::from("src/boilerplate.html"),
        ];
        let test_config: Input = Input::new(&input).unwrap();
        assert!(file_checker(test_config).is_err());
    }
}
