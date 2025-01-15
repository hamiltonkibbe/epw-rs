use crate::error::EPWParseError;
use crate::Header;
use chrono::LocalResult::Single;
use chrono::{DateTime, FixedOffset, TimeZone};
use std::io::{BufRead, Lines};

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

impl PresentWeather {
    pub(crate) fn parse(value: &str) -> Result<Self, EPWParseError> {
        _parse_present_weather(value)
    }
}

/// # EPW weather data
///
/// The weather data from the file is provided in a column-oriented format for efficient analysis.
/// This library uses the convention of inserting NaN when a value is not available, rather than
/// using the in-band magic numbers (e.g. 999) to signify missing data.
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

impl WeatherData {
    pub fn parse<R: BufRead>(
        lines: &mut Lines<R>,
        header: &Header,
    ) -> Result<WeatherData, EPWParseError> {
        _parse_data(lines, header)
    }
}

fn _parse_data<R: BufRead>(
    lines: &mut Lines<R>,
    header: &Header,
) -> Result<WeatherData, EPWParseError> {
    let estimated_capacity = 8760 * header.data_periods.records_per_hour;
    let mut data = WeatherData {
        timestamp: Vec::with_capacity(estimated_capacity),
        flags: Vec::with_capacity(estimated_capacity),
        dry_bulb_temperature: Vec::with_capacity(estimated_capacity),
        dew_point_temperature: Vec::with_capacity(estimated_capacity),
        relative_humidity: Vec::with_capacity(estimated_capacity),
        atmospheric_pressure: Vec::with_capacity(estimated_capacity),
        extraterrestrial_horizontal_radiation: Vec::with_capacity(estimated_capacity),
        extraterrestrial_direct_normal_radiation: Vec::with_capacity(estimated_capacity),
        horizontal_infrared_radiation_intensity: Vec::with_capacity(estimated_capacity),
        global_horizontal_radiation: Vec::with_capacity(estimated_capacity),
        direct_normal_radiation: Vec::with_capacity(estimated_capacity),
        diffuse_horizontal_radiation: Vec::with_capacity(estimated_capacity),
        global_horizontal_illuminance: Vec::with_capacity(estimated_capacity),
        direct_normal_illuminance: Vec::with_capacity(estimated_capacity),
        diffuse_horizontal_illuminance: Vec::with_capacity(estimated_capacity),
        zenith_luminance: Vec::with_capacity(estimated_capacity),
        wind_direction: Vec::with_capacity(estimated_capacity),
        wind_speed: Vec::with_capacity(estimated_capacity),
        total_sky_cover: Vec::with_capacity(estimated_capacity),
        opaque_sky_cover: Vec::with_capacity(estimated_capacity),
        visibility: Vec::with_capacity(estimated_capacity),
        ceiling_height: Vec::with_capacity(estimated_capacity),
        present_weather_observation: Vec::with_capacity(estimated_capacity),
        present_weather_codes: Vec::with_capacity(estimated_capacity),
        precipitable_water: Vec::with_capacity(estimated_capacity),
        aerosol_optical_depth: Vec::with_capacity(estimated_capacity),
        snow_depth: Vec::with_capacity(estimated_capacity),
        days_since_last_snowfall: Vec::with_capacity(estimated_capacity),
        albedo: Vec::with_capacity(estimated_capacity),
        liquid_precipitation_depth: Vec::with_capacity(estimated_capacity),
        liquid_precipitation_quantity: Vec::with_capacity(estimated_capacity),
    };

    for line in lines {
        let line = line.expect("Unable to read line");
        _parse_row(&line, &mut data, &header.location.time_zone)?
    }

    Ok(data)
}

