use crate::date_time;

use chrono::NaiveDate;
use statrs::statistics::Statistics;
use statrs::distribution::{ContinuousCDF, Normal};
use rand::prelude::*;
use rayon::prelude::*;

pub fn run_monte_carlo_simulation(end_date: &String, hist_data: Vec<(NaiveDate, f64)>, sims_per_day: &String) {
    let num_sims: &i32 = &sims_per_day.parse::<i32>().unwrap();
    let mut periodic_daily_returns: Vec<f64> = Vec::new();
    for res in 1..hist_data.len() {
        periodic_daily_returns.push((hist_data[res].1 / hist_data[res - 1].1).ln());
    }
    
    let latest_date: NaiveDate = hist_data[hist_data.len()-1].0;
    let latest_price: f64 = hist_data[hist_data.len()-1].1;
    let days_to_sim: i64 = date_time::days_between(&end_date, latest_date);
    let mean_daily_return: f64 = Statistics::mean(&periodic_daily_returns);
    let var_p_daily_return: f64 = Statistics::population_variance(&periodic_daily_returns);
    let stdev_p_daily_return: f64 = Statistics::population_std_dev(&periodic_daily_returns);
    let drift: f64 = &mean_daily_return - (&var_p_daily_return / 2.0) as f64;

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

    let results: Vec<Vec<(NaiveDate, f64)>> = (0..num_sims.clone())
        .into_par_iter() 
        .map(|_| simulation_run(&days_to_sim, 1, &latest_date, &latest_price, &stdev_p_daily_return, &drift)) 
        .collect(); 



    for res in results.iter() {
        println!("{:?}", res);
    }

    println!("\nSimulation complete! {} price pints generated in total.", num_sims.clone() as i64 * days_to_sim);
}

fn simulation_run(days_to_sim: &i64, curr_day: i64, latest_date: &NaiveDate, latest_price: &f64, stdev_p: &f64, drift: &f64) -> Vec<(NaiveDate, f64)> {
    let sim_date: NaiveDate = date_time::add_days_to_date(latest_date, &curr_day);
    let day_price = get_price_for_next_day(&latest_price, &stdev_p, &drift);
    let mut day_n_result: Vec<(NaiveDate, f64)> = Vec::new();
    day_n_result.push((sim_date, day_price));
    
    if &curr_day < days_to_sim {
        let mut day_n_plus_one_result: Vec<(NaiveDate, f64)> = simulation_run(days_to_sim, &curr_day + 1, latest_date, latest_price, &stdev_p, &drift);
        day_n_result.append(&mut day_n_plus_one_result);
    }

    return day_n_result;
}

fn get_price_for_next_day(last_hist_price: &f64, stdev_p: &f64, drift: &f64) -> f64 {
    let mut rng: ThreadRng = rand::thread_rng();
    let normal: Normal = Normal::new(0.0, 1.0).unwrap();
    let random_value: f64 = normal.inverse_cdf(rng.gen()) * stdev_p;
    let multiplier: f64 = (drift + &random_value).exp();
    let next_day_price: f64 = last_hist_price * multiplier;

    return next_day_price;
}
