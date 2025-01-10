use crate::error::EPWParseError;
use chrono::FixedOffset;
use std::collections::VecDeque;
use std::io::{BufRead, Lines};

const LOCATION_KEY: &str = "LOCATION";
const DESIGN_CONDITIONS_KEY: &str = "DESIGN CONDITIONS";
const TYPICAL_EXTREME_PERIODS_KEY: &str = "TYPICAL/EXTREME PERIODS";

const GROUND_TEMPERATURES_KEY: &str = "GROUND TEMPERATURES";
const HOLIDAYS_DAYLIGHT_SAVINGS_KEY: &str = "HOLIDAYS/DAYLIGHT SAVINGS";
const COMMENTS_KEY: &str = "COMMENTS";
const DATA_PERIODS_KEY: &str = "DATA PERIODS";

#[derive(Debug)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug)]
pub struct Location {
    pub city: String,
    pub state_province_region: String,
    pub country: String,
    pub source: String,
    pub wmo: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time_zone: FixedOffset,
    pub elevation: f64,
}

#[derive(Debug)]
pub struct GroundTemperatureSample {
    pub depth: f64,
    pub soil_conductivity: Option<f64>,
    pub soil_density: Option<f64>,
    pub soil_specific_heat: Option<f64>,
    pub january: f64,
    pub february: f64,
    pub march: f64,
    pub april: f64,
    pub may: f64,
    pub june: f64,
    pub july: f64,
    pub august: f64,
    pub september: f64,
    pub october: f64,
    pub november: f64,
    pub december: f64,
}

#[derive(Debug)]
pub struct Holiday {
    pub date: String,
    pub name: String,
}
#[derive(Debug)]
pub struct HolidayDaylightSavings {
    pub leap_year: bool,
    pub daylight_savings_start: String,
    pub daylight_savings_end: String,
    pub holidays: Vec<Holiday>,
}

#[derive(Debug)]
pub struct DataPeriod {
    pub name: String,
    pub start_day_of_week: DayOfWeek,
    pub start_day: String,
    pub end_day: String,
}

#[derive(Debug)]
pub enum PeriodType {
    Typical,
    Extreme,
}

#[derive(Debug)]
pub struct TypicalExtremePeriod {
    name: String,
    period_type: PeriodType,
    start: String,
    end: String,
}

#[derive(Debug)]
pub struct DataPeriods {
    pub records_per_hour: usize,
    pub periods: Vec<DataPeriod>,
}

#[derive(Debug)]
pub struct Header {
    pub location: Location,
    pub design_conditions: Option<Vec<String>>,
    pub typical_extreme_periods: Vec<TypicalExtremePeriod>,
    pub ground_temperatures: Vec<GroundTemperatureSample>,
    pub holidays_daylight_savings: HolidayDaylightSavings,
    pub comments: Vec<String>,
    pub data_periods: DataPeriods,
}

