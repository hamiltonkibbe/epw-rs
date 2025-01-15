/*! Example demonstrating use of the polars feature to get weather data as a DataFrame!*/
use epw_rs::EPWFile;

fn main() {
    let mut epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
    let df = epw.get_dataframe().unwrap();
    println!("{}", df);
}