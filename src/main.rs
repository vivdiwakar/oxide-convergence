extern crate getopts;

mod date_time;
mod disk_io;
mod simulator;
mod oxide_stats;

use std::env;
use getopts::{Options, Matches};
use chrono::NaiveDate;
use std::process;
use num_format::{Locale, ToFormattedString};

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
    opts.optopt("f", "date_format", "Date regular expression, with captures, used for interpreting and parsing dates.", "\"^(\\d{4})-(\\d{2})-(\\d{2}).*$\"");
    opts.optopt("d", "date_column_index", "Zero-indexed column number containing dates.", "0");
    opts.optopt("p", "price_column_index", "Zero-indexed column number containing prices.", "1");
    opts.optopt("s", "sims_per_day", "Number of simulations to run per day.", "5000");
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

    let sims_per_day: String = if matches.opt_str("s").is_some() {
        matches.opt_str("s").clone().unwrap()
    } else {
        usage(&program, opts);
        return;
    };

    let hist_prices: Vec<(NaiveDate, f64)> = disk_io::ingest_historical_data(in_file, &date_regex, &date_column, &price_column);
    let latest_date: NaiveDate = hist_prices[hist_prices.len()-1].0;
    let target_date: NaiveDate = date_time::get_naive_date_from_string(&end_date);

    if &target_date < &latest_date {
        println!("Target date falls within available historical data, nothing to forecast; exiting.");
        process::exit(2);
    }

    let num_sims: &i64 = &sims_per_day.parse::<i64>().unwrap();
    let (latest_date, latest_price, days_to_sim, mean, min, max, var_p, stdev_p, drift) = 
        simulator::setup_historical_data(&end_date, &hist_prices);
    let results: Vec<(NaiveDate, f64, f64, f64, f64, f64)> = 
        simulator::run_simulation(1, &latest_date, &days_to_sim, &num_sims, &latest_price, &stdev_p, &drift);
    let _ = disk_io::write_results_to_file(&results, &out_file);

    println!("\nStatistics calculated for historical data ...");
    println!("    Total records ingested: {}", &hist_prices.len());
    println!("    Average Periodic Daily Return : {}", mean);
    println!("    Minimun Periodic Daily Return: {}", min);
    println!("    Maximum Periodic Daily Return: {}", max);
    println!("    Variance: {}", var_p);
    println!("    Std Deviation: {}", stdev_p);
    println!("    Drift: {}", drift);
    println!("\nStarting price simulation to {} ({} days, {} simulations per day) ...", end_date, &days_to_sim, &num_sims.to_formatted_string(&Locale::en));
    println!("    Latest price date: {}", latest_date);
    println!("    Latest price (USD): {}", latest_price);
    println!("    Simulation complete! {} price points generated in total", (num_sims.clone() as i64 * days_to_sim).to_formatted_string(&Locale::en));
    println!("\nSimulation Results:");
    println!("    Expected price on {}: {}", &end_date, &results[&results.len() - 1].1);
    println!("\nGranular Results:");
    println!("    Granular results available in file '{}'", &out_file);
}
