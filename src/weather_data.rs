// https://designbuilder.co.uk/cahelp/Content/EnergyPlusWeatherFileFormat.htm

use chrono::{DateTime, FixedOffset};

/// Present Weather codes
///
/// The present weather struct based on TMY2 conventions.  Note that the most important fields are
/// those representing liquid precipitation - where the surfaces of the  building would be wet.
/// EnergyPlus uses “Snow Depth” to determine if snow is on the ground
#[derive(Debug)]
pub struct PresentWeather {
    /// Occurrence of Thunderstorm, Tornado, or Squall
    /// ### Definition:
    /// - `0` = Thunderstorm-lightning and thunder. Wind gusts less than 25.7 m/s, and hail, if any, less than 1.9 cm diameter
    /// - `1` = Heavy or severe thunderstorm-frequent intense lightning and thunder. Wind gusts greater than 25.7 m/s and hail, if any, 1.9 cm or greater diameter
    /// - `2` = Report of tornado or waterspout
    /// - `4` = Moderate squall-sudden increase of wind speed by at least 8.2 m/s, reaching 11.3 m/s or more and lasting for at least 1 minute
    /// - `6` = Water spout (beginning January 1984)
    /// - `7` = Funnel cloud (beginning January 1984)
    /// - `8` = Tornado (beginning January 1984)
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    pub thunderstorm: u8,

    /// Occurrence of Rain, Rain Showers, or Freezing Rain
    /// ### Definition:
    /// - `0` = Light rain
    /// - `1` = Moderate rain
    /// - `2` = Heavy rain
    /// - `3` = Light rain showers
    /// - `4` = Moderate rain showers
    /// - `5` = Heavy rain showers
    /// - `6` = Light freezing rain
    /// - `7` = Moderate freezing rain
    /// - `8` = Heavy freezing rain
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    /// ### Notes:
    /// <dl>
    ///   <dt>Light</dt>
    ///   <dd>up to 0.25 cm per hour</dd>
    ///   <dt>Heavy</dt>
    ///   <dd>greater than 0.76cm per hour</dd>
    /// </dl>
    pub rain: u8,

    /// Occurrence of Rain Squalls, Drizzle, or Freezing Drizzle
    /// ### Definition:
    /// - `0` = Light rain squalls
    /// - `1` = Moderate rain squalls
    /// - `3` = Light drizzle
    /// - `4` = Moderate drizzle
    /// - `5` = Heavy drizzle
    /// - `6` = Light freezing drizzle
    /// - `7` = Moderate freezing drizzle
    /// - `8` = Heavy freezing drizzle
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    /// ### Notes:
    /// #### When drizzle or freezing drizzle occurs with other weather phenomena:
    /// <dl>
    ///   <dt>Light</dt>
    ///   <dd>up to 0.025 cm per hour</dd>
    ///   <dt>Moderate</dt>
    ///   <dd>0.025 to 0.051cm per hour</dd>
    ///   <dt>Heavy</dt>
    ///   <dd>greater than 0.051 cm per hour</dd>
    /// </dl>
    ///
    /// #### When drizzle or freezing drizzle occurs alone:
    /// <dl>
    ///   <dt>Light</dt>
    ///   <dd> visibility 1 km or greater</dd>
    ///   <dt>Moderate</dt>
    ///   <dd>visibility between 0.5 and 1 km</dd>
    ///   <dt>Heavy</dt>
    ///   <dd>visibility 0.5 km or less</dd>
    /// </dl>
    pub rain_squalls: u8,

    /// Occurrence of Snow, Snow Pellets, or Ice Crystals
    /// ### Definition:
    /// - `0` = Light snow
    /// - `1` = Moderate snow
    /// - `2` = Heavy snow
    /// - `3` = Light snow pellets
    /// - `4` = Moderate snow pellets
    /// - `5` = Heavy snow pellets
    /// - `6` = Light ice crystals
    /// - `7` = Moderate ice crystals
    /// - `8` = Heavy ice crystals
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    /// ### Notes:
    /// Beginning in April 1963, any occurrence of ice crystals is recorded as a `7`.
    pub snow: u8,

