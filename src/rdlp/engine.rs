use logger::Logger;
use result::Result;
use std::collections::HashMap;
use crate::conf::RuleItem;
use crate::conf::SINGLETON_CONF;
use regex::Regex;
use std::collections::HashSet;
use std::sync::Arc;
pub struct Process {
    log: Logger,
    result: Result,
}

common_struct_trait!(Detect, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Filter, reg: Vec<String>, list: Vec<String>);
common_struct_trait!(Verify, reg: Vec<String>, list: Vec<String>);

impl Process {
    pub fn detect(&self, input_text: &str) -> Result<Vec<DetectResult>, &'static str> {}

    pub fn mask(&self, input_text: &str, method_name: &str) -> Result<String, &'static str> {}

    pub fn verify(&self, input_text: &str) -> Result<bool, &'static str> {}

    pub fn close(&self) {}
}



pub struct Worker {
    conf: Arc<Conf>,
    rule: RuleItem, // rule item in conf
    rule_type: i32, // VALUE if there is no KReg and KDict
    // Detect section in conf
    k_reg: Vec<Regex>,       // regex list for Key
    k_dict: HashSet<String>, // Dict for Key
    v_reg: Vec<Regex>,       // Regex list for Value
    v_dict: Vec<String>,     // Dict for Value
    // Filter section in conf
    b_algo: Vec<String>, // algorithm for blacklist, supports MASKED
    b_dict: Vec<String>, // Dict for blacklist
    b_reg: Vec<Regex>,   // Regex list for blacklist
    // Verify section in conf
    c_dict: Vec<String>, // Dict for Context Verification
    c_reg: Vec<Regex>,   // Regex List for Context Verification
    v_algo: Vec<String>, // algorithm for Verification, such as IDCARD
}

impl Worker {
    pub fn new() -> Self {
        let conf = match SINGLETON_CONF.get_conf() {
            Ok(conf) => conf,
            Err(e) => {
                println!("Failed to get configuration: {}", e);
                return;
            }
        };

        Self { conf }
    }

    pub fn detect(&self, data: &str) -> Result<(), &'static str> {
        // Implement your detection logic here
        // This is where you would use the configuration and the input data
        // to perform some kind of detection

        Ok(())
    }
}
