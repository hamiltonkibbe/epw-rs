#![doc = include_str!("../README.md")]

#![cfg_attr(feature = "polars", doc = r##"
## Features

### `polars`

The `polars` feature provides support for building a DataFrame from the weather data

```rust
use epw_rs::*;


let mut epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
let df = epw.get_dataframe();
println!("{}", df.unwrap())

// output:
// ┌─────────────────────┬──────────────────────┬───────────────────────┬───────────────────┬───┬───────────────────────┬────────────┬──────────────────────────┬────────┐
// │ timestamp           ┆ dry_bulb_temperature ┆ dew_point_temperature ┆ relative_humidity ┆ … ┆ aerosol_optical_depth ┆ snow_depth ┆ days_since_last_snowfall ┆ albedo │
// │ ---                 ┆ ---                  ┆ ---                   ┆ ---               ┆   ┆ ---                   ┆ ---        ┆ ---                      ┆ ---    │
// │ datetime[ms]        ┆ f64                  ┆ f64                   ┆ f64               ┆   ┆ f64                   ┆ f64        ┆ f64                      ┆ f64    │
// ╞═════════════════════╪══════════════════════╪═══════════════════════╪═══════════════════╪═══╪═══════════════════════╪════════════╪══════════════════════════╪════════╡
// │ 1987-01-01 05:00:00 ┆ 20.6                 ┆ 18.9                  ┆ 90.0              ┆ … ┆ 0.062                 ┆ 0.0        ┆ 88.0                     ┆ NaN    │
// │ 1987-01-01 06:00:00 ┆ 20.0                 ┆ 18.3                  ┆ 90.0              ┆ … ┆ 0.062                 ┆ 0.0        ┆ 88.0                     ┆ NaN    │
// │ 1987-01-01 07:00:00 ┆ 20.0                 ┆ 17.2                  ┆ 84.0              ┆ … ┆ 0.062                 ┆ 0.0        ┆ 88.0                     ┆ NaN    │
// │ 1987-01-01 08:00:00 ┆ 18.3                 ┆ 16.1                  ┆ 87.0              ┆ … ┆ 0.062                 ┆ 0.0        ┆ 88.0                     ┆ NaN    │
// │ 1987-01-01 09:00:00 ┆ 17.8                 ┆ 15.0                  ┆ 84.0              ┆ … ┆ 0.062                 ┆ 0.0        ┆ 88.0                     ┆ NaN    │
// │ …                   ┆ …                    ┆ …                     ┆ …                 ┆ … ┆ …                     ┆ …          ┆ …                        ┆ …      │
```

For a more detailed example see [examples/polars.rs](examples/polars.rs).
"##)]
pub mod epw_file;
mod error;
pub mod header;
pub mod weather_data;

pub use epw_file::EPWFile;
pub use header::Header;
pub use weather_data::WeatherData;

#[cfg(feature = "polars")]
pub use weather_data::polars_ext;
