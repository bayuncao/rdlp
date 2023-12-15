use lazy_static::lazy_static;
use rdlp::common_struct_trait;
use serde::Deserialize;
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};

common_struct_trait!(Summary, name: String, date: String, version: String, author: String, license: String);
common_struct_trait!(Switch, enable: Vec<u32>, disable: Vec<u32>);
common_struct_trait!(Mask, symbol: String, length: u32, start_index: u32);
common_struct_trait!(Item, id: u32, name: String, category: String, desc: String, level: u32, detect: Detect, filter: Filter, verify: Verify);
common_struct_trait!(Detect, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Filter, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Verify, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Rules, mask:Mask,switch: Switch, item: Vec<Item>);
common_struct_trait!(Conf, summary: Summary, rules: Rules);

#[allow(dead_code)]
pub struct SingletonConf {
    conf: Mutex<Option<Arc<Conf>>>,
}

impl SingletonConf {
    pub fn new() -> Self {
        Self {
            conf: Mutex::new(None),
        }
    }
    #[allow(dead_code)]
    pub fn set_conf(&self, conf_path: &str) -> io::Result<()> {
        let conf_string = fs::read_to_string(conf_path)?;
        let conf: Conf = toml::from_str(&conf_string)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        *self.conf.lock().unwrap() = Some(Arc::new(conf));
        Ok(())
    }

    pub fn get_conf(&self) -> Result<Arc<Conf>, &'static str> {
        self.conf
            .lock()
            .unwrap()
            .as_ref()
            .cloned()
            .ok_or("Configuration not set")
    }
}

lazy_static! {
    pub static ref SINGLETON_CONF: SingletonConf = SingletonConf::new();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_singleton_conf() {
        // Set the path to your test configuration file
        let conf_path = "./development.toml";

        // Set the configuration
        SINGLETON_CONF.set_conf(conf_path).unwrap();

        // Get the configuration
        let conf = SINGLETON_CONF.get_conf().unwrap();

        // Print the configuration items
        // This assumes that your Conf struct has a method to print its items
        println!("Mask:");
        println!("Symbol: {}", conf.rules.mask.symbol);
        println!("Length: {}", conf.rules.mask.length);
        println!("Start Index: {}", conf.rules.mask.start_index);

        println!("\nSummary:");
        println!("Name: {}", conf.summary.name);
        println!("Date: {}", conf.summary.date);
        println!("Version: {}", conf.summary.version);
        println!("Author: {}", conf.summary.author);
        println!("License: {}", conf.summary.license);

        println!("\nSwitch:");
        println!("Enable: {:?}", conf.rules.switch.enable);
        println!("Disable: {:?}", conf.rules.switch.disable);

        println!("\nItems:");

        for item in &conf.rules.item {
            println!("\n\nID: {}", item.id);
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
        }
    }
       
}
