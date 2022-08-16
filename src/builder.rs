use std::error::Error;
use regex::{Regex, RegexSet};
use chrono::NaiveDateTime;
use celes::Country;
use std::str::FromStr;

use crate::models::record::Record;

fn parse_raw_records(data: Vec<String>) -> Result<Vec<Record>, Box<dyn Error>> {
    let pattern_set = RegexSet::new(&[
        r"(\d{4})-(\d{2})-(\d{2})",
        r"(\d{2}):(\d{2}):(\d{2})",
        r"([a-zA-Z].*$)",
    ]).unwrap();

    let regexes: Vec<Regex> = pattern_set.patterns().iter()
        .map(|pattern| Regex::new(pattern).unwrap())
        .collect();

    let mut records: Vec<Record> = Vec::new();

    for line in data {
        let matches: Vec<&str> = pattern_set.matches(&line).into_iter()
            .map(|match_index| &regexes[match_index])
            .map(|pattern| pattern.find(&line).unwrap().as_str())
            .collect();
        let (date, time, status) = (matches[0].to_string(), matches[1].to_string(), matches[2].to_string());
        let datetime = parse_naive_datetime(format!("{} {}", date, time).as_str()).unwrap();
        let record = Record::new(datetime, status).unwrap();
        records.push(record);
    }

    records.sort_by_key(|x| x.datetime);

    Ok(records)
}

fn parse_naive_datetime(datetime_str: &str) -> Result<NaiveDateTime, chrono::ParseError> {
    NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S")
}

fn parse_raw_locale(status_str: &str) -> Option<String> {
    let delimiters = &['.', ',', '-', ';', ' '];
    let mut reverse_split: Vec<&str> = status_str.rsplitn(3, delimiters).collect();

    reverse_split.retain(|x| !x.is_empty());

    let raw_locale: Option<String> = match Country::from_str(reverse_split[0]) {
        Ok(x) => Some(x.to_string()),
        Err(..) => None
    };

    raw_locale
}

fn set_locales(records: Vec<Record>) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut locale_records = Vec::new();

    for mut rec in records {
        let new_locale = parse_raw_locale(&rec.status);
        if new_locale.is_some() {
            rec.set_locale(new_locale);
            locale_records.push(rec)
        } else {
            rec.set_locale(None);
            locale_records.push(rec);
        }
    }

    Ok(locale_records)
}

fn set_null_locales(records: Vec<Record>) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut curr_locale = String::new();
    let mut ammended_locales = Vec::new();

    for mut rec in records {
        match rec.locale {
            Some(..) => {
                curr_locale = rec.locale.as_ref().unwrap().to_owned();
                ammended_locales.push(rec.to_owned());
            },
            None => {
                rec.set_locale(Some(curr_locale.to_owned()));
                ammended_locales.push(rec);
            }
        }
    }

    Ok(ammended_locales)
}

pub fn data_builder(raw_data: Vec<String>) -> Result<Vec<Record>, Box<dyn Error>> {
    let raw_records = parse_raw_records(raw_data).unwrap();
    let base_locales = set_locales(raw_records).unwrap();
    let filled_locales = set_null_locales(base_locales).unwrap();

    Ok(filled_locales)
}



#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_datetime() {
        assert!(parse_naive_datetime("2017-01-23 16:02:24").is_ok())
    }

    #[test]
    fn get_raw_locale() {
        assert!(parse_raw_locale("Package moved to customs office; Maimi, US").is_some())
    }

    #[test]
    fn raw_records() {
        let str_recs = vec![
            "2017-01-23, 16:02:24, Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
            "2017-01-23, 14:48:08, Processed at CINCINNATI HUB,OH-USA".to_string()
        ];
        assert!(parse_raw_records(str_recs).is_ok())
    }

    #[test]
    fn set_empty_locale() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: None
            }
        ];
        assert!(set_locales(recs).is_ok())
    }

    #[test]
    fn set_nulls() {
        let recs = vec![
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Departed Facility in CINCINNATI HUB,OH-USA".to_string(),
                locale: Some(Country::the_united_states_of_america().to_string()),
            },
            Record {
                datetime: NaiveDateTime::parse_from_str("2017-01-24 18:10:36", "%Y-%m-%d %H:%M:%S").unwrap(),
                status: "Customs status updated;".to_string(),
                locale: None,
            },
        ];
        let new_recs = set_null_locales(recs).unwrap();
        assert_eq!(new_recs[0].locale, new_recs[1].locale)
    }
}
