# epw-rs
[![Crate][crate_img]][crate_link]
[![Documentation][docs_img]][docs_link]
[![License][license_img]][license_file]

## Parser for the EnergyPlus Weather file format

### Note:
This library is still alpha-stage and the API is subject to change until we stabilize it in the 0.2 release.

## Summary
Rust library for reading [EnergyPlus](https://github.com/NREL/EnergyPlus) weather data files. These files typically
contain detailed hourly (or sub-hourly) weather data for a given location used for energy modeling.

## Introduction
The library presents a fairly small API, built around the `EPWFile` struct. It includes functions for reading from a 
`BufRead` buffer, or from a filepath.


### Reading an EPW file

Heres a basic example of using the library to read a TMY file in epw format.
```rust
use epw_rs::*;

let epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
println!("Header: {:?}\nData:   {:?}", epw.header, epw.data);

```

## Feature Roadmap
- [x] Read Header and Data
- [x] Polars DataFrame output
- [ ] Lazy load data
- [ ] PresentWeather Enum
- [ ] Write EPW files


## Features

### `polars`

The `polars` feature provides support for building a DataFrame from the weather data

```rust,ignore
use epw_rs::*;


let epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
let df = epw.data.to_dataframe();
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


<!-- Badges -->
[crate_link]: https://crates.io/crates/epw-rs "Crate listing"
[crate_img]: https://img.shields.io/crates/v/epw-rs?style=for-the-badge "Crate badge"
[docs_link]: https://docs.rs/epw-rs/latest/epw-rs "Crate documentation"
[docs_img]: https://img.shields.io/docsrs/epw-rs/latest.svg?style=for-the-badge "Documentation badge"
[license_file]: https://github.com/ferrilab/epw-rs/blob/main/LICENSE.txt "Project license"
[license_img]: https://img.shields.io/crates/l/epw-rs.svg?style=for-the-badge "License badge"