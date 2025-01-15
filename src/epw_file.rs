/*!
This module contains the definition for the [EPWFile] struct that the parsing API is built around.

It implements two important methods, [EPWFile::from_path] and [EPWFile::from_reader], which handle
parsing the specified file, or provided file content.

*/
use crate::error::EPWParseError;
use crate::{Header, WeatherData};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

#[cfg(feature="polars")]
use polars::frame::DataFrame;

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
        let header = Header::parse(&mut lines)?;
        Ok(Self { header, data: None, content: lines })
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_data(&mut self) -> Result<&WeatherData, EPWParseError> {
        if self.data.is_none() {
            let data = WeatherData::parse(&mut self.content, &self.header)?;
            self.data = Some(data);
        }
        Ok(self.data.as_ref().unwrap())
    }

    #[cfg(feature="polars")]
    pub fn get_dataframe(&mut self) -> Result<DataFrame, EPWParseError> {
        let data = self.get_data()?;
        match data.to_dataframe() {
            Ok(df) => Ok(df),
            Err(e) => Err(EPWParseError::Data(e.to_string())),
        }
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