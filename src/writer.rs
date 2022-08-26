use std::fs::File;

use std::io::prelude::*;

use crate::models::record::Record;
use crate::formatter::{format_total_shipment_time, format_layover_times, format_longest_delay};

fn write_total_shipment_time(records: &[Record]) -> String {
    format!("Total transit time: {}", format_total_shipment_time(records))
}

fn write_layover_times(records: &[Record]) -> String {
    let fmt_layovers = format_layover_times(records);
    let header = "Total layover times:\n".to_owned();
    let body = fmt_layovers.into_iter().collect::<String>();
    format!("{}{}", header, body)
}

fn write_longest_delay(records: &[Record]) -> String {
    format!("The longest delay occured:\n{}", format_longest_delay(records))
}

pub fn generate_file_report(records: &[Record], filename: String) {
    let header ="\n\t*All times formatted [hh:mm]\n\n".to_string(); 
    let ship_time = write_total_shipment_time(records);
    let layover = write_layover_times(records);
    let delay = write_longest_delay(records);

    let mut file = File::create(filename).unwrap();
    let contents = format!("{}\n{}\n{}\n{}", &header, &ship_time, &layover, &delay);

    file.write_all(contents.as_bytes()).expect("IO Error");
}

pub fn generate_report(records: &[Record]) {
    println!("\n\t*All times formatted [hh:mm]\n\n");
    println!("{}\n\n", write_total_shipment_time(records));
    println!("{}\n", write_layover_times(records));
    println!("{}\n", write_longest_delay(records));
}



#[cfg(test)]
mod tests {

    use crate::{reader::reader, builder::data_builder, models::input::Input};

    use super::*;

    #[test]
    fn test_file() {
        let inp = Input::new(&["target/debug/shiptracker".to_string(),
                           "tests/data/test_data.txt".to_string()]).unwrap();
        let reader = reader(inp).unwrap();
        let built_data = data_builder(reader).unwrap();
        let filename = "test.txt".to_string();

        generate_file_report(&built_data, filename);

        let file = File::open("test.txt");

        assert!(file.is_ok())
    }

}
