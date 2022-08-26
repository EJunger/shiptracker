use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

pub struct Input {
    pub filename: String,
    pub ext: String,
    pub file: File,
    pub tag: Option<String>,
}

impl Input {
    pub fn new(args: &[String]) -> Result<Input, &str> {
        if args.len() > 3 {
            return Err("Usage: shiptracker <FILE> <OP:FILENAME>\n\nInvalid Args: Only include the filename to be executed and an optional output filename with extension.")
        }
        if args.len() == 2 {
            let filename = Self::parse_args_noout(args);
            let raw_ext = Self::extract_file_extension(&filename);
            if raw_ext.is_none() {
                return Err("No file extension detected.")
            }

            let ext = raw_ext.unwrap().to_string();
            let file = Self::parse_file(&filename).unwrap();

            Ok(Input { filename, ext, file, tag: None })
        }

        else {
            let (filename, flag) = Self::parse_args_stout(args);
            let raw_ext = Self::extract_file_extension(&filename);
            if raw_ext.is_none() {
                return Err("No file extension detected.")
            }

            let tag = flag;
            let ext = raw_ext.unwrap().to_string();
            let file = Self::parse_file(&filename).unwrap();

            Ok(Input { filename, ext, file, tag: Some(tag) })
        }
    }

    fn parse_args_noout(args: &[String]) -> String {
        args[1].clone()
    }

    fn parse_args_stout(args: &[String]) -> (String, String) {
        let file = args[1].clone();
        let flag = args[2].clone();
        (file, flag)
    }

    fn extract_file_extension(filename: &str) -> Option<&str> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
    }

    fn parse_file(filename: &str) -> Result<File, std::io::Error> {
        let file = File::open(filename)?;
        Ok(file)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn good_input() {
        assert!(Input::new(&["target/debug/shiptracker".to_string(),
                           "tests/data/test_data.csv".to_string()]).is_ok());
    }

    #[test]
    #[should_panic]
    fn bad_input() {
        Input::new(&["some path".to_string(), "some_file.txt".to_string()]).unwrap();
    }

    // #[test]
    // fn try_args() {
    //     assert_eq!("tests/data/test_data.csv".to_string(),
    //                Input::parse_args(&["target/debug/shiptracker".to_string(),
    //                                  "tests/data/test_data.csv".to_string()]))
    // }

    #[test]
    fn try_ext() {
        assert_eq!("txt", Input::extract_file_extension("some_file.txt").unwrap())
    }

    #[test]
    fn test_file_open() {
        let file = Input::parse_file("tests/data/test_data.txt");
        assert!(file.is_ok());
    }
}
