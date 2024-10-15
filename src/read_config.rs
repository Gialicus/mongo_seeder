use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use crate::types::Config;

pub fn read_config(file_path: &str) -> Result<Config, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
