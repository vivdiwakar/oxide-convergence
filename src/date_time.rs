extern crate chrono;

use chrono::offset::Local;
use chrono::prelude::{DateTime, NaiveDate};

pub fn days_forward(end_date: String) -> i64 {
    let now: DateTime<Local> = Local::now();
    let today_naive: NaiveDate = now.date_naive();
    let end_date_naive: NaiveDate = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();
    return (end_date_naive - today_naive).num_days();
}
