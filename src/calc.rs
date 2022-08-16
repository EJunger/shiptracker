use std::error::Error;
use chrono::NaiveDateTime;
use itertools::Itertools;

use crate::models::{record::Record, transfer::Transfer};

fn calc_transfers(records: &[Record]) -> Result<Vec<Transfer>, Box<dyn Error>> {
    let transfers: Vec<Transfer> = records.windows(2)
        .map(|pair| {
            let mins = diff_naive_datetime(&pair[0].datetime, &pair[1].datetime);
            let transfer = Transfer::new(pair[0].clone(), pair[1].clone(), mins).unwrap();
            transfer
        })
        .collect();

    Ok(transfers)
}

fn longest_transfer(transfers: Vec<Transfer>) -> Transfer {
    transfers.into_iter().max_by_key(|x| x.minutes).unwrap()
}

fn diff_naive_datetime(start: &NaiveDateTime, end: &NaiveDateTime) -> i64 {
    let diff: chrono::Duration = *end - *start;
    diff.num_minutes()
}

fn generate_locales_list(records: &[Record]) -> Vec<String> {
    let mut locales = Vec::new();

    for rec in records {
        locales.push(rec.locale.clone().unwrap())
    }

    locales.into_iter().unique().collect()
}

fn filter_layover_time(records: &[Record], search_locale: &str) -> Result<Transfer, String> {
    let filter_records: Vec<Record> = records.iter().cloned()
        .filter(|x| x.locale.as_ref().unwrap() == search_locale)
        .collect();

    if filter_records.is_empty() {
        return Err("No records found for the given locale.".to_owned());
    }

    let from = filter_records[0].clone();
    let to = filter_records.last().unwrap().to_owned();
    let mins = diff_naive_datetime(&from.datetime, &to.datetime);
    let transfer = Transfer::new(from, to, mins).unwrap();

    Ok(transfer)
}

pub fn calc_total_shipment_time(records: &[Record]) -> Result<i64, Box<dyn Error>> {
    Ok(diff_naive_datetime(&records[0].datetime, &records.last().unwrap().datetime))
}

pub fn calc_layover_times(records: &[Record]) -> Result<Vec<Transfer>, Box<dyn Error>> {
    let search_locales = generate_locales_list(records);
    let mut layovers = Vec::new();

    for local in search_locales {
        let tranfer = filter_layover_time(records, &local).unwrap();
        layovers.push(tranfer);
    }

    Ok(layovers)
}

pub fn calc_longest_delay(records: &[Record]) -> Result<Transfer, Box<dyn Error>> {
    let transfers = calc_transfers(records).unwrap();
    Ok(longest_transfer(transfers))
}



#[cfg(test)]
mod test {

    use super::*;

    use celes::Country;
    use chrono::NaiveDateTime;

    #[test]
    fn test_diff_naive_datetime() {
        let dt_1 = NaiveDateTime::parse_from_str("2017-01-22 15:23:58", "%Y-%m-%d %H:%M:%S").unwrap();
        let dt_2 = NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap();
        let sum = diff_naive_datetime(&dt_1, &dt_2);
        assert_eq!(sum, 1478);
    }

    #[test]
    fn test_transfers() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-24 18:10:36", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Customs status updated;".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
        ];
        let transfers = calc_transfers(&recs);
        assert!(transfers.is_ok());
    }

    #[test]
    fn gen_locales_vec() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-24 18:10:36", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Customs status updated;".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
        ];
        let locales = generate_locales_list(&recs);
        assert_eq!(locales[0], Country::the_united_states_of_america().to_string())
    }

    #[test]
    fn filter_for_good_locale() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-24 18:10:36", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Customs status updated;".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
        ];
        let search_locale = Country::the_united_states_of_america().to_string();
        assert!(filter_layover_time(&recs, &search_locale).is_ok())
    }

    #[test]
    #[should_panic]
    fn filter_for_bad_locale() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-24 18:10:36", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Customs status updated;".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
        ];
        let search_locale = Country::canada().to_string();
        filter_layover_time(&recs, &search_locale).unwrap();
    }
}
