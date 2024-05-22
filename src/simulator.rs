use crate::date_time;
use crate::oxide_stats;

use chrono::NaiveDate;
use rayon::prelude::*;

pub fn setup_historical_data(end_date: &String, hist_data: &Vec<(NaiveDate, f64)>) 
    -> (NaiveDate, f64, i64, f64, f64, f64, f64, f64, f64) 
{
    let mut periodic_daily_returns: Vec<f64> = Vec::new();
    for res in 1..hist_data.len() {
        periodic_daily_returns.push((hist_data[res].1 / hist_data[res - 1].1).ln());
    }
    
    let latest_date: NaiveDate = hist_data[hist_data.len()-1].0;
    let latest_price: f64 = hist_data[hist_data.len()-1].1;
    let days_to_sim: i64 = date_time::days_between(&end_date, latest_date);
    let (mean, min, max, var_p, stdev_p, drift) = 
        oxide_stats::get_daily_return_stats(&periodic_daily_returns);

    return (latest_date, latest_price, days_to_sim, mean, min, max, var_p, stdev_p, drift);
}

pub fn run_simulation(curr_day: i64, latest_date: &NaiveDate, days_to_sim: &i64, num_sims: &i64, 
        last_hist_price: &f64, stdev_p: &f64, drift: &f64) -> Vec<(NaiveDate, f64, f64, f64, f64, f64)> 
{
    let sim_date: NaiveDate = date_time::add_days_to_date(latest_date, &curr_day);
    let day_results: Vec<f64> = (0..num_sims.clone())
        .into_par_iter()
        .map(|_| oxide_stats::get_statistical_price(last_hist_price, stdev_p, drift))
        .collect();
    let (mean_res, min_res, max_res, var_p_res, stdev_p_res, drift_res) = 
        oxide_stats::get_daily_return_stats(&day_results);
    let mut day_n_result: Vec<(NaiveDate, f64, f64, f64, f64, f64)> = Vec::new();
    day_n_result.push((sim_date, mean_res, min_res, max_res, stdev_p_res, var_p_res));

    if &curr_day < days_to_sim {
        let mut day_n_plus_one_result: Vec<(NaiveDate, f64, f64, f64, f64, f64)> = 
            run_simulation(&curr_day + 1, latest_date, days_to_sim, num_sims, &mean_res, &stdev_p_res, &drift_res);
        day_n_result.append(&mut day_n_plus_one_result);
    }

    return day_n_result;
}
