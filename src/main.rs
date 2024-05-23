use csv::Reader;
use csv::Writer;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Location {
    x: f64,
    y: f64,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Paths to the CSV files
    let first_csv_path = "zandvoort_data.csv";
    let second_csv_path = "zandvoort_led_coordinates.csv";
    let normalized_csv_path = "zandvoort_led_coordinates_normalized.csv";

    // Read the first dataset
    let mut first_rdr = Reader::from_path(first_csv_path)?;
    let first_locations: Vec<Location> = first_rdr.deserialize().collect::<Result<_, _>>()?;

    // Read the second dataset
    let mut second_rdr = Reader::from_path(second_csv_path)?;
    let mut second_locations: Vec<Location> = second_rdr.deserialize().collect::<Result<_, _>>()?;

    // Find the range of x and y coordinates in the first dataset
    let (x_min_first, x_max_first) = find_range(&first_locations, |loc| loc.x);
    let (y_min_first, y_max_first) = find_range(&first_locations, |loc| loc.y);

    // Find the range of x and y coordinates in the second dataset
    let (x_min_second, x_max_second) = find_range(&second_locations, |loc| loc.x);
    let (y_min_second, y_max_second) = find_range(&second_locations, |loc| loc.y);

    // Normalize the second dataset
    for loc in &mut second_locations {
        loc.x = normalize(loc.x, x_min_second, x_max_second, x_min_first, x_max_first);
        loc.y = normalize(loc.y, y_min_second, y_max_second, y_min_first, y_max_first);
    }

    // Write the normalized second dataset to a new CSV file
    let mut wtr = Writer::from_path(normalized_csv_path)?;
    for loc in &second_locations {
        wtr.serialize(loc)?;
    }
    wtr.flush()?;

    println!("Normalized dataset saved to {}", normalized_csv_path);

    Ok(())
}

fn find_range<F>(locations: &[Location], f: F) -> (f64, f64)
where
    F: Fn(&Location) -> f64,
{
    let min = locations.iter().map(&f).fold(f64::INFINITY, f64::min);
    let max = locations.iter().map(f).fold(f64::NEG_INFINITY, f64::max);
    (min, max)
}

fn normalize(value: f64, min_value: f64, max_value: f64, new_min: f64, new_max: f64) -> f64 {
    (value - min_value) / (max_value - min_value) * (new_max - new_min) + new_min
}