fn _parse_row(
    line: &str,
    dest: &mut WeatherData,
    timezone: &FixedOffset,
) -> Result<(), EPWParseError> {
    let parts = line.split(",").collect::<Vec<&str>>();
    if parts.len() < 32 {
        return Err(EPWParseError::Data(format!("Invalid Data Row: {}", line)));
    }

    let year = match parts[0].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Year: {} [{}]",
                parts[0], e
            )))
        }
    };
    let month = match parts[1].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Month: {} [{}]",
                parts[1], e
            )))
        }
    };
    let day = match parts[2].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Day: {} [{}]",
                parts[2], e
            )))
        }
    };

    let hour: u32 = match parts[3].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Hour: {} [{}]",
                parts[3], e
            )))
        }
    };
    let minute = match parts[4].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Minute: {} [{}]",
                parts[4], e
            )))
        }
    };

    let timestamp = match timezone.with_ymd_and_hms(
        year,
        month,
        day,
        hour - 1,
        match minute == 60 {
            true => 0,
            false => minute,
        },
        0,
    ) {
        Single(val) => val,
        _ => {
            return Err(EPWParseError::Data(format!(
                "Invalid Timestamp: {}-{}-{} {}:{}:00",
                year, month, day, hour, minute
            )))
        }
    };

    let dry_bulb_temperature = _parse_float_value(parts[6], "Dry Bulb Temperature", 99.9)?;
    let dew_point_temperature = _parse_float_value(parts[7], "Dew Point Temperature", 99.9)?;
    let relative_humidity = _parse_float_value(parts[8], "Relative Humidity", 999.)?;
    let atmospheric_pressure = _parse_float_value(parts[9], "Atmospheric Pressure", 999999.)?;
    let extraterrestrial_horizontal_radiation =
        _parse_float_value(parts[10], "Extraterrestrial Horizontal Radiation", 9999.)?;
    let extraterrestrial_direct_normal_radiation =
        _parse_float_value(parts[11], "Extraterrestrial Direct Normal Radiation", 9999.)?;
    let horizontal_infrared_radiation_intensity =
        _parse_float_value(parts[12], "Horizontal Infrared Radiation Intensity", 9999.)?;
    let global_horizontal_radiation =
        _parse_float_value(parts[13], "Global Horizontal Radiation", 9999.)?;
    let direct_normal_radiation = _parse_float_value(parts[14], "Direct Normal Radiation", 9999.)?;
    let diffuse_horizontal_radiation =
        _parse_float_value(parts[15], "Diffuse Horizontal Radiation", 9999.)?;

    let global_horizontal_illuminance = match parts[16].parse() {
        Ok(val) => match val < 999900. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Global Horizontal Illuminance: {}",
                e
            )))
        }
    };

    let direct_normal_illuminance = match parts[17].parse() {
        Ok(val) => match val < 999900. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Direct Normal Illuminance: {}",
                e
            )))
        }
    };

    let diffuse_horizontal_illuminance = match parts[18].parse() {
        Ok(val) => match val < 999900. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Diffuse Horizontal Illuminance: {}",
                e
            )))
        }
    };

    let zenith_luminance = _parse_float_value(parts[19], "Zenith Luminance", 9999.)?;
    let wind_direction = _parse_float_value(parts[20], "Wind Direction", 999.)?;
    let wind_speed = _parse_float_value(parts[21], "Wind Speed", 999.)?;
    let total_sky_cover = _parse_float_value(parts[22], "Total Sky Cover", 99.)?;
    let opaque_sky_cover = _parse_float_value(parts[23], "Opaque Sky Cover", 99.)?;
    let visibility = _parse_float_value(parts[24], "Visibility", 9999.)?;
    let ceiling_height = _parse_float_value(parts[25], "Ceiling Height", 99999.)?;

    let present_weather = PresentWeather::parse(parts[27])?;

    let precipitable_water = _parse_float_value(parts[28], "Precipitable water", 999.)?;
    let aerosol_optical_depth = _parse_float_value(parts[29], "Aerosol Optical Depth", 999.)?;
    let snow_depth = _parse_float_value(parts[30], "Snow Depth", 999.)?;
    let days_since_last_snowfall = _parse_float_value(parts[31], "Days Since Last Snowfall", 99.)?;

    let albedo = match parts.len() > 32 {
        true => _parse_float_value(parts[32], "Albedo", 999.)?,
        false => f64::NAN,
    };

    let liquid_precipitation_depth = match parts.len() > 33 {
        true => _parse_float_value(parts[33], "Liquid Precipitation Depth", 999.)?,
        false => f64::NAN,
    };

    let liquid_precipitation_quantity = match parts.len() > 34 {
        true => _parse_float_value(parts[34], "Liquid Precipitation Quantity", 999.)?,
        false => f64::NAN,
    };

    dest.timestamp.push(timestamp);
    dest.flags.push(parts[5].to_string());
    dest.dry_bulb_temperature.push(dry_bulb_temperature);
    dest.dew_point_temperature.push(dew_point_temperature);
    dest.relative_humidity.push(relative_humidity);
    dest.atmospheric_pressure.push(atmospheric_pressure);
    dest.extraterrestrial_horizontal_radiation
        .push(extraterrestrial_horizontal_radiation);
    dest.extraterrestrial_direct_normal_radiation
        .push(extraterrestrial_direct_normal_radiation);
    dest.horizontal_infrared_radiation_intensity
        .push(horizontal_infrared_radiation_intensity);
    dest.global_horizontal_radiation
        .push(global_horizontal_radiation);
    dest.direct_normal_radiation.push(direct_normal_radiation);
    dest.diffuse_horizontal_radiation
        .push(diffuse_horizontal_radiation);
    dest.global_horizontal_illuminance
        .push(global_horizontal_illuminance);
    dest.direct_normal_illuminance
        .push(direct_normal_illuminance);
    dest.diffuse_horizontal_illuminance
        .push(diffuse_horizontal_illuminance);
    dest.zenith_luminance.push(zenith_luminance);
    dest.wind_direction.push(wind_direction);
    dest.wind_speed.push(wind_speed);
    dest.total_sky_cover.push(total_sky_cover);
    dest.opaque_sky_cover.push(opaque_sky_cover);
    dest.visibility.push(visibility);
    dest.ceiling_height.push(ceiling_height);
    dest.present_weather_observation.push(parts[26] == "0");

    dest.present_weather_codes.push(present_weather);
    dest.precipitable_water.push(precipitable_water);
    dest.aerosol_optical_depth.push(aerosol_optical_depth);
    dest.snow_depth.push(snow_depth);
    dest.days_since_last_snowfall.push(days_since_last_snowfall);
    dest.albedo.push(albedo);
    dest.liquid_precipitation_depth
        .push(liquid_precipitation_depth);
    dest.liquid_precipitation_quantity
        .push(liquid_precipitation_quantity);
    Ok(())
}

