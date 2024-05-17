extern crate regex;

use regex::Regex;

pub fn ingest_historical_data(in_file: String, date_regex: String, date_column: String, price_column: String) {
    let re: Regex = Regex::new(date_regex.as_str()).unwrap();
}