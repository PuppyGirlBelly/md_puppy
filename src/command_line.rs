use std::error::Error;
use std::fs;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        match args.len() {
            arg if arg < 1 => Err("Not enough arguments"),
            arg if arg > 1 => Err("Too many arguments"),
            _ => {
                let filename = args[0].clone();
                Ok(Config { filename })
            }
        }
    }
}

pub fn file_checker(config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;

    Ok(())
}

pub fn usage() {
    print_long_banner();
}

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));

    the_title.push_str(" (v: ");
    the_title.push_str(&get_version()[..]);
    the_title.push_str(") ");
    the_title.push_str(&get_description()[..]);

    the_title
}

fn get_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
}

fn get_description() -> String {
    String::from(env!("CARGO_PKG_DESCRIPTION"))
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
        let input = [String::from("test")];
        let test_config: Config = Config::new(&input).unwrap();

        assert_eq!(test_config.filename, "test");
    }

    #[test]
    // #[should_panic]
    fn config_fail_test() {
        let input = [];
        assert!(Config::new(&input).is_err());
        let input = [String::from("test"), String::from("test")];
        assert!(Config::new(&input).is_err());
    }

    #[test]
    fn file_checker_test() {
        let input = [String::from("src/example_short.md")];
        let test_config: Config = Config::new(&input).unwrap();
        assert!(file_checker(test_config).is_ok());

        let input = [String::from("src/not_an_example.md")];
        let test_config: Config = Config::new(&input).unwrap();
        assert!(file_checker(test_config).is_err());
    }

    #[test]
    fn banner_tests() {
        assert_eq!(get_version(), String::from("0.1.0"));
        assert_eq!(
            get_description(),
            String::from("A personal static site generator writeen by AnnaLee")
        );
        assert_eq!(
            get_title(),
            String::from("md_puppy (v: 0.1.0) A personal static site generator writeen by AnnaLee")
        );
    }
}
