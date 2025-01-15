/*! Example usage of library demonstrating how to read an EPW File from the filesystem and
access both header data and weather data
!*/
use epw_rs::EPWFile;

fn main() {
    let parsed = EPWFile::from_path("./data/USA_FL_Tampa_TMY2.epw");
    match parsed {
        Ok(mut epw) => {
            println!("Location:        {}", epw.get_header().location);
            let data = epw.get_data().unwrap();
            let max_temp = match data
                .dry_bulb_temperature
                .clone()
                .into_iter()
                .reduce(f64::max)
            {
                Some(t) => t,
                None => panic!("Couldn't calculate max temperature"),
            };
            println!("Max Temperature: {:.2}°C", max_temp);
        }
        Err(e) => println!("{:?}", e),
    }
}
