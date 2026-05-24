use crate::date_time;
use crate::oxide_stats;

use chrono::NaiveDate;
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand_chacha::ChaCha8Rng;
use rayon::prelude::*;
use rand::distr::{Distribution, StandardUniform};


pub fn setup_historical_data(end_date: &str, hist_data: &[(NaiveDate, f64)])
    -> (NaiveDate, f64, i64, f64, f64, f64, f64, f64, f64)
{
    let mut periodic_daily_returns: Vec<f64> = Vec::new();
    for res in 1..hist_data.len() {
        periodic_daily_returns.push((hist_data[res].1 / hist_data[res - 1].1).ln());
    }

    let latest_date: NaiveDate = hist_data[hist_data.len() - 1].0;
    let latest_price: f64 = hist_data[hist_data.len() - 1].1;
    let days_to_sim: i64 = date_time::days_between(end_date, latest_date);
    let (mean, min, max, var_p, stdev_p, drift) =
        oxide_stats::get_daily_return_stats(&periodic_daily_returns);

    (latest_date, latest_price, days_to_sim, mean, min, max, var_p, stdev_p, drift)
}

pub fn run_simulation(latest_date: &NaiveDate, days_to_sim: i64, num_sims: i64, last_hist_price: f64, stdev_p: f64, drift: f64, master_rng: &mut StdRng) 
    -> Vec<(NaiveDate, f64, f64, f64, f64, f64)>
{
    let days = days_to_sim as usize;
    let n_paths = num_sims as usize;

    // Get per-path seeds from master RNG
    let path_seeds: Vec<u64> = (0..n_paths).map(|_| StandardUniform.sample(master_rng)).collect();

    // Simulate each path in parallel
    let paths: Vec<Vec<f64>> = path_seeds
        .into_par_iter()
        .map(|seed| {
            let mut rng = ChaCha8Rng::seed_from_u64(seed);
            let mut path = Vec::with_capacity(days);
            let mut price = last_hist_price;
            for _ in 0..days {
                price = oxide_stats::get_statistical_price(price, stdev_p, drift, &mut rng);
                path.push(price);
            }
            path
        })
        .collect();

    // stats per day
    (0..days)
        .into_par_iter()
        .map(|t| {
            let day_slice: Vec<f64> = paths.iter().map(|p| p[t]).collect();
            let (mean_res, min_res, max_res, var_p_res, stdev_p_res, _drift_res) =
                oxide_stats::get_daily_return_stats(&day_slice);
            let sim_date = date_time::add_days_to_date(latest_date, &((t as i64) + 1));
            (sim_date, mean_res, min_res, max_res, stdev_p_res, var_p_res)
        })
        .collect()
}
