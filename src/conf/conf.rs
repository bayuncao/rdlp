use rdlp::common_struct_trait;
use serde::Deserialize;
use std::fs;

common_struct_trait!(Summary, name: String, date: String, version: String, author: String, license: String);
common_struct_trait!(Switch, enable: Vec<u32>, disable: Vec<u32>);
common_struct_trait!(Item, id: u32, name: String, category: String, desc: String, level: u32, detect: Detect, filter: Filter, verify: Verify);
common_struct_trait!(Detect, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Filter, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Verify, reg: Vec<String>, list: Vec<String>);
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
        let toml_str = fs::read_to_string("./development.toml").expect("Could not read TOML file");

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
            println!("ID: {}", item.id);
            println!("Name: {}", item.name);
            println!("Category: {}", item.category);
            println!("Desc: {}", item.desc);
            println!("Level: {}", item.level);

            println!("\nDetect:");
            println!("Reg: {:?}", item.detect.reg);
            println!("List: {:?}", item.detect.list);

            println!("\nFilter:");
            println!("Reg: {:?}", item.filter.reg);
            println!("List: {:?}", item.filter.list);

            println!("\nVerify:");
            println!("Reg: {:?}", item.verify.reg);
            println!("List: {:?}", item.verify.list);
        };
        

  

    }
}
