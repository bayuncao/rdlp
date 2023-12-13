use rdlp::common_struct_trait;
use serde::Deserialize;
use std::fs;

common_struct_trait!(Summary, name: String, date: String, version: String, author: String, license: String);
common_struct_trait!(Switch, enable: Vec<u32>, disable: Vec<u32>);
common_struct_trait!(Item, id: u32, name: String);
common_struct_trait!(Rules, switch: Switch, item: Vec<Item>);
common_struct_trait!(Conf, summary: Summary, rules: Rules);

#[allow(dead_code)]
pub fn parse_config(toml_str: &str) -> Result<Conf, toml::de::Error> {
    toml::from_str(toml_str)
}

pub fn from_file(path: &str) -> Result<Conf, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config: Conf = toml::from_str(&contents)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let toml_str = fs::read_to_string("development.toml").expect("Could not read TOML file");

        let config = parse_config(&toml_str).unwrap();

        println!("Summary:");
        println!("Name: {}", config.summary.name);
        println!("Date: {}", config.summary.date);
        println!("Version: {}", config.summary.version);
        println!("Author: {}", config.summary.author);
        println!("License: {}", config.summary.license);

        println!("\nSwitch:");
        println!("Enable: {:?}", config.rules.switch.enable);
        println!("Disable: {:?}", config.rules.switch.disable);

        println!("\nItems:");
        for item in config.rules.item {
            println!("Id: {}, Name: {}", item.id, item.name);
        }
    }
}
