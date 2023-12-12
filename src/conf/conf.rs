use std::fs;
use serde::Deserialize;


#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Conf {
    pub setting1: String,
    pub setting2: i32,
    // Add more settings as needed
}


pub fn from_file(path: &str) -> Result<Conf, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Conf = toml::from_str(&contents)?;
    Ok(config)
}