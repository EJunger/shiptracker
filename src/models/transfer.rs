use std::error::Error;

use crate::models::record::Record;

#[derive(Debug, PartialEq, Clone)]
pub struct Transfer {
    pub from: Record,
    pub to: Record,
    pub minutes: i64,
}

impl Transfer {
    pub fn new(from: Record, to: Record, minutes: i64) -> Result<Transfer, Box<dyn Error>> {
        Ok(Transfer { from, to, minutes})
    }
}

#[cfg(test)]
mod test {

    use super::*;

    use celes::Country;
    use chrono::NaiveDateTime;

    #[test]
    fn good_input() {
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
        assert!(Transfer::new(recs[0].clone(), recs[1].clone(), 69).is_ok());
    }
}
