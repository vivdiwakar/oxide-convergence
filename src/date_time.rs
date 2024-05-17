extern crate chrono;

use chrono::offset::Local;
use chrono::prelude::{DateTime, NaiveDate};

pub fn days_forward(end_date: String) -> i64 {
    let now: DateTime<Local> = Local::now();
    let today_naive: NaiveDate = now.date_naive();
    let end_date_naive: NaiveDate = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();
    return (end_date_naive - today_naive).num_days();
}

pub fn get_date_from_strings(year: &str, month: &str, day: &str) -> NaiveDate{
    let rec_year: i32 = year.to_string().parse::<i32>().unwrap();
    let rec_month: u32 = month.to_string().parse::<u32>().unwrap();
    let rec_day: u32 = day.to_string().parse::<u32>().unwrap();
    let rec_date_naive: Option<NaiveDate> = NaiveDate::from_ymd_opt(rec_year, rec_month, rec_day);
    return rec_date_naive.unwrap();
}