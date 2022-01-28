use std::error::Error;
use std::fs::{create_dir_all, read_dir};

fn _check_and_create_directory(dir: &str) -> Result<(), Box<dyn Error>> {
    if read_dir(&dir).is_err() {
        create_dir_all(dir)?;
        Ok(())
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_directory_test() {
        assert!(_check_and_create_directory("site/").is_ok());
    }
}
