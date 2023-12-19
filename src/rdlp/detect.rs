use std::sync::Arc;
use crate::conf::SINGLETON_CONF;
use regex::Regex;
use std::collections::HashSet;
use crate::conf::RuleItem;


pub struct Detector {
    conf: Arc<Conf>,
    rule: RuleItem, // rule item in conf
    rule_type: i32, // VALUE if there is no KReg and KDict
    // Detect section in conf
    k_reg: Vec<Regex>, // regex list for Key
    k_dict: HashSet<String>, // Dict for Key
    v_reg: Vec<Regex>, // Regex list for Value
    v_dict: Vec<String>, // Dict for Value
    // Filter section in conf
    b_algo: Vec<String>, // algorithm for blacklist, supports MASKED
    b_dict: Vec<String>, // Dict for blacklist
    b_reg: Vec<Regex>, // Regex list for blacklist
    // Verify section in conf
    c_dict: Vec<String>, // Dict for Context Verification
    c_reg: Vec<Regex>, // Regex List for Context Verification
    v_algo: Vec<String>, // algorithm for Verification, such as IDCARD
}

impl Detector {
    pub fn new() -> Self {
        let conf = match SINGLETON_CONF.get_conf() {
            Ok(conf) => conf,
            Err(e) => {
                println!("Failed to get configuration: {}", e);
                return;
            }
        };

        Self {
            conf,
        }
    }

    pub fn detect(&self, data: &str) -> Result<(), &'static str> {
        // Implement your detection logic here
        // This is where you would use the configuration and the input data
        // to perform some kind of detection

        Ok(())
    }
}