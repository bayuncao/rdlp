use lazy_static::lazy_static;
use rdlp::common_struct_trait;
use serde::Deserialize;
use std::fs;
use std::io;
use std::sync::{Arc, Mutex};

common_struct_trait!(Summary, name: String, date: String, version: String, author: String, license: String);
common_struct_trait!(Switch, enable: Vec<u32>, disable: Vec<u32>);
common_struct_trait!(Mask, symbol: String, length: u32, start_index: u32);
common_struct_trait!(Item, id: u32, name: String, category: String, description: String, level: u32, detect: Detect, filter: Filter, verify: Verify);
common_struct_trait!(Detect, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Filter, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Verify, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Rules, mask:Mask,switch: Switch, item: Vec<Item>);
common_struct_trait!(Logger, output: String, log_file_path: String, log_level: String, log_output: String);

common_struct_trait!(Conf, summary: Summary, logger:Logger,rules: Rules);

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
