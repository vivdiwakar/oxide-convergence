use statrs::statistics::Statistics;
use rand::Rng;
use rand_distr::{Distribution, StandardNormal};

pub fn get_daily_return_stats(rets_list: &[f64]) -> (f64, f64, f64, f64, f64, f64) {
    let mean: f64 = Statistics::mean(rets_list);
    let min: f64 = Statistics::min(rets_list);
    let max: f64 = Statistics::max(rets_list);
    let stdev_p: f64 = Statistics::population_std_dev(rets_list);
    let var_p: f64 = Statistics::population_variance(rets_list);
    let drift: f64 = mean - (var_p / 2.0);

    (mean, min, max, var_p, stdev_p, drift)
}

pub fn get_statistical_price<R: Rng + ?Sized>(last_hist_price: f64, stdev_p: f64, drift: f64, rng: &mut R) -> f64 {
    let z: f64 = StandardNormal.sample(rng);
    last_hist_price * (drift + stdev_p * z).exp()
}

