use crate::conf::conf::Item;
use crate::conf::conf::SINGLETON_CONF;
use std::result::Result;  // Changed from crate::result::Result
use crate::conf::conf::Logger;
use regex::bytes::Regex;
use std::sync::Arc;
use tokio::task;
use crate::Conf;
// Removed unused import: use crate::conf::conf::Filter;
use crate::conf::conf::Detect;
use crate::conf::conf::Verify;

pub struct Process {
    log: Logger,
    result: Result<(), String>,  // Assuming Result is a standard Result
}

pub struct Worker {
    conf: Arc<Conf>,  // No change needed here
    rule: Item,       // rule item in conf
    rule_type: i32,   // VALUE if there is no KReg and KDict
}

impl Worker {
    pub fn new() -> Result<Self, &'static str> {  // Changed return type to Result
        let conf = match SINGLETON_CONF.get_conf() {
            Ok(conf) => conf,
            Err(e) => {
                println!("Failed to get configuration: {}", e);
                return Err("Failed to get configuration");
            }
        };

        let rule = conf.rules.item[0].clone();  // Assuming Item implements Clone
        let rule_type = 0;  // Placeholder value for rule_type

        Ok(Self {
            conf,  // Directly using conf, no need for Arc::new
            rule,
            rule_type,
        })
    }

    pub fn detect(&self, text: &str) -> Vec<Item> {
        let mut matched_items = Vec::new();

        for item in &self.conf.rules.item {
            let detect = &item.detect;

            // Check regex matches
            for reg in &detect.reg {
                let regex = Regex::new(reg).unwrap(); // Handle errors as appropriate
        // Convert text to &str if it's &[u8]
        match std::str::from_utf8(text) {
            Ok(str_text) => {
                if regex.is_match(str_text) {
                    matched_items.push(item.clone()); // Assuming Item implements Clone
                    break;
                }
            }
            Err(e) => {
                eprintln!("Text is not valid UTF-8: {}", e);
                // Handle the error
            }
        }
                    matched_items.push(item.clone()); // Assuming Item implements Clone
                    break;
                }
            }

            // Check list matches
            for word in &detect.list {
                if text.contains(word) {
                    matched_items.push(item.clone()); // Assuming Item implements Clone
                    break;
                }
            }
        }

        matched_items
    }
}
