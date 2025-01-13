/*!
This module contains the definition for the [EPWFile] struct that the parsing API is built around.

It implements two important methods, [EPWFile::from_path] and [EPWFile::from_reader], which handle
parsing the specified file, or provided file content.

*/
use crate::error::EPWParseError;
use crate::header::parse_header;
use crate::weather_data::PresentWeather;
use crate::{Header, WeatherData};
use chrono::LocalResult::Single;
use chrono::{FixedOffset, TimeZone};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

/// EPWFile is the representation of the parsed file
///
/// it has only two attributes, `header` which is an instance of the [Header] struct,
/// and `data` which contains the weather data in a [WeatherData] struct.
#[derive(Debug)]
pub struct EPWFile<R: BufRead> {
    header: Header,
    data: Option<WeatherData>,
    content: Lines<R>,
}

impl<R: BufRead> EPWFile<R> {
    /// Construct an EPWFile instance from a buffered reader.
    /// ## Type Parameters
    ///  - `R`: the type of the reader
    ///
    /// ## Parameters
    /// - `reader`:  Reader that returns file contents.
    ///
    /// ## Returns
    /// An initialized EPWReader or an EPWParseError
    ///
    pub fn from_reader(reader: R) -> Result<Self, EPWParseError> {
        let mut lines = reader.lines();
        let header = parse_header(&mut lines)?;
        //let data = _parse_data(&mut lines, &header)?;

        Ok(Self { header, data: None, content: lines })
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_data(&mut self) -> Result<&WeatherData, EPWParseError> {
        if self.data.is_none() {
            let data = _parse_data(&mut self.content, &self.header)?;
            self.data = Some(data);
        }
        Ok(self.data.as_ref().unwrap())
    }
}

impl EPWFile<BufReader<File>> {
    /// Create an EPWFile instance from a file path
    ///
    /// ## Parameters
    /// - `path`: Path to file on the filesystem
    ///
    /// ## Returns
    /// An initialized EPWFile or an EPWParseError
    pub fn from_path<>(path: &str) -> Result<Self, EPWParseError> {
        let f = match File::open(path) {
            Ok(val) => val,
            Err(e) => return Err(EPWParseError::FileNotFound(e.to_string())),
        };

        let reader: BufReader<File> = BufReader::new(f);
        Self::from_reader(reader)
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

    let present_weather = _parse_present_weather(parts[27])?;

    let precipitable_water = _parse_float_value(parts[28], "Precipitable water", 999.)?;
    let aerosol_optical_depth = _parse_float_value(parts[29], "Aerosol Optical Depth", 999.)?;
    let snow_depth = _parse_float_value(parts[30], "Snow Depth", 999.)?;
    let days_since_last_snowfall = _parse_float_value(parts[31], "Days Since Last Snowfall", 99.)?;

    let albedo = match parts.len() > 32 {
        true => _parse_float_value(parts[32], "Albedo", 999.)?,
        false => f64::NAN,
    };

    let liquid_precipitation_depth = match parts.len() > 33 {
        true => parts[33].parse().unwrap(),
        false => f64::NAN,
    };

    let liquid_precipitation_quantity = match parts.len() > 34 {
        true => parts[34].parse().unwrap(),
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
