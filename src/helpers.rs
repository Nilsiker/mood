use chrono::{Local, NaiveDate};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}
