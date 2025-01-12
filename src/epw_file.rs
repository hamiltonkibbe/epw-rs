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
pub struct EPWFile {
    pub header: Header,
    pub data: WeatherData,
}

impl EPWFile {
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
    pub fn from_reader<R: BufRead>(reader: R) -> Result<Self, EPWParseError> {
        let mut lines = reader.lines();
        let header = parse_header(&mut lines)?;
        let data = _parse_data(&mut lines, &header)?;

        Ok(Self { header, data })
    }

    /// Create an EPWFile instance from a file path
    ///
    /// ## Parameters
    /// - `path`: Path to file on the filesystem
    ///
    /// ## Returns
    /// An initialized EPWFile or an EPWParseError
    pub fn from_path(path: &str) -> Result<Self, EPWParseError> {
        let f = match File::open(path) {
            Ok(val) => val,
            Err(e) => return Err(EPWParseError::FileNotFound(e.to_string())),
        };

        let reader = BufReader::new(f);
        Self::from_reader(reader)
    }
}

fn _parse_data<R: BufRead>(
    lines: &mut Lines<R>,
    header: &Header,
) -> Result<WeatherData, EPWParseError> {
    // TODO: Don't panic
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

    let dry_bulb_temperature = match parts[6].parse() {
        Ok(val) => match val != 99.9 {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Dry Bulb Temperature: {}",
                e
            )))
        }
    };

    let dew_point_temperature = match parts[7].parse() {
        Ok(val) => match val != 99.9 {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Dew Point Temperature: {}",
                e
            )))
        }
    };

    let relative_humidity = match parts[8].parse() {
        Ok(val) => match val != 999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Relative Humidity: {}",
                e
            )))
        }
    };

    let atmospheric_pressure = match parts[9].parse() {
        Ok(val) => match val != 999999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Atmospheric Pressure: {}",
                e
            )))
        }
    };

    let extraterrestrial_horizontal_radiation = match parts[10].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Extraterrestrial Horizontal Radiation: {}",
                e
            )))
        }
    };

    let extraterrestrial_direct_normal_radiation = match parts[11].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Extraterrestrial Direct Normal Radiation: {}",
                e
            )))
        }
    };

    let horizontal_infrared_radiation_intensity = match parts[12].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Horizontal Infrared Radiation Intensity: {}",
                e
            )))
        }
    };

    let global_horizontal_radiation = match parts[13].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Global Horizontal Radiation: {}",
                e
            )))
        }
    };

    let direct_normal_radiation = match parts[14].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Direct Normal Radiation: {}",
                e
            )))
        }
    };

    let diffuse_horizontal_radiation = match parts[15].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Diffuse Horizontal Radiation: {}",
                e
            )))
        }
    };

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

    let zenith_luminance = match parts[19].parse() {
        Ok(val) => match val != 9999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Zenith Luminance: {}",
                e
            )))
        }
    };

    let wind_direction = match parts[20].parse() {
        Ok(val) => match val != 999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Wind Direction: {}",
                e
            )))
        }
    };

     let wind_speed = match parts[21].parse() {
        Ok(val) => match val != 999. {
            true => val,
            false => f64::NAN,
        },
        Err(e) => {
            return Err(EPWParseError::Data(format!(
                "Invalid Wind Speed: {}",
                e
            )))
        }
    };

    let present_weather = _parse_present_weather(parts[27])?;

    let albedo = match parts.len() > 32 {
        true => parts[32].parse().unwrap(),
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
    dest.total_sky_cover.push(parts[22].parse().unwrap());
    dest.opaque_sky_cover.push(parts[23].parse().unwrap());
    dest.visibility.push(parts[24].parse().unwrap());
    dest.ceiling_height.push(parts[25].parse().unwrap());
    dest.present_weather_observation.push(parts[26] == "0");

    dest.present_weather_codes.push(present_weather);
    dest.precipitable_water.push(parts[28].parse().unwrap());
    dest.aerosol_optical_depth.push(parts[29].parse().unwrap());
    dest.snow_depth.push(parts[30].parse().unwrap());
    dest.days_since_last_snowfall
        .push(parts[31].parse().unwrap());
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
