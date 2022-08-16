use chrono::NaiveDateTime;

#[derive(Debug, PartialEq, Clone)]
pub struct Record {
    pub datetime: NaiveDateTime,
    pub status: String,
    pub locale: Option<String>,
}

impl Record {
    pub fn new(datetime: NaiveDateTime, status: String) -> Result<Record, String> {
        let locale: Option<String> = None;

        Ok(Record { datetime, status, locale })
    }

    pub fn set_locale(&mut self, new_locale: Option<String>) {
        self.locale = new_locale
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn good_input() {
        let dt = NaiveDateTime::parse_from_str("2017-01-23 16:02:24", "%Y-%m-%d %H:%M:%S").unwrap();
        assert!(Record::new(dt, "Package arrived at destination".to_string()).is_ok())
    }
}
