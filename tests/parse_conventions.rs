//! Header and record conventions of the EPW format that the shipped Tampa file does
//! not exercise: fractional time zones, sub-hourly records and the missing-value
//! markers of the last columns.

use chrono::{FixedOffset, TimeDelta};
use epw_rs::EPWFile;
use std::io::Cursor;

const TAMPA: &str = include_str!("../data/USA_FL_Tampa_TMY2.epw");

/// The Tampa header with another time zone and records per hour, followed by `rows`.
fn epw(time_zone: &str, records_per_hour: usize, rows: &[String]) -> String {
    let mut lines: Vec<String> = TAMPA.lines().take(8).map(str::to_string).collect();
    lines[0] = format!("LOCATION,TAMPA,FL,USA,TMY2-12842,722110,27.97,-82.53,{time_zone},3.0");
    lines[7] = format!("DATA PERIODS,1,{records_per_hour},Data,Sunday, 1/ 1,12/31");
    lines.extend_from_slice(rows);
    lines.join("\n")
}

/// The first Tampa record with another hour, minute and last five columns.
fn row(hour: u32, minute: u32, tail: &str) -> String {
    format!(
        "1987,1,1,{hour},{minute},A7A7A7A7*0?0?0?0?0?0?0?0A7A7A7A7A7A7F8F8A7E7,20.6,18.9,90,\
         100300,0,0,372,0,0,0,0,0,0,0,160,6.2,4,4,16.1,77777,0,999999999,33,{tail}"
    )
}

fn parse(text: &str) -> EPWFile {
    EPWFile::from_reader(Cursor::new(text)).expect("valid EPW")
}

#[test]
fn fractional_time_zone_is_kept() {
    // India (+5.5), Newfoundland (-3.5), Adelaide (+9.5) and Nepal (+5.75).
    for (zone, seconds) in [
        ("5.5", 19_800),
        ("-3.5", -12_600),
        ("9.5", 34_200),
        ("5.75", 20_700),
    ] {
        let file = parse(&epw(zone, 1, &[row(1, 60, "0.0620,0,88,0.2,0,0")]));
        assert_eq!(
            file.header.location.time_zone,
            FixedOffset::east_opt(seconds).unwrap(),
            "time zone {zone}"
        );
    }
}

#[test]
fn sub_hourly_records_get_increasing_timestamps() {
    let rows: Vec<String> = [
        (1, 15),
        (1, 30),
        (1, 45),
        (1, 60),
        (2, 15),
        (2, 30),
        (2, 45),
        (2, 60),
    ]
    .iter()
    .map(|&(hour, minute)| row(hour, minute, "0.0620,0,88,0.2,0,0"))
    .collect();
    let file = parse(&epw("-5.0", 4, &rows));
    let stamps = &file.data.timestamp;
    assert_eq!(stamps.len(), 8);
    for pair in stamps.windows(2) {
        assert_eq!(pair[1] - pair[0], TimeDelta::minutes(15), "{stamps:?}");
    }
}

#[test]
fn hourly_timestamps_are_unchanged() {
    let file = parse(&epw(
        "-5.0",
        1,
        &[
            row(1, 60, "0.0620,0,88,0.2,0,0"),
            row(2, 60, "0.0620,0,88,0.2,0,0"),
        ],
    ));
    let stamps: Vec<String> = file.data.timestamp.iter().map(|t| t.to_rfc3339()).collect();
    assert_eq!(
        stamps,
        ["1987-01-01T00:00:00-05:00", "1987-01-01T01:00:00-05:00"]
    );
}

#[test]
fn missing_markers_of_the_last_columns_read_as_nan() {
    // Aerosol optical depth is missing at .999, liquid precipitation depth at 999
    // and its quantity at 99 (EnergyPlus Auxiliary Programs, EPW data dictionary).
    let file = parse(&epw("-5.0", 1, &[row(1, 60, ".999,0,88,0.2,999,99")]));
    assert!(file.data.aerosol_optical_depth[0].is_nan());
    assert!(file.data.liquid_precipitation_depth[0].is_nan());
    assert!(file.data.liquid_precipitation_quantity[0].is_nan());
}

#[test]
fn malformed_liquid_precipitation_is_an_error() {
    for tail in ["0.0620,0,88,0.2,,0", "0.0620,0,88,0.2,0,x"] {
        let text = epw("-5.0", 1, &[row(1, 60, tail)]);
        assert!(EPWFile::from_reader(Cursor::new(text)).is_err(), "{tail}");
    }
}
