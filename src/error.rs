#[derive(Debug)]
pub enum EPWParseError {
    FileNotFound(String),
    UnexpectedData(String),
    Location(String),
    GroundTemperature(String),
    HolidayDaylightSavings(String),
    DataPeriods(String),
    TypicalExtremePeriods(String),
    DesignConditions(String),
    Data(String),
}
