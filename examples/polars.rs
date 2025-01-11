/*! Example demonstrating use of the polars feature to get weather data as a DataFrame!*/
use epw_rs::EPWFile;

#[cfg(feature = "polars")]
fn main() {
    let epw = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw").unwrap();
    let df = epw.data.to_dataframe().unwrap();
    println!("{}", df);
}

#[cfg(not(feature = "polars"))]
fn main() {
    println!("This example requires `polars`. Ensure you installed with the `polars` feature and run with `--features polars` ");
}
