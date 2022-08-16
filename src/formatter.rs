use crate::models::record::Record;
use crate::calc::{calc_total_shipment_time, calc_layover_times, calc_longest_delay};

pub fn format_total_shipment_time(records: &[Record]) -> String {
    let total_mins = calc_total_shipment_time(records).unwrap();
    format!("[{}:{}]", &total_mins/60, &total_mins%60)
}

pub fn format_layover_times(records: &[Record]) -> Vec<String> {
    let layover_times = calc_layover_times(records).unwrap();
    let mut fmt_times = Vec::new();

    for layover in layover_times {
        let fmt_str = format!("{}: [{}:{}]\n", &layover.to.locale.unwrap(), &layover.minutes/60, &layover.minutes%60);
        fmt_times.push(fmt_str);
    }

    fmt_times
}

pub fn format_longest_delay(records: &[Record]) -> String {
    let transfer = calc_longest_delay(records).unwrap();
    format!(
        "From: {} - {}\nTo: {} - {}\nDuration: [{}:{}]",
        &transfer.from.locale.unwrap(),
        &transfer.from.status,
        &transfer.to.locale.unwrap(),
        &transfer.to.status,
        &transfer.minutes/60,
        &transfer.minutes%60,
        )
}