    /// Occurrence of Snow Showers, Snow Squalls, or Snow Grains
    /// ### Definition:
    /// - `0` = Light snow
    /// - `1` = Moderate snow showers
    /// - `2` = Heavy snow showers
    /// - `3` = Light snow squall
    /// - `4` = Moderate snow squall
    /// - `5` = Heavy snow squall
    /// - `6` = Light snow grains
    /// - `7` = Moderate snow grains
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    pub snow_showers: u8,

    /// Occurrence of Sleet, Sleet Showers, or Hail
    /// ### Definition:
    /// - `0` = Light ice pellet showers
    /// - `1` = Moderate ice pellet showers
    /// - `2` = Heavy ice pellet showers
    /// - `4` = Hail
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`=
    /// > Notes: Prior to April 1970, ice pellets were coded as sleet. Beginning in April 1970, sleet and small hail were redefined as ice pellets and are coded as `0`, `1`, or `2`.
    pub sleet: u8,

    /// Occurrence of Fog, Blowing Dust, or Blowing Sand
    /// ### Definition:
    /// - `0` = Fog
    /// - `1` = Ice fog
    /// - `2` = Ground fog
    /// - `3` = Blowing dust
    /// - `4` = Blowing sand
    /// - `5` = Heavy fog
    /// - `6` = Glaze (beginning 1984)
    /// - `7` = Heavy ice fog (beginning 1984)
    /// - `8` = Heavy ground fog (beginning 1984)
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    /// > Notes: These values recorded only when visibility is less than 11 km.
    pub fog: u8,

    /// Occurrence of Smoke, Haze, Smoke and Haze, Blowing Snow, Blowing Spray, or Dust
    /// ### Definition:
    /// - `0` = Smoke
    /// - `1` = Haze
    /// - `2` = Smoke and haze
    /// - `3` = Dust
    /// - `4` = Blowing snow
    /// - `5` = Blowing spray
    /// - `6` = Dust storm (beginning 1984)
    /// - `7` = Volcanic ash
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
    /// > Notes: These values recorded only when visibility is less than 11 km.
    pub smoke: u8,

    /// Occurrence of Ice Pellets
    /// ### Definition:
    /// - `0` = Light ice pellets
    /// - `1` = Moderate ice pellets
    /// - `2` = Heavy ice pellets
    /// - `9` = None if Observation Indicator element equals `0`, or else unknown or missing if Observation Indicator element equals `9`
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

    /// Dew point temperature in °C
    pub dew_point_temperature: Vec<f64>,

    /// Relative humidity in % [0-100]
    pub relative_humidity: Vec<f64>,

    /// Atmospheric pressure in Pascals
    pub atmospheric_pressure: Vec<f64>,

    /// Extraterrestrial Horizontal Radiation in Wh/m²
    pub extraterrestrial_horizontal_radiation: Vec<f64>,

    /// Extraterrestrial Direct Normal Radiation in Wh/m²
    pub extraterrestrial_direct_normal_radiation: Vec<f64>,

    /// Horizontal Infrared Radiation in Wh/m²
    pub horizontal_infrared_radiation_intensity: Vec<f64>,

    /// Glob al Horizontal Radiation in Wh/m²
    pub global_horizontal_radiation: Vec<f64>,

    /// Direct Normal Radiation in Wh/m²
    pub direct_normal_radiation: Vec<f64>,

    /// Diffuse Horizontal Radiation in Wh/m²
    pub diffuse_horizontal_radiation: Vec<f64>,

    /// Global Horizontal Illuminance in lux
    pub global_horizontal_illuminance: Vec<f64>,

    /// Direct Normal Illuminance in lux
    pub direct_normal_illuminance: Vec<f64>,

    /// Diffuse Horizontal Illuminance in lux
    pub diffuse_horizontal_illuminance: Vec<f64>,

    /// Zenith Luminance in Cd/m²
    pub zenith_luminance: Vec<f64>,

    /// Wind direction in degrees [0-360]
    pub wind_direction: Vec<f64>,

    /// Wind speed in m/s
    pub wind_speed: Vec<f64>,

    /// Total sky cover
    pub total_sky_cover: Vec<f64>,

    /// Opaque sky cover
    pub opaque_sky_cover: Vec<f64>,

    /// Visibility in km
    pub visibility: Vec<f64>,

    /// Ceiling height in m
    pub ceiling_height: Vec<f64>,

    /// Whether present weather should be taken from the following field
    pub present_weather_observation: Vec<bool>,

    /// Present weather
    pub present_weather_codes: Vec<PresentWeather>,

    /// Precipitable water in mm
    pub precipitable_water: Vec<f64>,

    /// Aerosol optical depth in thousandths
    pub aerosol_optical_depth: Vec<f64>,

    /// Snow depth in cm
    pub snow_depth: Vec<f64>,

    /// Days since last snowfall
    pub days_since_last_snowfall: Vec<f64>,

    /// Albedo
    pub albedo: Vec<f64>,

    /// Liquid precipitation depth in mm
    pub liquid_precipitation_depth: Vec<f64>,

    /// Liquid precipitation quantity in hours
    pub liquid_precipitation_quantity: Vec<f64>,
}

#[cfg(feature = "polars")]
pub mod polars {
    use super::WeatherData;
    use polars::prelude::*;

    impl WeatherData {
        pub fn to_dataframe(&self) -> Result<DataFrame, PolarsError> {
            let millisecond_timestamps: Vec<i64> = self
                .timestamp
                .iter()
                .map(|dt| dt.timestamp_millis())
                .collect();
            let timestamp = match Series::new("timestamp".into(), millisecond_timestamps)
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))
            {
                Ok(ts) => ts,
                Err(e) => return Err(e),
            };

            let series_length = self.present_weather_codes.len();
            let mut present_thunderstorm: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_rain: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_rain_squalls: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_snow: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_snow_showers: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_sleet: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_fog: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_smoke: Vec<u8> = Vec::with_capacity(series_length);
            let mut present_ice_pellets: Vec<u8> = Vec::with_capacity(series_length);

            for pw in &self.present_weather_codes {
                present_thunderstorm.push(pw.thunderstorm);
                present_rain.push(pw.rain);
                present_rain_squalls.push(pw.rain_squalls);
                present_snow.push(pw.snow);
                present_snow_showers.push(pw.snow_showers);
                present_sleet.push(pw.sleet);
                present_fog.push(pw.fog);
                present_smoke.push(pw.smoke);
                present_ice_pellets.push(pw.ice_pellets);
            }

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
                "present_thunderstorm" => &present_thunderstorm,
                "present_rain" => &present_rain,
                "present_rain_squalls" => &present_rain_squalls,
                "present_snow" => &present_snow,
                "present_snow_showers" => &present_snow_showers,
                "present_sleet" => &present_sleet,
                "present_fog" => &present_fog,
                "present_smoke" => &present_smoke,
                "present_ice_pellets" => &present_ice_pellets,
                "precipitable_water" => &self.precipitable_water,
                "aerosol_optical_depth" => &self.aerosol_optical_depth,
                "snow_depth" => &self.snow_depth,
                "days_since_last_snowfall" => &self.days_since_last_snowfall,
                "albedo" => &self.albedo,
                "liquid_precipitation_depth" => &self.liquid_precipitation_depth,
                "liquid_precipitation_quantity" => &self.liquid_precipitation_quantity,
            ) {
                Ok(df) => Ok(df),
                Err(e) => Err(e),
            }
        }
    }
}
