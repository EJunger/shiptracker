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

pub fn generate_report(records: &[Record]) {
    println!("\n\t*All times formatted [hh:mm]\n\n");
    println!("{}\n\n", write_total_shipment_time(records));
    println!("{}\n", write_layover_times(records));
    println!("{}\n", write_longest_delay(records));
}
