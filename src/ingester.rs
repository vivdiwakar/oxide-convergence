extern crate csv;
extern crate regex;

// use crate::date_time;

use std::fs::File;
// use chrono::NaiveDate;
use csv::{Reader, ReaderBuilder};
// use regex::{Regex, Captures};
use regex::Regex;

pub fn ingest_historical_data(in_file: String, date_regex: &str, date_column: String, price_column: String) -> Vec<f64> {
    let date_col: usize = date_column.parse().unwrap();
    let price_col: usize = price_column.parse().unwrap();
    let mut prices: Vec<f64> = Vec::new();

    let file: File = match File::open(in_file) {
        Err(error) => panic!("couldn't open {}", error),
        Ok(file) => file
    };

    let mut rdr: Reader<_> = ReaderBuilder::new().from_reader(file);
    for result in rdr.records() {
        match result {
            Err(_error) => (),
            Ok(record) => {
                let re: Regex = Regex::new(date_regex).unwrap();
                let rec_date_str: &str = record.get(date_col).unwrap();
                if re.is_match(rec_date_str) {
                    // let date_caps: Captures = re.captures(rec_date_str).unwrap();
                    // let rec_date: NaiveDate = date_time::get_date_from_strings(&date_caps[1], &date_caps[2], &date_caps[3]);
                    let rec_price: f64  = record.get(price_col).unwrap().parse().unwrap();
                    prices.push(rec_price);
                }
            }
        };
    }

    return prices;
}
