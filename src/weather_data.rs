// https://designbuilder.co.uk/cahelp/Content/EnergyPlusWeatherFileFormat.htm

use chrono::{DateTime, FixedOffset};

#[derive(Debug)]
pub struct PresentWeather {
    pub thunderstorm: u8,
    pub rain: u8,
    pub rain_squalls: u8,
    pub snow: u8,
    pub snow_showers: u8,
    pub sleet: u8,
    pub fog: u8,
    pub smoke: u8,
    pub ice_pellets: u8,
}

/// # EPW weather data
///
/// The weather data from the file is provided in a column-oriented format for efficient analysis
///
#[derive(Debug)]
pub struct WeatherData {
    /// Timestamps for the weather data samples
    pub timestamp: Vec<DateTime<FixedOffset>>,

    /// Data Source and validity flags. The format is not documented
    pub flags: Vec<String>,

    /// Dry bulb temperature in °C
    pub dry_bulb_temperature: Vec<f64>,
    pub dew_point_temperature: Vec<f64>,
    pub relative_humidity: Vec<f64>,
    pub atmospheric_pressure: Vec<f64>,
    pub extraterrestrial_horizontal_radiation: Vec<f64>,
    pub extraterrestrial_direct_normal_radiation: Vec<f64>,
    pub horizontal_infrared_radiation_intensity: Vec<f64>,
    pub global_horizontal_radiation: Vec<f64>,
    pub direct_normal_radiation: Vec<f64>,
    pub diffuse_horizontal_radiation: Vec<f64>,
    pub global_horizontal_illuminance: Vec<f64>,
    pub direct_normal_illuminance: Vec<f64>,
    pub diffuse_horizontal_illuminance: Vec<f64>,
    pub zenith_luminance: Vec<f64>,
    pub wind_direction: Vec<f64>,
    pub wind_speed: Vec<f64>,
    pub total_sky_cover: Vec<f64>,
    pub opaque_sky_cover: Vec<f64>,
    pub visibility: Vec<f64>,
    pub ceiling_height: Vec<f64>,
    pub present_weather_observation: Vec<bool>,
    pub present_weather_codes: Vec<PresentWeather>,
    pub precipitable_water: Vec<f64>,
    pub aerosol_optical_depth: Vec<f64>,
    pub snow_depth: Vec<f64>,
    pub days_since_last_snowfall: Vec<f64>,
    pub albedo: Vec<f64>,
    pub liquid_precipitation_depth: Vec<Option<f64>>,
    pub liquid_precipitation_quantity: Vec<Option<f64>>,
}

#[cfg(feature = "polars")]
pub mod polars {
    use super::WeatherData;
    use polars::prelude::*;

    #[derive(Debug)]
    pub enum DataFrameError {
        TimestampError(String),
        GenericError(String),
    }

    impl WeatherData {
        pub fn to_dataframe(&self) -> Result<DataFrame, DataFrameError> {
            let millisecond_timestamps: Vec<i64> = self
                .timestamp
                .iter()
                .map(|dt| dt.timestamp_millis())
                .collect();
            let timestamp = match Series::new("timestamp".into(), millisecond_timestamps)
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
            {
                Ok(ts) => ts,
                Err(e) => return Err(DataFrameError::TimestampError(e.to_string())),
            };

            match df!(
                "timestamp" => timestamp,
                "dry_bulb_temperature" => &self.dry_bulb_temperature,
                "dew_point_temperature" => &self.dew_point_temperature,
                "relative_humidity" => &self.relative_humidity,
                "atmospheric_pressure" => &self.atmospheric_pressure,
                "extraterrestrial_horizontal_radiation" => &self.extraterrestrial_horizontal_radiation,
                "extraterrestrial_direct_normal_radiation" => &self.extraterrestrial_direct_normal_radiation,
                "horizontal_infrared_radiation_intensity" => &self.horizontal_infrared_radiation_intensity,
                "global_horizontal_radiation" => &self.global_horizontal_radiation,
                "direct_normal_radiation" => &self.direct_normal_radiation,
                "diffuse_horizontal_radiation" => &self.diffuse_horizontal_radiation,
                "global_horizontal_illuminance" => &self.global_horizontal_illuminance,
                "direct_normal_illuminance" => &self.direct_normal_illuminance,
                "diffuse_horizontal_illuminance" => &self.diffuse_horizontal_illuminance,
                "zenith_luminance" => &self.zenith_luminance,
                "wind_direction" => &self.wind_direction,
                "wind_speed" => &self.wind_speed,
                "total_sky_cover" => &self.total_sky_cover,
                "opaque_sky_cover" => &self.opaque_sky_cover,
                "visibility" => &self.visibility,
                "ceiling_height" => &self.ceiling_height,
                "present_weather_observation" => &self.present_weather_observation,
                //"present_weather_codes" => &self.present_weather_codes,
                "precipitable_water" => &self.precipitable_water,
                "aerosol_optical_depth" => &self.aerosol_optical_depth,
                "snow_depth" => &self.snow_depth,
                "days_since_last_snowfall" => &self.days_since_last_snowfall,
                "albedo" => &self.albedo,
            ) {
                Ok(df) => Ok(df),
                Err(e) => Err(DataFrameError::GenericError(e.to_string())),
            }
        }
    }
}