pub fn parse_header<R: BufRead>(lines: &mut Lines<R>) -> Result<Header, EPWParseError> {
    let mut location: Option<Location> = None;
    let mut design_conditions: Option<Vec<String>> = None;
    let mut typical_extreme_periods: Option<Vec<TypicalExtremePeriod>> = None;
    let mut ground_temperature: Option<Vec<GroundTemperatureSample>> = None;
    let mut data_periods: Option<DataPeriods> = None;
    let mut holidays: Option<HolidayDaylightSavings> = None;
    let mut comments: Vec<String> = Vec::with_capacity(2);

    for line in lines.by_ref().take(8) {
        let line = line.expect("Unable to read line");
        if line.starts_with(LOCATION_KEY) {
            location = match _parse_location(&line) {
                Ok(val) => Some(val),
                Err(e) => {
                    return Err(e);
                }
            };
        } else if line.starts_with(GROUND_TEMPERATURES_KEY) {
            ground_temperature = match _parse_ground_temperature(&line) {
                Ok(val) => Some(val),
                Err(e) => {
                    return Err(e);
                }
            }
        } else if line.starts_with(DATA_PERIODS_KEY) {
            data_periods = match _parse_data_periods(&line) {
                Ok(val) => Some(val),
                Err(e) => {
                    return Err(e);
                }
            };
        } else if line.starts_with(TYPICAL_EXTREME_PERIODS_KEY) {
            typical_extreme_periods = match _parse_typical_extreme_periods(&line) {
                Ok(val) => Some(val),
                Err(e) => return Err(e),
            };
        } else if line.starts_with(HOLIDAYS_DAYLIGHT_SAVINGS_KEY) {
            holidays = match _parse_holiday_daylight_savings(&line) {
                Ok(val) => Some(val),
                Err(e) => {
                    return Err(e);
                }
            }
        } else if line.starts_with(COMMENTS_KEY) {
            comments.push(_parse_comment(&line));
        } else if line.starts_with(DESIGN_CONDITIONS_KEY) {
            design_conditions = Some(_parse_design_conditions(&line));
        } else {
            return Err(EPWParseError::UnexpectedData(format!(
                "Unexpected Row: {}",
                line
            )));
        }
    }

    Ok(Header {
        location: match location {
            Some(val) => val,
            None => return Err(EPWParseError::Location("No Location Found".to_string())),
        },
        ground_temperatures: match ground_temperature {
            Some(val) => val,
            None => {
                return Err(EPWParseError::GroundTemperature(
                    "No Ground Temperatures Found".to_string(),
                ))
            }
        },
        holidays_daylight_savings: match holidays {
            Some(val) => val,
            None => {
                return Err(EPWParseError::HolidayDaylightSavings(
                    "No Holidays/Daylight Savings Found".to_string(),
                ))
            }
        },
        data_periods: match data_periods {
            Some(val) => val,
            None => {
                return Err(EPWParseError::DataPeriods(
                    "No Data Periods Found".to_string(),
                ))
            }
        },
        typical_extreme_periods: match typical_extreme_periods {
            Some(val) => val,
            None => {
                return Err(EPWParseError::TypicalExtremePeriods(
                    "No Typical/Extreme Periods Found".to_string(),
                ))
            }
        },
        design_conditions,
        comments,
    })
}

fn _parse_location(line: &str) -> Result<Location, EPWParseError> {
    if !line.starts_with(LOCATION_KEY) {
        // This should never happen
        panic!("_parse_location called with a line that doesn't start with LOCATION");
    }
    let parts: Vec<&str> = line.split(",").collect();
    if parts.len() != 10 {
        return Err(EPWParseError::Location(format!(
            "Invalid Location Line: {}",
            line
        )));
    }

    let latitude = match parts[6].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Location(format!(
                "Invalid Latitude: {} [{}]",
                parts[6], e
            )))
        }
    };

    let longitude = match parts[7].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Location(format!(
                "Invalid Longitude: {} [{}]",
                parts[7], e
            )))
        }
    };

    let time_zone = match FixedOffset::east_opt(parts[8].parse::<f64>().unwrap() as i32 * 3600) {
        Some(val) => val,
        None => {
            return Err(EPWParseError::Location(format!(
                "Invalid Time Zone: {}",
                parts[8]
            )))
        }
    };

    let elevation = match parts[9].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::Location(format!(
                "Invalid Elevation: {} [{}]",
                parts[9], e
            )))
        }
    };

    Ok(Location {
        city: parts[1].to_string(),
        state_province_region: parts[2].to_string(),
        country: parts[3].to_string(),
        source: parts[4].to_string(),
        wmo: parts[5].to_string(),
        latitude,
        longitude,
        time_zone,
        elevation,
    })
}

