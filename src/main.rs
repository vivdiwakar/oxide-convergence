extern crate getopts;

use std::env;
use getopts::{Options, Matches};
use chrono::NaiveDate;

mod date_time;
mod ingester;
mod simulator;

fn usage(program: &str, opts: Options) {
    let message: String = format!("Usage: {} -i input_file.csv -o output_file.csv", program);
    println!("{}", opts.usage(&message));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let full_path: String = args[0].clone();
    let path_elems: Vec<&str> = full_path.split("/").collect();
    let program: &str = path_elems[path_elems.len()-1];

    let mut opts: Options = Options::new();
    opts.optopt("i", "in_file", "Input file of historical price data, in CSV format.", "hist_data.csv");
    opts.optopt("o", "out_file", "File to output price forecast data.", "results.csv");
    opts.optopt("e", "end_date", "Target price date, in YYYY-MM-DD format.", "2019-09-18");
    opts.optopt("f", "date_format", "Date regular expression, with captures, used for interpreting and parsing dates.", "\"^\\d{4}-\\d{2}-\\d{2}.*$\"");
    opts.optopt("d", "date_column_index", "Zero-indexed column number containing dates.", "0");
    opts.optopt("p", "price_column_index", "Zero-indexed column number containing prices.", "1");
    opts.optflag("h", "help", "Print this help menu.");
    let matches: Matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!("{}", f.to_string()) }
    };

    if matches.opt_present("h") {
        usage(&program, opts);
        return;
    };

    let in_file: String = if matches.opt_str("i").is_some() {
        matches.opt_str("i").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let out_file: String = if matches.opt_str("o").is_some() {
        matches.opt_str("o").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let end_date: String = if matches.opt_str("e").is_some() {
        matches.opt_str("e").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let date_regex: String = if matches.opt_str("f").is_some() {
        matches.opt_str("f").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let date_column: String = if matches.opt_str("d").is_some() {
        matches.opt_str("d").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let price_column: String = if matches.opt_str("p").is_some() {
        matches.opt_str("p").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let days_to_sim: i64 = date_time::days_forward(&end_date);
    let hist_prices: Vec<(NaiveDate, f64)> = ingester::ingest_historical_data(in_file, &date_regex, &date_column, &price_column);
    simulator::run_monte_carlo_simulation(&end_date, days_to_sim, hist_prices);

    println!("Simulation complete, results in {}.", &out_file);
    return;
}