fn _parse_present_weather(condition_str: &str) -> Result<PresentWeather, EPWParseError> {
    let thunderstorm = match condition_str[0..1].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let rain = match condition_str[1..2].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let rain_squalls = match condition_str[2..3].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let snow = match condition_str[3..4].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let snow_showers = match condition_str[4..5].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let sleet = match condition_str[5..6].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let fog = match condition_str[6..7].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let smoke = match condition_str[7..8].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    let ice_pellets = match condition_str[8..9].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Conditions: {} [{}]",
                &condition_str, e
            )))
        }
    };

    Ok(PresentWeather {
        thunderstorm,
        rain,
        rain_squalls,
        snow,
        snow_showers,
        sleet,
        fog,
        smoke,
        ice_pellets,
    })
}

fn _parse_float_value(value: &str, name: &str, missing_value: f64) -> Result<f64, EPWParseError> {
    let value = match value.parse() {
        Ok(val) => match val != missing_value {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid {} value: {}",
                name, e
            )))
        }
    };
    Ok(value)
}

/// Functionalitu included in the `polars` feature
///
/// This module includes a single additional method: [WeatherData::to_dataframe] which returns the
/// WeatherData instance as a new polars DataFrame.
#[cfg(feature = "polars")]
pub mod polars_ext {

