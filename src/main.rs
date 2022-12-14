mod reader;
mod builder;
mod calc;
mod formatter;
mod writer;

mod models;

use std::env;
use std::process;

use reader::reader;
use builder::data_builder;
use writer::generate_file_report;
use writer::generate_report;

use crate::models::input::Input;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = Input::new(&args).unwrap_or_else(|err| {
        println!("Error parsing input: {}", err);
        process::exit(1);
    });

    if input.tag.is_some() {
        let filename = input.tag.clone().unwrap();
        let raw_string_data = reader(input).unwrap();
        let completed_records = data_builder(raw_string_data).unwrap();
        generate_file_report(&completed_records, filename)
    } else {
        let raw_string_data = reader(input).unwrap();
        let completed_records = data_builder(raw_string_data).unwrap();
        generate_report(&completed_records)
    }
}
