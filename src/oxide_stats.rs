use statrs::statistics::Statistics;
use rand::prelude::*;
use rand_distr::StandardNormal;
use rand::rngs::StdRng;


pub fn get_daily_return_stats(rets_list: &Vec<f64>) -> (f64, f64, f64, f64, f64, f64) {
    let mean: f64 = Statistics::mean(rets_list);
    let min: f64 = Statistics::min(rets_list);
    let max: f64 = Statistics::max(rets_list);
    let stdev_p: f64 = Statistics::population_std_dev(rets_list);
    let var_p: f64 = Statistics::population_variance(rets_list);
    let drift: f64 = &mean - (&var_p / 2.0) as f64;

    (mean, min, max, var_p, stdev_p, drift)
}

pub fn get_statistical_price(last_hist_price: &f64, stdev_p: &f64, drift: &f64, rng: &mut StdRng) -> f64 {
    let random_value: f64 = StandardNormal.sample(rng);
    let multiplier: f64 = (drift + (stdev_p * random_value)).exp();
    let next_day_price: f64 = last_hist_price * multiplier;

    next_day_price
}
