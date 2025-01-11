# epw-rs

## Parser for the Energy Plus Weather file format

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

fn main() {
    let epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
    println!("Header: {}\nData:   {}", epw.header, epw.data);
}
```


## Features

### `polars`

The `polars` feature provides support for returning weather data as a dataframe

```rust
use epw_rs::*;
fn main() {
    let epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
    let df = epw.data.to_dataframe();
}
```

For a more detailed example see [examples/polars.rs](examples/polars.rs).




## Features
- [x] Read Header and Data
- [ ] Lazy load data