    use super::*;
    use polars::prelude::*;

    impl WeatherData {
        ///Return the WeatherData instance as a polars [DataFrame].
        ///
        /// **Note: Requires the `polars` feature to be enabled.**
        ///
        /// The column names in the generated DataFrame match the attribute names, with the exception
        /// of `present_weather_codes`, which is flattened. The [PresentWeather] attributes are
        /// given their own columns named like `present_<attribute name>`: `"present_thunderstorm"`,
        /// `"present_rain"`, etc.
        pub fn to_dataframe(&self) -> Result<DataFrame, PolarsError> {
            let millisecond_timestamps: Vec<i64> = self
                .timestamp
                .iter()
                .map(|dt| dt.timestamp_millis())
                .collect();
            let timestamp = Series::new("timestamp".into(), millisecond_timestamps)
                .cast(&DataType::Datetime(TimeUnit::Milliseconds, None))?;

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

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(feature = "polars")]
    use polars::frame::DataFrame;

    fn _get_test_data() -> WeatherData {
        let tz = FixedOffset::east_opt(0).unwrap();
        let ts = match tz.with_ymd_and_hms(1990, 1, 1, 0, 0, 0) {
            Single(val) => val,
            _ => panic!("Invalid timestamp"),
        };
        WeatherData {
            timestamp: vec![ts],
            flags: vec!["flags".to_string()],
            dry_bulb_temperature: vec![1.0],
            dew_point_temperature: vec![2.0],
            relative_humidity: vec![3.0],
            atmospheric_pressure: vec![4.0],
            extraterrestrial_horizontal_radiation: vec![5.0],
            extraterrestrial_direct_normal_radiation: vec![6.0],
            horizontal_infrared_radiation_intensity: vec![7.0],
            global_horizontal_radiation: vec![8.0],
            direct_normal_radiation: vec![9.0],
            diffuse_horizontal_radiation: vec![10.0],
            global_horizontal_illuminance: vec![11.0],
            direct_normal_illuminance: vec![12.0],
            diffuse_horizontal_illuminance: vec![13.0],
            zenith_luminance: vec![14.0],
            wind_direction: vec![15.0],
            wind_speed: vec![16.0],
            total_sky_cover: vec![17.0],
            opaque_sky_cover: vec![18.0],
            visibility: vec![19.0],
            ceiling_height: vec![20.0],
            present_weather_observation: vec![true],
            present_weather_codes: vec![PresentWeather {
                thunderstorm: 0,
                rain: 1,
                rain_squalls: 2,
                snow: 3,
                snow_showers: 4,
                sleet: 5,
                fog: 6,
                smoke: 7,
                ice_pellets: 8,
            }],
            precipitable_water: vec![21.0],
            aerosol_optical_depth: vec![22.0],
            snow_depth: vec![23.0],
            days_since_last_snowfall: vec![24.0],
            albedo: vec![25.0],
            liquid_precipitation_depth: vec![26.0],
            liquid_precipitation_quantity: vec![27.0],
        }
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_to_dataframe_returns_appropriate_columns() {
        let expected_columns = [
            "timestamp",
            "dry_bulb_temperature",
            "dew_point_temperature",
            "relative_humidity",
            "atmospheric_pressure",
            "extraterrestrial_horizontal_radiation",
            "extraterrestrial_direct_normal_radiation",
            "horizontal_infrared_radiation_intensity",
            "global_horizontal_radiation",
            "direct_normal_radiation",
            "diffuse_horizontal_radiation",
            "global_horizontal_illuminance",
            "direct_normal_illuminance",
            "diffuse_horizontal_illuminance",
            "zenith_luminance",
            "wind_direction",
            "wind_speed",
            "total_sky_cover",
            "opaque_sky_cover",
            "visibility",
            "ceiling_height",
            "present_weather_observation",
            "present_thunderstorm",
            "present_rain",
            "present_rain_squalls",
            "present_snow",
            "present_snow_showers",
            "present_sleet",
            "present_fog",
            "present_smoke",
            "present_ice_pellets",
            "precipitable_water",
            "aerosol_optical_depth",
            "snow_depth",
            "days_since_last_snowfall",
            "albedo",
            "liquid_precipitation_depth",
            "liquid_precipitation_quantity",
        ];
        let data = _get_test_data();
        let df = data.to_dataframe().unwrap();
        assert_eq!(df.shape(), (1, 38));
        let cols = df.get_column_names_str();
        expected_columns.iter().for_each(|col| {
            assert!(cols.contains(col), "Missing column: {}", col);
        })
    }

    #[cfg(feature = "polars")]
    #[test]
    fn test_to_dataframe_returns_appropriate_values() {
        let data = _get_test_data();
        let df = data.to_dataframe().unwrap();
        assert_eq!(1.0, _get_value_from_df(&df, "dry_bulb_temperature", 0));
        assert_eq!(2.0, _get_value_from_df(&df, "dew_point_temperature", 0));
        assert_eq!(3.0, _get_value_from_df(&df, "relative_humidity", 0));
        assert_eq!(4.0, _get_value_from_df(&df, "atmospheric_pressure", 0));
        assert_eq!(
            5.0,
            _get_value_from_df(&df, "extraterrestrial_horizontal_radiation", 0)
        );
        assert_eq!(
            6.0,
            _get_value_from_df(&df, "extraterrestrial_direct_normal_radiation", 0)
        );
        assert_eq!(
            7.0,
            _get_value_from_df(&df, "horizontal_infrared_radiation_intensity", 0)
        );
        assert_eq!(
            8.0,
            _get_value_from_df(&df, "global_horizontal_radiation", 0)
        );
        assert_eq!(9.0, _get_value_from_df(&df, "direct_normal_radiation", 0));
        assert_eq!(
            10.0,
            _get_value_from_df(&df, "diffuse_horizontal_radiation", 0)
        );
        assert_eq!(
            11.0,
            _get_value_from_df(&df, "global_horizontal_illuminance", 0)
        );
        assert_eq!(
            12.0,
            _get_value_from_df(&df, "direct_normal_illuminance", 0)
        );
        assert_eq!(
            13.0,
            _get_value_from_df(&df, "diffuse_horizontal_illuminance", 0)
        );
        assert_eq!(14.0, _get_value_from_df(&df, "zenith_luminance", 0));
        assert_eq!(15.0, _get_value_from_df(&df, "wind_direction", 0));
        assert_eq!(16.0, _get_value_from_df(&df, "wind_speed", 0));
        assert_eq!(17.0, _get_value_from_df(&df, "total_sky_cover", 0));
        assert_eq!(18.0, _get_value_from_df(&df, "opaque_sky_cover", 0));
        assert_eq!(19.0, _get_value_from_df(&df, "visibility", 0));
        assert_eq!(20.0, _get_value_from_df(&df, "ceiling_height", 0));
        assert_eq!(21.0, _get_value_from_df(&df, "precipitable_water", 0));
        assert_eq!(22.0, _get_value_from_df(&df, "aerosol_optical_depth", 0));
        assert_eq!(23.0, _get_value_from_df(&df, "snow_depth", 0));
        assert_eq!(24.0, _get_value_from_df(&df, "days_since_last_snowfall", 0));
        assert_eq!(25.0, _get_value_from_df(&df, "albedo", 0));
        assert_eq!(
            26.0,
            _get_value_from_df(&df, "liquid_precipitation_depth", 0)
        );
        assert_eq!(
            27.0,
            _get_value_from_df(&df, "liquid_precipitation_quantity", 0)
        );
    }

    #[cfg(feature = "polars")]
    fn _get_value_from_df(df: &DataFrame, column: &str, row: usize) -> f64 {
        df.column(column)
            .expect("Missing column")
            .f64()
            .expect("not a float")
            .get(row)
            .expect("Missing Row")
    }
}
