# Oxide Convergence

Monte Carlo simulator of price data written in Rust

**!!! WARNING !!!**

The code is technically correct and works, but no guarantees are made for the correctness of the formulae or the steps in the process; **_USER BEWARE_** - use at your own risk!

## Building

```sh
cargo build --release
```

The compiled binary can be found under `./target/release/`.

## Running

```sh
oxide-convergence -i IN_FILE.csv -o OUT_FILE -e END_DATE -f "DATE_REGEX" -d DATE_COLUMN_INDEX -p PRICE_COLUMN_INDEX -s INTEGER
```

Options:

- _-i_ or _--in_file_: Input file of historical price data, in CSV format
- _-o_ or _--out_file_: File to output price forecast data
- _-e_ or _--end_date_: Target price date, in YYYY-MM-DD format
- _-f_ or _--date_format_: Date format regex, with captures, used for interpreting and parsing dates
- _-d_ or _--date_column_index_: Zero-indexed column number containing dates
- _-p_ or _--price_column_index_: Zero-indexed column number containing prices
- _-s_ or _--sims_per_day_: Number of simulations to run per day

Example:

```sh
oxide-convergence -i hist_data.csv -o /tmp/mc_res.csv -e 2028-03-31 -f "^(\d{4})-(\d{2})-(\d{2}).*$" -d0 -p1 -s 5000
```

Sample run:

```sh
$ target/release/oxide-convergence - -i data/coingecko-hbar-usd-genesis-20240522.csv -o /tmp/hbar_price_forecast_20240522-20241231.csv -e 2024-12-31 -f '^(\d{4})-(\d{2})-(\d{2}).*$' -d0 -p1 -s 1000000`

Statistics calculated for historical data ...
    Total records ingested: 1709
    Average Periodic Daily Return : 0.000003912096542561562
    Minimun Periodic Daily Return: -0.5910989317376647
    Maximum Periodic Daily Return: 0.7162084731433174
    Variance: 0.004403904535001393
    Std Deviation: 0.06636192082061364
    Drift: -0.002198040170958135

Starting price simulation to 2024-12-31 (223 days, 1,000,000 simulations per day) ...
    Latest price date: 2024-05-22
    Latest price (USD): 0.11517014118269715
    Simulation complete! 223,000,000 price points generated in total

Simulation Results:
    Expected price on 2024-12-31: 0.11529989849831523

Granular Results:
    Granular results available in file '/tmp/hbar_price_forecast_20240522-20241231.csv'
```

Granular results:

```sh
$ head /tmp/hbar_price_forecast_20240522-20241231.csv 
date,mean,min,max,stdev_p,var_p
2024-05-23,0.11516323570167566,0.0839082982922755,0.15831909226205793,0.007648172508418186,0.00005849454271852373
2024-05-24,0.1151689583128451,0.08168939927851085,0.15930332503768105,0.007655587278523286,0.00005860801657908758
2024-05-25,0.11516601277701675,0.08293300586227746,0.15517392805755098,0.007650689241921117,0.00005853304587644752
2024-05-26,0.11516837737873861,0.08292782648554664,0.1620598950566647,0.007655203630621842,0.000058602142626285835
2024-05-27,0.11518010934773278,0.08108850378106763,0.15998029988835888,0.007656924789108562,0.000058628497226065194
2024-05-28,0.11516897725662929,0.0845897365601167,0.15959008003503147,0.007652190895622956,0.00005855602550305485
2024-05-29,0.11515522316191233,0.08388016062229756,0.15739070627507112,0.007663907589648068,0.00005873547954266526
2024-05-30,0.1151627213070215,0.0831433783001105,0.15524596900429047,0.007652309325092416,0.00005855783800689635
2024-05-31,0.11516920215107922,0.08426801571528787,0.16039875077751659,0.007650165380773309,0.000058525030353182424
```