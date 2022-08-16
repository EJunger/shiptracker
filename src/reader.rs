use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{BufRead, BufReader};
use csv::Reader;

use crate::models::input::Input;

fn build_csv_buffer_reader(filename: &str) -> Result<Reader<File>, Box<dyn Error>> {
    let path = Path::new(filename);
    Ok(Reader::from_path(path).unwrap())
}

fn parse_raw_csv_data(mut buf: Reader<File>) -> Result<Vec<String>, String> {
    let data: Vec<String> = buf.records()
        .map(|x| x.unwrap_or_else(|_| Err("!!Error parsing line!!").unwrap())
                    .as_slice()
                    .to_owned())
        .collect();

    Ok(data)
}

fn build_text_buffer_reader(file: File) -> Result<BufReader<File>, Box<dyn Error>> {
    Ok(BufReader::new(file))
}

fn parse_raw_text_data(buf: BufReader<File>) -> Result<Vec<String>, String> {
    let data: Vec<String> = buf.lines()
        .skip(1)
        .filter_map(|x| x.unwrap_or_else(|_| Err("!!Error parsing line!!").unwrap())
                    .parse::<String>()
                    .ok())
        .collect();

    Ok(data)
}

pub fn reader(inp: Input) -> Result<Vec<String>, String> {
    match inp.ext.as_str() {
        "txt" => {
            let buffer_reader = build_text_buffer_reader(inp.file).unwrap();
            let string_data = parse_raw_text_data(buffer_reader)
                .unwrap_or_else(|err| Err(format!("Error parsing data from: {} - {}", inp.filename, err))
                                .unwrap());

            Ok(string_data)
        },
        "csv" => {
            let buffer_reader = build_csv_buffer_reader(&inp.filename).unwrap();
            let string_data = parse_raw_csv_data(buffer_reader)
                .unwrap_or_else(|err| Err(format!("Error parsing data from: {} - {}", inp.filename, err))
                                .unwrap());

            Ok(string_data)
        },
        _ => Err("!!Shiptracker can only parse '.txt' & '.csv' files!!".to_string())
    }
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_text_buffer() {
        let file = File::open("tests/data/test_data.txt").unwrap();
        assert!(build_text_buffer_reader(file).is_ok());
    }

    #[test]
    fn build_csv_buffer() {
        assert!(build_csv_buffer_reader("tests/data/test_data.csv").is_ok());
    }
}
