

use chrono::{NaiveDate, Local};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
}