fn _parse_ground_temperature(line: &str) -> Result<Vec<GroundTemperatureSample>, EPWParseError> {
    if !line.starts_with(GROUND_TEMPERATURES_KEY) {
        panic!("_parse_ground_temperature called with a line that doesn't start with GROUND TEMPERATURES");
    }

    let mut parts = line.split(",").collect::<Vec<&str>>();
    let sample_count: u16 = parts[1].parse().unwrap();
    let mut samples: Vec<GroundTemperatureSample> = Vec::with_capacity(sample_count as usize);
    let mut sample_data = parts.split_off(2);
    for idx in 0..sample_count {
        if sample_data.len() < 16 {
            return Err(EPWParseError::GroundTemperature(format!(
                "Not enough data for sample at index {}: {}",
                idx,
                sample_data.join(",")
            )));
        }

        let depth = match sample_data[0].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid Depth at index: {} {} [{}]",
                    idx, sample_data[0], e
                )))
            }
        };

        let january = match sample_data[4].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid January temp value at index: {} {} [{}]",
                    idx, sample_data[4], e
                )))
            }
        };

        let february = match sample_data[5].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid February temp value at index: {} {} [{}]",
                    idx, sample_data[5], e
                )))
            }
        };

        let march = match sample_data[6].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid March temp value at index: {} {} [{}]",
                    idx, sample_data[6], e
                )))
            }
        };

        let april = match sample_data[7].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid April temp value at index: {} {} [{}]",
                    idx, sample_data[7], e
                )))
            }
        };

        let may_value = match sample_data[8].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid May temp value at index: {} {} [{}]",
                    idx, sample_data[8], e
                )))
            }
        };

        let june = match sample_data[9].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid June temp value at index: {} {} [{}]",
                    idx, sample_data[9], e
                )))
            }
        };

        let july = match sample_data[10].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid July temp value at index: {} {} [{}]",
                    idx, sample_data[10], e
                )))
            }
        };

        let august = match sample_data[11].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid August temp value at index: {} {} [{}]",
                    idx, sample_data[11], e
                )))
            }
        };

        let september = match sample_data[12].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid September temp value at index: {} {} [{}]",
                    idx, sample_data[12], e
                )))
            }
        };

        let october = match sample_data[13].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid October temp value at index: {} {} [{}]",
                    idx, sample_data[13], e
                )))
            }
        };

        let november = match sample_data[14].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid November temp value at index: {} {} [{}]",
                    idx, sample_data[14], e
                )))
            }
        };

        let december = match sample_data[15].parse() {
            Ok(val) => val,
            Err(e) => {
                return Err(EPWParseError::GroundTemperature(format!(
                    "Invalid December temp value at index: {} {} [{}]",
                    idx, sample_data[15], e
                )))
            }
        };

        let sample = GroundTemperatureSample {
            depth,
            soil_conductivity: sample_data[1].parse().ok(),
            soil_density: sample_data[2].parse().ok(),
            soil_specific_heat: sample_data[3].parse().ok(),
            january,
            february,
            march,
            april,
            may: may_value,
            june,
            july,
            august,
            september,
            october,
            november,
            december,
        };
        samples.push(sample);
        sample_data = sample_data.split_off(16)
    }
    Ok(samples)
}

fn _parse_comment(line: &str) -> String {
    if !line.starts_with(COMMENTS_KEY) {
        panic!(
            "_parse_comment called with a line that doesn't start with {}",
            COMMENTS_KEY
        );
    }
    line.splitn(2, ",").collect::<Vec<&str>>()[1].to_string()
}
fn _parse_data_periods(line: &str) -> Result<DataPeriods, EPWParseError> {
    if !line.starts_with(DATA_PERIODS_KEY) {
        panic!(
            "_parse_data_periods called with a line that doesn't start with {}",
            DATA_PERIODS_KEY
        );
    }

    let mut parts = line.split(",").collect::<Vec<&str>>();

    let period_count = match parts[1].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::DataPeriods(format!(
                "Invalid period count: {} [{}]",
                parts[1], e
            )))
        }
    };

    let records_per_hour = match parts[2].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::DataPeriods(format!(
                "Invalid records per hour: {} [{}]",
                parts[2], e
            )))
        }
    };
    let mut periods: Vec<DataPeriod> = Vec::with_capacity(period_count);
    let mut period_data = parts.split_off(3);
    for idx in 0..period_count {
        if period_data.len() < 4 {
            return Err(EPWParseError::DataPeriods(format!(
                "Not enough data for period at index {}: {}",
                idx,
                period_data.join(",")
            )));
        }

        let start_day_of_week = match period_data[1] {
            "Sunday" => DayOfWeek::Sunday,
            "Monday" => DayOfWeek::Monday,
            "Tuesday" => DayOfWeek::Tuesday,
            "Wednesday" => DayOfWeek::Wednesday,
            "Thursday" => DayOfWeek::Thursday,
            "Friday" => DayOfWeek::Friday,
            "Saturday" => DayOfWeek::Saturday,
            e => {
                return Err(EPWParseError::DataPeriods(format!(
                    "Invalid day of week at index {}: {} [{}]",
                    idx, period_data[1], e
                )))
            }
        };

        let period = DataPeriod {
            name: period_data[0].to_string(),
            start_day_of_week,
            start_day: period_data[2].to_string(),
            end_day: period_data[3].to_string(),
        };
        periods.push(period);
        period_data = period_data.split_off(4)
    }
    Ok(DataPeriods {
        records_per_hour,
        periods,
    })
}

