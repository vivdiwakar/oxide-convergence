extern crate getopts;

use std::env;
use getopts::Options;

fn usage(program: &str, opts: Options) {
    let message: String = format!("Usage: {} -i input_file.csv -o output_file.csv", program);
    println!("{}", opts.usage(&message));
}

fn load_historical_data(in_file: &str, out_file: &str) {
    println!("Loading historical price data from '{}' ...", in_file);

    let days_to_sim: u32 = 1450;

    run_simulation(days_to_sim, &out_file);
    return;
}

fn run_simulation(days: u32, out_file: &str) {
    println!("Running simulations for {} days ...", days);

    compile_results(&out_file);
    return;
}

fn compile_results(out_file: &str) {
    println!("Simulation complete, results in {}.", &out_file);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let full_path: String = args[0].clone();
    let path_elems: Vec<&str> = full_path.split("/").collect();
    let program: &str = path_elems[path_elems.len()-1];

    let mut opts: Options = Options::new();
    opts.optopt("i", "in_file", "Input file of historical price data, in CSV format.", "IN_FILE");
    opts.optopt("o", "out_file", "File to output price forecast data.", "OUT_FILE");
    opts.optflag("h", "help", "Print this help menu.");
    let matches: getopts::Matches = match opts.parse(&args[1..]) {
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

    load_historical_data(&in_file, &out_file);
    return;
}
