use chrono::NaiveDate;
use statrs::statistics::Statistics;
use statrs::distribution::{ContinuousCDF, Normal};
use rand::prelude::*;

use crate::date_time;

pub fn run_monte_carlo_simulation(end_date: &String, hist_data: Vec<(NaiveDate, f64)>, sims_per_day: &String) {
    let num_sims: &i32 = &sims_per_day.parse::<i32>().unwrap();
    let mut periodic_daily_returns: Vec<f64> = Vec::new();
    for res in 1..hist_data.len() {
        periodic_daily_returns.push((hist_data[res].1 / hist_data[res - 1].1).ln());
    }

    let mean_daily_return: f64 = Statistics::mean(&periodic_daily_returns);
    let var_p_daily_return: f64 = Statistics::population_variance(&periodic_daily_returns);
    let stdev_p_daily_return: f64 = Statistics::population_std_dev(&periodic_daily_returns);
    let drift: f64 = &mean_daily_return - (&var_p_daily_return / 2.0) as f64;
    let latest_date: NaiveDate = hist_data[hist_data.len()-1].0;
    let latest_price: f64 = hist_data[hist_data.len()-1].1;
    let days_to_sim: i64 = date_time::days_to_simulate(&end_date, latest_date);

    println!("\nStatistics calculated for historical data ...");
    println!("    Total records ingested: {}", hist_data.len() as i32);
    println!("    Average Daily Return: {}", mean_daily_return);
    println!("    Variance: {}", var_p_daily_return);
    println!("    Std Deviation: {}", stdev_p_daily_return);
    println!("    Drift: {}", drift);
    print!("\n");

    println!("Starting price simulation to {} ({} days, {} simulations per day) ...", end_date, &days_to_sim, &num_sims);
    println!("    Latest price date: {}", latest_date);
    println!("    Latest price (USD): {}", latest_price);

    print!("\n");
    get_price_for_next_day(&latest_price, &stdev_p_daily_return, &drift);
}

fn get_price_for_next_day(last_hist_price: &f64, stdev_p: &f64, drift: &f64) {
    let mut rng: ThreadRng = rand::thread_rng();
    let random_value: f64 = normsinv(rng.gen()) * stdev_p;
    let multiplier: f64 = (drift + &random_value).exp();
    let next_day_price: f64 = last_hist_price * multiplier;

    // println!("{} -> {}", &last_hist_price, &next_day_price);
}

fn normsinv(p: f64) -> f64 {
    let normal: Normal = Normal::new(0.0, 1.0).unwrap();
    normal.inverse_cdf(p)
}