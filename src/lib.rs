#![doc = include_str!("../README.md")]
mod epw_file;
mod error;
mod header;
mod weather_data;

pub use epw_file::EPWFile;
pub use header::Header;
pub use weather_data::WeatherData;

#[cfg(feature = "polars")]
pub use weather_data::polars;
