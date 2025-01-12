#![doc = include_str!("../README.md")]
pub mod epw_file;
mod error;
pub mod header;
pub mod weather_data;

pub use epw_file::EPWFile;
pub use header::Header;
pub use weather_data::WeatherData;

#[cfg(feature = "polars")]
pub use weather_data::polars;
