extern crate chrono;

use chrono::prelude::NaiveDate;

pub fn days_to_simulate(end_date: &String, last_price_date: NaiveDate) -> i64 {
    println!("{}, {}", &end_date, &last_price_date);
    let end_date_naive: NaiveDate = NaiveDate::parse_from_str(&end_date, "%Y-%m-%d").unwrap();
    return (end_date_naive - last_price_date).num_days() + 1;
}

pub fn get_date_from_strings(year: &str, month: &str, day: &str) -> NaiveDate{
    let rec_year: i32 = year.to_string().parse::<i32>().unwrap();
    let rec_month: u32 = month.to_string().parse::<u32>().unwrap();
    let rec_day: u32 = day.to_string().parse::<u32>().unwrap();
    let rec_date_naive: Option<NaiveDate> = NaiveDate::from_ymd_opt(rec_year, rec_month, rec_day);
    return rec_date_naive.unwrap();
}