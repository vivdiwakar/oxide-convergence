# Oxide Convergence

Monte Carlo simulator of price data written in Rust

## Building

```sh
cargo build --release
```

## Running

```sh
target/release/oxide-convergence -i IN_FILE.csv -o OUT_FILE -e END_DATE -f "DATE_REGEX" -d DATE_COLUMN -p PRICE_COLUMN
```

Options:

- _-i_ or _--in_file_: Input file of historical price data, in CSV format
- _-o_ or _--out_file_: File to output price forecast data
- _-e_ or _--end_date_: Target price date, in YYYY-MM-DD format
- _-f_ or _--date_format_: Date format regex, with captures, used for interpreting and parsing dates
- _-d_ or _--date_column_index: Zero-indexed column number containing dates
- _-p_ or _--price_column_index: Zero-indexed column number containing prices

Example:

```sh
oxide-convergence -i hist_data.csv -o /tmp/mc_res.csv -e 2028-03-31 -f "^(\d{4})-(\d{2})-(\d{2}).*$" -d0 -p1
```
