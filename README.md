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

let mut epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();

{
    let header = epw.get_header();
    println!("Header: {:?}", header);
}
let data = epw.get_data().expect("Couldn't get data");
println!("Data:   {:?}", &data);

```

## Feature Roadmap
- [x] Read Header and Data
- [x] Polars DataFrame output
- [ ] Lazy load data
- [ ] PresentWeather Enum
- [ ] Write EPW files

<!-- Badges -->
[crate_link]: https://crates.io/crates/epw-rs "Crate listing"
[crate_img]: https://img.shields.io/crates/v/epw-rs?style=for-the-badge "Crate badge"
[docs_link]: https://docs.rs/epw-rs/latest/epw-rs "Crate documentation"
[docs_img]: https://img.shields.io/docsrs/epw-rs/latest.svg?style=for-the-badge "Documentation badge"
[license_file]: https://github.com/ferrilab/epw-rs/blob/main/LICENSE.txt "Project license"
[license_img]: https://img.shields.io/crates/l/epw-rs.svg?style=for-the-badge "License badge"