fn _parse_typical_extreme_periods(line: &str) -> Result<Vec<TypicalExtremePeriod>, EPWParseError> {
    if !line.starts_with(TYPICAL_EXTREME_PERIODS_KEY) {
        panic!(
            "_parse_typical_extreme_periods called with a line that doesn't start with {}",
            TYPICAL_EXTREME_PERIODS_KEY
        );
    }

    let mut parts = line.split(",").collect::<Vec<&str>>();

    let period_count = match parts[1].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::TypicalExtremePeriods(format!(
                "Invalid period count: {} [{}]",
                parts[1], e
            )))
        }
    };

    let mut periods: Vec<TypicalExtremePeriod> = Vec::with_capacity(period_count);
    let mut period_data = parts.split_off(2);
    for idx in 0..period_count {
        if period_data.len() < 4 {
            return Err(EPWParseError::TypicalExtremePeriods(format!(
                "Not enough data for period at index {}: {}",
                idx,
                period_data.join(",")
            )));
        }

        let name = period_data[0].to_string();
        let period_type = match period_data[1] {
            "Typical" => PeriodType::Typical,
            "Extreme" => PeriodType::Extreme,
            _ => {
                return Err(EPWParseError::TypicalExtremePeriods(format!(
                    "Invalid period type at index {}: {}",
                    idx, period_data[1]
                )))
            }
        };
        let start = period_data[2].to_string();
        let end = period_data[3].to_string();

        let period = TypicalExtremePeriod {
            name,
            period_type,
            start,
            end,
        };
        periods.push(period);
        period_data = period_data.split_off(4)
    }
    Ok(periods)
}

fn _parse_holiday_daylight_savings(line: &str) -> Result<HolidayDaylightSavings, EPWParseError> {
    if !line.starts_with(HOLIDAYS_DAYLIGHT_SAVINGS_KEY) {
        panic!(
            "_parse_holidays_daylight_savings called with a line that doesn't start with '{}'",
            HOLIDAYS_DAYLIGHT_SAVINGS_KEY
        );
    }

    let mut parts = line.split(",").collect::<Vec<&str>>();

    let leap_year = match parts[1] {
        "Yes" => true,
        "No" => false,
        _ => {
            return Err(EPWParseError::HolidayDaylightSavings(format!(
                "Invalid Leap Year Value: {}",
                parts[1]
            )))
        }
    };

    let daylight_savings_start = match parts[2].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::HolidayDaylightSavings(format!(
                "Invalid Daylight Savings Start Day: {} [{}]",
                parts[2], e
            )))
        }
    };

    let daylight_savings_end = match parts[3].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::HolidayDaylightSavings(format!(
                "Invalid Daylight Savings End Day: {} [{}]",
                parts[3], e
            )))
        }
    };

    let holiday_count = match parts[4].parse() {
        Ok(val) => val,
        Err(e) => {
            return Err(EPWParseError::HolidayDaylightSavings(format!(
                "Invalid holiday count: {} [{}]",
                parts[4], e
            )))
        }
    };

    let mut holidays: Vec<Holiday> = Vec::with_capacity(holiday_count);
    let mut holiday_data = parts.split_off(4);
    for idx in 0..holiday_count {
        if holiday_data.len() < 2 {
            return Err(EPWParseError::HolidayDaylightSavings(format!(
                "Not enough data for holiday at index {}: {}",
                idx,
                holiday_data.join(",")
            )));
        }

        holidays.push(Holiday {
            name: holiday_data[0].to_string(),
            date: holiday_data[1].to_string(),
        });
        holiday_data = holiday_data.split_off(2);
    }

    Ok(HolidayDaylightSavings {
        leap_year,
        daylight_savings_start,
        daylight_savings_end,
        holidays,
    })
}

fn _parse_design_conditions(line: &str) -> Vec<String> {
    let mut parts: VecDeque<&str> = line.split(",").collect();
    parts.pop_front();
    parts.into_iter().map(String::from).collect()
}