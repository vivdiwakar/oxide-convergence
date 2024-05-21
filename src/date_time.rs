extern crate chrono;

use chrono::{prelude::NaiveDate, Duration};

pub fn get_naive_date_from_string(date_string: &String) -> NaiveDate {
    return NaiveDate::parse_from_str(&date_string, "%Y-%m-%d").unwrap();
}

pub fn days_between(end_date: &String, last_price_date: NaiveDate) -> i64 {
    let end_date_naive: NaiveDate = get_naive_date_from_string(&end_date);
    return (end_date_naive - last_price_date).num_days();
}

pub fn get_date_from_strings(year: &str, month: &str, day: &str) -> NaiveDate{
    let rec_year: i32 = year.to_string().parse::<i32>().unwrap();
    let rec_month: u32 = month.to_string().parse::<u32>().unwrap();
    let rec_day: u32 = day.to_string().parse::<u32>().unwrap();
    let rec_date_naive: Option<NaiveDate> = NaiveDate::from_ymd_opt(rec_year, rec_month, rec_day);
    return rec_date_naive.unwrap();
}

pub fn add_days_to_date(latest_date: &NaiveDate, curr_day: &i64) -> NaiveDate {
    return latest_date.clone() + Duration::days(curr_day.clone());
}
