# Oxide Convergence

Monte Carlo simulator of price data written in Rust

**!!! NOTICE !!!**

This codebase works from a theoretical/student exercise perspective, but not particularly suitable for crypto modelling as it does not factor in volatility properly, and does not handle fat tals properly either.

Also, from a software engineering perspective, it is a bit of a dog's breakfast!

**!!! WARNING !!!**

The code is technically correct and works, but no guarantees are made for the correctness of the formulae or the steps in the process; **_USER BEWARE_** - use at your own risk!

## Building

```sh
cargo build --release
```

The compiled binary can be found under `./target/release/`.

## Source Data

- [Coingecko - Canton USD Historical Data](https://www.coingecko.com/en/coins/canton/historical_data)

## Running

```sh
oxide-convergence -i IN_FILE.csv -o OUT_FILE -e END_DATE -f "DATE_REGEX" -d DATE_COLUMN_INDEX -p PRICE_COLUMN_INDEX -s INTEGER [-r UNSIGNED_INTEGER]
```

Options:

- _-i_ or _--in_file_: Input file of historical price data, in CSV format
- _-o_ or _--out_file_: File to output price forecast data
- _-e_ or _--end_date_: Target price date, in YYYY-MM-DD format
- _-f_ or _--date_format_: Date format regex, with captures, used for interpreting and parsing dates
- _-d_ or _--date_column_index_: Zero-indexed column number containing dates
- _-p_ or _--price_column_index_: Zero-indexed column number containing prices
- _-s_ or _--sims_per_day_: Number of simulations to run per day
- _-r_ or _--seed_: Optional u64 seed for reproducible runs

Example:

```sh
oxide-convergence -i hist_data.csv -o /tmp/mc_res.csv -e 2028-03-31 -f "^(\d{4})-(\d{2})-(\d{2}).*$" -d0 -p1 -s 5000 -r 1779650345
```

Sample run:

```sh
$ target/release/oxide-convergence - -i data/coingecko-cc-usd-genesis-20260524.csv -o /tmp/cc_price_forecast_2026-12-31.csv -e 2026-12-31 -f '^(\d{4})-(\d{2})-(\d{2}).*$' -d0 -p1 -s 1000000 -r `date +%s`

Statistics calculated for historical data ...
    Total records ingested: 196
    Average Periodic Daily Return : 0.001586367388
    Minimum Periodic Daily Return: -0.182798536624
    Maximum Periodic Daily Return: 0.218216925375
    Variance on Daily Return: 0.003268429445
    Std Deviation on Daily Return: 0.057170179686
    Daily Return Drift: -0.000047847335

Starting price simulation to 2026-12-31 (221 days, 1,000,000 simulations per day) ...
    Latest price date: 2026-05-24
    Latest price (USD): 0.161025
    Simulation complete! 221,000,000 price points generated in total

Simulation Results:
    Expected price on 2026-12-31: 0.231764

Granular Results:
    Granular results available in file '/tmp/cc_price_forecast_2026-12-31.csv'

Seed used for simulation: 1779643359
```

Granular results:

```sh
$ head /tmp/cc_price_forecast_2026-12-31.csv
date,mean,min,max,stdev_p,var_p
2026-05-25,0.1612907849992635,0.12068439291589264,0.21513954115026324,0.009226301922335223,0.00008512464716208664
2026-05-26,0.1615567755950568,0.12088341800945121,0.21549433572038978,0.009241517357893343,0.00008540564307624396
2026-05-27,0.16182320484577606,0.12108277132264893,0.21584971539535763,0.009256757885788669,0.00008568756655611071
2026-05-28,0.16209007347481663,0.1212824533967661,0.2162056811400869,0.009272023547402303,0.00008597042066358279
2026-05-29,0.16235738220677895,0.12148246477396951,0.2165622339210779,0.00928731438418209,0.00008625420847063555
2026-05-30,0.16262513176745624,0.12168280599732909,0.21691937470644107,0.009302630437646696,0.00008653893305943075
2026-05-31,0.16289332288382724,0.12188347761080873,0.21727710446588047,0.009317971749381282,0.00008682459752226769
2026-06-01,0.1631619562840834,0.12208448015926085,0.2176354241706841,0.009333338361039549,0.00008711120496165241
2026-06-02,0.16343103269761292,0.12228581418844679,0.21799433479376013,0.009348730314345296,0.0000873987584903587
```
