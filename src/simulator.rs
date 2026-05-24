use crate::date_time;
use crate::oxide_stats;

use chrono::NaiveDate;
use rand::SeedableRng;
use rand::rngs::StdRng;

pub fn setup_historical_data(end_date: &str, hist_data: &[(NaiveDate, f64)]) 
    -> (NaiveDate, f64, i64, f64, f64, f64, f64, f64, f64) 
{
    let mut periodic_daily_returns: Vec<f64> = Vec::new();
    for res in 1..hist_data.len() {
        periodic_daily_returns.push((hist_data[res].1 / hist_data[res - 1].1).ln());
    }
    
    let latest_date: NaiveDate = hist_data[hist_data.len()-1].0;
    let latest_price: f64 = hist_data[hist_data.len()-1].1;
    let days_to_sim: i64 = date_time::days_between(end_date, latest_date);
    let (mean, min, max, var_p, stdev_p, drift) = 
        oxide_stats::get_daily_return_stats(&periodic_daily_returns);

    (latest_date, latest_price, days_to_sim, mean, min, max, var_p, stdev_p, drift)
}

pub fn run_simulation(curr_day: i64, latest_date: &NaiveDate, days_to_sim: &i64, num_sims: &i64, last_hist_price: &f64, setup_params: (&f64, &f64, &u64)) 
    -> Vec<(NaiveDate, f64, f64, f64, f64, f64)> 
{
    let (stdev_p, drift, seed) = setup_params;
    let mut rng: StdRng = StdRng::seed_from_u64(*seed);
    let sim_date: NaiveDate = date_time::add_days_to_date(latest_date, &curr_day);
    let day_results: Vec<f64> = (0..*num_sims)
        .map(|_| oxide_stats::get_statistical_price(last_hist_price, stdev_p, drift, &mut rng))
        .collect();
    let (mean_res, min_res, max_res, var_p_res, stdev_p_res, _drift_res) = 
        oxide_stats::get_daily_return_stats(&day_results);
    let mut day_n_result: Vec<(NaiveDate, f64, f64, f64, f64, f64)> = Vec::new();
    day_n_result.push((sim_date, mean_res, min_res, max_res, stdev_p_res, var_p_res));

    if &curr_day < days_to_sim {
        let mut day_n_plus_one_result: Vec<(NaiveDate, f64, f64, f64, f64, f64)> = 
            run_simulation(&curr_day + 1, latest_date, days_to_sim, num_sims, &mean_res, setup_params);
        day_n_result.append(&mut day_n_plus_one_result);
    }

    day_n_result
}
