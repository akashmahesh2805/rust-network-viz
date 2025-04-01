use csv::Reader;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct SpeedData {
    pub timestamp: String,
    pub speed: f64,
}

pub fn load_data(filename: &str) -> Vec<SpeedData> {
    let path = Path::new(filename);
    if !path.exists() {
        eprintln!("Error: File {} does not exist", filename);
        return Vec::new();
    }

    match Reader::from_path(path) {
        Ok(mut rdr) => rdr
            .deserialize()
            .filter_map(|res| match res {
                Ok(record) => Some(record),
                Err(e) => {
                    eprintln!("Error parsing CSV row: {}", e);
                    None
                }
            })
            .collect(),
        Err(e) => {
            eprintln!("Error opening file {}: {}", filename, e);
            Vec::new()
        }
    }
}
